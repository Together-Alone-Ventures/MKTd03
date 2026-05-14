use crate::host_api::{
    HostIssuanceError, HostPhaseAInputs, HostPhaseAOutputs, HostPhaseBInputs, HostPhaseBOutputs,
    HostPhaseCInputs, HostPhaseCOutputs, HostReceiptLookupInputs, HostReceiptLookupOutputs,
};
use crate::library;
use crate::{
    issuance, leaf_hash, no_payload_certification_provenance, provenance, record_position, verifier,
};
use candid::{decode_one, encode_one, CandidType, Deserialize};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell as StableCell, Memory, Storable};
use std::borrow::Cow;
use std::collections::BTreeMap;

const TREE_STATE_MAX_BYTES: u32 = 1_048_576;
const PENDING_ISSUANCE_MAX_BYTES: u32 = 262_144;
const ISSUED_RECEIPTS_MAX_BYTES: u32 = 4_194_304;

#[derive(CandidType, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedLeafEntry {
    pub(crate) position: Vec<u8>,
    pub(crate) leaf_hash: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedIssuanceTree {
    pub(crate) committed_leaves: Vec<PersistedLeafEntry>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedReceiptEntry {
    pub(crate) subject_reference: Vec<u8>,
    pub(crate) receipt: library::Receipt,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedIssuedReceipts {
    pub(crate) receipts: Vec<PersistedReceiptEntry>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedPendingIssuance {
    pub(crate) pending_id: Vec<u8>,
    pub(crate) certified_commitment: Vec<u8>,
    pub(crate) receipt: library::Receipt,
    pub(crate) target_position: Vec<u8>,
    pub(crate) post_state_leaf: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub(crate) struct PersistedPendingIssuanceState {
    pub(crate) pending: Option<PersistedPendingIssuance>,
}

impl Storable for PersistedIssuanceTree {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("persisted issuance tree should encode"))
    }

    fn into_bytes(self) -> Vec<u8> {
        encode_one(self).expect("persisted issuance tree should encode")
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(bytes.as_ref()).expect("persisted issuance tree should decode")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: TREE_STATE_MAX_BYTES,
        is_fixed_size: false,
    };
}

impl Storable for PersistedPendingIssuanceState {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("persisted pending issuance should encode"))
    }

    fn into_bytes(self) -> Vec<u8> {
        encode_one(self).expect("persisted pending issuance should encode")
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(bytes.as_ref()).expect("persisted pending issuance should decode")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: PENDING_ISSUANCE_MAX_BYTES,
        is_fixed_size: false,
    };
}

impl Storable for PersistedIssuedReceipts {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("persisted receipts should encode"))
    }

    fn into_bytes(self) -> Vec<u8> {
        encode_one(self).expect("persisted receipts should encode")
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(bytes.as_ref()).expect("persisted receipts should decode")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: ISSUED_RECEIPTS_MAX_BYTES,
        is_fixed_size: false,
    };
}

/// Host-owned protocol storage handles for embedded or standalone MKTd03 use.
pub struct MKTd03State<M: Memory> {
    issuance_tree: StableCell<PersistedIssuanceTree, M>,
    pending_issuance: StableCell<PersistedPendingIssuanceState, M>,
    issued_receipts: StableCell<PersistedIssuedReceipts, M>,
}

impl<M: Memory> MKTd03State<M> {
    pub fn new(tree_storage: M, pending_issuance_storage: M, issued_receipts_storage: M) -> Self {
        Self {
            issuance_tree: StableCell::init(tree_storage, PersistedIssuanceTree::default()),
            pending_issuance: StableCell::init(
                pending_issuance_storage,
                PersistedPendingIssuanceState::default(),
            ),
            issued_receipts: StableCell::init(
                issued_receipts_storage,
                PersistedIssuedReceipts::default(),
            ),
        }
    }

    pub(crate) fn issuance_tree(&self) -> &StableCell<PersistedIssuanceTree, M> {
        &self.issuance_tree
    }

