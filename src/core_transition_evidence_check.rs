use crate::deletion_state_material_check::{
    validate_deletion_state_material, DeletionStateMaterialError,
};
use crate::library::CoreTransitionEvidence;
use crate::proof_envelope::{parse_proof_envelope, ProofEnvelopeError};

const DIGEST_LENGTH: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CoreTransitionEvidenceError {
    EmptySubjectReference,
    EmptyScopeReference,
    PreStateCommitmentUnexpectedLength { found: usize },
    PostStateCommitmentUnexpectedLength { found: usize },
    TransitionMaterialUnexpectedLength { found: usize },
    TreeProofEnvelopeInvalid { source: ProofEnvelopeError },
    DeletionStateMaterialInvalid { source: DeletionStateMaterialError },
}

// consumed by validate_receipt as the first structural pre-check.
pub(crate) fn validate_core_transition_evidence(
    evidence: &CoreTransitionEvidence,
) -> Result<(), CoreTransitionEvidenceError> {
    if evidence.subject_reference.is_empty() {
        return Err(CoreTransitionEvidenceError::EmptySubjectReference);
    }

    if matches!(evidence.scope_reference.as_ref(), Some(scope_reference) if scope_reference.is_empty())
    {
        return Err(CoreTransitionEvidenceError::EmptyScopeReference);
    }

    if evidence.pre_state_commitment.len() != DIGEST_LENGTH {
        return Err(
            CoreTransitionEvidenceError::PreStateCommitmentUnexpectedLength {
                found: evidence.pre_state_commitment.len(),
            },
        );
    }

    if evidence.post_state_commitment.len() != DIGEST_LENGTH {
        return Err(
            CoreTransitionEvidenceError::PostStateCommitmentUnexpectedLength {
                found: evidence.post_state_commitment.len(),
            },
        );
    }

    if evidence.transition_material.len() != DIGEST_LENGTH {
        return Err(
            CoreTransitionEvidenceError::TransitionMaterialUnexpectedLength {
                found: evidence.transition_material.len(),
            },
        );
    }

    // Successful parsing here establishes envelope shape only. It does not verify sibling
    // content, direction-vs-position relationship, record-position correctness, or any root claim.
    parse_proof_envelope(&evidence.tree_proof)
        .map_err(|source| CoreTransitionEvidenceError::TreeProofEnvelopeInvalid { source })?;

    validate_deletion_state_material(&evidence.deletion_state_material)
        .map_err(|source| CoreTransitionEvidenceError::DeletionStateMaterialInvalid { source })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{DeletionStateMaterial, SemanticVersion};
    use crate::proof_envelope::{serialize_proof_envelope, ProofEnvelope};
    use crate::proof_frame::{ProofDirection, ProofFrame, ProofFrameSibling};

    fn canonical_empty_frame(direction: ProofDirection) -> ProofFrame {
        ProofFrame {
            direction,
            sibling: ProofFrameSibling::CanonicalEmpty,
        }
    }

    fn valid_tree_proof_bytes() -> Vec<u8> {
        let envelope = ProofEnvelope {
            frames: std::array::from_fn(|_| canonical_empty_frame(ProofDirection::Left)),
        };
        serialize_proof_envelope(&envelope)
    }

    fn structurally_valid_unchecked_direction_tree_proof_bytes() -> Vec<u8> {
        let envelope = ProofEnvelope {
            frames: std::array::from_fn(|index| {
                if index % 2 == 0 {
                    canonical_empty_frame(ProofDirection::Right)
                } else {
                    ProofFrame {
                        direction: ProofDirection::Left,
                        sibling: ProofFrameSibling::Explicit([0x42; 32]),
                    }
                }
            }),
        };
        serialize_proof_envelope(&envelope)
    }

    fn minimal_evidence() -> CoreTransitionEvidence {
        CoreTransitionEvidence {
            subject_reference: vec![0x42],
            scope_reference: None,
            pre_state_commitment: vec![0x11; 32],
            post_state_commitment: vec![0x22; 32],
            transition_material: vec![0x33; 32],
            transition_derivation_version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            tree_proof: valid_tree_proof_bytes(),
            deletion_state_material: DeletionStateMaterial::TombstonedPosition(vec![0x01]),
        }
    }

    #[test]
    fn minimal_structurally_valid_core_transition_evidence_passes() {
        assert_eq!(
            validate_core_transition_evidence(&minimal_evidence()),
            Ok(())
        );
    }

    #[test]
    fn empty_subject_reference_fails() {
        let mut evidence = minimal_evidence();
        evidence.subject_reference = vec![];
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::EmptySubjectReference)
        );
    }

    #[test]
    fn empty_some_scope_reference_fails() {
        let mut evidence = minimal_evidence();
        evidence.scope_reference = Some(vec![]);
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::EmptyScopeReference)
        );
    }

    #[test]
    fn pre_state_commitment_length_31_fails() {
        let mut evidence = minimal_evidence();
        evidence.pre_state_commitment = vec![0x11; 31];
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::PreStateCommitmentUnexpectedLength { found: 31 })
        );
    }

    #[test]
    fn post_state_commitment_length_33_fails() {
        let mut evidence = minimal_evidence();
        evidence.post_state_commitment = vec![0x22; 33];
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::PostStateCommitmentUnexpectedLength { found: 33 })
        );
    }

    #[test]
    fn transition_material_length_31_fails() {
        let mut evidence = minimal_evidence();
        evidence.transition_material = vec![0x33; 31];
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::TransitionMaterialUnexpectedLength { found: 31 })
        );
    }

    #[test]
    fn malformed_tree_proof_header_truncated_fails() {
        let mut evidence = minimal_evidence();
        evidence.tree_proof = vec![0x01];
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::TreeProofEnvelopeInvalid {
                source: ProofEnvelopeError::HeaderTruncated {
                    needed: 2,
                    available: 1,
                },
            })
        );
    }

    #[test]
    fn empty_position_deletion_state_material_fails() {
        let mut evidence = minimal_evidence();
        evidence.deletion_state_material = DeletionStateMaterial::EmptyPosition(vec![]);
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::DeletionStateMaterialInvalid {
                source: DeletionStateMaterialError::EmptyPositionVariantNotPermittedInV1,
            })
        );
    }

    #[test]
    fn wrong_tombstoned_payload_deletion_state_material_fails() {
        let mut evidence = minimal_evidence();
        evidence.deletion_state_material = DeletionStateMaterial::TombstonedPosition(vec![0x00]);
        assert_eq!(
            validate_core_transition_evidence(&evidence),
            Err(CoreTransitionEvidenceError::DeletionStateMaterialInvalid {
                source: DeletionStateMaterialError::TombstonedPositionUnexpectedBytes {
                    found: vec![0x00],
                },
            })
        );
    }

    #[test]
    fn structurally_valid_tree_proof_with_unchecked_direction_pattern_still_passes() {
        let mut evidence = minimal_evidence();
        evidence.tree_proof = structurally_valid_unchecked_direction_tree_proof_bytes();
        assert_eq!(validate_core_transition_evidence(&evidence), Ok(()));
    }
}
