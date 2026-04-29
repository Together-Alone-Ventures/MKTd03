use crate::fixtures::{load_all_typed_fixtures_from_index, GenericInputSummary, TypedFixtureCase};
use crate::library::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubjectScope {
    pub subject_reference: Vec<u8>,
    pub scope_reference: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCapability {
    SubjectScopeResolution,
    PreStateCapture,
    PostStateCapture,
    TransitionMutationExecution,
    StatusFacts,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterCapabilityReport {
    pub contract_version: SemanticVersion,
    pub supported_capabilities: Vec<AdapterCapability>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterErrorCode {
    InvalidRequestMaterial,
    SubjectNotFound,
    ScopeNotSupported,
    CapabilityNotSupported,
    PreStateCaptureUnavailable,
    PostStateCaptureUnavailable,
    TransitionMutationRejected,
    StalePrecondition,
    InitialisationIncomplete,
    RebuildRequired,
    Blocked,
    InternalAdapterFailure,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterError {
    pub code: AdapterErrorCode,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubjectScopeRequest {
    pub request_material: Vec<u8>,
    pub operation_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateCapture {
    pub subject_scope: SubjectScope,
    pub state_material: Vec<u8>,
    pub capture_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransitionMutationRequest {
    pub subject_scope: SubjectScope,
    pub mutation_material: Vec<u8>,
    pub operation_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransitionMutationApplied {
    pub mutation_applied: bool,
    pub result_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterBlockedReasonCode {
    InitialisationIncomplete,
    RebuildRequired,
    BlockedByHostPolicy,
    UnsupportedBoundaryCondition,
    UnknownBlockedState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterBlockedReason {
    pub code: AdapterBlockedReasonCode,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterStatusFacts {
    pub contract_version: SemanticVersion,
    pub is_blocked: bool,
    pub blocked_reason: Option<AdapterBlockedReason>,
    pub progress_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterResult<T> {
    Ok(T),
    Err(AdapterError),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SubjectScopeKey {
    subject_reference: Vec<u8>,
    scope_reference: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ResolveSubjectScopeKey {
    request_material: Vec<u8>,
    operation_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ResolveSubjectScopeFixtureOutcome {
    Ok(SubjectScope),
    Err(AdapterError),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct TransitionMutationKey {
    subject_scope: SubjectScopeKey,
    mutation_material: Vec<u8>,
    operation_context_material: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceAdapterRuntimeError {
    FixtureLoad(String),
    MissingConfiguration(&'static str),
    MissingFixture(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceAdapterRuntime {
    pub contract_version: SemanticVersion,
    pub supported_capabilities: Vec<AdapterCapability>,
    pub status_fixtures: BTreeMap<String, AdapterStatusFacts>,
    pub selected_status_fixture_id: Option<String>,
    pub capability_error_fixtures: BTreeMap<String, AdapterError>,
    pub selected_capability_error_fixture_id: Option<String>,
    resolve_subject_scope_outcomes:
        BTreeMap<ResolveSubjectScopeKey, ResolveSubjectScopeFixtureOutcome>,
    capture_pre_state_errors: BTreeMap<SubjectScopeKey, AdapterError>,
    capture_post_state_errors: BTreeMap<SubjectScopeKey, AdapterError>,
    transition_mutation_error_fixtures: BTreeMap<String, (TransitionMutationKey, AdapterError)>,
    pub selected_transition_mutation_fixture_id: Option<String>,
}

pub trait AdapterPort {
    fn resolve_subject_scope(&self, request: &SubjectScopeRequest) -> AdapterResult<SubjectScope>;

    fn capture_pre_state(&self, subject_scope: &SubjectScope) -> AdapterResult<StateCapture>;

    fn execute_transition_mutation(
        &self,
        request: &TransitionMutationRequest,
    ) -> AdapterResult<TransitionMutationApplied>;

    fn capture_post_state(&self, subject_scope: &SubjectScope) -> AdapterResult<StateCapture>;

    fn get_adapter_status_facts(&self) -> AdapterResult<AdapterStatusFacts>;

    fn get_adapter_capabilities(&self) -> AdapterResult<AdapterCapabilityReport>;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct ResolveSubjectScopeMethodArgs {
    request_material: String,
    operation_context_material: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct CaptureMethodArgs {
    subject_reference: String,
    scope_reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct FixtureSubjectScopeArgs {
    subject_reference: String,
    scope_reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct TransitionMutationMethodArgs {
    subject_scope: FixtureSubjectScopeArgs,
    mutation_material: String,
    operation_context_material: Option<String>,
}

pub fn validate_adapter_status_facts(
    status_facts: &AdapterStatusFacts,
) -> Result<(), ReferenceAdapterRuntimeError> {
    if status_facts.is_blocked != status_facts.blocked_reason.is_some() {
        return Err(ReferenceAdapterRuntimeError::FixtureLoad(
            "adapter blocked_reason must be present iff is_blocked is true".to_string(),
        ));
    }

    if matches!(
        status_facts
            .blocked_reason
            .as_ref()
            .map(|reason| &reason.code),
        Some(AdapterBlockedReasonCode::RebuildRequired)
    ) && !status_facts.is_blocked
    {
        return Err(ReferenceAdapterRuntimeError::FixtureLoad(
            "adapter rebuild_required must be represented only through blocked-state signaling"
                .to_string(),
        ));
    }

    Ok(())
}

impl ReferenceAdapterRuntime {
    pub fn from_fixture_index(
        index_path: impl AsRef<Path>,
        supported_capabilities: Vec<AdapterCapability>,
    ) -> Result<Self, ReferenceAdapterRuntimeError> {
        let fixtures = load_all_typed_fixtures_from_index(index_path)
            .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;
        Self::from_typed_fixtures(&fixtures, supported_capabilities)
    }

    pub fn from_typed_fixtures(
        fixtures: &[crate::fixtures::TypedFixtureDocument],
        supported_capabilities: Vec<AdapterCapability>,
    ) -> Result<Self, ReferenceAdapterRuntimeError> {
        let mut observed_contract_versions = Vec::new();
        let mut status_fixtures = BTreeMap::new();
        let mut selected_status_fixture_id = None;
        let mut capability_error_fixtures = BTreeMap::new();
        let mut resolve_subject_scope_outcomes = BTreeMap::new();
        let mut capture_pre_state_errors = BTreeMap::new();
        let mut capture_post_state_errors = BTreeMap::new();
        let mut transition_mutation_error_fixtures = BTreeMap::new();

        for fixture in fixtures {
            match &fixture.case {
                TypedFixtureCase::AdapterPositiveSubjectScopeResolution(positive_fixture) => {
                    let key = parse_resolve_subject_scope_key(&fixture.envelope)?;
                    let subject_scope = SubjectScope {
                        subject_reference: positive_fixture
                            .expected
                            .subject_scope
                            .subject_reference
                            .as_bytes()
                            .to_vec(),
                        scope_reference: positive_fixture
                            .expected
                            .subject_scope
                            .scope_reference
                            .as_ref()
                            .map(|value| value.as_bytes().to_vec()),
                    };
                    insert_unique_lookup_key(
                        &mut resolve_subject_scope_outcomes,
                        key,
                        ResolveSubjectScopeFixtureOutcome::Ok(subject_scope),
                        "resolve_subject_scope outcome key",
                    )?;
                }
                TypedFixtureCase::AdapterNegativeBlockedStatus(blocked_fixture) => {
                    let status_facts = blocked_fixture.expected.status_facts.clone();
                    let materialized = AdapterStatusFacts {
                        contract_version: status_facts.contract_version.clone(),
                        is_blocked: status_facts.is_blocked,
                        blocked_reason: status_facts.blocked_reason.clone(),
                        progress_material: status_facts
                            .progress_material
                            .as_ref()
                            .map(|value| value.as_bytes().to_vec()),
                    };
                    validate_adapter_status_facts(&materialized)?;
                    observe_contract_version(
                        &mut observed_contract_versions,
                        &materialized.contract_version,
                    );
                    if selected_status_fixture_id.is_none() {
                        selected_status_fixture_id = Some(fixture.envelope.fixture_id.clone());
                    }
                    insert_unique_fixture_id(
                        &mut status_fixtures,
                        fixture.envelope.fixture_id.clone(),
                        materialized,
                        "adapter status fixture id",
                    )?;
                }
                TypedFixtureCase::AdapterNegativeError(error_fixture) => {
                    let adapter_error = AdapterError {
                        code: error_fixture.expected.error_code.clone(),
                        description: error_fixture.semantic_context.clone(),
                    };

                    match error_fixture.expected.error_code {
                        AdapterErrorCode::InvalidRequestMaterial
                        | AdapterErrorCode::ScopeNotSupported
                        | AdapterErrorCode::SubjectNotFound => {
                            let key = parse_resolve_subject_scope_key(&fixture.envelope)?;
                            insert_unique_lookup_key(
                                &mut resolve_subject_scope_outcomes,
                                key,
                                ResolveSubjectScopeFixtureOutcome::Err(adapter_error),
                                "resolve_subject_scope outcome key",
                            )?;
                        }
                        AdapterErrorCode::PreStateCaptureUnavailable => {
                            let key = parse_capture_key(&fixture.envelope)?;
                            insert_unique_lookup_key(
                                &mut capture_pre_state_errors,
                                key,
                                adapter_error,
                                "capture_pre_state error key",
                            )?;
                        }
                        AdapterErrorCode::PostStateCaptureUnavailable => {
                            let key = parse_capture_key(&fixture.envelope)?;
                            insert_unique_lookup_key(
                                &mut capture_post_state_errors,
                                key,
                                adapter_error,
                                "capture_post_state error key",
                            )?;
                        }
                        AdapterErrorCode::StalePrecondition
                        | AdapterErrorCode::TransitionMutationRejected
                        | AdapterErrorCode::InternalAdapterFailure => {
                            let key = parse_transition_mutation_key(&fixture.envelope)?;
                            insert_unique_fixture_id(
                                &mut transition_mutation_error_fixtures,
                                fixture.envelope.fixture_id.clone(),
                                (key, adapter_error),
                                "transition_mutation fixture id",
                            )?;
                        }
                        AdapterErrorCode::CapabilityNotSupported => {
                            insert_unique_fixture_id(
                                &mut capability_error_fixtures,
                                fixture.envelope.fixture_id.clone(),
                                adapter_error,
                                "adapter capability fixture id",
                            )?;
                        }
                        AdapterErrorCode::InitialisationIncomplete
                        | AdapterErrorCode::RebuildRequired
                        | AdapterErrorCode::Blocked => {}
                    }
                }
                _ => {}
            }
        }

        let contract_version = match observed_contract_versions.len() {
            0 => {
                return Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                    "adapter contract version fixture is not configured",
                ))
            }
            1 => observed_contract_versions.remove(0),
            _ => {
                return Err(ReferenceAdapterRuntimeError::FixtureLoad(
                    "inconsistent adapter contract versions observed across contract-version-bearing fixtures"
                        .to_string(),
                ))
            }
        };

        Ok(Self {
            contract_version,
            supported_capabilities,
            status_fixtures,
            selected_status_fixture_id,
            capability_error_fixtures,
            selected_capability_error_fixture_id: None,
            resolve_subject_scope_outcomes,
            capture_pre_state_errors,
            capture_post_state_errors,
            transition_mutation_error_fixtures,
            selected_transition_mutation_fixture_id: None,
        })
    }

    pub fn select_status_fixture(
        &mut self,
        fixture_id: impl Into<String>,
    ) -> Result<(), ReferenceAdapterRuntimeError> {
        let fixture_id = fixture_id.into();
        if self.status_fixtures.contains_key(&fixture_id) {
            self.selected_status_fixture_id = Some(fixture_id);
            Ok(())
        } else {
            Err(ReferenceAdapterRuntimeError::MissingFixture(fixture_id))
        }
    }

    pub fn select_capability_error_fixture(
        &mut self,
        fixture_id: impl Into<String>,
    ) -> Result<(), ReferenceAdapterRuntimeError> {
        let fixture_id = fixture_id.into();
        if self.capability_error_fixtures.contains_key(&fixture_id) {
            self.selected_capability_error_fixture_id = Some(fixture_id);
            Ok(())
        } else {
            Err(ReferenceAdapterRuntimeError::MissingFixture(fixture_id))
        }
    }

    pub fn clear_capability_error_fixture(&mut self) {
        self.selected_capability_error_fixture_id = None;
    }

    pub fn select_transition_mutation_fixture(
        &mut self,
        fixture_id: impl Into<String>,
    ) -> Result<(), ReferenceAdapterRuntimeError> {
        let fixture_id = fixture_id.into();
        if self
            .transition_mutation_error_fixtures
            .contains_key(&fixture_id)
        {
            self.selected_transition_mutation_fixture_id = Some(fixture_id);
            Ok(())
        } else {
            Err(ReferenceAdapterRuntimeError::MissingFixture(fixture_id))
        }
    }

    pub fn clear_transition_mutation_fixture(&mut self) {
        self.selected_transition_mutation_fixture_id = None;
    }

    pub fn resolve_subject_scope(
        &self,
        request: &SubjectScopeRequest,
    ) -> Result<AdapterResult<SubjectScope>, ReferenceAdapterRuntimeError> {
        let key = resolve_subject_scope_key(request);
        Ok(match self.resolve_subject_scope_outcomes.get(&key) {
            Some(ResolveSubjectScopeFixtureOutcome::Ok(subject_scope)) => {
                AdapterResult::Ok(subject_scope.clone())
            }
            Some(ResolveSubjectScopeFixtureOutcome::Err(error)) => AdapterResult::Err(error.clone()),
            None => {
                return Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                    "no fixture-backed resolve_subject_scope success path or matching error is configured",
                ))
            }
        })
    }

    pub fn capture_pre_state(
        &self,
        subject_scope: &SubjectScope,
    ) -> Result<AdapterResult<StateCapture>, ReferenceAdapterRuntimeError> {
        let key = subject_scope_key(subject_scope);
        Ok(match self.capture_pre_state_errors.get(&key) {
            Some(error) => AdapterResult::Err(error.clone()),
            None => return Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                "no fixture-backed pre-state capture success path or matching error is configured",
            )),
        })
    }

    pub fn capture_post_state(
        &self,
        subject_scope: &SubjectScope,
    ) -> Result<AdapterResult<StateCapture>, ReferenceAdapterRuntimeError> {
        let key = subject_scope_key(subject_scope);
        Ok(match self.capture_post_state_errors.get(&key) {
            Some(error) => AdapterResult::Err(error.clone()),
            None => return Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                "no fixture-backed post-state capture success path or matching error is configured",
            )),
        })
    }

    pub fn execute_transition_mutation(
        &self,
        request: &TransitionMutationRequest,
    ) -> Result<AdapterResult<TransitionMutationApplied>, ReferenceAdapterRuntimeError> {
        let key = transition_mutation_key(request);
        if let Some(fixture_id) = &self.selected_transition_mutation_fixture_id {
            let (fixture_key, error) = self
                .transition_mutation_error_fixtures
                .get(fixture_id)
                .ok_or_else(|| ReferenceAdapterRuntimeError::MissingFixture(fixture_id.clone()))?;
            if fixture_key == &key {
                return Ok(AdapterResult::Err(error.clone()));
            }
            return Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                "selected transition-mutation fixture does not match the supplied request",
            ));
        }

        let mut matches = self
            .transition_mutation_error_fixtures
            .values()
            .filter(|(fixture_key, _)| fixture_key == &key)
            .map(|(_, error)| error.clone());

        match (matches.next(), matches.next()) {
            (Some(error), None) => Ok(AdapterResult::Err(error)),
            (Some(_), Some(_)) => Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                "multiple fixture-backed transition-mutation outcomes exist for this request; select one explicitly",
            )),
            (None, _) => Err(ReferenceAdapterRuntimeError::MissingConfiguration(
                "no fixture-backed transition-mutation success path or matching error is configured",
            )),
        }
    }

    pub fn get_adapter_status_facts(
        &self,
    ) -> Result<AdapterResult<AdapterStatusFacts>, ReferenceAdapterRuntimeError> {
        let fixture_id = self.selected_status_fixture_id.as_ref().ok_or(
            ReferenceAdapterRuntimeError::MissingConfiguration(
                "adapter status fixture selection is not configured",
            ),
        )?;
        let status_facts = self
            .status_fixtures
            .get(fixture_id)
            .cloned()
            .ok_or_else(|| ReferenceAdapterRuntimeError::MissingFixture(fixture_id.clone()))?;
        Ok(AdapterResult::Ok(status_facts))
    }

    pub fn get_adapter_capabilities(
        &self,
    ) -> Result<AdapterResult<AdapterCapabilityReport>, ReferenceAdapterRuntimeError> {
        if let Some(fixture_id) = &self.selected_capability_error_fixture_id {
            let error = self
                .capability_error_fixtures
                .get(fixture_id)
                .cloned()
                .ok_or_else(|| ReferenceAdapterRuntimeError::MissingFixture(fixture_id.clone()))?;
            return Ok(AdapterResult::Err(error));
        }

        Ok(AdapterResult::Ok(AdapterCapabilityReport {
            contract_version: self.contract_version.clone(),
            supported_capabilities: self.supported_capabilities.clone(),
        }))
    }
}

fn parse_capture_key(
    envelope: &crate::fixtures::FixtureEnvelope,
) -> Result<SubjectScopeKey, ReferenceAdapterRuntimeError> {
    let input_summary: GenericInputSummary = serde_json::from_value(envelope.input_summary.clone())
        .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;
    let method_args: CaptureMethodArgs = serde_json::from_value(input_summary.method_args)
        .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;

    Ok(SubjectScopeKey {
        subject_reference: method_args.subject_reference.as_bytes().to_vec(),
        scope_reference: method_args
            .scope_reference
            .as_ref()
            .map(|value| value.as_bytes().to_vec()),
    })
}

fn parse_resolve_subject_scope_key(
    envelope: &crate::fixtures::FixtureEnvelope,
) -> Result<ResolveSubjectScopeKey, ReferenceAdapterRuntimeError> {
    let input_summary: GenericInputSummary = serde_json::from_value(envelope.input_summary.clone())
        .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;
    let method_args: ResolveSubjectScopeMethodArgs =
        serde_json::from_value(input_summary.method_args)
            .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;

    Ok(ResolveSubjectScopeKey {
        request_material: method_args.request_material.as_bytes().to_vec(),
        operation_context_material: method_args
            .operation_context_material
            .as_ref()
            .map(|value| value.as_bytes().to_vec()),
    })
}

fn parse_transition_mutation_key(
    envelope: &crate::fixtures::FixtureEnvelope,
) -> Result<TransitionMutationKey, ReferenceAdapterRuntimeError> {
    let input_summary: GenericInputSummary = serde_json::from_value(envelope.input_summary.clone())
        .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;
    let method_args: TransitionMutationMethodArgs =
        serde_json::from_value(input_summary.method_args)
            .map_err(|error| ReferenceAdapterRuntimeError::FixtureLoad(error.to_string()))?;

    Ok(TransitionMutationKey {
        subject_scope: SubjectScopeKey {
            subject_reference: method_args
                .subject_scope
                .subject_reference
                .as_bytes()
                .to_vec(),
            scope_reference: method_args
                .subject_scope
                .scope_reference
                .as_ref()
                .map(|value| value.as_bytes().to_vec()),
        },
        mutation_material: method_args.mutation_material.as_bytes().to_vec(),
        operation_context_material: method_args
            .operation_context_material
            .as_ref()
            .map(|value| value.as_bytes().to_vec()),
    })
}

fn resolve_subject_scope_key(request: &SubjectScopeRequest) -> ResolveSubjectScopeKey {
    ResolveSubjectScopeKey {
        request_material: request.request_material.clone(),
        operation_context_material: request.operation_context_material.clone(),
    }
}

fn subject_scope_key(subject_scope: &SubjectScope) -> SubjectScopeKey {
    SubjectScopeKey {
        subject_reference: subject_scope.subject_reference.clone(),
        scope_reference: subject_scope.scope_reference.clone(),
    }
}

fn transition_mutation_key(request: &TransitionMutationRequest) -> TransitionMutationKey {
    TransitionMutationKey {
        subject_scope: subject_scope_key(&request.subject_scope),
        mutation_material: request.mutation_material.clone(),
        operation_context_material: request.operation_context_material.clone(),
    }
}

fn observe_contract_version(
    observed_contract_versions: &mut Vec<SemanticVersion>,
    contract_version: &SemanticVersion,
) {
    if !observed_contract_versions.contains(contract_version) {
        observed_contract_versions.push(contract_version.clone());
    }
}

fn insert_unique_fixture_id<V>(
    map: &mut BTreeMap<String, V>,
    fixture_id: String,
    value: V,
    collision_class: &'static str,
) -> Result<(), ReferenceAdapterRuntimeError> {
    if map.contains_key(&fixture_id) {
        return Err(ReferenceAdapterRuntimeError::FixtureLoad(format!(
            "duplicate {collision_class} collision for {fixture_id}"
        )));
    }
    map.insert(fixture_id, value);
    Ok(())
}

fn insert_unique_lookup_key<K, V>(
    map: &mut BTreeMap<K, V>,
    key: K,
    value: V,
    collision_class: &'static str,
) -> Result<(), ReferenceAdapterRuntimeError>
where
    K: Ord,
{
    if map.contains_key(&key) {
        return Err(ReferenceAdapterRuntimeError::FixtureLoad(format!(
            "duplicate {collision_class} collision in adapter fixture corpus"
        )));
    }
    map.insert(key, value);
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnimplementedAdapter;

impl UnimplementedAdapter {
    fn not_implemented(code: AdapterErrorCode, description: &'static str) -> AdapterError {
        AdapterError {
            code,
            description: description.to_string(),
        }
    }
}

impl AdapterPort for UnimplementedAdapter {
    fn resolve_subject_scope(&self, _request: &SubjectScopeRequest) -> AdapterResult<SubjectScope> {
        // TODO: bind request material to canonical subject/scope resolution once host integration is authorized.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::CapabilityNotSupported,
            "resolve_subject_scope is not implemented in the first scaffold pass",
        ))
    }

    fn capture_pre_state(&self, _subject_scope: &SubjectScope) -> AdapterResult<StateCapture> {
        // TODO: expose host-owned pre-state capture through the narrow adapter seam.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::PreStateCaptureUnavailable,
            "capture_pre_state is not implemented in the first scaffold pass",
        ))
    }

    fn execute_transition_mutation(
        &self,
        _request: &TransitionMutationRequest,
    ) -> AdapterResult<TransitionMutationApplied> {
        // TODO: implement host-side mutation execution without broadening into orchestration.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::TransitionMutationRejected,
            "execute_transition_mutation is not implemented in the first scaffold pass",
        ))
    }

    fn capture_post_state(&self, _subject_scope: &SubjectScope) -> AdapterResult<StateCapture> {
        // TODO: expose host-owned post-state capture through the narrow adapter seam.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::PostStateCaptureUnavailable,
            "capture_post_state is not implemented in the first scaffold pass",
        ))
    }

    fn get_adapter_status_facts(&self) -> AdapterResult<AdapterStatusFacts> {
        // TODO: surface adapter-boundary status facts once host state plumbing is authorized.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::CapabilityNotSupported,
            "get_adapter_status_facts is not implemented in the first scaffold pass",
        ))
    }

    fn get_adapter_capabilities(&self) -> AdapterResult<AdapterCapabilityReport> {
        // TODO: return live capability reports once adapter selection/wiring is authorized.
        AdapterResult::Err(Self::not_implemented(
            AdapterErrorCode::CapabilityNotSupported,
            "get_adapter_capabilities is not implemented in the first scaffold pass",
        ))
    }
}
