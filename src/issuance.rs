use crate::empty_subtree::empty_subtree_root;
use crate::internal_node::hash_internal_node;
use crate::leaf_hash::{compute_occupied_leaf, compute_tombstoned_leaf, LeafHashError};
use crate::library::{
    CertificationProvenanceBlock, CoreTransitionEvidence, DeletionStateMaterial, Receipt,
    SemanticVersion,
};
use crate::proof_envelope::{parse_proof_envelope, serialize_proof_envelope, ProofEnvelope};
use crate::proof_frame::{ProofDirection, ProofFrame, ProofFrameSibling};
use crate::record_position::{compute_record_position_key, RecordPositionError};
use crate::state_commitment::{post_state_commitment, pre_state_commitment};
use crate::verifier::{validate_receipt, VerificationFailure};
use std::collections::BTreeMap;

type Position = [u8; 32];
type LeafHash = [u8; 32];
type Entry = (Position, LeafHash);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseIssuanceTree {
    committed_leaves: BTreeMap<Position, LeafHash>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuanceInputs<'a> {
    pub subject_reference: &'a [u8],
    pub scope_reference: Option<&'a [u8]>,
    pub transition_material: &'a [u8; 32],
    pub deletion_state_material: &'a [u8],
    pub certification_provenance: CertificationProvenanceBlock,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssuanceError {
    InvalidSubjectReference,
    InvalidScopeReference,
    InvalidDeletionStateMaterial(Vec<u8>),
    TargetAlreadyCommitted,
    ProofGenerationFailed(&'static str),
    ValidationFailed(VerificationFailure),
}

impl Default for SparseIssuanceTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseIssuanceTree {
    pub fn new() -> Self {
        Self {
            committed_leaves: BTreeMap::new(),
        }
    }

    pub fn committed_leaf_at(&self, position: &Position) -> Option<LeafHash> {
        self.committed_leaves.get(position).copied()
    }

    pub fn committed_leaves(&self) -> &BTreeMap<Position, LeafHash> {
        &self.committed_leaves
    }

    pub fn from_committed_leaves(committed_leaves: BTreeMap<Position, LeafHash>) -> Self {
        Self { committed_leaves }
    }

    pub fn insert_committed_leaf(
        &mut self,
        position: Position,
        leaf_hash: LeafHash,
    ) -> Result<(), IssuanceError> {
        if self.committed_leaves.contains_key(&position) {
            return Err(IssuanceError::TargetAlreadyCommitted);
        }

        self.committed_leaves.insert(position, leaf_hash);
        Ok(())
    }

    pub fn issue_unprovenanced_receipt(
        &mut self,
        inputs: IssuanceInputs<'_>,
    ) -> Result<Receipt, IssuanceError> {
        let target_position =
            compute_record_position_key(inputs.subject_reference, inputs.scope_reference)
                .map_err(map_record_position_error)?;

        self.issue_unprovenanced_receipt_at_position(target_position, &inputs)
    }

    fn issue_unprovenanced_receipt_at_position(
        &mut self,
        target_position: Position,
        inputs: &IssuanceInputs<'_>,
    ) -> Result<Receipt, IssuanceError> {
        if self.committed_leaves.contains_key(&target_position) {
            return Err(IssuanceError::TargetAlreadyCommitted);
        }

        let pre_state_leaf = compute_occupied_leaf(
            inputs.subject_reference,
            inputs.scope_reference,
            inputs.transition_material,
        )
        .map_err(map_leaf_hash_error)?;
        let post_state_leaf = compute_tombstoned_leaf(
            inputs.subject_reference,
            inputs.scope_reference,
            inputs.deletion_state_material,
        )
        .map_err(map_leaf_hash_error)?;

        let proof_envelope = self.build_proof_envelope_excluding_target(target_position);
        let tree_proof = serialize_proof_envelope(&proof_envelope);
        parse_proof_envelope(&tree_proof).map_err(|_| {
            IssuanceError::ProofGenerationFailed("generated_proof_envelope_invalid")
        })?;

        let pre_root = reconstruct_root_from_proof(pre_state_leaf, &proof_envelope);
        let post_root = reconstruct_root_from_proof(post_state_leaf, &proof_envelope);

        let receipt = Receipt {
            protocol_version: SemanticVersion {
                major: crate::PROTOCOL_VERSION.major,
                minor: crate::PROTOCOL_VERSION.minor,
                patch: crate::PROTOCOL_VERSION.patch,
            },
            receipt_version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            core_transition_evidence: CoreTransitionEvidence {
                subject_reference: inputs.subject_reference.to_vec(),
                scope_reference: inputs.scope_reference.map(|bytes| bytes.to_vec()),
                pre_state_commitment: pre_state_commitment(&pre_root).to_vec(),
                post_state_commitment: post_state_commitment(&post_root).to_vec(),
                transition_material: inputs.transition_material.to_vec(),
                transition_derivation_version: SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
                tree_proof,
                deletion_state_material: DeletionStateMaterial::TombstonedPosition(
                    inputs.deletion_state_material.to_vec(),
                ),
            },
            certification_provenance: inputs.certification_provenance.clone(),
        };

        validate_receipt(&receipt).map_err(IssuanceError::ValidationFailed)?;

        self.committed_leaves
            .insert(target_position, post_state_leaf);
        Ok(receipt)
    }

    fn build_proof_envelope_excluding_target(&self, target_position: Position) -> ProofEnvelope {
        let entries: Vec<Entry> = self
            .committed_leaves
            .iter()
            .filter(|(position, _)| **position != target_position)
            .map(|(position, leaf_hash)| (*position, *leaf_hash))
            .collect();

        ProofEnvelope {
            frames: std::array::from_fn(|frame_index| {
                let branch_level = 255 - frame_index;
                let target_bit = key_bit_from_root(&target_position, branch_level);
                let direction = if target_bit == 0 {
                    ProofDirection::Left
                } else {
                    ProofDirection::Right
                };

                let mut sibling_prefix = Vec::with_capacity(branch_level + 1);
                for level in 0..branch_level {
                    sibling_prefix.push(key_bit_from_root(&target_position, level));
                }
                sibling_prefix.push(1 - target_bit);

                let sibling_state = subtree_root_for_prefix(&entries, &sibling_prefix);
                let sibling = if sibling_state.is_empty {
                    ProofFrameSibling::CanonicalEmpty
                } else {
                    ProofFrameSibling::Explicit(sibling_state.root)
                };

                ProofFrame { direction, sibling }
            }),
        }
    }
}

pub fn issue_unprovenanced_receipt(
    tree: &mut SparseIssuanceTree,
    inputs: IssuanceInputs<'_>,
) -> Result<Receipt, IssuanceError> {
    tree.issue_unprovenanced_receipt(inputs)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SubtreeRoot {
    is_empty: bool,
    root: LeafHash,
}

fn map_record_position_error(error: RecordPositionError) -> IssuanceError {
    match error {
        RecordPositionError::EmptySubjectReference => IssuanceError::InvalidSubjectReference,
        RecordPositionError::EmptyScopeReference => IssuanceError::InvalidScopeReference,
    }
}

fn map_leaf_hash_error(error: LeafHashError) -> IssuanceError {
    match error {
        LeafHashError::EmptySubjectReference => IssuanceError::InvalidSubjectReference,
        LeafHashError::EmptyScopeReference => IssuanceError::InvalidScopeReference,
        LeafHashError::InvalidDeletionStateMaterial(bytes) => {
            IssuanceError::InvalidDeletionStateMaterial(bytes)
        }
    }
}

fn key_bit_from_root(key: &Position, level: usize) -> u8 {
    let byte_index = level / 8;
    let bit_index = 7 - (level % 8);
    (key[byte_index] >> bit_index) & 0x01
}

fn prefix_matches(position: &Position, prefix_bits: &[u8]) -> bool {
    prefix_bits
        .iter()
        .enumerate()
        .all(|(level, bit)| key_bit_from_root(position, level) == *bit)
}

fn subtree_root_for_prefix(entries: &[Entry], prefix_bits: &[u8]) -> SubtreeRoot {
    let filtered: Vec<Entry> = entries
        .iter()
        .copied()
        .filter(|(position, _)| prefix_matches(position, prefix_bits))
        .collect();

    if filtered.is_empty() {
        return SubtreeRoot {
            is_empty: true,
            root: empty_subtree_root(256 - prefix_bits.len()),
        };
    }

    SubtreeRoot {
        is_empty: false,
        root: compute_subtree_root(&filtered, prefix_bits.len()),
    }
}

fn compute_subtree_root(entries: &[Entry], level: usize) -> LeafHash {
    if entries.is_empty() {
        return empty_subtree_root(256 - level);
    }

    if level == 256 {
        return entries[0].1;
    }

    let mut left_entries = Vec::new();
    let mut right_entries = Vec::new();
    for (position, leaf_hash) in entries {
        if key_bit_from_root(position, level) == 0 {
            left_entries.push((*position, *leaf_hash));
        } else {
            right_entries.push((*position, *leaf_hash));
        }
    }

    let left_root = compute_subtree_root(&left_entries, level + 1);
    let right_root = compute_subtree_root(&right_entries, level + 1);
    hash_internal_node(&left_root, &right_root)
}

fn reconstruct_root_from_proof(leaf_hash: LeafHash, envelope: &ProofEnvelope) -> LeafHash {
    let mut current = leaf_hash;

    for (frame_index, frame) in envelope.frames.iter().enumerate() {
        let sibling_hash = match frame.sibling {
            ProofFrameSibling::Explicit(bytes) => bytes,
            ProofFrameSibling::CanonicalEmpty => empty_subtree_root(frame_index),
        };

        current = match frame.direction {
            ProofDirection::Left => hash_internal_node(&current, &sibling_hash),
            ProofDirection::Right => hash_internal_node(&sibling_hash, &current),
        };
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{CertificationProvenancePosture, CertificationProvenanceRoute};
    use crate::proof_direction_check::validate_proof_directions;
    use crate::record_position::record_position_key;

    fn no_payload_certification_provenance() -> CertificationProvenanceBlock {
        CertificationProvenanceBlock {
            posture: CertificationProvenancePosture::NoPayloadForRoute,
            route: CertificationProvenanceRoute::DirectInline,
            certification_material: None,
            provenance_material: None,
            route_context_material: None,
        }
    }

    fn issuance_inputs<'a>(
        subject_reference: &'a [u8],
        transition_material: &'a [u8; 32],
    ) -> IssuanceInputs<'a> {
        IssuanceInputs {
            subject_reference,
            scope_reference: None,
            transition_material,
            deletion_state_material: b"\x01",
            certification_provenance: no_payload_certification_provenance(),
        }
    }

    #[test]
    fn empty_tree_issuance_returns_receipt_that_validates() {
        let mut tree = SparseIssuanceTree::new();
        let subject_reference = [0x42; 32];
        let transition_material = [0x11; 32];

        let receipt = tree
            .issue_unprovenanced_receipt(issuance_inputs(&subject_reference, &transition_material))
            .expect("issuance should succeed on empty tree");

        assert_eq!(validate_receipt(&receipt), Ok(()));

        let target_position = record_position_key(&subject_reference, None);
        let expected_tombstoned_leaf =
            compute_tombstoned_leaf(&subject_reference, None, b"\x01").expect("test leaf");
        assert_eq!(
            tree.committed_leaf_at(&target_position),
            Some(expected_tombstoned_leaf)
        );

        assert!(parse_proof_envelope(&receipt.core_transition_evidence.tree_proof).is_ok());
    }

    #[test]
    fn issuance_with_prior_committed_leaf_uses_explicit_sibling_when_needed() {
        let mut tree = SparseIssuanceTree::new();
        let existing_subject_reference = [0x42; 32];
        let existing_position = record_position_key(&existing_subject_reference, None);
        let existing_leaf =
            compute_tombstoned_leaf(&existing_subject_reference, None, b"\x01").expect("leaf");
        tree.committed_leaves
            .insert(existing_position, existing_leaf);

        let new_subject_reference = [0x43; 32];
        let transition_material = [0x22; 32];
        let receipt = tree
            .issue_unprovenanced_receipt(issuance_inputs(
                &new_subject_reference,
                &transition_material,
            ))
            .expect("issuance should succeed with prior committed leaf");

        assert_eq!(validate_receipt(&receipt), Ok(()));

        let envelope = parse_proof_envelope(&receipt.core_transition_evidence.tree_proof)
            .expect("issued proof should parse");
        assert!(envelope
            .frames
            .iter()
            .any(|frame| { matches!(frame.sibling, ProofFrameSibling::Explicit(_)) }));
    }

    #[test]
    fn second_issuance_for_same_target_fails_loud() {
        let mut tree = SparseIssuanceTree::new();
        let subject_reference = [0x52; 32];
        let transition_material = [0x33; 32];

        tree.issue_unprovenanced_receipt(issuance_inputs(&subject_reference, &transition_material))
            .expect("first issuance should succeed");

        assert_eq!(
            tree.issue_unprovenanced_receipt(issuance_inputs(
                &subject_reference,
                &transition_material,
            )),
            Err(IssuanceError::TargetAlreadyCommitted)
        );
    }

    #[test]
    fn generated_proof_for_all_left_position_round_trips_and_matches_directions() {
        let tree = SparseIssuanceTree::new();
        let target_position = [0x00; 32];

        let envelope = tree.build_proof_envelope_excluding_target(target_position);
        let bytes = serialize_proof_envelope(&envelope);
        let parsed = parse_proof_envelope(&bytes).expect("proof should parse");

        assert_eq!(parsed, envelope);
        assert_eq!(validate_proof_directions(&parsed, &target_position), Ok(()));
        assert!(parsed.frames.iter().all(|frame| {
            frame.direction == ProofDirection::Left
                && matches!(frame.sibling, ProofFrameSibling::CanonicalEmpty)
        }));
    }

    #[test]
    fn generated_proof_for_all_right_position_round_trips_and_matches_directions() {
        let tree = SparseIssuanceTree::new();
        let target_position = [0xff; 32];

        let envelope = tree.build_proof_envelope_excluding_target(target_position);
        let bytes = serialize_proof_envelope(&envelope);
        let parsed = parse_proof_envelope(&bytes).expect("proof should parse");

        assert_eq!(parsed, envelope);
        assert_eq!(validate_proof_directions(&parsed, &target_position), Ok(()));
        assert!(parsed.frames.iter().all(|frame| {
            frame.direction == ProofDirection::Right
                && matches!(frame.sibling, ProofFrameSibling::CanonicalEmpty)
        }));
    }
}
