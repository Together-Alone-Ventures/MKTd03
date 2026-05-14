use crate::{library::Receipt, library::ReceiptResult, IssuanceApiError};
use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Host-owned Phase A request material.
/// `module_hash` is supplied separately by the embedding host.
#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseAInputs {
    pub subject_reference: Vec<u8>,
    pub scope_reference: Option<Vec<u8>>,
    pub transition_material: Vec<u8>,
    pub deletion_state_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseAOutputs {
    pub pending_id: Vec<u8>,
    pub certified_commitment: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseBInputs {
    pub pending_id: Vec<u8>,
    /// Host-supplied IC data certificate, obtained from
    /// `ic_cdk::api::data_certificate()` in a query context.
    pub host_data_certificate: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseBOutputs {
    pub certificate_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseCInputs {
    pub pending_id: Vec<u8>,
    pub certificate_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostPhaseCOutputs {
    pub receipt: Receipt,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostReceiptLookupInputs {
    pub subject_reference: Vec<u8>,
}

pub type HostReceiptLookupOutputs = ReceiptResult;
/// In host API context, `StorageUnavailable` indicates a stable-storage
/// failure or corruption condition rather than standalone canister lifecycle
/// storage not being connected.
pub type HostIssuanceError = IssuanceApiError;
