use crate::adapter::{
    AdapterCapability, AdapterCapabilityReport, AdapterError, AdapterResult, AdapterStatusFacts,
    ReferenceAdapterRuntime, ReferenceAdapterRuntimeError, SubjectScope, SubjectScopeRequest,
};
use crate::library::{ReferenceLibraryRuntime, ReferenceRuntimeError, StatusSurface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceBoundaryEvaluation {
    ReadyToContinuePastBoundaryGate {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
        capability_report: AdapterCapabilityReport,
    },
    BlockedAtAdapterBoundary {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
    },
    AdapterCapabilityReportError {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
        error: AdapterError,
    },
    MissingRequiredCapability {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
        capability_report: AdapterCapabilityReport,
        missing_capability: AdapterCapability,
    },
    ResolveSubjectScopeAdapterError {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
        capability_report: AdapterCapabilityReport,
        error: AdapterError,
    },
    ResolveSubjectScopeContinuationDeferred {
        library_status: StatusSurface,
        adapter_status: AdapterStatusFacts,
        capability_report: AdapterCapabilityReport,
        subject_scope: SubjectScope,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceOrchestrationError {
    LibraryRuntime(ReferenceRuntimeError),
    AdapterRuntime(ReferenceAdapterRuntimeError),
    UnexpectedAdapterStatusError(AdapterError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceOrchestrator {
    pub library: ReferenceLibraryRuntime,
    pub adapter: ReferenceAdapterRuntime,
    pub required_adapter_capabilities: Vec<AdapterCapability>,
}

impl ReferenceOrchestrator {
    pub fn new(
        library: ReferenceLibraryRuntime,
        adapter: ReferenceAdapterRuntime,
        required_adapter_capabilities: Vec<AdapterCapability>,
    ) -> Self {
        Self {
            library,
            adapter,
            required_adapter_capabilities,
        }
    }

    pub fn evaluate_reference_boundary_readiness(
        &self,
    ) -> Result<ReferenceBoundaryEvaluation, ReferenceOrchestrationError> {
        let library_status = self
            .library
            .get_status()
            .map_err(ReferenceOrchestrationError::LibraryRuntime)?;
        let adapter_status = match self
            .adapter
            .get_adapter_status_facts()
            .map_err(ReferenceOrchestrationError::AdapterRuntime)?
        {
            AdapterResult::Ok(status) => status,
            AdapterResult::Err(error) => {
                return Err(ReferenceOrchestrationError::UnexpectedAdapterStatusError(
                    error,
                ))
            }
        };

        if adapter_status.is_blocked {
            return Ok(ReferenceBoundaryEvaluation::BlockedAtAdapterBoundary {
                library_status,
                adapter_status,
            });
        }

        match self
            .adapter
            .get_adapter_capabilities()
            .map_err(ReferenceOrchestrationError::AdapterRuntime)?
        {
            AdapterResult::Err(error) => {
                Ok(ReferenceBoundaryEvaluation::AdapterCapabilityReportError {
                    library_status,
                    adapter_status,
                    error,
                })
            }
            AdapterResult::Ok(capability_report) => {
                if let Some(missing_capability) =
                    self.find_missing_required_capability(&capability_report)
                {
                    Ok(ReferenceBoundaryEvaluation::MissingRequiredCapability {
                        library_status,
                        adapter_status,
                        capability_report,
                        missing_capability,
                    })
                } else {
                    Ok(
                        ReferenceBoundaryEvaluation::ReadyToContinuePastBoundaryGate {
                            library_status,
                            adapter_status,
                            capability_report,
                        },
                    )
                }
            }
        }
    }

    pub fn evaluate_reference_subject_scope_gate(
        &self,
        request: &SubjectScopeRequest,
    ) -> Result<ReferenceBoundaryEvaluation, ReferenceOrchestrationError> {
        match self.evaluate_reference_boundary_readiness()? {
            ready @ ReferenceBoundaryEvaluation::BlockedAtAdapterBoundary { .. }
            | ready @ ReferenceBoundaryEvaluation::AdapterCapabilityReportError { .. }
            | ready @ ReferenceBoundaryEvaluation::MissingRequiredCapability { .. } => Ok(ready),
            ReferenceBoundaryEvaluation::ReadyToContinuePastBoundaryGate {
                library_status,
                adapter_status,
                capability_report,
            } => match self
                .adapter
                .resolve_subject_scope(request)
                .map_err(ReferenceOrchestrationError::AdapterRuntime)?
            {
                AdapterResult::Err(error) => Ok(
                    ReferenceBoundaryEvaluation::ResolveSubjectScopeAdapterError {
                        library_status,
                        adapter_status,
                        capability_report,
                        error,
                    },
                ),
                AdapterResult::Ok(subject_scope) => Ok(
                    ReferenceBoundaryEvaluation::ResolveSubjectScopeContinuationDeferred {
                        library_status,
                        adapter_status,
                        capability_report,
                        subject_scope,
                    },
                ),
            },
            ReferenceBoundaryEvaluation::ResolveSubjectScopeAdapterError { .. } => {
                unreachable!("resolve_subject_scope evaluation should not recurse through a prior resolve result")
            }
            ReferenceBoundaryEvaluation::ResolveSubjectScopeContinuationDeferred { .. } => {
                unreachable!("resolve_subject_scope evaluation should not recurse through a prior resolve success result")
            }
        }
    }

    fn find_missing_required_capability(
        &self,
        capability_report: &AdapterCapabilityReport,
    ) -> Option<AdapterCapability> {
        self.required_adapter_capabilities
            .iter()
            .find(|capability| {
                !capability_report
                    .supported_capabilities
                    .contains(capability)
            })
            .cloned()
    }
}