    pub(crate) fn issuance_tree_mut(&mut self) -> &mut StableCell<PersistedIssuanceTree, M> {
        &mut self.issuance_tree
    }

    pub(crate) fn pending_issuance(&self) -> &StableCell<PersistedPendingIssuanceState, M> {
        &self.pending_issuance
    }

    pub(crate) fn pending_issuance_mut(
        &mut self,
    ) -> &mut StableCell<PersistedPendingIssuanceState, M> {
        &mut self.pending_issuance
    }

    pub(crate) fn issued_receipts(&self) -> &StableCell<PersistedIssuedReceipts, M> {
        &self.issued_receipts
    }

    pub(crate) fn issued_receipts_mut(&mut self) -> &mut StableCell<PersistedIssuedReceipts, M> {
        &mut self.issued_receipts
    }

    fn has_pending_issuance(&self) -> bool {
        self.pending_issuance().get().pending.is_some()
    }

    fn load_issuance_tree(&self) -> Result<issuance::SparseIssuanceTree, HostIssuanceError> {
        let mut committed_leaves = BTreeMap::new();
        for entry in &self.issuance_tree().get().committed_leaves {
            let position: [u8; 32] = entry
                .position
                .as_slice()
                .try_into()
                .map_err(|_| HostIssuanceError::StorageUnavailable)?;
            let leaf_hash: [u8; 32] = entry
                .leaf_hash
                .as_slice()
                .try_into()
                .map_err(|_| HostIssuanceError::StorageUnavailable)?;
            committed_leaves.insert(position, leaf_hash);
        }

        Ok(issuance::SparseIssuanceTree::from_committed_leaves(
            committed_leaves,
        ))
    }

    fn persist_pending_issuance(
        &mut self,
        pending: PersistedPendingIssuance,
    ) -> Result<(), HostIssuanceError> {
        let _ = self
            .pending_issuance_mut()
            .set(PersistedPendingIssuanceState {
                pending: Some(pending),
            });
        Ok(())
    }

    fn persist_issuance_tree(
        &mut self,
        tree: &issuance::SparseIssuanceTree,
    ) -> Result<(), HostIssuanceError> {
        let committed_leaves = tree
            .committed_leaves()
            .iter()
            .map(|(position, leaf_hash)| PersistedLeafEntry {
                position: position.to_vec(),
                leaf_hash: leaf_hash.to_vec(),
            })
            .collect();
        let _ = self
            .issuance_tree_mut()
            .set(PersistedIssuanceTree { committed_leaves });
        Ok(())
    }

    fn persist_issued_receipt(
        &mut self,
        receipt: library::Receipt,
    ) -> Result<(), HostIssuanceError> {
        let mut state = self.issued_receipts().get().clone();
        let subject_reference = receipt.core_transition_evidence.subject_reference.clone();
        state
            .receipts
            .retain(|entry| entry.subject_reference != subject_reference);
        state.receipts.push(PersistedReceiptEntry {
            subject_reference,
            receipt,
        });
        let _ = self.issued_receipts_mut().set(state);
        Ok(())
    }

    fn clear_pending_issuance(&mut self) -> Result<(), HostIssuanceError> {
        let _ = self
            .pending_issuance_mut()
            .set(PersistedPendingIssuanceState { pending: None });
        Ok(())
    }

    fn lookup_issued_receipt(&self, subject_reference: &[u8]) -> Option<library::Receipt> {
        self.issued_receipts()
            .get()
            .receipts
            .iter()
            .find(|entry| entry.subject_reference == subject_reference)
            .map(|entry| entry.receipt.clone())
    }

    fn pending_matches_subject(&self, subject_reference: &[u8]) -> bool {
        self.pending_issuance()
            .get()
            .pending
            .as_ref()
            .map(|pending| {
                pending.receipt.core_transition_evidence.subject_reference == subject_reference
            })
            .unwrap_or(false)
    }

