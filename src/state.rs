use crate::library;
use candid::{decode_one, encode_one, CandidType, Deserialize};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell as StableCell, Memory, Storable};
use std::borrow::Cow;

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
}
