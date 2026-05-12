use crate::hashing::hash_with_tag;
use crate::issuance::{
    issue_unprovenanced_receipt, IssuanceError, IssuanceInputs, SparseIssuanceTree,
};
use crate::library::{
    CertificationProvenanceBlock, CertificationProvenancePosture, CertificationProvenanceRoute,
    DeletionStateMaterial, Receipt,
};
use crate::tags::TAG_CERTIFIED_COMMITMENT;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceError {
    IssuanceFailed(IssuanceError),
    InvalidPreStateCommitmentLength { found: usize },
    InvalidPostStateCommitmentLength { found: usize },
    InvalidTransitionMaterialLength { found: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenancedIssuanceInputs<'a> {
    pub subject_reference: &'a [u8],
    pub scope_reference: Option<&'a [u8]>,
    pub transition_material: &'a [u8; 32],
    pub deletion_state_material: &'a [u8],
    pub certificate: &'a [u8],
    pub module_hash: &'a [u8; 32],
}

pub fn compute_tree_certified_preimage(
    receipt: &Receipt,
    module_hash: &[u8; 32],
) -> Result<Vec<u8>, ProvenanceError> {
    let pre_state_commitment = expect_32_bytes(
        &receipt.core_transition_evidence.pre_state_commitment,
        "pre_state_commitment",
    )?;
    let post_state_commitment = expect_32_bytes(
        &receipt.core_transition_evidence.post_state_commitment,
        "post_state_commitment",
    )?;
    let transition_material = expect_32_bytes(
        &receipt.core_transition_evidence.transition_material,
        "transition_material",
    )?;

    let mut bytes = Vec::new();
    bytes.extend_from_slice(TAG_CERTIFIED_COMMITMENT);
    push_semantic_version(&mut bytes, &receipt.protocol_version);
    push_semantic_version(&mut bytes, &receipt.receipt_version);
    push_semantic_version(
        &mut bytes,
        &receipt
            .core_transition_evidence
            .transition_derivation_version,
    );
    push_length_prefixed_bytes(
        &mut bytes,
        &receipt.core_transition_evidence.subject_reference,
    );
    match &receipt.core_transition_evidence.scope_reference {
        None => bytes.push(0x00),
        Some(scope_reference) => {
            bytes.push(0x01);
            push_length_prefixed_bytes(&mut bytes, scope_reference);
        }
    }
    bytes.extend_from_slice(pre_state_commitment);
    bytes.extend_from_slice(post_state_commitment);
    bytes.extend_from_slice(transition_material);
    match &receipt.core_transition_evidence.deletion_state_material {
        DeletionStateMaterial::TombstonedPosition(payload) => {
            bytes.push(0x01);
            push_length_prefixed_bytes(&mut bytes, payload);
        }
        DeletionStateMaterial::EmptyPosition(payload) => {
            bytes.push(0x00);
            push_length_prefixed_bytes(&mut bytes, payload);
        }
    }
    bytes.extend_from_slice(module_hash);
    Ok(bytes)
}

pub fn compute_tree_certified_commitment(
    receipt: &Receipt,
    module_hash: &[u8; 32],
) -> Result<[u8; 32], ProvenanceError> {
    let preimage = compute_tree_certified_preimage(receipt, module_hash)?;
    Ok(hash_with_tag(
        TAG_CERTIFIED_COMMITMENT,
        &[&preimage[TAG_CERTIFIED_COMMITMENT.len()..]],
    ))
}

pub fn build_provenanced_certification_provenance_block(
    certificate: &[u8],
    module_hash: &[u8; 32],
) -> CertificationProvenanceBlock {
    CertificationProvenanceBlock {
        posture: CertificationProvenancePosture::InlinePayload,
        route: CertificationProvenanceRoute::DirectInline,
        certification_material: Some(certificate.to_vec()),
        provenance_material: Some(module_hash.to_vec()),
        route_context_material: None,
    }
}

pub fn issue_provenanced_receipt(
    tree: &mut SparseIssuanceTree,
    inputs: ProvenancedIssuanceInputs<'_>,
) -> Result<Receipt, ProvenanceError> {
    let receipt = issue_unprovenanced_receipt(
        tree,
        IssuanceInputs {
            subject_reference: inputs.subject_reference,
            scope_reference: inputs.scope_reference,
            transition_material: inputs.transition_material,
            deletion_state_material: inputs.deletion_state_material,
            certification_provenance: build_provenanced_certification_provenance_block(
                inputs.certificate,
                inputs.module_hash,
            ),
        },
    )
    .map_err(ProvenanceError::IssuanceFailed)?;

    let _ = compute_tree_certified_commitment(&receipt, inputs.module_hash)?;
    Ok(receipt)
}

fn push_semantic_version(bytes: &mut Vec<u8>, version: &crate::library::SemanticVersion) {
    bytes.extend_from_slice(&version.major.to_be_bytes());
    bytes.extend_from_slice(&version.minor.to_be_bytes());
    bytes.extend_from_slice(&version.patch.to_be_bytes());
}

fn push_length_prefixed_bytes(bytes: &mut Vec<u8>, payload: &[u8]) {
    bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    bytes.extend_from_slice(payload);
}

fn expect_32_bytes<'a>(bytes: &'a [u8], field: &str) -> Result<&'a [u8; 32], ProvenanceError> {
    bytes.try_into().map_err(|_| match field {
        "pre_state_commitment" => {
            ProvenanceError::InvalidPreStateCommitmentLength { found: bytes.len() }
        }
        "post_state_commitment" => {
            ProvenanceError::InvalidPostStateCommitmentLength { found: bytes.len() }
        }
        "transition_material" => {
            ProvenanceError::InvalidTransitionMaterialLength { found: bytes.len() }
        }
        _ => unreachable!("unknown fixed-width field"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuance::SparseIssuanceTree;
    use crate::verifier::validate_receipt;

    const EXPECTED_CERTIFIED_COMMITMENT_HEX: &str =
        "c94519aea3da449e16478e6f62e5554afc4788992b132d10293ee96168efa9d9";

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn sample_provenanced_receipt() -> Receipt {
        let mut tree = SparseIssuanceTree::new();
        let subject_reference = [0x42; 32];
        let transition_material = [0x11; 32];
        let certificate = b"MOCK_CERTIFICATE_BYTES";
        let module_hash = [0x55; 32];

        issue_provenanced_receipt(
            &mut tree,
            ProvenancedIssuanceInputs {
                subject_reference: &subject_reference,
                scope_reference: None,
                transition_material: &transition_material,
                deletion_state_material: b"\x01",
                certificate,
                module_hash: &module_hash,
            },
        )
        .expect("sample provenanced issuance should succeed")
    }

    #[test]
    fn certified_preimage_is_deterministic() {
        let receipt = sample_provenanced_receipt();
        let module_hash = [0x55; 32];

        let left = compute_tree_certified_preimage(&receipt, &module_hash)
            .expect("preimage should compute");
        let right = compute_tree_certified_preimage(&receipt, &module_hash)
            .expect("preimage should compute");

        assert_eq!(left, right);
    }

    #[test]
    fn certified_commitment_matches_pinned_golden_hex() {
        let receipt = sample_provenanced_receipt();
        let module_hash = [0x55; 32];

        let commitment = compute_tree_certified_commitment(&receipt, &module_hash)
            .expect("commitment should compute");
        assert_eq!(to_hex(commitment), EXPECTED_CERTIFIED_COMMITMENT_HEX);
    }

    #[test]
    fn changing_module_hash_changes_certified_commitment() {
        let receipt = sample_provenanced_receipt();
        let left = compute_tree_certified_commitment(&receipt, &[0x11; 32])
            .expect("commitment should compute");
        let right = compute_tree_certified_commitment(&receipt, &[0x22; 32])
            .expect("commitment should compute");
        assert_ne!(left, right);
    }

    #[test]
    fn provenance_block_embeds_certificate_and_module_hash() {
        let certificate = b"CERT_BYTES";
        let module_hash = [0x77; 32];
        let block = build_provenanced_certification_provenance_block(certificate, &module_hash);

        assert_eq!(block.posture, CertificationProvenancePosture::InlinePayload);
        assert_eq!(block.route, CertificationProvenanceRoute::DirectInline);
        assert_eq!(block.certification_material, Some(certificate.to_vec()));
        assert_eq!(block.provenance_material, Some(module_hash.to_vec()));
        assert_eq!(block.route_context_material, None);
    }

    #[test]
    fn issue_provenanced_receipt_returns_receipt_that_validates() {
        let receipt = sample_provenanced_receipt();
        assert_eq!(validate_receipt(&receipt), Ok(()));
    }

    #[test]
    fn issued_provenanced_receipt_contains_expected_provenance_material() {
        let receipt = sample_provenanced_receipt();
        assert_eq!(
            receipt.certification_provenance.certification_material,
            Some(b"MOCK_CERTIFICATE_BYTES".to_vec())
        );
        assert_eq!(
            receipt.certification_provenance.provenance_material,
            Some([0x55; 32].to_vec())
        );
        assert_eq!(
            receipt.certification_provenance.posture,
            CertificationProvenancePosture::InlinePayload
        );
        assert_eq!(
            receipt.certification_provenance.route,
            CertificationProvenanceRoute::DirectInline
        );
    }
}
