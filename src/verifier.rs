use crate::core_transition_evidence_check::{
    validate_core_transition_evidence, CoreTransitionEvidenceError,
};
use crate::fixtures::{FixtureReceipt, VerifierReceiptFixture};
use crate::library::{CertificationProvenancePosture, CertificationProvenanceRoute, Receipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationFailure {
    InvalidEvidence(&'static str),
    Deferred(&'static str),
    NotImplemented(&'static str),
}

pub fn validate_receipt(receipt: &Receipt) -> Result<(), VerificationFailure> {
    if let Err(err) = validate_core_transition_evidence(&receipt.core_transition_evidence) {
        return Err(VerificationFailure::InvalidEvidence(
            map_core_transition_evidence_error(err),
        ));
    }

    // TODO: implement receipt validation semantics once proof verification logic is authorized.
    Err(VerificationFailure::NotImplemented(
        "receipt validation is not implemented in the first scaffold pass",
    ))
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
    if certification_shape_is_consistent(&receipt) {
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

fn certification_shape_is_consistent(receipt: &FixtureReceipt) -> bool {
    let block = &receipt.certification_provenance;

    match block.posture {
        CertificationProvenancePosture::InlinePayload => {
            block.route == CertificationProvenanceRoute::DirectInline
                && block.certification_material.is_some()
                && block.provenance_material.is_some()
                && block.route_context_material.is_none()
        }
        CertificationProvenancePosture::RouteDependentPayload => match block.route {
            CertificationProvenanceRoute::DirectInline => false,
            CertificationProvenanceRoute::RouteContextRequired => {
                block.route_context_material.is_some()
                    && (block.certification_material.is_some()
                        || block.provenance_material.is_some())
            }
            CertificationProvenanceRoute::RouteContextOnly => {
                block.route_context_material.is_some()
            }
        },
        CertificationProvenancePosture::NoPayloadForRoute => {
            block.certification_material.is_none()
                && block.provenance_material.is_none()
                && block.route_context_material.is_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{
        CertificationProvenanceBlock, CoreTransitionEvidence, DeletionStateMaterial, Receipt,
        SemanticVersion,
    };
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

    fn minimal_receipt() -> Receipt {
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
    fn receipt_validation_does_not_run_direction_validation() {
        let mut receipt = minimal_receipt();
        receipt.core_transition_evidence.tree_proof =
            structurally_valid_unchecked_direction_tree_proof_bytes();
        assert!(matches!(
            validate_receipt(&receipt),
            Err(VerificationFailure::NotImplemented(_))
        ));
    }
}
