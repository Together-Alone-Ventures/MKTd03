use crate::fixtures::{
    load_all_typed_fixtures_from_index, FixtureDeletionStateMaterial, FixtureReceipt,
    FixtureStatusSurface, GenericInputSummary, TypedFixtureCase, TypedFixtureDocument,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Compatibility {
    Compatible,
    ConditionallyCompatible,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildIdentity {
    pub build_version: SemanticVersion,
    pub build_label: Option<String>,
    pub module_hash: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedCode {
    RebuildRequired,
    RebuildConsistencyFailure,
    InitialisationIncomplete,
    OperatorHold,
    UnsupportedVersion,
    IncompatibleStatusSurface,
    UnknownBlockedState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlockedReason {
    pub code: BlockedCode,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Uninitialised,
    Initialising,
    Ready,
    Rebuilding,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationContext {
    None,
    ReceiptRetrieval,
    StatusCheck,
    VersionCheck,
    ReadinessCheck,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceReadiness {
    EvidenceReady,
    RebuildRequired,
    NotEvidenceReady,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletionStateMaterial {
    TombstonedPosition(Vec<u8>),
    EmptyPosition(Vec<u8>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CoreTransitionEvidence {
    pub subject_reference: Vec<u8>,
    pub scope_reference: Option<Vec<u8>>,
    pub pre_state_commitment: Vec<u8>,
    pub post_state_commitment: Vec<u8>,
    pub transition_material: Vec<u8>,
    pub tree_proof: Vec<u8>,
    pub deletion_state_material: DeletionStateMaterial,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationProvenancePosture {
    InlinePayload,
    RouteDependentPayload,
    NoPayloadForRoute,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationProvenanceRoute {
    DirectInline,
    RouteContextRequired,
    RouteContextOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertificationProvenanceBlock {
    pub posture: CertificationProvenancePosture,
    pub route: CertificationProvenanceRoute,
    pub certification_material: Option<Vec<u8>>,
    pub provenance_material: Option<Vec<u8>>,
    pub route_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Receipt {
    pub protocol_version: SemanticVersion,
    pub receipt_version: SemanticVersion,
    pub core_transition_evidence: CoreTransitionEvidence,
    pub certification_provenance: CertificationProvenanceBlock,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptError {
    NotFound,
    NotYetIssued,
    InvalidSubjectReference,
    UnsupportedVersion,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result_variant", rename_all = "snake_case")]
pub enum ReceiptResult {
    Ok { receipt: Receipt },
    Err { error_code: ReceiptError },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VersionInfo {
    pub protocol_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub compatibility: Compatibility,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result_variant", rename_all = "snake_case")]
pub enum VersionCheckResult {
    Supported { version_info: VersionInfo },
    UnsupportedVersion { version_info: VersionInfo },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryScaffold;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LibraryScaffoldError {
    NotImplemented(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LibrarySemanticError {
    InvalidCompatibilityPolicy(&'static str),
    InvalidStatusSurface(&'static str),
    InvalidEvidenceReadiness(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceRuntimeError {
    FixtureLoad(String),
    MissingConfiguration(&'static str),
    MissingFixture(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceLibraryRuntime {
    pub supported_protocol_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub status_fixtures: BTreeMap<String, StatusSurface>,
    pub readiness_fixtures: BTreeMap<String, EvidenceReadiness>,
    pub selected_status_fixture_id: Option<String>,
    pub selected_readiness_fixture_id: Option<String>,
    pub receipt_successes: BTreeMap<Vec<u8>, Receipt>,
    pub receipt_errors: BTreeMap<Vec<u8>, ReceiptError>,
}

pub fn classify_exact_version_compatibility(
    supported_protocol_version: &SemanticVersion,
    requested_protocol_version: &SemanticVersion,
) -> Compatibility {
    if supported_protocol_version == requested_protocol_version {
        Compatibility::Compatible
    } else {
        Compatibility::Unsupported
    }
}

pub fn evaluate_version_support(
    supported_protocol_version: &SemanticVersion,
    interface_version: &SemanticVersion,
    requested_protocol_version: &SemanticVersion,
) -> VersionCheckResult {
    let compatibility =
        classify_exact_version_compatibility(supported_protocol_version, requested_protocol_version);
    let version_info = VersionInfo {
        protocol_version: requested_protocol_version.clone(),
        interface_version: interface_version.clone(),
        compatibility: compatibility.clone(),
    };

    match compatibility {
        Compatibility::Compatible | Compatibility::ConditionallyCompatible => {
            VersionCheckResult::Supported { version_info }
        }
        Compatibility::Unsupported => VersionCheckResult::UnsupportedVersion { version_info },
    }
}

pub fn validate_status_surface_semantics(
    status: &StatusSurface,
) -> Result<(), LibrarySemanticError> {
    if status.is_blocked != status.blocked_reason.is_some() {
        return Err(LibrarySemanticError::InvalidStatusSurface(
            "blocked_reason must be present iff is_blocked is true",
        ));
    }

    Ok(())
}

pub fn validate_evidence_readiness_semantics(
    readiness: &EvidenceReadiness,
) -> Result<(), LibrarySemanticError> {
    match readiness {
        EvidenceReadiness::EvidenceReady
        | EvidenceReadiness::RebuildRequired
        | EvidenceReadiness::NotEvidenceReady => Ok(()),
    }
}

impl ReferenceLibraryRuntime {
    pub fn from_fixture_index(
        index_path: impl AsRef<Path>,
    ) -> Result<Self, ReferenceRuntimeError> {
        let documents = load_all_typed_fixtures_from_index(index_path)
            .map_err(|error| ReferenceRuntimeError::FixtureLoad(error.to_string()))?;
        Self::from_typed_fixtures(&documents)
    }

    pub fn from_typed_fixtures(
        documents: &[TypedFixtureDocument],
    ) -> Result<Self, ReferenceRuntimeError> {
        let mut supported_protocol_version = None;
        let mut interface_version = None;
        let mut status_fixtures = BTreeMap::new();
        let mut readiness_fixtures = BTreeMap::new();
        let mut selected_status_fixture_id = None;
        let mut selected_readiness_fixture_id = None;
        let mut receipt_successes = BTreeMap::new();
        let mut receipt_errors = BTreeMap::new();

        for document in documents {
            match &document.case {
                TypedFixtureCase::LibraryPositiveVersionSupport(fixture) => {
                    supported_protocol_version =
                        Some(fixture.expected.version_info.protocol_version.clone());
                    interface_version = Some(fixture.expected.version_info.interface_version.clone());
                }
                TypedFixtureCase::LibraryPositiveStatus(fixture) => {
                    let status = materialize_status_surface(&fixture.expected.status_surface);
                    validate_status_surface_semantics(&status)
                        .map_err(|error| ReferenceRuntimeError::FixtureLoad(format!("{error:?}")))?;
                    if selected_status_fixture_id.is_none() {
                        selected_status_fixture_id = Some(document.envelope.fixture_id.clone());
                    }
                    status_fixtures.insert(document.envelope.fixture_id.clone(), status);
                }
                TypedFixtureCase::LibraryNegativeBlockedStatus(fixture) => {
                    let status = materialize_status_surface(&fixture.expected.status_surface);
                    validate_status_surface_semantics(&status)
                        .map_err(|error| ReferenceRuntimeError::FixtureLoad(format!("{error:?}")))?;
                    status_fixtures.insert(document.envelope.fixture_id.clone(), status);
                }
                TypedFixtureCase::LibraryNegativeEvidenceReadiness(fixture) => {
                    validate_evidence_readiness_semantics(&fixture.expected.enum_value)
                        .map_err(|error| ReferenceRuntimeError::FixtureLoad(format!("{error:?}")))?;
                    if selected_readiness_fixture_id.is_none() {
                        selected_readiness_fixture_id = Some(document.envelope.fixture_id.clone());
                    }
                    readiness_fixtures.insert(
                        document.envelope.fixture_id.clone(),
                        fixture.expected.enum_value.clone(),
                    );
                }
                TypedFixtureCase::LibraryPositiveReceipt(fixture) => {
                    let subject_reference = extract_subject_reference(&document.envelope)?;
                    receipt_successes.insert(
                        subject_reference,
                        materialize_receipt(&fixture.expected.receipt),
                    );
                }
                TypedFixtureCase::LibraryNegativeReceiptError(fixture) => {
                    let subject_reference = extract_subject_reference(&document.envelope)?;
                    receipt_errors.insert(subject_reference, fixture.expected.error_code.clone());
                }
                _ => {}
            }
        }

        Ok(Self {
            supported_protocol_version: supported_protocol_version.ok_or(
                ReferenceRuntimeError::MissingConfiguration(
                    "supported protocol version fixture is not configured",
                ),
            )?,
            interface_version: interface_version.ok_or(
                ReferenceRuntimeError::MissingConfiguration(
                    "interface version fixture is not configured",
                ),
            )?,
            status_fixtures,
            readiness_fixtures,
            selected_status_fixture_id,
            selected_readiness_fixture_id,
            receipt_successes,
            receipt_errors,
        })
    }

    pub fn select_status_fixture(
        &mut self,
        fixture_id: impl Into<String>,
    ) -> Result<(), ReferenceRuntimeError> {
        let fixture_id = fixture_id.into();
        if self.status_fixtures.contains_key(&fixture_id) {
            self.selected_status_fixture_id = Some(fixture_id);
            Ok(())
        } else {
            Err(ReferenceRuntimeError::MissingFixture(fixture_id))
        }
    }

    pub fn select_evidence_readiness_fixture(
        &mut self,
        fixture_id: impl Into<String>,
    ) -> Result<(), ReferenceRuntimeError> {
        let fixture_id = fixture_id.into();
        if self.readiness_fixtures.contains_key(&fixture_id) {
            self.selected_readiness_fixture_id = Some(fixture_id);
            Ok(())
        } else {
            Err(ReferenceRuntimeError::MissingFixture(fixture_id))
        }
    }

    pub fn get_version_info(&self) -> VersionInfo {
        VersionInfo {
            protocol_version: self.supported_protocol_version.clone(),
            interface_version: self.interface_version.clone(),
            compatibility: Compatibility::Compatible,
        }
    }

    pub fn check_version_support(&self, requested_protocol_version: &SemanticVersion) -> VersionCheckResult {
        evaluate_version_support(
            &self.supported_protocol_version,
            &self.interface_version,
            requested_protocol_version,
        )
    }

    pub fn get_status(&self) -> Result<StatusSurface, ReferenceRuntimeError> {
        let fixture_id = self.selected_status_fixture_id.as_ref().ok_or(
            ReferenceRuntimeError::MissingConfiguration("status fixture selection is not configured"),
        )?;
        self.status_fixtures
            .get(fixture_id)
            .cloned()
            .ok_or_else(|| ReferenceRuntimeError::MissingFixture(fixture_id.clone()))
    }

    pub fn get_evidence_readiness(&self) -> Result<EvidenceReadiness, ReferenceRuntimeError> {
        let fixture_id = self.selected_readiness_fixture_id.as_ref().ok_or(
            ReferenceRuntimeError::MissingConfiguration(
                "evidence-readiness fixture selection is not configured",
            ),
        )?;
        self.readiness_fixtures
            .get(fixture_id)
            .cloned()
            .ok_or_else(|| ReferenceRuntimeError::MissingFixture(fixture_id.clone()))
    }

    pub fn get_receipt(
        &self,
        subject_reference: &[u8],
    ) -> Option<ReceiptResult> {
        if let Some(receipt) = self.receipt_successes.get(subject_reference) {
            return Some(ReceiptResult::Ok {
                receipt: receipt.clone(),
            });
        }

        self.receipt_errors
            .get(subject_reference)
            .cloned()
            .map(|error_code| ReceiptResult::Err { error_code })
    }
}

fn extract_subject_reference(
    envelope: &crate::fixtures::FixtureEnvelope,
) -> Result<Vec<u8>, ReferenceRuntimeError> {
    let input_summary: GenericInputSummary = serde_json::from_value(envelope.input_summary.clone())
        .map_err(|error| ReferenceRuntimeError::FixtureLoad(error.to_string()))?;
    let subject_reference = input_summary
        .method_args
        .as_array()
        .and_then(|values| values.first())
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            ReferenceRuntimeError::FixtureLoad(format!(
                "{} does not carry a string subject-reference method arg",
                envelope.fixture_id
            ))
        })?;

    Ok(subject_reference.as_bytes().to_vec())
}

fn materialize_status_surface(fixture_status: &FixtureStatusSurface) -> StatusSurface {
    StatusSurface {
        protocol_version: fixture_status.protocol_version.clone(),
        status_schema_version: fixture_status.status_schema_version.clone(),
        interface_version: fixture_status.interface_version.clone(),
        build_identity: BuildIdentity {
            build_version: fixture_status.build_identity.build_version.clone(),
            build_label: fixture_status.build_identity.build_label.clone(),
            module_hash: None,
        },
        lifecycle_state: fixture_status.lifecycle_state.clone(),
        is_blocked: fixture_status.is_blocked,
        blocked_reason: fixture_status.blocked_reason.clone(),
        compatibility: fixture_status.compatibility.clone(),
        operation_context: fixture_status.operation_context.clone(),
    }
}

fn materialize_receipt(fixture_receipt: &FixtureReceipt) -> Receipt {
    Receipt {
        protocol_version: fixture_receipt.protocol_version.clone(),
        receipt_version: fixture_receipt.receipt_version.clone(),
        core_transition_evidence: CoreTransitionEvidence {
            subject_reference: fixture_receipt
                .core_transition_evidence
                .subject_reference
                .as_bytes()
                .to_vec(),
            scope_reference: fixture_receipt
                .core_transition_evidence
                .scope_reference
                .as_ref()
                .map(|value| value.as_bytes().to_vec()),
            pre_state_commitment: fixture_receipt
                .core_transition_evidence
                .pre_state_commitment
                .as_bytes()
                .to_vec(),
            post_state_commitment: fixture_receipt
                .core_transition_evidence
                .post_state_commitment
                .as_bytes()
                .to_vec(),
            transition_material: fixture_receipt
                .core_transition_evidence
                .transition_material
                .as_bytes()
                .to_vec(),
            tree_proof: fixture_receipt
                .core_transition_evidence
                .tree_proof
                .as_bytes()
                .to_vec(),
            deletion_state_material: match &fixture_receipt.core_transition_evidence.deletion_state_material {
                FixtureDeletionStateMaterial::TombstonedPosition { tombstoned_position } => {
                    DeletionStateMaterial::TombstonedPosition(
                        tombstoned_position.as_bytes().to_vec(),
                    )
                }
                FixtureDeletionStateMaterial::EmptyPosition { empty_position } => {
                    DeletionStateMaterial::EmptyPosition(empty_position.as_bytes().to_vec())
                }
            },
        },
        certification_provenance: CertificationProvenanceBlock {
            posture: fixture_receipt.certification_provenance.posture.clone(),
            route: fixture_receipt.certification_provenance.route.clone(),
            certification_material: fixture_receipt
                .certification_provenance
                .certification_material
                .as_ref()
                .map(|value| value.as_bytes().to_vec()),
            provenance_material: fixture_receipt
                .certification_provenance
                .provenance_material
                .as_ref()
                .map(|value| value.as_bytes().to_vec()),
            route_context_material: fixture_receipt
                .certification_provenance
                .route_context_material
                .as_ref()
                .map(|value| value.as_bytes().to_vec()),
        },
    }
}

impl LibraryScaffold {
    pub fn get_status(&self) -> Result<StatusSurface, LibraryScaffoldError> {
        // TODO: wire status publication to library-owned predicates and adapter-host state.
        Err(LibraryScaffoldError::NotImplemented("get_status"))
    }

    pub fn get_evidence_readiness(&self) -> Result<EvidenceReadiness, LibraryScaffoldError> {
        // TODO: implement protocol predicate evaluation without duplicating blocked diagnosis.
        Err(LibraryScaffoldError::NotImplemented("get_evidence_readiness"))
    }

    pub fn get_version_info(&self) -> Result<VersionInfo, LibraryScaffoldError> {
        // TODO: return live version information once version wiring is authorized.
        Err(LibraryScaffoldError::NotImplemented("get_version_info"))
    }

    pub fn check_version_support(
        &self,
        _version: &SemanticVersion,
    ) -> Result<VersionCheckResult, LibraryScaffoldError> {
        // TODO: implement explicit multi-surface compatibility policy dispatch.
        Err(LibraryScaffoldError::NotImplemented("check_version_support"))
    }

    pub fn get_receipt(&self, _subject_reference: &[u8]) -> Result<Receipt, LibraryScaffoldError> {
        // TODO: wire receipt retrieval to authorized host/library integration once cryptographic logic is in scope.
        Err(LibraryScaffoldError::NotImplemented("get_receipt"))
    }
}
