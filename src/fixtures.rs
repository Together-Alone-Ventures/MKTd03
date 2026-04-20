use crate::adapter::{AdapterBlockedReason, AdapterBlockedReasonCode, AdapterErrorCode};
use crate::library::{
    BlockedReason, CertificationProvenancePosture, CertificationProvenanceRoute, Compatibility,
    EvidenceReadiness, LifecycleState, OperationContext, SemanticVersion, VersionInfo,
};
use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FixtureSurface {
    Library,
    Adapter,
    Verifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FixturePolarity {
    Positive,
    Negative,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureIndexEntry {
    pub filename: String,
    pub surface: FixtureSurface,
    pub polarity: FixturePolarity,
    pub family: String,
    pub target_method: String,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureIndex {
    pub fixture_index_version: String,
    pub entries: Vec<FixtureIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureEnvelope {
    pub fixture_id: String,
    pub fixture_schema_version: String,
    pub surface: FixtureSurface,
    pub polarity: FixturePolarity,
    pub family: String,
    pub case_id: String,
    pub title: String,
    pub authority_refs: Vec<String>,
    pub target_method: String,
    pub protocol_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub rules_version_ref: String,
    pub input_summary: Value,
    pub expected_outcome: Value,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedFixtureDocument {
    pub envelope: FixtureEnvelope,
    pub case: TypedFixtureCase,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypedFixtureCase {
    LibraryPositiveStatus(LibraryStatusFixture),
    LibraryPositiveReceipt(LibraryPositiveReceiptFixture),
    LibraryPositiveVersionSupport(LibraryVersionSupportFixture),
    LibraryNegativeReceiptError(LibraryReceiptErrorFixture),
    LibraryNegativeBlockedStatus(LibraryStatusFixture),
    LibraryNegativeEvidenceReadiness(LibraryEvidenceReadinessFixture),
    VerifierNegativeReceipt(VerifierReceiptFixture),
    AdapterPositiveSubjectScopeResolution(AdapterPositiveSubjectScopeFixture),
    AdapterNegativeError(AdapterErrorFixture),
    AdapterNegativeBlockedStatus(AdapterBlockedStatusFixture),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct GenericInputSummary {
    #[serde(default)]
    pub method_args: Value,
    pub semantic_context: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureBuildIdentity {
    pub build_version: SemanticVersion,
    pub build_label: Option<String>,
    pub module_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureStatusSurface {
    pub protocol_version: SemanticVersion,
    pub status_schema_version: SemanticVersion,
    pub interface_version: SemanticVersion,
    pub build_identity: FixtureBuildIdentity,
    pub lifecycle_state: LifecycleState,
    pub is_blocked: bool,
    pub blocked_reason: Option<BlockedReason>,
    pub compatibility: Compatibility,
    pub operation_context: Option<OperationContext>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibraryStatusExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub status_surface: FixtureStatusSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryStatusFixture {
    pub semantic_context: String,
    pub expected: LibraryStatusExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum FixtureDeletionStateMaterial {
    TombstonedPosition { tombstoned_position: String },
    EmptyPosition { empty_position: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureCoreTransitionEvidence {
    pub subject_reference: String,
    pub scope_reference: Option<String>,
    pub pre_state_commitment: String,
    pub post_state_commitment: String,
    pub transition_material: String,
    pub transition_derivation_version: SemanticVersion,
    pub tree_proof: String,
    pub deletion_state_material: FixtureDeletionStateMaterial,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureCertificationProvenanceBlock {
    pub posture: CertificationProvenancePosture,
    pub route: CertificationProvenanceRoute,
    pub certification_material: Option<String>,
    pub provenance_material: Option<String>,
    pub route_context_material: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureReceipt {
    pub protocol_version: SemanticVersion,
    pub receipt_version: SemanticVersion,
    pub core_transition_evidence: FixtureCoreTransitionEvidence,
    pub certification_provenance: FixtureCertificationProvenanceBlock,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibraryPositiveReceiptExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub receipt: FixtureReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryPositiveReceiptFixture {
    pub semantic_context: String,
    pub expected: LibraryPositiveReceiptExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibraryVersionSupportExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub version_info: VersionInfo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryVersionSupportFixture {
    pub semantic_context: String,
    pub expected: LibraryVersionSupportExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibraryReceiptErrorExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub error_code: crate::library::ReceiptError,
    pub must_fail_loud: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryReceiptErrorFixture {
    pub semantic_context: String,
    pub expected: LibraryReceiptErrorExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibraryEvidenceReadinessExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub enum_value: EvidenceReadiness,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryEvidenceReadinessFixture {
    pub semantic_context: String,
    pub expected: LibraryEvidenceReadinessExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct VerifierInputSummary {
    pub receipt_artifact_under_validation: FixtureReceipt,
    pub semantic_context: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct VerifierExpectedOutcome {
    pub primary_class: String,
    pub family: String,
    pub must_fail_loud: bool,
    pub validation_outcome: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifierReceiptFixture {
    pub input: VerifierInputSummary,
    pub expected: VerifierExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AdapterErrorExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub error_code: AdapterErrorCode,
    pub must_fail_loud: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureSubjectScope {
    pub subject_reference: String,
    pub scope_reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AdapterPositiveSubjectScopeExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub subject_scope: FixtureSubjectScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterPositiveSubjectScopeFixture {
    pub semantic_context: String,
    pub expected: AdapterPositiveSubjectScopeExpectedOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterErrorFixture {
    pub semantic_context: String,
    pub expected: AdapterErrorExpectedOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FixtureAdapterStatusFacts {
    pub contract_version: SemanticVersion,
    pub is_blocked: bool,
    pub blocked_reason: Option<AdapterBlockedReason>,
    pub progress_material: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AdapterBlockedStatusExpectedOutcome {
    pub result_variant: String,
    pub primary_class: String,
    pub status_facts: FixtureAdapterStatusFacts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterBlockedStatusFixture {
    pub semantic_context: String,
    pub expected: AdapterBlockedStatusExpectedOutcome,
}

#[derive(Debug)]
pub enum FixtureError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Validation(String),
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "fixture io error: {error}"),
            Self::Json(error) => write!(f, "fixture json error: {error}"),
            Self::Validation(error) => write!(f, "fixture validation error: {error}"),
        }
    }
}

impl std::error::Error for FixtureError {}

impl From<std::io::Error> for FixtureError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for FixtureError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn load_fixture_index(path: impl AsRef<Path>) -> Result<FixtureIndex, FixtureError> {
    let raw = fs::read_to_string(path)?;
    let index: FixtureIndex = serde_json::from_str(&raw)?;
    if index.entries.is_empty() {
        return Err(FixtureError::Validation(
            "fixture index must contain at least one entry".to_string(),
        ));
    }
    Ok(index)
}

pub fn load_fixture(path: impl AsRef<Path>) -> Result<FixtureEnvelope, FixtureError> {
    let raw = fs::read_to_string(path.as_ref())?;
    let fixture: FixtureEnvelope = serde_json::from_str(&raw)?;
    validate_fixture_envelope(path.as_ref(), &fixture)?;
    Ok(fixture)
}

pub fn load_typed_fixture(path: impl AsRef<Path>) -> Result<TypedFixtureDocument, FixtureError> {
    let path = path.as_ref();
    let envelope = load_fixture(path)?;
    let case = parse_typed_case(path, &envelope)?;
    Ok(TypedFixtureDocument { envelope, case })
}

pub fn load_all_typed_fixtures_from_index(
    index_path: impl AsRef<Path>,
) -> Result<Vec<TypedFixtureDocument>, FixtureError> {
    let index_path = index_path.as_ref();
    let index = load_fixture_index(index_path)?;
    let base_dir = index_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut documents = Vec::with_capacity(index.entries.len());

    for entry in &index.entries {
        let full_path = validate_index_entry(base_dir, entry)?;
        documents.push(load_typed_fixture(full_path)?);
    }

    Ok(documents)
}

pub fn validate_fixture_envelope(path: &Path, fixture: &FixtureEnvelope) -> Result<(), FixtureError> {
    if fixture.authority_refs.is_empty() {
        return Err(FixtureError::Validation(format!(
            "{} has no authority_refs",
            path.display()
        )));
    }
    if fixture.rules_version_ref.is_empty() {
        return Err(FixtureError::Validation(format!(
            "{} has empty rules_version_ref",
            path.display()
        )));
    }
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| FixtureError::Validation(format!("{} has invalid filename", path.display())))?;
    if fixture.fixture_id != stem {
        return Err(FixtureError::Validation(format!(
            "{} fixture_id {} does not match filename stem {}",
            path.display(),
            fixture.fixture_id,
            stem
        )));
    }
    validate_target_method(&fixture.surface, &fixture.target_method, path)?;
    Ok(())
}

pub fn validate_index_entry(base_dir: &Path, entry: &FixtureIndexEntry) -> Result<PathBuf, FixtureError> {
    let full_path = base_dir.join(&entry.filename);
    if !full_path.exists() {
        return Err(FixtureError::Validation(format!(
            "fixture file {} does not exist",
            full_path.display()
        )));
    }
    validate_target_method(&entry.surface, &entry.target_method, &full_path)?;
    if entry.status.is_empty() {
        return Err(FixtureError::Validation(format!(
            "fixture entry {} has empty status",
            full_path.display()
        )));
    }
    Ok(full_path)
}

fn parse_typed_case(path: &Path, envelope: &FixtureEnvelope) -> Result<TypedFixtureCase, FixtureError> {
    match (&envelope.surface, &envelope.polarity, envelope.family.as_str()) {
        (FixtureSurface::Library, FixturePolarity::Positive, "status") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryStatusExpectedOutcome>(path, envelope)?;
            validate_status_surface(path, &expected.status_surface)?;
            require(
                envelope.target_method == "get_status",
                path,
                "positive status fixture must target get_status",
            )?;
            require(expected.result_variant == "status_surface", path, "positive status fixture must use result_variant status_surface")?;
            require(expected.primary_class == "ready", path, "positive status fixture must use primary_class ready")?;
            require(!expected.status_surface.is_blocked, path, "positive status fixture must not be blocked")?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "status_surface"],
                "positive status fixture must use the status-surface shape only",
            )?;
            Ok(TypedFixtureCase::LibraryPositiveStatus(LibraryStatusFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        (FixtureSurface::Library, FixturePolarity::Positive, "receipt") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryPositiveReceiptExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "get_receipt",
                path,
                "positive receipt fixture must target get_receipt",
            )?;
            require(expected.result_variant == "ok", path, "positive receipt fixture must return ok")?;
            require(expected.primary_class == "receipt_returned", path, "positive receipt fixture must use primary_class receipt_returned")?;
            require(
                certification_shape_is_consistent(&expected.receipt.certification_provenance),
                path,
                "positive receipt fixture must use a shape-consistent certification/provenance block",
            )?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "receipt"],
                "positive receipt fixture must remain a receipt-return fixture and must not masquerade as verifier invalidity",
            )?;
            Ok(TypedFixtureCase::LibraryPositiveReceipt(LibraryPositiveReceiptFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        (FixtureSurface::Library, FixturePolarity::Positive, "version_support") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryVersionSupportExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "check_version_support",
                path,
                "positive version support fixture must target check_version_support",
            )?;
            require(expected.result_variant == "supported", path, "positive version support fixture must return supported")?;
            require(expected.primary_class == "supported", path, "positive version support fixture must use primary_class supported")?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "version_info"],
                "positive version support fixture must use the version-info shape only",
            )?;
            Ok(TypedFixtureCase::LibraryPositiveVersionSupport(LibraryVersionSupportFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        (FixtureSurface::Library, FixturePolarity::Negative, "unsupported_version")
        | (FixtureSurface::Library, FixturePolarity::Negative, "not_found")
        | (FixtureSurface::Library, FixturePolarity::Negative, "not_yet_issued") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryReceiptErrorExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "get_receipt",
                path,
                "library retrieval negative fixture must target get_receipt",
            )?;
            require(expected.result_variant == "err", path, "library retrieval negative fixture must return err")?;
            require(expected.must_fail_loud, path, "library retrieval negative fixture must fail loud")?;
            require(
                matches!(
                    expected.error_code,
                    crate::library::ReceiptError::UnsupportedVersion
                        | crate::library::ReceiptError::NotFound
                        | crate::library::ReceiptError::NotYetIssued
                ),
                path,
                "library retrieval negative fixture must use a retrieval error from the frozen interface",
            )?;
            require(
                expected.primary_class == envelope.family,
                path,
                "library retrieval negative primary_class must match fixture family",
            )?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "error_code", "must_fail_loud"],
                "library retrieval negative fixture must stay on the retrieval-result surface and must not masquerade as verifier invalidity",
            )?;
            Ok(TypedFixtureCase::LibraryNegativeReceiptError(LibraryReceiptErrorFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        (FixtureSurface::Library, FixturePolarity::Negative, "blocked_status") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryStatusExpectedOutcome>(path, envelope)?;
            validate_status_surface(path, &expected.status_surface)?;
            require(
                envelope.target_method == "get_status",
                path,
                "blocked status fixture must target get_status",
            )?;
            require(expected.result_variant == "status_surface", path, "blocked status fixture must use result_variant status_surface")?;
            require(expected.primary_class == "blocked_status", path, "blocked status fixture must use primary_class blocked_status")?;
            require(expected.status_surface.is_blocked, path, "blocked status fixture must have is_blocked true")?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "status_surface"],
                "blocked status fixture must use the status-surface shape only",
            )?;
            Ok(TypedFixtureCase::LibraryNegativeBlockedStatus(LibraryStatusFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        (FixtureSurface::Library, FixturePolarity::Negative, "rebuild_required")
        | (FixtureSurface::Library, FixturePolarity::Negative, "not_evidence_ready") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<LibraryEvidenceReadinessExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "get_evidence_readiness",
                path,
                "EvidenceReadiness fixture must target get_evidence_readiness",
            )?;
            validate_evidence_readiness_expected(path, envelope, &expected)?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "enum_value"],
                "EvidenceReadiness fixture must remain narrow and must not carry blocked diagnosis fields",
            )?;
            Ok(TypedFixtureCase::LibraryNegativeEvidenceReadiness(
                LibraryEvidenceReadinessFixture {
                    semantic_context: input.semantic_context,
                    expected,
                },
            ))
        }
        (FixtureSurface::Verifier, FixturePolarity::Negative, "wrong_tree_proof")
        | (FixtureSurface::Verifier, FixturePolarity::Negative, "wrong_commitment_relationship")
        | (FixtureSurface::Verifier, FixturePolarity::Negative, "malformed_certification_provenance")
        | (
            FixtureSurface::Verifier,
            FixturePolarity::Negative,
            "missing_transition_derivation_version",
        )
        | (FixtureSurface::Verifier, FixturePolarity::Negative, "receipt_subject_scope_mismatch") => {
            require(
                envelope.target_method == "receipt_validation",
                path,
                "verifier fixtures must use receipt_validation",
            )?;
            let input = parse_input_summary::<VerifierInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<VerifierExpectedOutcome>(path, envelope)?;
            require(expected.primary_class == "invalid_evidence", path, "verifier invalidity fixture must use primary_class invalid_evidence")?;
            require(expected.family == envelope.family, path, "verifier expected family must match fixture family")?;
            require(expected.must_fail_loud, path, "verifier invalidity fixture must fail loud")?;
            require(expected.validation_outcome == "reject_receipt_artifact", path, "verifier invalidity fixture must reject receipt artifact")?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["primary_class", "family", "must_fail_loud", "validation_outcome"],
                "verifier invalidity fixture must stay on the verifier-input shape only",
            )?;
            let cert_consistent =
                certification_shape_is_consistent(&input.receipt_artifact_under_validation.certification_provenance);
            if envelope.family == "malformed_certification_provenance" {
                require(
                    !cert_consistent,
                    path,
                    "malformed_certification_provenance fixture must intentionally use an inconsistent certification/provenance shape",
                )?;
            } else {
                require(
                    cert_consistent,
                    path,
                    "non-malformed verifier fixtures must use a shape-consistent certification/provenance block",
                )?;
            }
            Ok(TypedFixtureCase::VerifierNegativeReceipt(VerifierReceiptFixture {
                input,
                expected,
            }))
        }
        (FixtureSurface::Adapter, FixturePolarity::Positive, "subject_scope_resolution") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected =
                parse_expected_outcome::<AdapterPositiveSubjectScopeExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "resolve_subject_scope",
                path,
                "adapter positive resolve fixture must target resolve_subject_scope",
            )?;
            require(
                expected.result_variant == "ok",
                path,
                "adapter positive resolve fixture must use result_variant ok",
            )?;
            require(
                expected.primary_class == "subject_scope_resolved",
                path,
                "adapter positive resolve fixture must use primary_class subject_scope_resolved",
            )?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "subject_scope"],
                "adapter positive resolve fixture must use the subject-scope shape only",
            )?;
            Ok(TypedFixtureCase::AdapterPositiveSubjectScopeResolution(
                AdapterPositiveSubjectScopeFixture {
                    semantic_context: input.semantic_context,
                    expected,
                },
            ))
        }
        (FixtureSurface::Adapter, FixturePolarity::Negative, "blocked_boundary_state") => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<AdapterBlockedStatusExpectedOutcome>(path, envelope)?;
            require(
                envelope.target_method == "get_adapter_status_facts",
                path,
                "adapter blocked status fixture must target get_adapter_status_facts",
            )?;
            require(expected.result_variant == "ok", path, "adapter blocked status fixture must use result_variant ok")?;
            require(expected.primary_class == "blocked_boundary_state", path, "adapter blocked status fixture must use primary_class blocked_boundary_state")?;
            require(expected.status_facts.is_blocked, path, "adapter blocked status fixture must have is_blocked true")?;
            require(
                expected.status_facts.blocked_reason.is_some(),
                path,
                "adapter blocked status fixture must carry blocked_reason when blocked",
            )?;
            require(
                matches!(
                    expected
                        .status_facts
                        .blocked_reason
                        .as_ref()
                        .map(|reason| &reason.code),
                    Some(AdapterBlockedReasonCode::RebuildRequired)
                ),
                path,
                "adapter blocked rebuild-required must be represented only through blocked_reason.code = rebuild_required",
            )?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "status_facts"],
                "adapter blocked status fixture must use the status-facts shape only",
            )?;
            Ok(TypedFixtureCase::AdapterNegativeBlockedStatus(
                AdapterBlockedStatusFixture {
                    semantic_context: input.semantic_context,
                    expected,
                },
            ))
        }
        (
            FixtureSurface::Adapter,
            FixturePolarity::Negative,
            "stale_precondition"
                | "transition_mutation_rejected"
                | "invalid_request_material"
                | "scope_not_supported"
                | "subject_not_found"
                | "pre_state_capture_unavailable"
                | "post_state_capture_unavailable"
                | "capability_not_supported"
                | "internal_adapter_failure",
        ) => {
            let input = parse_input_summary::<GenericInputSummary>(path, envelope)?;
            let expected = parse_expected_outcome::<AdapterErrorExpectedOutcome>(path, envelope)?;
            require(expected.result_variant == "err", path, "adapter negative fixture must return err")?;
            require(expected.must_fail_loud, path, "adapter negative fixture must fail loud")?;
            validate_adapter_error_family(path, envelope.family.as_str(), &expected)?;
            validate_adapter_target_method(path, envelope.family.as_str(), envelope.target_method.as_str())?;
            ensure_exact_keys(
                path,
                &envelope.expected_outcome,
                &["result_variant", "primary_class", "error_code", "must_fail_loud"],
                "adapter negative error fixture must use the explicit adapter error shape only",
            )?;
            Ok(TypedFixtureCase::AdapterNegativeError(AdapterErrorFixture {
                semantic_context: input.semantic_context,
                expected,
            }))
        }
        _ => Err(FixtureError::Validation(format!(
            "{} uses unsupported surface/polarity/family combination {:?}/{:?}/{}",
            path.display(),
            envelope.surface,
            envelope.polarity,
            envelope.family
        ))),
    }
}

fn parse_input_summary<T>(path: &Path, envelope: &FixtureEnvelope) -> Result<T, FixtureError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(envelope.input_summary.clone()).map_err(|error| {
        FixtureError::Validation(format!(
            "{} input_summary does not match expected family shape: {}",
            path.display(),
            error
        ))
    })
}

fn parse_expected_outcome<T>(path: &Path, envelope: &FixtureEnvelope) -> Result<T, FixtureError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(envelope.expected_outcome.clone()).map_err(|error| {
        FixtureError::Validation(format!(
            "{} expected_outcome does not match expected family shape: {}",
            path.display(),
            error
        ))
    })
}

fn validate_status_surface(path: &Path, status: &FixtureStatusSurface) -> Result<(), FixtureError> {
    require(
        status.is_blocked == status.blocked_reason.is_some(),
        path,
        "blocked_reason must be present iff is_blocked is true",
    )?;
    Ok(())
}

fn validate_evidence_readiness_expected(
    path: &Path,
    envelope: &FixtureEnvelope,
    expected: &LibraryEvidenceReadinessExpectedOutcome,
) -> Result<(), FixtureError> {
    require(
        expected.result_variant == "evidence_readiness",
        path,
        "EvidenceReadiness fixture must use result_variant evidence_readiness",
    )?;
    require(
        expected.primary_class == envelope.family,
        path,
        "EvidenceReadiness primary_class must match fixture family",
    )?;
    require(
        matches!(
            expected.enum_value,
            EvidenceReadiness::RebuildRequired | EvidenceReadiness::NotEvidenceReady
        ),
        path,
        "EvidenceReadiness fixture must use a narrow non-ready enum value",
    )?;
    Ok(())
}

fn validate_adapter_error_family(
    path: &Path,
    family: &str,
    expected: &AdapterErrorExpectedOutcome,
) -> Result<(), FixtureError> {
    let matches_family = match family {
        "stale_precondition" => matches!(expected.error_code, AdapterErrorCode::StalePrecondition),
        "transition_mutation_rejected" => {
            matches!(expected.error_code, AdapterErrorCode::TransitionMutationRejected)
        }
        "invalid_request_material" => {
            matches!(expected.error_code, AdapterErrorCode::InvalidRequestMaterial)
        }
        "scope_not_supported" => matches!(expected.error_code, AdapterErrorCode::ScopeNotSupported),
        "subject_not_found" => matches!(expected.error_code, AdapterErrorCode::SubjectNotFound),
        "pre_state_capture_unavailable" => {
            matches!(expected.error_code, AdapterErrorCode::PreStateCaptureUnavailable)
        }
        "post_state_capture_unavailable" => {
            matches!(expected.error_code, AdapterErrorCode::PostStateCaptureUnavailable)
        }
        "capability_not_supported" => {
            matches!(expected.error_code, AdapterErrorCode::CapabilityNotSupported)
        }
        "internal_adapter_failure" => {
            matches!(expected.error_code, AdapterErrorCode::InternalAdapterFailure)
        }
        _ => false,
    };
    require(
        matches_family && expected.primary_class == family,
        path,
        "adapter negative fixture primary_class and error_code must match the explicit adapter family",
    )
}

fn validate_adapter_target_method(
    path: &Path,
    family: &str,
    target_method: &str,
) -> Result<(), FixtureError> {
    let matches_method = match family {
        "stale_precondition" | "transition_mutation_rejected" | "internal_adapter_failure" => {
            target_method == "execute_transition_mutation"
        }
        "invalid_request_material" | "scope_not_supported" | "subject_not_found" => {
            target_method == "resolve_subject_scope"
        }
        "pre_state_capture_unavailable" => target_method == "capture_pre_state",
        "post_state_capture_unavailable" => target_method == "capture_post_state",
        "capability_not_supported" => target_method == "get_adapter_capabilities",
        _ => false,
    };

    require(
        matches_method,
        path,
        "adapter negative fixture target_method must match the current explicit adapter family mapping",
    )
}

fn certification_shape_is_consistent(block: &FixtureCertificationProvenanceBlock) -> bool {
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

fn ensure_exact_keys(
    path: &Path,
    value: &Value,
    allowed: &[&str],
    message: &str,
) -> Result<(), FixtureError> {
    let object = value.as_object().ok_or_else(|| {
        FixtureError::Validation(format!("{} expected_outcome is not an object", path.display()))
    })?;
    let allowed: std::collections::BTreeSet<&str> = allowed.iter().copied().collect();
    let actual: std::collections::BTreeSet<&str> = object.keys().map(String::as_str).collect();
    require(actual == allowed, path, message)
}

fn require(condition: bool, path: &Path, message: &str) -> Result<(), FixtureError> {
    if condition {
        Ok(())
    } else {
        Err(FixtureError::Validation(format!(
            "{} {}",
            path.display(),
            message
        )))
    }
}

fn validate_target_method(
    surface: &FixtureSurface,
    target_method: &str,
    path: &Path,
) -> Result<(), FixtureError> {
    let allowed = match surface {
        FixtureSurface::Library => &[
            "get_status",
            "get_evidence_readiness",
            "get_version_info",
            "check_version_support",
            "get_receipt",
        ][..],
        FixtureSurface::Adapter => &[
            "resolve_subject_scope",
            "capture_pre_state",
            "execute_transition_mutation",
            "capture_post_state",
            "get_adapter_status_facts",
            "get_adapter_capabilities",
        ][..],
        FixtureSurface::Verifier => &["receipt_validation"][..],
    };
    if allowed.contains(&target_method) {
        Ok(())
    } else {
        Err(FixtureError::Validation(format!(
            "{} has invalid target_method {} for surface {:?}",
            path.display(),
            target_method,
            surface
        )))
    }
}