    pub(crate) fn load_pending_issuance(
        &self,
    ) -> Result<PersistedPendingIssuance, HostIssuanceError> {
        self.pending_issuance()
            .get()
            .pending
            .clone()
            .ok_or(HostIssuanceError::NoPendingIssuance)
    }

    pub fn host_begin_phase_a(
        &mut self,
        module_hash: &[u8; 32],
        inputs: HostPhaseAInputs,
    ) -> Result<HostPhaseAOutputs, HostIssuanceError> {
        let transition_material: [u8; 32] = inputs
            .transition_material
            .as_slice()
            .try_into()
            .map_err(|_| HostIssuanceError::InvalidTransitionMaterial)?;

        if self.has_pending_issuance() {
            return Err(HostIssuanceError::PendingIssuanceInProgress);
        }

        let tree = self.load_issuance_tree()?;
        let mut preview_tree = tree.clone();
        let receipt = preview_tree
            .issue_unprovenanced_receipt(issuance::IssuanceInputs {
                subject_reference: &inputs.subject_reference,
                scope_reference: inputs.scope_reference.as_deref(),
                transition_material: &transition_material,
                deletion_state_material: &inputs.deletion_state_material,
                certification_provenance: no_payload_certification_provenance(),
            })
            .map_err(crate::map_issuance_error)?;

        let target_position = record_position::compute_record_position_key(
            &inputs.subject_reference,
            inputs.scope_reference.as_deref(),
        )
        .map_err(|error| match error {
            record_position::RecordPositionError::EmptySubjectReference => {
                HostIssuanceError::InvalidSubjectReference
            }
            record_position::RecordPositionError::EmptyScopeReference => {
                HostIssuanceError::InvalidScopeReference
            }
        })?;
        let post_state_leaf = leaf_hash::compute_tombstoned_leaf(
            &inputs.subject_reference,
            inputs.scope_reference.as_deref(),
            &inputs.deletion_state_material,
        )
        .map_err(|error| match error {
            leaf_hash::LeafHashError::EmptySubjectReference => {
                HostIssuanceError::InvalidSubjectReference
            }
            leaf_hash::LeafHashError::EmptyScopeReference => {
                HostIssuanceError::InvalidScopeReference
            }
            leaf_hash::LeafHashError::InvalidDeletionStateMaterial(_) => {
                HostIssuanceError::InvalidDeletionStateMaterial
            }
        })?;

        let certified_commitment =
            provenance::compute_tree_certified_commitment(&receipt, module_hash)
                .map_err(|_| HostIssuanceError::IssuanceFailed)?;
        let pending_id = crate::compute_pending_id(&certified_commitment);

        self.persist_pending_issuance(PersistedPendingIssuance {
            pending_id: pending_id.to_vec(),
            certified_commitment: certified_commitment.to_vec(),
            receipt,
            target_position: target_position.to_vec(),
            post_state_leaf: post_state_leaf.to_vec(),
        })?;

        Ok(HostPhaseAOutputs {
            pending_id: pending_id.to_vec(),
            certified_commitment: certified_commitment.to_vec(),
        })
    }

    pub fn host_get_phase_b(
        &self,
        inputs: HostPhaseBInputs,
    ) -> Result<HostPhaseBOutputs, HostIssuanceError> {
        let pending = self.load_pending_issuance()?;
        if pending.pending_id != inputs.pending_id {
            return Err(HostIssuanceError::PendingIdMismatch);
        }

        Ok(HostPhaseBOutputs {
            certificate_material: inputs.host_data_certificate,
        })
    }

