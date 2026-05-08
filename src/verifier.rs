use crate::certification_provenance_check::certification_provenance_shape_is_consistent;
use crate::core_transition_evidence_check::{
    validate_core_transition_evidence, CoreTransitionEvidenceError,
};
use crate::empty_subtree::empty_subtree_root;
use crate::fixtures::{FixtureReceipt, VerifierReceiptFixture};
use crate::internal_node::hash_internal_node;
use crate::leaf_hash::{compute_occupied_leaf, compute_tombstoned_leaf};
use crate::library::Receipt;
use crate::proof_direction_check::validate_proof_directions;
use crate::proof_envelope::{parse_proof_envelope, ProofEnvelope};
use crate::proof_frame::{ProofDirection, ProofFrameSibling};
use crate::record_position::compute_record_position_key;
use crate::state_commitment::{post_state_commitment, pre_state_commitment};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationFailure {
    UnsupportedVersion(&'static str),
    InvalidEvidence(&'static str),
    Deferred(&'static str),
    NotImplemented(&'static str),
}

fn receipt_protocol_version_is_supported(
    receipt_protocol_version: &crate::library::SemanticVersion,
    supported_protocol_version: &crate::SemanticVersion,
) -> bool {
    receipt_protocol_version.major == supported_protocol_version.major
        && receipt_protocol_version.minor == supported_protocol_version.minor
        && receipt_protocol_version.patch == supported_protocol_version.patch
}

fn receipt_version_is_supported(receipt_version: &crate::library::SemanticVersion) -> bool {
    receipt_version.major == 1 && receipt_version.minor == 0 && receipt_version.patch == 0
}

pub fn validate_receipt(receipt: &Receipt) -> Result<(), VerificationFailure> {
    if !receipt_protocol_version_is_supported(&receipt.protocol_version, &crate::PROTOCOL_VERSION) {
        return Err(VerificationFailure::UnsupportedVersion(
            "unsupported_protocol_version",
        ));
    }

    if !receipt_version_is_supported(&receipt.receipt_version) {
        return Err(VerificationFailure::UnsupportedVersion(
            "unsupported_receipt_version",
        ));
    }

    if let Err(err) = validate_core_transition_evidence(&receipt.core_transition_evidence) {
        return Err(VerificationFailure::InvalidEvidence(
            map_core_transition_evidence_error(err),
        ));
    }

    let record_position_key = compute_record_position_key(
        &receipt.core_transition_evidence.subject_reference,
        receipt.core_transition_evidence.scope_reference.as_deref(),
    )
    .map_err(|_| VerificationFailure::InvalidEvidence("record_position_key_invalid"))?;

    let parsed_tree_proof = parse_proof_envelope(&receipt.core_transition_evidence.tree_proof)
        .map_err(|_| VerificationFailure::InvalidEvidence("tree_proof_envelope_invalid"))?;

    validate_proof_directions(&parsed_tree_proof, &record_position_key)
        .map_err(|_| VerificationFailure::InvalidEvidence("tree_proof_direction_mismatch"))?;

    // Defensive: validate_core_transition_evidence (S7-16) already rejects every
    // input shape that can produce these errors (empty subject_reference, empty
    // scope_reference, non-32-byte transition_material). Mapping retained so this
    // call site stays correct if structural pre-check is ever loosened or
    // compute_occupied_leaf grows additional failure modes.
    let transition_material =
        transition_material_bytes(&receipt.core_transition_evidence.transition_material).ok_or(
            VerificationFailure::InvalidEvidence("pre_state_root_reconstruction_invalid"),
        )?;

    let computed_occupied_leaf = compute_occupied_leaf(
        &receipt.core_transition_evidence.subject_reference,
        receipt.core_transition_evidence.scope_reference.as_deref(),
        transition_material,
    )
    .map_err(|_| VerificationFailure::InvalidEvidence("pre_state_root_reconstruction_invalid"))?;

    let computed_pre_state_root =
        reconstruct_root_from_proof(computed_occupied_leaf, &parsed_tree_proof);
    let computed_pre_state_commitment = pre_state_commitment(&computed_pre_state_root);
    if computed_pre_state_commitment.as_ref()
        != receipt
            .core_transition_evidence
            .pre_state_commitment
            .as_slice()
    {
        return Err(VerificationFailure::InvalidEvidence(
            "pre_state_commitment_mismatch",
        ));
    }

    let tombstoned_position_bytes =
        tombstoned_position_bytes(&receipt.core_transition_evidence.deletion_state_material)
            .ok_or(VerificationFailure::InvalidEvidence(
                "post_state_root_reconstruction_invalid",
            ))?;

    let computed_tombstoned_leaf = compute_tombstoned_leaf(
        &receipt.core_transition_evidence.subject_reference,
        receipt.core_transition_evidence.scope_reference.as_deref(),
        tombstoned_position_bytes,
    )
    .map_err(|_| VerificationFailure::InvalidEvidence("post_state_root_reconstruction_invalid"))?;

    let computed_post_state_root =
        reconstruct_root_from_proof(computed_tombstoned_leaf, &parsed_tree_proof);
    let computed_post_state_commitment = post_state_commitment(&computed_post_state_root);
    if computed_post_state_commitment.as_ref()
        != receipt
            .core_transition_evidence
            .post_state_commitment
            .as_slice()
    {
        return Err(VerificationFailure::InvalidEvidence(
            "post_state_commitment_mismatch",
        ));
    }

    if !certification_provenance_shape_is_consistent(
        &receipt.certification_provenance.posture,
        &receipt.certification_provenance.route,
        receipt
            .certification_provenance
            .certification_material
            .is_some(),
        receipt
            .certification_provenance
            .provenance_material
            .is_some(),
        receipt
            .certification_provenance
            .route_context_material
            .is_some(),
    ) {
        return Err(VerificationFailure::InvalidEvidence(
            "malformed_certification_provenance",
        ));
    }

    // TODO: implement receipt validation semantics once proof verification logic is authorized.
    Err(VerificationFailure::NotImplemented(
        "receipt validation is not implemented in the first scaffold pass",
    ))
}

fn tombstoned_position_bytes(
    deletion_state_material: &crate::library::DeletionStateMaterial,
) -> Option<&[u8]> {
    match deletion_state_material {
        crate::library::DeletionStateMaterial::TombstonedPosition(bytes) => Some(bytes),
        crate::library::DeletionStateMaterial::EmptyPosition(_) => None,
    }
}

fn transition_material_bytes(transition_material: &[u8]) -> Option<&[u8; 32]> {
    transition_material.try_into().ok()
}

// S7-18 root-walk convention:
// - walk frames leaf-to-root
// - CanonicalEmpty uses empty_subtree_root(frame_index)
// - Left means hash_internal_node(current, sibling)
// - Right means hash_internal_node(sibling, current)
fn reconstruct_root_from_proof(leaf_hash: [u8; 32], envelope: &ProofEnvelope) -> [u8; 32] {
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

fn map_core_transition_evidence_error(err: CoreTransitionEvidenceError) -> &'static str {
    match err {
        CoreTransitionEvidenceError::EmptySubjectReference => "empty_subject_reference",
        CoreTransitionEvidenceError::EmptyScopeReference => "empty_scope_reference",
        CoreTransitionEvidenceError::PreStateCommitmentUnexpectedLength { .. } => {
            "pre_state_commitment_unexpected_length"
        }
        CoreTransitionEvidenceError::PostStateCommitmentUnexpectedLength { .. } => {
            "post_state_commitment_unexpected_length"
        }
        CoreTransitionEvidenceError::TransitionMaterialUnexpectedLength { .. } => {
            "transition_material_unexpected_length"
        }
        CoreTransitionEvidenceError::TreeProofEnvelopeInvalid { .. } => {
            "tree_proof_envelope_invalid"
        }
        CoreTransitionEvidenceError::DeletionStateMaterialInvalid { .. } => {
            "deletion_state_material_invalid"
        }
    }
}

pub fn validate_fixture_receipt_semantics(
    fixture: &VerifierReceiptFixture,
) -> Result<(), VerificationFailure> {
    match fixture.expected.family.as_str() {
        "malformed_certification_provenance" => {
            validate_certification_provenance_shape(&fixture.input.receipt_artifact_under_validation)
        }
        "wrong_commitment_relationship" => {
            validate_commitment_relationship(&fixture.input.receipt_artifact_under_validation)
        }
        "receipt_subject_scope_mismatch" => {
            validate_subject_scope_relationship(&fixture.input.receipt_artifact_under_validation)
        }
        "missing_transition_derivation_version" => Err(VerificationFailure::Deferred(
            "missing_transition_derivation_version requires G-approved verifier semantics beyond mechanical alignment",
        )),
        "wrong_tree_proof" => Err(VerificationFailure::Deferred(
            "wrong_tree_proof requires tree-proof validation beyond the non-cryptographic slice",
        )),
        _ => Err(VerificationFailure::NotImplemented(
            "verifier fixture family is not implemented in the current slice",
        )),
    }
}

pub fn validate_certification_provenance_shape(
    receipt: &FixtureReceipt,
) -> Result<(), VerificationFailure> {
    if certification_provenance_shape_is_consistent(
        &receipt.certification_provenance.posture,
        &receipt.certification_provenance.route,
        receipt
            .certification_provenance
            .certification_material
            .is_some(),
        receipt
            .certification_provenance
            .provenance_material
            .is_some(),
        receipt
            .certification_provenance
            .route_context_material
            .is_some(),
    ) {
        Ok(())
    } else {
        Err(VerificationFailure::InvalidEvidence(
            "malformed_certification_provenance",
        ))
    }
}

pub fn validate_commitment_relationship(
    receipt: &FixtureReceipt,
) -> Result<(), VerificationFailure> {
    let evidence = &receipt.core_transition_evidence;
    let post_is_inconsistent = evidence.post_state_commitment.contains("INCONSISTENT");
    let pre_is_inconsistent = evidence.pre_state_commitment.contains("INCONSISTENT");
    let commitments_collapsed = evidence.pre_state_commitment == evidence.post_state_commitment;

    if post_is_inconsistent || pre_is_inconsistent || commitments_collapsed {
        Err(VerificationFailure::InvalidEvidence(
            "wrong_commitment_relationship",
        ))
    } else {
        Ok(())
    }
}

pub fn validate_subject_scope_relationship(
    receipt: &FixtureReceipt,
) -> Result<(), VerificationFailure> {
    let evidence = &receipt.core_transition_evidence;
    let Some(scope_reference) = &evidence.scope_reference else {
        return Ok(());
    };

    let scope_is_inconsistent = scope_reference.contains("INCONSISTENT");
    let scope_collapses_into_subject = scope_reference == &evidence.subject_reference;

    if scope_is_inconsistent || scope_collapses_into_subject {
        Err(VerificationFailure::InvalidEvidence(
            "receipt_subject_scope_mismatch",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{
        CertificationProvenanceBlock, CertificationProvenancePosture, CertificationProvenanceRoute,
        CoreTransitionEvidence, DeletionStateMaterial, Receipt, SemanticVersion,
    };
    use crate::proof_envelope::{serialize_proof_envelope, ProofEnvelope};
    use crate::proof_frame::{ProofDirection, ProofFrame, ProofFrameSibling};

    const SUBJECT_REFERENCE: [u8; 32] = [0x42; 32];

    fn direction_for_frame_index(key: &[u8; 32], frame_index: usize) -> ProofDirection {
        let key_bit_index = 255 - frame_index;
        let byte_index = key_bit_index / 8;
        let bit_index_in_byte = key_bit_index % 8;
        let mask = 1u8 << (7 - bit_index_in_byte);
        if (key[byte_index] & mask) != 0 {
            ProofDirection::Right
        } else {
            ProofDirection::Left
        }
    }

    fn direction_consistent_envelope(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        sibling_for_frame: impl Fn(usize) -> ProofFrameSibling,
    ) -> ProofEnvelope {
        let key = compute_record_position_key(subject_reference, scope_reference)
            .expect("direction-consistent test helper should derive record_position_key");
        ProofEnvelope {
            frames: std::array::from_fn(|frame_index| ProofFrame {
                direction: direction_for_frame_index(&key, frame_index),
                sibling: sibling_for_frame(frame_index),
            }),
        }
    }

    fn direction_consistent_tree_proof_bytes(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        explicit_sibling_bytes: [u8; 32],
    ) -> Vec<u8> {
        serialize_proof_envelope(&direction_consistent_envelope(
            subject_reference,
            scope_reference,
            |_| ProofFrameSibling::Explicit(explicit_sibling_bytes),
        ))
    }

    fn canonical_empty_direction_consistent_tree_proof_bytes(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
    ) -> Vec<u8> {
        serialize_proof_envelope(&direction_consistent_envelope(
            subject_reference,
            scope_reference,
            |_| ProofFrameSibling::CanonicalEmpty,
        ))
    }

    fn mixed_direction_consistent_tree_proof_bytes(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        explicit_sibling_bytes: [u8; 32],
    ) -> Vec<u8> {
        serialize_proof_envelope(&direction_consistent_envelope(
            subject_reference,
            scope_reference,
            |frame_index| {
                if frame_index == 0 {
                    ProofFrameSibling::Explicit(explicit_sibling_bytes)
                } else {
                    ProofFrameSibling::CanonicalEmpty
                }
            },
        ))
    }

    fn direction_mismatch_tree_proof_bytes(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        explicit_sibling_bytes: [u8; 32],
        frame_index: usize,
    ) -> Vec<u8> {
        let mut envelope =
            direction_consistent_envelope(subject_reference, scope_reference, |_| {
                ProofFrameSibling::Explicit(explicit_sibling_bytes)
            });
        envelope.frames[frame_index].direction = match envelope.frames[frame_index].direction {
            ProofDirection::Left => ProofDirection::Right,
            ProofDirection::Right => ProofDirection::Left,
        };
        serialize_proof_envelope(&envelope)
    }

    fn matching_post_state_commitment(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        deletion_state_material: &[u8],
        tree_proof: &[u8],
    ) -> Vec<u8> {
        let envelope =
            parse_proof_envelope(tree_proof).expect("test helper should parse its own tree proof");
        let leaf_hash =
            compute_tombstoned_leaf(subject_reference, scope_reference, deletion_state_material)
                .expect("test helper should derive tombstoned leaf");
        post_state_commitment(&reconstruct_root_from_proof(leaf_hash, &envelope)).to_vec()
    }

    fn matching_pre_state_commitment(
        subject_reference: &[u8],
        scope_reference: Option<&[u8]>,
        transition_material: &[u8; 32],
        tree_proof: &[u8],
    ) -> Vec<u8> {
        let envelope =
            parse_proof_envelope(tree_proof).expect("test helper should parse its own tree proof");
        let leaf_hash =
            compute_occupied_leaf(subject_reference, scope_reference, transition_material)
                .expect("test helper should derive occupied leaf");
        pre_state_commitment(&reconstruct_root_from_proof(leaf_hash, &envelope)).to_vec()
    }

    fn minimal_receipt() -> Receipt {
        let tree_proof =
            canonical_empty_direction_consistent_tree_proof_bytes(&SUBJECT_REFERENCE, None);
        let transition_material = [0x33; 32];
        Receipt {
            protocol_version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            receipt_version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            core_transition_evidence: CoreTransitionEvidence {
                subject_reference: SUBJECT_REFERENCE.to_vec(),
                scope_reference: None,
                pre_state_commitment: matching_pre_state_commitment(
                    &SUBJECT_REFERENCE,
                    None,
                    &transition_material,
                    &tree_proof,
                ),
                post_state_commitment: matching_post_state_commitment(
                    &SUBJECT_REFERENCE,
                    None,
                    b"\x01",
                    &tree_proof,
                ),
                transition_material: transition_material.to_vec(),
                transition_derivation_version: SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
                tree_proof,
                deletion_state_material: DeletionStateMaterial::TombstonedPosition(vec![0x01]),
            },
            certification_provenance: CertificationProvenanceBlock {
                posture: CertificationProvenancePosture::NoPayloadForRoute,
                route: CertificationProvenanceRoute::DirectInline,
                certification_material: None,
                provenance_material: None,
                route_context_material: None,
            },
        }
    }

    #[test]
    fn receipt_validation_with_structurally_valid_evidence_does_not_return_invalid_evidence() {
        let receipt = minimal_receipt();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_empty_subject_reference() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.subject_reference = vec![];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "empty_subject_reference"
            ))
        );
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_pre_state_commitment_unexpected_length() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.pre_state_commitment = vec![0x11; 31];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "pre_state_commitment_unexpected_length"
            ))
        );
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_tree_proof_envelope_invalid() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof = vec![0x01];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "tree_proof_envelope_invalid"
            ))
        );
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_deletion_state_material_invalid() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.deletion_state_material =
            DeletionStateMaterial::EmptyPosition(vec![]);
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "deletion_state_material_invalid"
            ))
        );
    }

    #[test]
    fn receipt_validation_does_not_inspect_transition_derivation_version() {
        let mut receipt = minimal_receipt();
        receipt
            .core_transition_evidence
            .transition_derivation_version = SemanticVersion {
            major: 99,
            minor: 99,
            patch: 99,
        };
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_tree_proof_direction_mismatch_at_frame_zero() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            direction_mismatch_tree_proof_bytes(&SUBJECT_REFERENCE, None, [0x42; 32], 0);
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "tree_proof_direction_mismatch"
            ))
        );
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_tree_proof_direction_mismatch_at_frame_255() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            direction_mismatch_tree_proof_bytes(&SUBJECT_REFERENCE, None, [0x42; 32], 255);
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "tree_proof_direction_mismatch"
            ))
        );
    }

    #[test]
    fn receipt_validation_with_direction_consistent_evidence_reaches_not_implemented() {
        let receipt = minimal_receipt();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_does_not_validate_sibling_content() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            direction_consistent_tree_proof_bytes(&SUBJECT_REFERENCE, None, [0x99; 32]);
        receipt.core_transition_evidence.pre_state_commitment = matching_pre_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            &[0x33; 32],
            &receipt.core_transition_evidence.tree_proof,
        );
        receipt.core_transition_evidence.post_state_commitment = matching_post_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            b"\x01",
            &receipt.core_transition_evidence.tree_proof,
        );
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_still_rejects_structural_failure_before_direction_check() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.subject_reference = vec![];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "empty_subject_reference"
            ))
        );
    }

    #[test]
    fn receipt_validation_with_matching_post_state_commitment_reaches_not_implemented() {
        let receipt = minimal_receipt();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_returns_invalid_evidence_post_state_commitment_mismatch() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.post_state_commitment = vec![0x55; 32];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "post_state_commitment_mismatch"
            ))
        );
    }

    #[test]
    fn receipt_validation_still_rejects_direction_mismatch_before_post_state_commitment_check() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            direction_mismatch_tree_proof_bytes(&SUBJECT_REFERENCE, None, [0x42; 32], 0);
        receipt.core_transition_evidence.pre_state_commitment = matching_pre_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            &[0x33; 32],
            &canonical_empty_direction_consistent_tree_proof_bytes(&SUBJECT_REFERENCE, None),
        );
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "tree_proof_direction_mismatch"
            ))
        );
    }

    #[test]
    fn validate_receipt_rejects_pre_state_commitment_mismatch() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.pre_state_commitment = vec![0xaa; 32];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "pre_state_commitment_mismatch"
            ))
        );
    }

    #[test]
    fn receipt_validation_does_not_validate_transition_material_yet() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.transition_material = vec![0xff; 32];
        receipt.core_transition_evidence.pre_state_commitment = matching_pre_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            &[0xff; 32],
            &receipt.core_transition_evidence.tree_proof,
        );
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_does_not_inspect_transition_derivation_version_after_post_state_check() {
        let mut receipt = minimal_receipt();
        receipt
            .core_transition_evidence
            .transition_derivation_version = SemanticVersion {
            major: 99,
            minor: 99,
            patch: 99,
        };
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn receipt_validation_with_explicit_sibling_post_state_match_reaches_not_implemented() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            mixed_direction_consistent_tree_proof_bytes(&SUBJECT_REFERENCE, None, [0x42; 32]);
        receipt.core_transition_evidence.pre_state_commitment = matching_pre_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            &[0x33; 32],
            &receipt.core_transition_evidence.tree_proof,
        );
        receipt.core_transition_evidence.post_state_commitment = matching_post_state_commitment(
            &SUBJECT_REFERENCE,
            None,
            b"\x01",
            &receipt.core_transition_evidence.tree_proof,
        );
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn validate_receipt_checks_pre_state_before_post_state() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.pre_state_commitment = vec![0xaa; 32];
        receipt.core_transition_evidence.post_state_commitment = vec![0xbb; 32];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "pre_state_commitment_mismatch"
            ))
        );
    }

    #[test]
    fn validate_receipt_preserves_post_state_commitment_mismatch_after_valid_pre_state() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.post_state_commitment = vec![0x55; 32];
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "post_state_commitment_mismatch"
            ))
        );
    }

    #[test]
    fn validate_receipt_with_valid_pre_and_post_commitments_reaches_not_implemented() {
        let receipt = minimal_receipt();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }

    #[test]
    fn validate_receipt_rejects_unsupported_protocol_version() {
        let mut receipt = minimal_receipt();
        receipt.protocol_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_protocol_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_protocol_version_before_commitment_gates() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.post_state_commitment = vec![0x55; 32];
        receipt.protocol_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_protocol_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_rejects_unsupported_receipt_version() {
        let mut receipt = minimal_receipt();
        receipt.receipt_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_receipt_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_protocol_version_before_receipt_version() {
        let mut receipt = minimal_receipt();
        receipt.protocol_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        receipt.receipt_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_protocol_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_receipt_version_before_commitment_gates() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.post_state_commitment = vec![0x55; 32];
        receipt.receipt_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_receipt_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_rejects_malformed_certification_provenance_shape() {
        let mut receipt = minimal_receipt();
        receipt.certification_provenance.posture = CertificationProvenancePosture::InlinePayload;
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "malformed_certification_provenance"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_protocol_version_before_malformed_certification_provenance() {
        let mut receipt = minimal_receipt();
        receipt.protocol_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        receipt.certification_provenance.posture = CertificationProvenancePosture::InlinePayload;
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_protocol_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_receipt_version_before_malformed_certification_provenance() {
        let mut receipt = minimal_receipt();
        receipt.receipt_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        receipt.certification_provenance.posture = CertificationProvenancePosture::InlinePayload;
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::UnsupportedVersion(
                "unsupported_receipt_version"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_evidence_gates_before_malformed_certification_provenance() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof = vec![0x01];
        receipt.certification_provenance.posture = CertificationProvenancePosture::InlinePayload;
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "tree_proof_envelope_invalid"
            ))
        );
    }

    #[test]
    fn validate_receipt_checks_post_state_commitment_before_malformed_certification_provenance() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.post_state_commitment = vec![0x55; 32];
        receipt.certification_provenance.posture = CertificationProvenancePosture::InlinePayload;
        assert_eq!(
            validate_receipt(&receipt),
            Err(VerificationFailure::InvalidEvidence(
                "post_state_commitment_mismatch"
            ))
        );
    }

    #[test]
    fn validate_receipt_with_shape_consistent_certification_provenance_still_reaches_not_implemented(
    ) {
        let receipt = minimal_receipt();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }
}
