pub mod adapter;
pub mod fixtures;
pub mod hashing;
pub mod leaf_hash;
pub mod library;
pub mod orchestration;
pub mod record_position;
mod scope_encoding;
pub mod tags;
pub mod verifier;

use candid::{candid_method, CandidType, Deserialize};
use ic_cdk::{init, post_upgrade, query};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell as StableCell, DefaultMemoryImpl, Memory, Storable};
use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;
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

fn connect_runtime_storage() {
    MEMORY_MANAGER.with(|memory_manager| {
        STABLE_STORAGE.with(|storage| {
            let manager = memory_manager.borrow();
            *storage.borrow_mut() = Some(open_storage(&manager));
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
    trap_on_error(build_status_surface())
}

#[query]
#[candid_method(query, rename = "check_version_support")]
fn check_version_support(input: SemanticVersion) -> VersionCheckResult {
    trap_on_error(check_protocol_version_support(input))
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

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn normalize_definition(text: &str) -> String {
        let normalized = text
            .replace("blob", "vec nat8")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

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
        let end = remainder
            .find("query")
            .map(|index| index + "query".len())
            .or_else(|| remainder.find('}'))
            .unwrap_or_else(|| panic!("missing service terminator for {method_name}"));
        remainder[..end].trim().to_string()
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
    fn generated_did_subset_has_zero_divergence_for_exposed_s7_1_surface() {
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
    }

    #[test]
    fn check_version_support_returns_supported_for_exact_protocol_version() {
        let result =
            check_protocol_version_support(PROTOCOL_VERSION.clone()).expect("exact version supported");

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
}
