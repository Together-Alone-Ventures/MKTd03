pub mod adapter;
mod certification_provenance_check;
mod core_transition_evidence_check;
mod deletion_state_material_check;
pub mod empty_subtree;
pub mod fixtures;
pub mod hashing;
pub mod host_api;
pub mod internal_node;
pub mod issuance;
pub mod leaf_hash;
pub mod library;
pub mod orchestration;
mod proof_direction_check;
pub mod proof_envelope;
pub mod proof_frame;
pub mod provenance;
pub mod record_position;
mod scope_encoding;
mod state;
pub mod state_commitment;
pub mod tags;
pub mod transition_material;
pub mod verifier;
pub use state::MKTd03State;

use candid::{candid_method, CandidType, Deserialize};
use ic_cdk::{init, post_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell as StableCell, DefaultMemoryImpl, Memory, Storable};
use serde::Serialize;
use state::{PersistedIssuanceTree, PersistedPendingIssuance, PersistedPendingIssuanceState};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;

const PROTOCOL_VERSION: SemanticVersion = SemanticVersion {
    major: 1,
    minor: 0,
    patch: 0,
};
const STATUS_SCHEMA_VERSION: SemanticVersion = SemanticVersion {
    major: 1,
    minor: 0,
    patch: 0,
};
const INTERFACE_VERSION: SemanticVersion = SemanticVersion {
    major: 2,
    minor: 0,
    patch: 0,
};
const BUILD_VERSION: SemanticVersion = SemanticVersion {
    major: 0,
    minor: 1,
    patch: 0,
};
const BUILD_LABEL: &str = env!("CARGO_PKG_VERSION");
const LIFECYCLE_STATE_MEMORY_ID: MemoryId = MemoryId::new(0);
const MODULE_HASH_MEMORY_ID: MemoryId = MemoryId::new(1);
const ISSUANCE_TREE_MEMORY_ID: MemoryId = MemoryId::new(2);
const PENDING_ISSUANCE_MEMORY_ID: MemoryId = MemoryId::new(3);
const ISSUED_RECEIPTS_MEMORY_ID: MemoryId = MemoryId::new(4);
const MODULE_HASH_LENGTH: usize = 32;
type RuntimeMemory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static STABLE_STORAGE: RefCell<Option<StableStorage<RuntimeMemory>>> = const { RefCell::new(None) };
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildIdentity {
    pub build_version: SemanticVersion,
    pub build_label: Option<String>,
    pub module_hash: Option<Vec<u8>>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BlockedCode {
    #[serde(rename = "rebuild_required")]
    RebuildRequired,
    #[serde(rename = "rebuild_consistency_failure")]
    RebuildConsistencyFailure,
    #[serde(rename = "initialisation_incomplete")]
    InitialisationIncomplete,
    #[serde(rename = "operator_hold")]
    OperatorHold,
    #[serde(rename = "unsupported_version")]
    UnsupportedVersion,
    #[serde(rename = "incompatible_status_surface")]
    IncompatibleStatusSurface,
    #[serde(rename = "unknown_blocked_state")]
    UnknownBlockedState,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockedReason {
    pub code: BlockedCode,
    pub description: String,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LifecycleState {
    #[serde(rename = "uninitialised")]
    Uninitialised,
    #[serde(rename = "initialising")]
    Initialising,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "rebuilding")]
    Rebuilding,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Compatibility {
    #[serde(rename = "compatible")]
    Compatible,
    #[serde(rename = "conditionally_compatible")]
    ConditionallyCompatible,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationContext {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "receipt_retrieval")]
    ReceiptRetrieval,
    #[serde(rename = "status_check")]
    StatusCheck,
    #[serde(rename = "version_check")]
    VersionCheck,
    #[serde(rename = "readiness_check")]
    ReadinessCheck,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StatusSurface {
    pub protocol_version: SemanticVersion,
    pub status_schema_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub build_identity: BuildIdentity,
    pub lifecycle_state: LifecycleState,
    pub is_blocked: bool,
    pub blocked_reason: Option<BlockedReason>,
    pub compatibility: Compatibility,
    pub operation_context: Option<OperationContext>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VersionInfo {
    pub protocol_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub compatibility: Compatibility,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VersionCheckResult {
    #[serde(rename = "supported")]
    Supported(VersionInfo),
    #[serde(rename = "unsupported_version")]
    UnsupportedVersion(VersionInfo),
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReceiptError {
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "not_yet_issued")]
    NotYetIssued,
    #[serde(rename = "invalid_subject_reference")]
    InvalidSubjectReference,
    #[serde(rename = "unsupported_version")]
    UnsupportedVersion,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReceiptResult {
    #[serde(rename = "ok")]
    Ok(library::Receipt),
    #[serde(rename = "err")]
    Err(ReceiptError),
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeginTreeReceiptIssuanceRequest {
    pub subject_reference: Vec<u8>,
    pub scope_reference: Option<Vec<u8>>,
    pub transition_material: Vec<u8>,
    pub deletion_state_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingReceiptInfo {
    pub pending_id: Vec<u8>,
    pub certified_commitment: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum IssuanceApiError {
    #[serde(rename = "invalid_subject_reference")]
    InvalidSubjectReference,
    #[serde(rename = "invalid_scope_reference")]
    InvalidScopeReference,
    #[serde(rename = "invalid_transition_material")]
    InvalidTransitionMaterial,
    #[serde(rename = "invalid_deletion_state_material")]
    InvalidDeletionStateMaterial,
    #[serde(rename = "pending_issuance_in_progress")]
    PendingIssuanceInProgress,
    #[serde(rename = "no_pending_issuance")]
    NoPendingIssuance,
    #[serde(rename = "pending_id_mismatch")]
    PendingIdMismatch,
    #[serde(rename = "certificate_unavailable")]
    CertificateUnavailable,
    #[serde(rename = "issuance_failed")]
    IssuanceFailed,
    #[serde(rename = "validation_failed")]
    ValidationFailed,
    #[serde(rename = "storage_unavailable")]
    StorageUnavailable,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BeginTreeReceiptIssuanceResult {
    #[serde(rename = "ok")]
    Ok(PendingReceiptInfo),
    #[serde(rename = "err")]
    Err(IssuanceApiError),
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingCertificateMaterial {
    pub pending_id: Vec<u8>,
    pub certified_commitment: Vec<u8>,
    pub certificate_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PendingCertificateMaterialResult {
    #[serde(rename = "ok")]
    Ok(PendingCertificateMaterial),
    #[serde(rename = "err")]
    Err(IssuanceApiError),
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FinalizeTreeReceiptRequest {
    pub pending_id: Vec<u8>,
    pub certificate_material: Vec<u8>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum FinalizeTreeReceiptResult {
    #[serde(rename = "ok")]
    Ok(library::Receipt),
    #[serde(rename = "err")]
    Err(IssuanceApiError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoredLifecycleState(u8);

impl StoredLifecycleState {
    const UNINITIALISED: Self = Self(0);
    const INITIALISING: Self = Self(1);
    const READY: Self = Self(2);
}

impl Storable for StoredLifecycleState {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(vec![self.0])
    }

    fn into_bytes(self) -> Vec<u8> {
        vec![self.0]
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(bytes.first().copied().unwrap_or_default())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1,
        is_fixed_size: true,
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoredModuleHash(Vec<u8>);

impl Default for StoredModuleHash {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Storable for StoredModuleHash {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.0.as_slice())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(bytes.into_owned())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MODULE_HASH_LENGTH as u32,
        is_fixed_size: false,
    };
}

struct StableStorage<M: Memory> {
    lifecycle_state: StableCell<StoredLifecycleState, M>,
    module_hash: StableCell<StoredModuleHash, M>,
    protocol_state: MKTd03State<M>,
}

#[derive(Debug, Eq, PartialEq)]
enum StatusSurfaceError {
    StorageNotConnected,
    InvalidLifecycleState(u8),
    UnexpectedLifecycleState {
        expected: LifecycleState,
        found: LifecycleState,
    },
    ModuleHashMissing,
    InvalidModuleHashLength(usize),
    ConditionallyCompatiblePolicyUndefined,
}

impl StatusSurfaceError {
    fn message(&self) -> String {
        match self {
            Self::StorageNotConnected => {
                "S7-1 status surface storage is not connected; run init or post_upgrade first."
                    .to_string()
            }
            Self::InvalidLifecycleState(code) => format!(
                "S7-1 lifecycle_state storage is invalid or inconsistent: unsupported code {}.",
                code
            ),
            Self::UnexpectedLifecycleState { expected, found } => format!(
                "S7-1 lifecycle_state storage is inconsistent: expected {}, found {}.",
                expected, found
            ),
            Self::ModuleHashMissing => {
                "S7-1 module_hash storage is unreadable or incomplete: no module hash persisted."
                    .to_string()
            }
            Self::InvalidModuleHashLength(length) => format!(
                "S7-1 module_hash storage is invalid or inconsistent: expected {} bytes, found {}.",
                MODULE_HASH_LENGTH, length
            ),
            Self::ConditionallyCompatiblePolicyUndefined => {
                "S7-2 conditionally_compatible policy not yet defined".to_string()
            }
        }
    }
}

impl fmt::Display for StatusSurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Uninitialised => "uninitialised",
            Self::Initialising => "initialising",
            Self::Ready => "ready",
            Self::Rebuilding => "rebuilding",
            Self::Failed => "failed",
        };
        f.write_str(name)
    }
}

impl TryFrom<StoredLifecycleState> for LifecycleState {
    type Error = StatusSurfaceError;

    fn try_from(value: StoredLifecycleState) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Uninitialised),
            1 => Ok(Self::Initialising),
            2 => Ok(Self::Ready),
            3 => Ok(Self::Rebuilding),
            4 => Ok(Self::Failed),
            other => Err(StatusSurfaceError::InvalidLifecycleState(other)),
        }
    }
}

fn with_storage<R>(
    f: impl FnOnce(&StableStorage<RuntimeMemory>) -> Result<R, StatusSurfaceError>,
) -> Result<R, StatusSurfaceError> {
    STABLE_STORAGE.with(|storage| {
        let borrowed = storage.borrow();
        let connected = borrowed
            .as_ref()
            .ok_or(StatusSurfaceError::StorageNotConnected)?;
        f(connected)
    })
}

fn with_storage_mut<R>(
    f: impl FnOnce(&mut StableStorage<RuntimeMemory>) -> Result<R, StatusSurfaceError>,
) -> Result<R, StatusSurfaceError> {
    STABLE_STORAGE.with(|storage| {
        let mut borrowed = storage.borrow_mut();
        let connected = borrowed
            .as_mut()
            .ok_or(StatusSurfaceError::StorageNotConnected)?;
        f(connected)
    })
}

fn with_storage_api<R>(
    f: impl FnOnce(&StableStorage<RuntimeMemory>) -> Result<R, IssuanceApiError>,
) -> Result<R, IssuanceApiError> {
    STABLE_STORAGE.with(|storage| {
        let borrowed = storage.borrow();
        let connected = borrowed
            .as_ref()
            .ok_or(IssuanceApiError::StorageUnavailable)?;
        f(connected)
    })
}

fn with_storage_api_mut<R>(
    f: impl FnOnce(&mut StableStorage<RuntimeMemory>) -> Result<R, IssuanceApiError>,
) -> Result<R, IssuanceApiError> {
    STABLE_STORAGE.with(|storage| {
        let mut borrowed = storage.borrow_mut();
        let connected = borrowed
            .as_mut()
            .ok_or(IssuanceApiError::StorageUnavailable)?;
        f(connected)
    })
}

fn connect_runtime_storage() {
    MEMORY_MANAGER.with(|memory_manager| {
        STABLE_STORAGE.with(|storage| {
            if storage.borrow().is_none() {
                let manager = memory_manager.borrow();
                *storage.borrow_mut() = Some(open_storage(&manager));
            }
        });
    });
}

fn open_storage<M: Memory>(memory_manager: &MemoryManager<M>) -> StableStorage<VirtualMemory<M>> {
    StableStorage {
        lifecycle_state: StableCell::init(
            memory_manager.get(LIFECYCLE_STATE_MEMORY_ID),
            StoredLifecycleState::UNINITIALISED,
        ),
        module_hash: StableCell::init(
            memory_manager.get(MODULE_HASH_MEMORY_ID),
            StoredModuleHash::default(),
        ),
        protocol_state: MKTd03State::new(
            memory_manager.get(ISSUANCE_TREE_MEMORY_ID),
            memory_manager.get(PENDING_ISSUANCE_MEMORY_ID),
            memory_manager.get(ISSUED_RECEIPTS_MEMORY_ID),
        ),
    }
}

fn validate_module_hash(module_hash: &[u8]) -> Result<(), StatusSurfaceError> {
    if module_hash.is_empty() {
        return Err(StatusSurfaceError::ModuleHashMissing);
    }
    if module_hash.len() != MODULE_HASH_LENGTH {
        return Err(StatusSurfaceError::InvalidModuleHashLength(
            module_hash.len(),
        ));
    }
    Ok(())
}

fn read_lifecycle_state<M: Memory>(
    storage: &StableStorage<M>,
) -> Result<LifecycleState, StatusSurfaceError> {
    LifecycleState::try_from(storage.lifecycle_state.get().clone())
}

fn read_module_hash<M: Memory>(storage: &StableStorage<M>) -> Result<Vec<u8>, StatusSurfaceError> {
    let module_hash = storage.module_hash.get().0.clone();
    validate_module_hash(&module_hash)?;
    Ok(module_hash)
}

fn set_lifecycle_state<M: Memory>(
    storage: &mut StableStorage<M>,
    lifecycle_state: StoredLifecycleState,
) {
    let _ = storage.lifecycle_state.set(lifecycle_state);
}

fn set_module_hash<M: Memory>(storage: &mut StableStorage<M>, module_hash: Vec<u8>) {
    let _ = storage.module_hash.set(StoredModuleHash(module_hash));
}

fn initialize_storage(module_hash: Vec<u8>) -> Result<(), StatusSurfaceError> {
    validate_module_hash(&module_hash)?;
    with_storage_mut(|storage| {
        let lifecycle_state = read_lifecycle_state(storage)?;
        if lifecycle_state != LifecycleState::Uninitialised {
            return Err(StatusSurfaceError::UnexpectedLifecycleState {
                expected: LifecycleState::Uninitialised,
                found: lifecycle_state,
            });
        }

        set_lifecycle_state(storage, StoredLifecycleState::INITIALISING);
        set_module_hash(storage, module_hash);
        set_lifecycle_state(storage, StoredLifecycleState::READY);
        Ok(())
    })
}

fn run_post_upgrade(module_hash: Vec<u8>) -> Result<(), StatusSurfaceError> {
    validate_module_hash(&module_hash)?;
    with_storage_mut(|storage| {
        let lifecycle_state = read_lifecycle_state(storage)?;
        if lifecycle_state != LifecycleState::Ready {
            return Err(StatusSurfaceError::UnexpectedLifecycleState {
                expected: LifecycleState::Ready,
                found: lifecycle_state,
            });
        }

        set_module_hash(storage, module_hash);
        Ok(())
    })
}

fn build_status_surface() -> Result<StatusSurface, StatusSurfaceError> {
    with_storage(|storage| {
        let lifecycle_state = read_lifecycle_state(storage)?;
        let module_hash = read_module_hash(storage)?;

        Ok(StatusSurface {
            protocol_version: PROTOCOL_VERSION.clone(),
            status_schema_version: STATUS_SCHEMA_VERSION.clone(),
            interface_version: INTERFACE_VERSION.clone(),
            build_identity: BuildIdentity {
                build_version: BUILD_VERSION.clone(),
                build_label: Some(BUILD_LABEL.to_string()),
                module_hash: Some(module_hash),
            },
            lifecycle_state,
            is_blocked: false,
            blocked_reason: None,
            compatibility: Compatibility::Compatible,
            operation_context: Some(OperationContext::StatusCheck),
        })
    })
}

fn build_version_info(compatibility: Compatibility) -> VersionInfo {
    VersionInfo {
        protocol_version: PROTOCOL_VERSION.clone(),
        interface_version: INTERFACE_VERSION.clone(),
        compatibility,
    }
}

fn build_evidence_readiness() -> Result<library::EvidenceReadiness, StatusSurfaceError> {
    with_storage(|storage| {
        let lifecycle_state = read_lifecycle_state(storage)?;
        let _ = read_module_hash(storage)?;

        match lifecycle_state {
            LifecycleState::Ready => Ok(library::EvidenceReadiness::EvidenceReady),
            LifecycleState::Uninitialised | LifecycleState::Initialising => {
                Ok(library::EvidenceReadiness::NotEvidenceReady)
            }
            LifecycleState::Rebuilding | LifecycleState::Failed => {
                Ok(library::EvidenceReadiness::RebuildRequired)
            }
        }
    })
}

fn build_public_version_info() -> Result<VersionInfo, StatusSurfaceError> {
    with_storage(|storage| {
        let _ = read_lifecycle_state(storage)?;
        let _ = read_module_hash(storage)?;
        Ok(build_version_info(Compatibility::Compatible))
    })
}

fn no_payload_certification_provenance() -> library::CertificationProvenanceBlock {
    library::CertificationProvenanceBlock {
        posture: library::CertificationProvenancePosture::NoPayloadForRoute,
        route: library::CertificationProvenanceRoute::DirectInline,
        certification_material: None,
        provenance_material: None,
        route_context_material: None,
    }
}

fn compute_pending_id(certified_commitment: &[u8; 32]) -> [u8; 32] {
    hashing::hash_with_tag(
        tags::TAG_RECEIPT_ID,
        &[b"PENDING_RECEIPT_V1", certified_commitment],
    )
}

fn map_issuance_error(error: issuance::IssuanceError) -> IssuanceApiError {
    match error {
        issuance::IssuanceError::InvalidSubjectReference => {
            IssuanceApiError::InvalidSubjectReference
        }
        issuance::IssuanceError::InvalidScopeReference => IssuanceApiError::InvalidScopeReference,
        issuance::IssuanceError::InvalidDeletionStateMaterial(_) => {
            IssuanceApiError::InvalidDeletionStateMaterial
        }
        issuance::IssuanceError::TargetAlreadyCommitted => IssuanceApiError::IssuanceFailed,
        issuance::IssuanceError::ProofGenerationFailed(_) => IssuanceApiError::IssuanceFailed,
        issuance::IssuanceError::ValidationFailed(_) => IssuanceApiError::ValidationFailed,
    }
}

fn module_hash_array_from_storage<M: Memory>(
    storage: &StableStorage<M>,
) -> Result<[u8; 32], IssuanceApiError> {
    let module_hash =
        read_module_hash(storage).map_err(|_| IssuanceApiError::StorageUnavailable)?;
    module_hash
        .try_into()
        .map_err(|_| IssuanceApiError::StorageUnavailable)
}

fn load_issuance_tree<M: Memory>(
    storage: &StableStorage<M>,
) -> Result<issuance::SparseIssuanceTree, IssuanceApiError> {
    let mut committed_leaves = BTreeMap::new();
    for entry in &storage
        .protocol_state
        .issuance_tree()
        .get()
        .committed_leaves
    {
        let position: [u8; 32] = entry
            .position
            .as_slice()
            .try_into()
            .map_err(|_| IssuanceApiError::StorageUnavailable)?;
        let leaf_hash: [u8; 32] = entry
            .leaf_hash
            .as_slice()
            .try_into()
            .map_err(|_| IssuanceApiError::StorageUnavailable)?;
        committed_leaves.insert(position, leaf_hash);
    }

    Ok(issuance::SparseIssuanceTree::from_committed_leaves(
        committed_leaves,
    ))
}

fn persist_issuance_tree<M: Memory>(
    storage: &mut StableStorage<M>,
    tree: &issuance::SparseIssuanceTree,
) -> Result<(), IssuanceApiError> {
    let committed_leaves = tree
        .committed_leaves()
        .iter()
        .map(|(position, leaf_hash)| state::PersistedLeafEntry {
            position: position.to_vec(),
            leaf_hash: leaf_hash.to_vec(),
        })
        .collect();
    let _ = storage
        .protocol_state
        .issuance_tree_mut()
        .set(PersistedIssuanceTree { committed_leaves });
    Ok(())
}

fn load_pending_issuance<M: Memory>(
    storage: &StableStorage<M>,
) -> Result<PersistedPendingIssuance, IssuanceApiError> {
    storage
        .protocol_state
        .pending_issuance()
        .get()
        .pending
        .clone()
        .ok_or(IssuanceApiError::NoPendingIssuance)
}

fn persist_pending_issuance<M: Memory>(
    storage: &mut StableStorage<M>,
    pending: PersistedPendingIssuance,
) -> Result<(), IssuanceApiError> {
    let _ = storage
        .protocol_state
        .pending_issuance_mut()
        .set(PersistedPendingIssuanceState {
            pending: Some(pending),
        });
    Ok(())
}

fn clear_pending_issuance<M: Memory>(
    storage: &mut StableStorage<M>,
) -> Result<(), IssuanceApiError> {
    let _ = storage
        .protocol_state
        .pending_issuance_mut()
        .set(PersistedPendingIssuanceState { pending: None });
    Ok(())
}

fn persist_issued_receipt<M: Memory>(
    storage: &mut StableStorage<M>,
    receipt: library::Receipt,
) -> Result<(), IssuanceApiError> {
    let mut state = storage.protocol_state.issued_receipts().get().clone();
    let subject_reference = receipt.core_transition_evidence.subject_reference.clone();
    state
        .receipts
        .retain(|entry| entry.subject_reference != subject_reference);
    state.receipts.push(state::PersistedReceiptEntry {
        subject_reference,
        receipt,
    });
    let _ = storage.protocol_state.issued_receipts_mut().set(state);
    Ok(())
}

fn lookup_issued_receipt<M: Memory>(
    storage: &StableStorage<M>,
    subject_reference: &[u8],
) -> Option<library::Receipt> {
    storage
        .protocol_state
        .issued_receipts()
        .get()
        .receipts
        .iter()
        .find(|entry| entry.subject_reference == subject_reference)
        .map(|entry| entry.receipt.clone())
}

fn pending_matches_subject<M: Memory>(
    storage: &StableStorage<M>,
    subject_reference: &[u8],
) -> bool {
    storage
        .protocol_state
        .pending_issuance()
        .get()
        .pending
        .as_ref()
        .map(|pending| {
            pending.receipt.core_transition_evidence.subject_reference == subject_reference
        })
        .unwrap_or(false)
}

fn begin_tree_receipt_issuance_impl(
    request: BeginTreeReceiptIssuanceRequest,
) -> Result<PendingReceiptInfo, IssuanceApiError> {
    let transition_material: [u8; 32] = request
        .transition_material
        .as_slice()
        .try_into()
        .map_err(|_| IssuanceApiError::InvalidTransitionMaterial)?;

    with_storage_api_mut(|storage| {
        if storage
            .protocol_state
            .pending_issuance()
            .get()
            .pending
            .is_some()
        {
            return Err(IssuanceApiError::PendingIssuanceInProgress);
        }

        let module_hash = module_hash_array_from_storage(storage)?;
        let tree = load_issuance_tree(storage)?;
        let mut preview_tree = tree.clone();
        let receipt = preview_tree
            .issue_unprovenanced_receipt(issuance::IssuanceInputs {
                subject_reference: &request.subject_reference,
                scope_reference: request.scope_reference.as_deref(),
                transition_material: &transition_material,
                deletion_state_material: &request.deletion_state_material,
                certification_provenance: no_payload_certification_provenance(),
            })
            .map_err(map_issuance_error)?;

        let target_position = record_position::compute_record_position_key(
            &request.subject_reference,
            request.scope_reference.as_deref(),
        )
        .map_err(|error| match error {
            record_position::RecordPositionError::EmptySubjectReference => {
                IssuanceApiError::InvalidSubjectReference
            }
            record_position::RecordPositionError::EmptyScopeReference => {
                IssuanceApiError::InvalidScopeReference
            }
        })?;
        let post_state_leaf = leaf_hash::compute_tombstoned_leaf(
            &request.subject_reference,
            request.scope_reference.as_deref(),
            &request.deletion_state_material,
        )
        .map_err(|error| match error {
            leaf_hash::LeafHashError::EmptySubjectReference => {
                IssuanceApiError::InvalidSubjectReference
            }
            leaf_hash::LeafHashError::EmptyScopeReference => {
                IssuanceApiError::InvalidScopeReference
            }
            leaf_hash::LeafHashError::InvalidDeletionStateMaterial(_) => {
                IssuanceApiError::InvalidDeletionStateMaterial
            }
        })?;

        let certified_commitment =
            provenance::compute_tree_certified_commitment(&receipt, &module_hash)
                .map_err(|_| IssuanceApiError::IssuanceFailed)?;
        let pending_id = compute_pending_id(&certified_commitment);

        persist_pending_issuance(
            storage,
            PersistedPendingIssuance {
                pending_id: pending_id.to_vec(),
                certified_commitment: certified_commitment.to_vec(),
                receipt,
                target_position: target_position.to_vec(),
                post_state_leaf: post_state_leaf.to_vec(),
            },
        )?;

        Ok(PendingReceiptInfo {
            pending_id: pending_id.to_vec(),
            certified_commitment: certified_commitment.to_vec(),
        })
    })
}

fn get_pending_certificate_material_impl(
    pending_id: Vec<u8>,
    certificate_material: Option<Vec<u8>>,
) -> PendingCertificateMaterialResult {
    match with_storage_api(|storage| {
        let pending = load_pending_issuance(storage)?;
        if pending.pending_id != pending_id {
            return Ok(PendingCertificateMaterialResult::Err(
                IssuanceApiError::PendingIdMismatch,
            ));
        }

        let certificate_material = match certificate_material {
            Some(bytes) => bytes,
            None => {
                return Ok(PendingCertificateMaterialResult::Err(
                    IssuanceApiError::CertificateUnavailable,
                ))
            }
        };

        Ok(PendingCertificateMaterialResult::Ok(
            PendingCertificateMaterial {
                pending_id: pending.pending_id,
                certified_commitment: pending.certified_commitment,
                certificate_material,
            },
        ))
    }) {
        Ok(result) => result,
        Err(error) => PendingCertificateMaterialResult::Err(error),
    }
}

fn finalize_tree_receipt_impl(
    request: FinalizeTreeReceiptRequest,
) -> Result<FinalizeTreeReceiptResult, IssuanceApiError> {
    with_storage_api_mut(|storage| {
        let pending = load_pending_issuance(storage)?;
        if pending.pending_id != request.pending_id {
            return Ok(FinalizeTreeReceiptResult::Err(
                IssuanceApiError::PendingIdMismatch,
            ));
        }

        let module_hash = module_hash_array_from_storage(storage)?;
        let mut receipt = pending.receipt.clone();
        receipt.certification_provenance =
            provenance::build_provenanced_certification_provenance_block(
                &request.certificate_material,
                &module_hash,
            );

        let certified_commitment =
            provenance::compute_tree_certified_commitment(&receipt, &module_hash)
                .map_err(|_| IssuanceApiError::IssuanceFailed)?;
        if certified_commitment.to_vec() != pending.certified_commitment {
            return Ok(FinalizeTreeReceiptResult::Err(
                IssuanceApiError::ValidationFailed,
            ));
        }

        verifier::validate_receipt(&receipt).map_err(|_| IssuanceApiError::ValidationFailed)?;

        let mut tree = load_issuance_tree(storage)?;
        let target_position: [u8; 32] = pending
            .target_position
            .as_slice()
            .try_into()
            .map_err(|_| IssuanceApiError::StorageUnavailable)?;
        let post_state_leaf: [u8; 32] = pending
            .post_state_leaf
            .as_slice()
            .try_into()
            .map_err(|_| IssuanceApiError::StorageUnavailable)?;
        tree.insert_committed_leaf(target_position, post_state_leaf)
            .map_err(map_issuance_error)?;
        persist_issuance_tree(storage, &tree)?;
        persist_issued_receipt(storage, receipt.clone())?;
        clear_pending_issuance(storage)?;

        Ok(FinalizeTreeReceiptResult::Ok(receipt))
    })
}

fn check_protocol_version_support(
    input: SemanticVersion,
) -> Result<VersionCheckResult, StatusSurfaceError> {
    if input == PROTOCOL_VERSION {
        return Ok(VersionCheckResult::Supported(build_version_info(
            Compatibility::Compatible,
        )));
    }

    if input.major != PROTOCOL_VERSION.major {
        return Ok(VersionCheckResult::UnsupportedVersion(build_version_info(
            Compatibility::Unsupported,
        )));
    }

    Err(StatusSurfaceError::ConditionallyCompatiblePolicyUndefined)
}

fn trap_on_error<T>(result: Result<T, StatusSurfaceError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => ic_cdk::trap(&error.message()),
    }
}

#[init]
fn init(module_hash: Vec<u8>) {
    connect_runtime_storage();
    trap_on_error(initialize_storage(module_hash));
}

#[post_upgrade]
fn post_upgrade(module_hash: Vec<u8>) {
    connect_runtime_storage();
    trap_on_error(run_post_upgrade(module_hash));
}

#[query]
#[candid_method(query, rename = "get_tree_mode_status")]
fn get_tree_mode_status() -> StatusSurface {
    connect_runtime_storage();
    trap_on_error(build_status_surface())
}

#[query]
#[candid_method(query, rename = "check_version_support")]
fn check_version_support(input: SemanticVersion) -> VersionCheckResult {
    connect_runtime_storage();
    trap_on_error(check_protocol_version_support(input))
}

#[query]
#[candid_method(query, rename = "get_evidence_readiness")]
fn get_evidence_readiness() -> library::EvidenceReadiness {
    connect_runtime_storage();
    trap_on_error(build_evidence_readiness())
}

#[query]
#[candid_method(query, rename = "get_version_info")]
fn get_version_info() -> VersionInfo {
    connect_runtime_storage();
    trap_on_error(build_public_version_info())
}

#[query]
#[candid_method(query, rename = "get_receipt")]
fn get_receipt(subject_reference: Vec<u8>) -> ReceiptResult {
    connect_runtime_storage();
    if subject_reference.is_empty() {
        return ReceiptResult::Err(ReceiptError::InvalidSubjectReference);
    }

    match with_storage_api(|storage| {
        if let Some(receipt) = lookup_issued_receipt(storage, &subject_reference) {
            return Ok(ReceiptResult::Ok(receipt));
        }
        if pending_matches_subject(storage, &subject_reference) {
            return Ok(ReceiptResult::Err(ReceiptError::NotYetIssued));
        }
        Ok(ReceiptResult::Err(ReceiptError::NotFound))
    }) {
        Ok(result) => result,
        Err(_) => ic_cdk::trap("S7-36 receipt storage is unavailable"),
    }
}

#[update]
#[candid_method(update, rename = "begin_tree_receipt_issuance")]
fn begin_tree_receipt_issuance(
    request: BeginTreeReceiptIssuanceRequest,
) -> BeginTreeReceiptIssuanceResult {
    connect_runtime_storage();
    match begin_tree_receipt_issuance_impl(request) {
        Ok(info) => {
            ic_cdk::api::certified_data_set(&info.certified_commitment);
            BeginTreeReceiptIssuanceResult::Ok(info)
        }
        Err(error) => BeginTreeReceiptIssuanceResult::Err(error),
    }
}

#[query]
#[candid_method(query, rename = "get_pending_certificate_material")]
fn get_pending_certificate_material(pending_id: Vec<u8>) -> PendingCertificateMaterialResult {
    connect_runtime_storage();
    get_pending_certificate_material_impl(pending_id, ic_cdk::api::data_certificate())
}

#[update]
#[candid_method(update, rename = "finalize_tree_receipt")]
fn finalize_tree_receipt(request: FinalizeTreeReceiptRequest) -> FinalizeTreeReceiptResult {
    connect_runtime_storage();
    match finalize_tree_receipt_impl(request) {
        Ok(result) => result,
        Err(error) => FinalizeTreeReceiptResult::Err(error),
    }
}

ic_cdk::export_candid!();

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::VectorMemory;
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;

    fn module_hash(byte: u8) -> Vec<u8> {
        vec![byte; MODULE_HASH_LENGTH]
    }

    fn with_test_storage<R>(
        f: impl FnOnce(&mut StableStorage<VirtualMemory<VectorMemory>>) -> R,
    ) -> R {
        let memory_manager = MemoryManager::init(VectorMemory::default());
        let mut storage = open_storage(&memory_manager);
        f(&mut storage)
    }

    fn with_connected_runtime_storage<R>(f: impl FnOnce() -> R) -> R {
        let memory_manager = MemoryManager::init(VectorMemory::default());
        STABLE_STORAGE.with(|storage| {
            *storage.borrow_mut() = Some(open_storage(&memory_manager));
        });
        let result = f();
        STABLE_STORAGE.with(|storage| *storage.borrow_mut() = None);
        result
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn normalize_definition(text: &str) -> String {
        let normalized = text
            .replace("blob", "vec nat8")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .replace("( ", "(")
            .replace(", );", ");")
            .replace(", )", ")");

        if let Some((prefix, body)) = normalized.split_once("{ ") {
            let body = body.trim_end_matches(" };").trim_end_matches(" }");
            let mut fields = body
                .split(';')
                .map(str::trim)
                .filter(|field| !field.is_empty())
                .collect::<Vec<_>>();
            fields.sort_unstable();
            return format!("{prefix}{{ {} }};", fields.join("; "));
        }

        normalized.replace("; }", " }")
    }

    fn extract_named_block(source: &str, name: &str) -> String {
        let header = format!("type {name} =");
        let start = source
            .find(&header)
            .unwrap_or_else(|| panic!("missing type block for {name}"));
        let remainder = &source[start..];
        let mut depth = 0usize;

        for (index, ch) in remainder.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => depth = depth.saturating_sub(1),
                ';' if depth == 0 => return remainder[..=index].to_string(),
                _ => {}
            }
        }

        panic!("missing terminator for type block {name}")
    }

    fn extract_service_method(source: &str, method_name: &str) -> String {
        let start = source
            .find(method_name)
            .unwrap_or_else(|| panic!("missing service method {method_name}"));
        let remainder = &source[start..];
        let mut depth = 0usize;
        for (index, ch) in remainder.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    if depth == 0 {
                        return remainder[..index].trim().to_string();
                    }
                    depth = depth.saturating_sub(1);
                }
                ';' if depth == 0 => return remainder[..=index].trim().to_string(),
                _ => {}
            }
        }
        panic!("missing service terminator for {method_name}")
    }

    #[test]
    fn initialize_storage_transitions_to_ready_and_surfaces_status() {
        with_test_storage(|storage| {
            validate_module_hash(&module_hash(7)).expect("module hash should be valid");
            let initial_state = read_lifecycle_state(storage).expect("default lifecycle state");
            assert_eq!(initial_state, LifecycleState::Uninitialised);

            set_lifecycle_state(storage, StoredLifecycleState::INITIALISING);
            set_module_hash(storage, module_hash(7));
            set_lifecycle_state(storage, StoredLifecycleState::READY);

            let lifecycle_state = read_lifecycle_state(storage).expect("ready lifecycle state");
            let persisted_hash = read_module_hash(storage).expect("persisted module hash");
            assert_eq!(lifecycle_state, LifecycleState::Ready);
            assert_eq!(persisted_hash, module_hash(7));
        });
    }

    #[test]
    fn post_upgrade_updates_module_hash_without_changing_ready_state() {
        with_test_storage(|storage| {
            set_lifecycle_state(storage, StoredLifecycleState::READY);
            set_module_hash(storage, module_hash(1));

            let lifecycle_state = read_lifecycle_state(storage).expect("ready lifecycle state");
            assert_eq!(lifecycle_state, LifecycleState::Ready);

            set_module_hash(storage, module_hash(9));
            let upgraded_hash = read_module_hash(storage).expect("upgraded module hash");
            assert_eq!(upgraded_hash, module_hash(9));
            assert_eq!(
                read_lifecycle_state(storage).expect("lifecycle state after upgrade"),
                LifecycleState::Ready
            );
        });
    }

    #[test]
    fn build_status_surface_fails_loud_when_storage_is_not_connected() {
        STABLE_STORAGE.with(|storage| *storage.borrow_mut() = None);
        let error = build_status_surface().expect_err("query should fail without storage");
        assert_eq!(
            error.message(),
            "S7-1 status surface storage is not connected; run init or post_upgrade first."
        );
    }

    #[test]
    fn build_status_surface_fails_loud_on_invalid_persisted_state() {
        with_test_storage(|storage| {
            set_lifecycle_state(storage, StoredLifecycleState(99));
            set_module_hash(storage, module_hash(3));
            let error = read_lifecycle_state(storage).expect_err("invalid state must fail loud");
            assert_eq!(
                error.message(),
                "S7-1 lifecycle_state storage is invalid or inconsistent: unsupported code 99."
            );
        });
    }

    #[test]
    fn status_surface_serialized_shape_matches_existing_positive_fixture() {
        let fixture_path = repo_root().join(
            "docs/test-vectors/fixtures/library/positive/mktd03_library_positive_status_ready_minimal_01_v1.json",
        );
        let fixture: Value =
            serde_json::from_str(&fs::read_to_string(fixture_path).expect("fixture file"))
                .expect("fixture json");

        let status = StatusSurface {
            protocol_version: PROTOCOL_VERSION.clone(),
            status_schema_version: STATUS_SCHEMA_VERSION.clone(),
            interface_version: INTERFACE_VERSION.clone(),
            build_identity: BuildIdentity {
                build_version: BUILD_VERSION.clone(),
                build_label: Some(BUILD_LABEL.to_string()),
                module_hash: Some(module_hash(5)),
            },
            lifecycle_state: LifecycleState::Ready,
            is_blocked: false,
            blocked_reason: None,
            compatibility: Compatibility::Compatible,
            operation_context: Some(OperationContext::StatusCheck),
        };

        let actual_keys = serde_json::to_value(status)
            .expect("status surface json")
            .as_object()
            .expect("status object")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let fixture_keys = fixture["expected_outcome"]["status_surface"]
            .as_object()
            .expect("fixture status object")
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(actual_keys, fixture_keys);
    }

    #[test]
    fn generated_did_subset_has_zero_divergence_for_current_public_surface() {
        let generated = __export_service();
        let authoritative = fs::read_to_string(repo_root().join("interfaces/mktd03_library.did"))
            .expect("authoritative did file");

        for name in [
            "SemanticVersion",
            "BuildIdentity",
            "LifecycleState",
            "Compatibility",
            "OperationContext",
            "BlockedCode",
            "BlockedReason",
            "StatusSurface",
            "EvidenceReadiness",
            "DeletionStateMaterial",
            "CoreTransitionEvidence",
            "CertificationProvenancePosture",
            "CertificationProvenanceRoute",
            "CertificationProvenanceBlock",
            "Receipt",
            "ReceiptError",
            "ReceiptResult",
            "BeginTreeReceiptIssuanceRequest",
            "PendingReceiptInfo",
            "IssuanceApiError",
            "BeginTreeReceiptIssuanceResult",
            "PendingCertificateMaterial",
            "PendingCertificateMaterialResult",
            "FinalizeTreeReceiptRequest",
            "FinalizeTreeReceiptResult",
            "VersionInfo",
            "VersionCheckResult",
        ] {
            assert_eq!(
                normalize_definition(&extract_named_block(&generated, name)),
                normalize_definition(&extract_named_block(&authoritative, name)),
                "generated .did diverged for type {name}",
            );
        }

        assert_eq!(
            normalize_definition(&extract_service_method(&generated, "get_tree_mode_status")),
            normalize_definition(&extract_service_method(
                &authoritative,
                "get_tree_mode_status"
            )),
            "generated .did diverged for get_tree_mode_status",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(&generated, "check_version_support")),
            normalize_definition(&extract_service_method(
                &authoritative,
                "check_version_support"
            )),
            "generated .did diverged for check_version_support",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(
                &generated,
                "get_evidence_readiness"
            )),
            normalize_definition(&extract_service_method(
                &authoritative,
                "get_evidence_readiness"
            )),
            "generated .did diverged for get_evidence_readiness",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(&generated, "get_version_info")),
            normalize_definition(&extract_service_method(&authoritative, "get_version_info")),
            "generated .did diverged for get_version_info",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(&generated, "get_receipt")),
            normalize_definition(&extract_service_method(&authoritative, "get_receipt")),
            "generated .did diverged for get_receipt",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(
                &generated,
                "begin_tree_receipt_issuance"
            )),
            normalize_definition(&extract_service_method(
                &authoritative,
                "begin_tree_receipt_issuance"
            )),
            "generated .did diverged for begin_tree_receipt_issuance",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(
                &generated,
                "get_pending_certificate_material"
            )),
            normalize_definition(&extract_service_method(
                &authoritative,
                "get_pending_certificate_material"
            )),
            "generated .did diverged for get_pending_certificate_material",
        );

        assert_eq!(
            normalize_definition(&extract_service_method(&generated, "finalize_tree_receipt")),
            normalize_definition(&extract_service_method(
                &authoritative,
                "finalize_tree_receipt"
            )),
            "generated .did diverged for finalize_tree_receipt",
        );
    }

    #[test]
    fn check_version_support_returns_supported_for_exact_protocol_version() {
        let result = check_protocol_version_support(PROTOCOL_VERSION.clone())
            .expect("exact version supported");

        assert_eq!(
            result,
            VersionCheckResult::Supported(VersionInfo {
                protocol_version: PROTOCOL_VERSION.clone(),
                interface_version: INTERFACE_VERSION.clone(),
                compatibility: Compatibility::Compatible,
            })
        );
    }

    #[test]
    fn check_version_support_returns_unsupported_for_different_major_version() {
        let result = check_protocol_version_support(SemanticVersion {
            major: PROTOCOL_VERSION.major + 1,
            minor: 0,
            patch: 0,
        })
        .expect("different major version should be unsupported");

        assert_eq!(
            result,
            VersionCheckResult::UnsupportedVersion(VersionInfo {
                protocol_version: PROTOCOL_VERSION.clone(),
                interface_version: INTERFACE_VERSION.clone(),
                compatibility: Compatibility::Unsupported,
            })
        );
    }

    #[test]
    fn check_version_support_fails_loud_for_same_major_different_version() {
        let error = check_protocol_version_support(SemanticVersion {
            major: PROTOCOL_VERSION.major,
            minor: PROTOCOL_VERSION.minor + 1,
            patch: PROTOCOL_VERSION.patch,
        })
        .expect_err("same-major different version must fail loud");

        assert_eq!(
            error.message(),
            "S7-2 conditionally_compatible policy not yet defined"
        );
    }

    #[test]
    fn get_receipt_distinguishes_invalid_subject_from_not_found() {
        assert_eq!(
            get_receipt(vec![]),
            ReceiptResult::Err(ReceiptError::InvalidSubjectReference)
        );
        assert_eq!(
            get_receipt(vec![0x42; 32]),
            ReceiptResult::Err(ReceiptError::NotFound)
        );
    }

    #[test]
    fn begin_tree_receipt_issuance_persists_pending_commitment_without_receipt_materialization() {
        with_connected_runtime_storage(|| {
            initialize_storage(module_hash(7)).expect("storage should initialize");

            let result = begin_tree_receipt_issuance_impl(BeginTreeReceiptIssuanceRequest {
                subject_reference: vec![0x42; 32],
                scope_reference: None,
                transition_material: vec![0x11; 32],
                deletion_state_material: vec![0x01],
            })
            .expect("begin issuance should succeed");

            assert_eq!(result.pending_id.len(), 32);
            assert_eq!(result.certified_commitment.len(), 32);
            assert!(matches!(
                get_receipt(vec![0x42; 32]),
                ReceiptResult::Err(ReceiptError::NotYetIssued)
            ));
        });
    }

    #[test]
    fn second_begin_tree_receipt_issuance_fails_while_pending_exists() {
        with_connected_runtime_storage(|| {
            initialize_storage(module_hash(8)).expect("storage should initialize");
            begin_tree_receipt_issuance_impl(BeginTreeReceiptIssuanceRequest {
                subject_reference: vec![0x52; 32],
                scope_reference: None,
                transition_material: vec![0x22; 32],
                deletion_state_material: vec![0x01],
            })
            .expect("first begin should succeed");

            assert_eq!(
                begin_tree_receipt_issuance_impl(BeginTreeReceiptIssuanceRequest {
                    subject_reference: vec![0x53; 32],
                    scope_reference: None,
                    transition_material: vec![0x33; 32],
                    deletion_state_material: vec![0x01],
                }),
                Err(IssuanceApiError::PendingIssuanceInProgress)
            );
        });
    }

    #[test]
    fn finalize_tree_receipt_persists_receipt_and_tree_and_clears_pending() {
        with_connected_runtime_storage(|| {
            initialize_storage(module_hash(9)).expect("storage should initialize");
            let pending = begin_tree_receipt_issuance_impl(BeginTreeReceiptIssuanceRequest {
                subject_reference: vec![0x62; 32],
                scope_reference: None,
                transition_material: vec![0x44; 32],
                deletion_state_material: vec![0x01],
            })
            .expect("begin issuance should succeed");

            let finalize = finalize_tree_receipt_impl(FinalizeTreeReceiptRequest {
                pending_id: pending.pending_id.clone(),
                certificate_material: b"TEST_CERTIFICATE".to_vec(),
            })
            .expect("finalize should complete");

            let receipt = match finalize {
                FinalizeTreeReceiptResult::Ok(receipt) => receipt,
                FinalizeTreeReceiptResult::Err(error) => {
                    panic!("expected finalized receipt, got {error:?}")
                }
            };
            assert_eq!(verifier::validate_receipt(&receipt), Ok(()));
            assert!(matches!(get_receipt(vec![0x62; 32]), ReceiptResult::Ok(_)));
            assert_eq!(
                get_pending_certificate_material_impl(pending.pending_id, Some(vec![0x01])),
                PendingCertificateMaterialResult::Err(IssuanceApiError::NoPendingIssuance)
            );
        });
    }
}
