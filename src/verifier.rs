use crate::fixtures::{FixtureReceipt, VerifierReceiptFixture};
use crate::library::{CertificationProvenancePosture, CertificationProvenanceRoute, Receipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationFailure {
    InvalidEvidence(&'static str),
    Deferred(&'static str),
    NotImplemented(&'static str),
}

pub fn validate_receipt(_receipt: &Receipt) -> Result<(), VerificationFailure> {
    // TODO: implement receipt validation semantics once proof verification logic is authorized.
    Err(VerificationFailure::NotImplemented(
        "receipt validation is not implemented in the first scaffold pass",
    ))
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