    pub fn host_finalize_phase_c(
        &mut self,
        module_hash: &[u8; 32],
        inputs: HostPhaseCInputs,
    ) -> Result<HostPhaseCOutputs, HostIssuanceError> {
        let pending = self.load_pending_issuance()?;
        if pending.pending_id != inputs.pending_id {
            return Err(HostIssuanceError::PendingIdMismatch);
        }

        let mut receipt = pending.receipt.clone();
        receipt.certification_provenance =
            provenance::build_provenanced_certification_provenance_block(
                &inputs.certificate_material,
                module_hash,
            );

        let certified_commitment =
            provenance::compute_tree_certified_commitment(&receipt, module_hash)
                .map_err(|_| HostIssuanceError::IssuanceFailed)?;
        if certified_commitment.to_vec() != pending.certified_commitment {
            return Err(HostIssuanceError::ValidationFailed);
        }

        verifier::validate_receipt(&receipt).map_err(|_| HostIssuanceError::ValidationFailed)?;

        let mut tree = self.load_issuance_tree()?;
        let target_position: [u8; 32] = pending
            .target_position
            .as_slice()
            .try_into()
            .map_err(|_| HostIssuanceError::StorageUnavailable)?;
        let post_state_leaf: [u8; 32] = pending
            .post_state_leaf
            .as_slice()
            .try_into()
            .map_err(|_| HostIssuanceError::StorageUnavailable)?;
        tree.insert_committed_leaf(target_position, post_state_leaf)
            .map_err(crate::map_issuance_error)?;
        self.persist_issuance_tree(&tree)?;
        self.persist_issued_receipt(receipt.clone())?;
        self.clear_pending_issuance()?;

        Ok(HostPhaseCOutputs { receipt })
    }

    pub fn host_get_receipt(&self, inputs: HostReceiptLookupInputs) -> HostReceiptLookupOutputs {
        if inputs.subject_reference.is_empty() {
            return library::ReceiptResult::Err {
                error_code: library::ReceiptError::InvalidSubjectReference,
            };
        }

        if let Some(receipt) = self.lookup_issued_receipt(&inputs.subject_reference) {
            return library::ReceiptResult::Ok { receipt };
        }
        if self.pending_matches_subject(&inputs.subject_reference) {
            return library::ReceiptResult::Err {
                error_code: library::ReceiptError::NotYetIssued,
            };
        }
        library::ReceiptResult::Err {
            error_code: library::ReceiptError::NotFound,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transition_material::derive_transition_material;
    use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
    use ic_stable_structures::VectorMemory;

    const HOST_TREE_MEMORY_ID: MemoryId = MemoryId::new(20);
    const HOST_PENDING_MEMORY_ID: MemoryId = MemoryId::new(21);
    const HOST_RECEIPTS_MEMORY_ID: MemoryId = MemoryId::new(22);

    fn host_state() -> MKTd03State<VirtualMemory<VectorMemory>> {
        let memory_manager = MemoryManager::init(VectorMemory::default());
        MKTd03State::new(
            memory_manager.get(HOST_TREE_MEMORY_ID),
            memory_manager.get(HOST_PENDING_MEMORY_ID),
            memory_manager.get(HOST_RECEIPTS_MEMORY_ID),
        )
    }

    fn module_hash(byte: u8) -> [u8; 32] {
        [byte; 32]
    }

    fn version_1_0_0() -> library::SemanticVersion {
        library::SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    fn transition_material_for(source: &[u8]) -> Vec<u8> {
        derive_transition_material(&version_1_0_0(), source).to_vec()
    }

    #[test]
    fn host_api_round_trip_succeeds_outside_canister_context() {
        let mut state = host_state();
        let module_hash = module_hash(9);
        let subject_reference = vec![0x62; 32];

        let phase_a = state
            .host_begin_phase_a(
                &module_hash,
                HostPhaseAInputs {
                    subject_reference: subject_reference.clone(),
                    scope_reference: None,
                    transition_material: transition_material_for(b"host-round-trip"),
                    deletion_state_material: vec![0x01],
                },
            )
            .expect("phase A should succeed");

        assert!(!phase_a.pending_id.is_empty());
        assert_eq!(phase_a.certified_commitment.len(), 32);

        let phase_b = state
            .host_get_phase_b(HostPhaseBInputs {
                pending_id: phase_a.pending_id.clone(),
                host_data_certificate: vec![0xCA, 0xFE],
            })
            .expect("phase B should succeed");

        let phase_c = state
            .host_finalize_phase_c(
                &module_hash,
                HostPhaseCInputs {
                    pending_id: phase_a.pending_id.clone(),
                    certificate_material: phase_b.certificate_material,
                },
            )
            .expect("phase C should succeed");

        assert_eq!(
            phase_c.receipt.core_transition_evidence.subject_reference,
            subject_reference
        );
        assert_eq!(verifier::validate_receipt(&phase_c.receipt), Ok(()));

        assert_eq!(
            state.host_get_receipt(HostReceiptLookupInputs { subject_reference }),
            library::ReceiptResult::Ok {
                receipt: phase_c.receipt
            }
        );
    }

    #[test]
    fn host_begin_phase_a_rejects_second_pending_issuance() {
        let mut state = host_state();
        let module_hash = module_hash(8);

        state
            .host_begin_phase_a(
                &module_hash,
                HostPhaseAInputs {
                    subject_reference: vec![0x52; 32],
                    scope_reference: None,
                    transition_material: transition_material_for(b"first-pending"),
                    deletion_state_material: vec![0x01],
                },
            )
            .expect("first begin should succeed");

        assert_eq!(
            state.host_begin_phase_a(
                &module_hash,
                HostPhaseAInputs {
                    subject_reference: vec![0x53; 32],
                    scope_reference: None,
                    transition_material: transition_material_for(b"second-pending"),
                    deletion_state_material: vec![0x01],
                },
            ),
            Err(HostIssuanceError::PendingIssuanceInProgress)
        );
    }

    #[test]
    fn host_get_phase_b_rejects_pending_id_mismatch() {
        let mut state = host_state();
        let module_hash = module_hash(7);

        state
            .host_begin_phase_a(
                &module_hash,
                HostPhaseAInputs {
                    subject_reference: vec![0x42; 32],
                    scope_reference: None,
                    transition_material: transition_material_for(b"phase-b-mismatch"),
                    deletion_state_material: vec![0x01],
                },
            )
            .expect("begin should succeed");

        assert_eq!(
            state.host_get_phase_b(HostPhaseBInputs {
                pending_id: vec![0xAA; 32],
                host_data_certificate: vec![0xCA, 0xFE],
            }),
            Err(HostIssuanceError::PendingIdMismatch)
        );
    }

    #[test]
    fn host_get_phase_b_reports_no_pending_issuance() {
        let state = host_state();

        assert_eq!(
            state.host_get_phase_b(HostPhaseBInputs {
                pending_id: vec![0xAA; 32],
                host_data_certificate: vec![0xCA, 0xFE],
            }),
            Err(HostIssuanceError::NoPendingIssuance)
        );
    }

    #[test]
    fn host_get_receipt_preserves_lookup_error_semantics() {
        let mut state = host_state();
        let module_hash = module_hash(6);
        let subject_reference = vec![0x42; 32];

        assert_eq!(
            state.host_get_receipt(HostReceiptLookupInputs {
                subject_reference: vec![],
            }),
            library::ReceiptResult::Err {
                error_code: library::ReceiptError::InvalidSubjectReference,
            }
        );

        assert_eq!(
            state.host_get_receipt(HostReceiptLookupInputs {
                subject_reference: subject_reference.clone(),
            }),
            library::ReceiptResult::Err {
                error_code: library::ReceiptError::NotFound,
            }
        );

        state
            .host_begin_phase_a(
                &module_hash,
                HostPhaseAInputs {
                    subject_reference: subject_reference.clone(),
                    scope_reference: None,
                    transition_material: transition_material_for(b"pending-receipt"),
                    deletion_state_material: vec![0x01],
                },
            )
            .expect("begin should succeed");

        assert_eq!(
            state.host_get_receipt(HostReceiptLookupInputs { subject_reference }),
            library::ReceiptResult::Err {
                error_code: library::ReceiptError::NotYetIssued,
            }
        );
    }
}
