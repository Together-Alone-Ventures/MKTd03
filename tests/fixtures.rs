use mktd03::adapter::{
    validate_adapter_status_facts, AdapterBlockedReasonCode, AdapterCapability,
    AdapterCapabilityReport, AdapterErrorCode, AdapterResult, ReferenceAdapterRuntime,
    ReferenceAdapterRuntimeError, SubjectScope, SubjectScopeRequest, TransitionMutationRequest,
};
use mktd03::fixtures::{
    load_all_typed_fixtures_from_index, load_fixture, load_fixture_index, load_typed_fixture,
    validate_index_entry, FixturePolarity, FixtureStatusSurface, FixtureSurface, TypedFixtureCase,
    TypedFixtureDocument,
};
use mktd03::library::{
    evaluate_version_support, validate_evidence_readiness_semantics,
    validate_status_surface_semantics, BlockedCode, BuildIdentity, Compatibility,
    EvidenceReadiness, LifecycleState, ReceiptError, ReferenceLibraryRuntime, SemanticVersion,
    StatusSurface, VersionCheckResult,
};
use mktd03::orchestration::{ReferenceBoundaryEvaluation, ReferenceOrchestrator};
use mktd03::verifier::{validate_fixture_receipt_semantics, VerificationFailure};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn fixture_index_loads() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let index = load_fixture_index(index_path).expect("fixture index should parse");
    assert!(
        !index.entries.is_empty(),
        "fixture index should not be empty"
    );
}

#[test]
fn fixture_entries_are_discoverable_and_parseable() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    for entry in &index.entries {
        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let fixture = load_fixture(&full_path).expect("fixture file should parse");

        assert_eq!(fixture.surface, entry.surface);
        assert_eq!(fixture.polarity, entry.polarity);
        assert_eq!(fixture.family, entry.family);
        assert_eq!(fixture.target_method, entry.target_method);
    }
}

#[test]
fn fixture_metadata_shape_matches_manifest_conventions() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    for entry in &index.entries {
        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let fixture = load_fixture(&full_path).expect("fixture file should parse");

        assert!(
            fixture.fixture_schema_version.starts_with("1."),
            "fixture schema version should be in the v1 series"
        );
        assert!(
            !fixture.case_id.is_empty(),
            "fixture case_id should not be empty"
        );
        assert!(
            !fixture.title.is_empty(),
            "fixture title should not be empty"
        );
        assert!(
            !fixture.authority_refs.is_empty(),
            "fixture authority_refs should not be empty"
        );
        assert!(
            !fixture.notes.is_empty(),
            "fixture notes should not be empty"
        );

        match (&fixture.surface, &fixture.polarity) {
            (FixtureSurface::Library, FixturePolarity::Positive)
            | (FixtureSurface::Library, FixturePolarity::Negative)
            | (FixtureSurface::Adapter, FixturePolarity::Positive)
            | (FixtureSurface::Adapter, FixturePolarity::Negative)
            | (FixtureSurface::Verifier, FixturePolarity::Negative) => {}
            unsupported => {
                panic!("unexpected fixture surface/polarity combination: {unsupported:?}")
            }
        }
    }
}

#[test]
fn fixture_files_parse_into_typed_families() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    for entry in &index.entries {
        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let typed = load_typed_fixture(&full_path).expect("fixture should parse into typed family");

        match typed.case {
            TypedFixtureCase::LibraryPositiveStatus(_)
            | TypedFixtureCase::LibraryPositiveReceipt(_)
            | TypedFixtureCase::LibraryPositiveVersionSupport(_)
            | TypedFixtureCase::LibraryNegativeReceiptError(_)
            | TypedFixtureCase::LibraryNegativeBlockedStatus(_)
            | TypedFixtureCase::LibraryNegativeEvidenceReadiness(_)
            | TypedFixtureCase::VerifierNegativeReceipt(_)
            | TypedFixtureCase::AdapterPositiveSubjectScopeResolution(_)
            | TypedFixtureCase::AdapterNegativeError(_)
            | TypedFixtureCase::AdapterNegativeBlockedStatus(_) => {}
        }
    }
}

#[test]
fn typed_fixture_parsing_enforces_current_invariants() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    let mut saw_verifier = false;
    let mut saw_adapter_blocked = false;
    let mut saw_library_readiness = false;
    let mut saw_ready_and_blocked_library_status = false;

    for entry in &index.entries {
        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let typed =
            load_typed_fixture(&full_path).expect("fixture should satisfy typed invariants");

        match typed.case {
            TypedFixtureCase::LibraryPositiveStatus(fixture) => {
                assert_eq!(fixture.expected.primary_class, "ready");
                assert_eq!(fixture.expected.result_variant, "status_surface");
                assert!(!fixture.expected.status_surface.is_blocked);
                assert_eq!(fixture.expected.status_surface.blocked_reason, None);
            }
            TypedFixtureCase::LibraryPositiveReceipt(fixture) => {
                assert_eq!(fixture.expected.result_variant, "ok");
                assert_eq!(fixture.expected.primary_class, "receipt_returned");
            }
            TypedFixtureCase::LibraryPositiveVersionSupport(fixture) => {
                assert_eq!(fixture.expected.result_variant, "supported");
                assert_eq!(fixture.expected.primary_class, "supported");
            }
            TypedFixtureCase::LibraryNegativeReceiptError(fixture) => {
                assert_eq!(fixture.expected.result_variant, "err");
                assert!(fixture.expected.must_fail_loud);
                match fixture.expected.error_code {
                    ReceiptError::UnsupportedVersion
                    | ReceiptError::NotFound
                    | ReceiptError::NotYetIssued => {}
                    ReceiptError::InvalidSubjectReference => {
                        panic!(
                            "current fixture set should not parse invalid_subject_reference here"
                        )
                    }
                }
            }
            TypedFixtureCase::LibraryNegativeBlockedStatus(fixture) => {
                assert_eq!(fixture.expected.result_variant, "status_surface");
                assert_eq!(fixture.expected.primary_class, "blocked_status");
                assert!(fixture.expected.status_surface.is_blocked);
                assert!(fixture.expected.status_surface.blocked_reason.is_some());

                if fixture.expected.status_surface.lifecycle_state == LifecycleState::Ready {
                    saw_ready_and_blocked_library_status = true;
                }

                match &fixture
                    .expected
                    .status_surface
                    .blocked_reason
                    .as_ref()
                    .expect("blocked status should carry blocked reason")
                    .code
                {
                    BlockedCode::OperatorHold | BlockedCode::InitialisationIncomplete => {}
                    other => {
                        panic!("unexpected blocked status code in current fixture set: {other:?}")
                    }
                }
            }
            TypedFixtureCase::VerifierNegativeReceipt(fixture) => {
                saw_verifier = true;
                assert_eq!(fixture.expected.primary_class, "invalid_evidence");
                assert!(fixture.expected.must_fail_loud);
                assert_eq!(
                    fixture.expected.validation_outcome,
                    "reject_receipt_artifact"
                );
            }
            TypedFixtureCase::AdapterPositiveSubjectScopeResolution(fixture) => {
                assert_eq!(fixture.expected.result_variant, "ok");
                assert_eq!(fixture.expected.primary_class, "subject_scope_resolved");
            }
            TypedFixtureCase::AdapterNegativeBlockedStatus(fixture) => {
                saw_adapter_blocked = true;
                assert_eq!(fixture.expected.result_variant, "ok");
                assert!(fixture.expected.status_facts.is_blocked);
                let blocked_reason = fixture
                    .expected
                    .status_facts
                    .blocked_reason
                    .as_ref()
                    .expect("blocked adapter status should carry blocked_reason");
                assert_eq!(
                    blocked_reason.code,
                    mktd03::adapter::AdapterBlockedReasonCode::RebuildRequired
                );
            }
            TypedFixtureCase::LibraryNegativeEvidenceReadiness(fixture) => {
                saw_library_readiness = true;
                match fixture.expected.enum_value {
                    EvidenceReadiness::RebuildRequired => {
                        assert_eq!(fixture.expected.primary_class, "rebuild_required");
                    }
                    EvidenceReadiness::NotEvidenceReady => {
                        assert_eq!(fixture.expected.primary_class, "not_evidence_ready");
                    }
                    EvidenceReadiness::EvidenceReady => {
                        panic!("negative readiness fixtures must remain narrow non-ready values")
                    }
                }
            }
            TypedFixtureCase::AdapterNegativeError(fixture) => {
                assert_eq!(fixture.expected.result_variant, "err");
                assert!(fixture.expected.must_fail_loud);
            }
        }
    }

    assert!(
        saw_verifier,
        "current fixture set should include verifier fixtures"
    );
    assert!(
        saw_adapter_blocked,
        "current fixture set should include adapter blocked-boundary fixtures"
    );
    assert!(
        saw_library_readiness,
        "current fixture set should include library evidence-readiness fixtures"
    );
    assert!(
        saw_ready_and_blocked_library_status,
        "current fixture set should preserve the ready-plus-blocked status distinction"
    );
}

#[test]
fn verifier_fixture_semantics_are_executed_without_claiming_tree_proof_validation() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    let mut saw_deferred_wrong_tree_proof = false;
    let mut saw_malformed_certification = false;
    let mut saw_wrong_commitment = false;
    let mut saw_subject_scope_mismatch = false;

    for entry in &index.entries {
        if entry.surface != FixtureSurface::Verifier {
            continue;
        }

        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let typed =
            load_typed_fixture(&full_path).expect("fixture should satisfy typed invariants");

        let TypedFixtureCase::VerifierNegativeReceipt(fixture) = typed.case else {
            panic!("verifier entry should parse as verifier negative receipt fixture");
        };

        match (
            fixture.expected.family.as_str(),
            validate_fixture_receipt_semantics(&fixture),
        ) {
            (
                "malformed_certification_provenance",
                Err(VerificationFailure::InvalidEvidence("malformed_certification_provenance")),
            ) => {
                saw_malformed_certification = true;
            }
            (
                "wrong_commitment_relationship",
                Err(VerificationFailure::InvalidEvidence("wrong_commitment_relationship")),
            ) => {
                saw_wrong_commitment = true;
            }
            (
                "receipt_subject_scope_mismatch",
                Err(VerificationFailure::InvalidEvidence("receipt_subject_scope_mismatch")),
            ) => {
                saw_subject_scope_mismatch = true;
            }
            ("wrong_tree_proof", Err(VerificationFailure::Deferred(_))) => {
                saw_deferred_wrong_tree_proof = true;
            }
            unexpected => panic!("unexpected verifier semantic outcome: {unexpected:?}"),
        }
    }

    assert!(saw_malformed_certification);
    assert!(saw_wrong_commitment);
    assert!(saw_subject_scope_mismatch);
    assert!(saw_deferred_wrong_tree_proof);
}

#[test]
fn library_semantic_helpers_cover_version_status_and_readiness_rules() {
    let supported_protocol_version = SemanticVersion {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let interface_version = SemanticVersion {
        major: 1,
        minor: 0,
        patch: 0,
    };

    match evaluate_version_support(
        &supported_protocol_version,
        &interface_version,
        &supported_protocol_version,
    ) {
        VersionCheckResult::Supported { version_info } => {
            assert_eq!(version_info.protocol_version, supported_protocol_version);
            assert_eq!(version_info.interface_version, interface_version);
            assert_eq!(version_info.compatibility, Compatibility::Compatible);
        }
        other => panic!("expected supported version result, got {other:?}"),
    }

    let unsupported_protocol_version = SemanticVersion {
        major: 9,
        minor: 0,
        patch: 0,
    };

    match evaluate_version_support(
        &supported_protocol_version,
        &interface_version,
        &unsupported_protocol_version,
    ) {
        VersionCheckResult::UnsupportedVersion { version_info } => {
            assert_eq!(version_info.protocol_version, unsupported_protocol_version);
            assert_eq!(version_info.interface_version, interface_version);
            assert_eq!(version_info.compatibility, Compatibility::Unsupported);
        }
        other => panic!("expected unsupported version result, got {other:?}"),
    }

    let blocked_status = StatusSurface {
        protocol_version: supported_protocol_version.clone(),
        status_schema_version: supported_protocol_version.clone(),
        interface_version: interface_version.clone(),
        build_identity: BuildIdentity {
            build_version: supported_protocol_version.clone(),
            build_label: Some("BUILD_LABEL_PLACEHOLDER".to_string()),
            module_hash: None,
        },
        lifecycle_state: LifecycleState::Ready,
        is_blocked: true,
        blocked_reason: Some(mktd03::library::BlockedReason {
            code: BlockedCode::InitialisationIncomplete,
            description: "INITIALISATION_INCOMPLETE_PLACEHOLDER".to_string(),
        }),
        compatibility: Compatibility::Compatible,
        operation_context: Some(mktd03::library::OperationContext::StatusCheck),
    };

    assert!(validate_status_surface_semantics(&blocked_status).is_ok());
    assert!(validate_evidence_readiness_semantics(&EvidenceReadiness::RebuildRequired).is_ok());
    assert!(validate_evidence_readiness_semantics(&EvidenceReadiness::NotEvidenceReady).is_ok());
}

#[test]
fn status_fixture_semantics_round_trip_through_library_helpers() {
    let index_path = Path::new("docs/test-vectors/fixtures/index.json");
    let fixture_base_dir = index_path.parent().expect("index path should have parent");
    let index = load_fixture_index(index_path).expect("fixture index should parse");

    let mut checked_status_fixture = false;

    for entry in &index.entries {
        if entry.surface != FixtureSurface::Library || entry.target_method != "get_status" {
            continue;
        }

        let full_path = validate_index_entry(fixture_base_dir, entry)
            .expect("fixture index entry should be valid");
        let typed =
            load_typed_fixture(&full_path).expect("fixture should satisfy typed invariants");

        let status_fixture = match typed.case {
            TypedFixtureCase::LibraryPositiveStatus(fixture)
            | TypedFixtureCase::LibraryNegativeBlockedStatus(fixture) => fixture,
            _ => continue,
        };

        let status_surface = materialize_status_surface(&status_fixture.expected.status_surface);
        assert!(validate_status_surface_semantics(&status_surface).is_ok());
        checked_status_fixture = true;
    }

    assert!(
        checked_status_fixture,
        "expected to check at least one status fixture"
    );
}

#[test]
fn reference_library_runtime_answers_version_status_and_readiness_from_library_fixtures() {
    let mut runtime =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("reference runtime should load from fixture index");

    let version_info = runtime.get_version_info();
    assert_eq!(
        version_info.protocol_version,
        SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0
        }
    );
    assert_eq!(version_info.compatibility, Compatibility::Compatible);

    match runtime.check_version_support(&SemanticVersion {
        major: 1,
        minor: 0,
        patch: 0,
    }) {
        VersionCheckResult::Supported { version_info } => {
            assert_eq!(version_info.compatibility, Compatibility::Compatible);
        }
        other => panic!("expected supported version from reference runtime, got {other:?}"),
    }

    match runtime.check_version_support(&SemanticVersion {
        major: 9,
        minor: 0,
        patch: 0,
    }) {
        VersionCheckResult::UnsupportedVersion { version_info } => {
            assert_eq!(version_info.compatibility, Compatibility::Unsupported);
        }
        other => panic!("expected unsupported version from reference runtime, got {other:?}"),
    }

    let ready_status = runtime
        .get_status()
        .expect("default status fixture should be configured");
    assert!(!ready_status.is_blocked);
    assert_eq!(ready_status.lifecycle_state, LifecycleState::Ready);

    runtime
        .select_status_fixture("mktd03_library_negative_status_blocked_operator_hold_01_v1")
        .expect("blocked status fixture should exist");
    let blocked_status = runtime
        .get_status()
        .expect("selected blocked status fixture should resolve");
    assert!(blocked_status.is_blocked);
    assert_eq!(
        blocked_status
            .blocked_reason
            .as_ref()
            .expect("blocked status should carry reason")
            .code,
        BlockedCode::OperatorHold
    );

    runtime
        .select_evidence_readiness_fixture(
            "mktd03_library_negative_evidence_readiness_rebuild_required_01_v1",
        )
        .expect("rebuild-required fixture should exist");
    let rebuild_required = runtime
        .get_evidence_readiness()
        .expect("selected rebuild-required fixture should resolve");
    assert_eq!(rebuild_required, EvidenceReadiness::RebuildRequired);

    runtime
        .select_evidence_readiness_fixture(
            "mktd03_library_negative_evidence_readiness_not_ready_01_v1",
        )
        .expect("not-ready fixture should exist");
    let not_ready = runtime
        .get_evidence_readiness()
        .expect("selected readiness fixture should resolve");
    assert_eq!(not_ready, EvidenceReadiness::NotEvidenceReady);
}

#[test]
fn reference_library_runtime_receipt_lookup_stays_on_retrieval_surface_only() {
    let runtime =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("reference runtime should load from fixture index");

    let subject = b"SUBJECT_REFERENCE_PLACEHOLDER".to_vec();
    let receipt_result = runtime
        .get_receipt(&subject)
        .expect("receipt fixture should resolve on the retrieval surface");

    match receipt_result {
        mktd03::library::ReceiptResult::Ok { receipt } => {
            assert_eq!(
                receipt.core_transition_evidence.subject_reference,
                b"SUBJECT_REFERENCE_PLACEHOLDER".to_vec()
            );
        }
        other => panic!("expected positive receipt fixture lookup, got {other:?}"),
    }

    let unknown_subject = b"UNKNOWN_SUBJECT_REFERENCE_PLACEHOLDER".to_vec();
    assert!(
        runtime.get_receipt(&unknown_subject).is_none(),
        "verifier-invalid fixtures must not be folded into library receipt retrieval"
    );
}

#[test]
fn adapter_reference_runtime_surfaces_fixture_backed_status_and_capabilities() {
    let mut runtime = ReferenceAdapterRuntime::from_fixture_index(
        "docs/test-vectors/fixtures/index.json",
        vec![
            AdapterCapability::SubjectScopeResolution,
            AdapterCapability::PreStateCapture,
            AdapterCapability::PostStateCapture,
            AdapterCapability::TransitionMutationExecution,
            AdapterCapability::StatusFacts,
        ],
    )
    .expect("adapter reference runtime should load from fixture index");

    let status_result = runtime
        .get_adapter_status_facts()
        .expect("status lookup should be configured");
    let AdapterResult::Ok(status_facts) = status_result else {
        panic!("expected blocked status facts fixture result");
    };

    assert!(status_facts.is_blocked);
    assert_eq!(
        status_facts
            .blocked_reason
            .as_ref()
            .expect("blocked status should carry blocked reason")
            .code,
        AdapterBlockedReasonCode::RebuildRequired
    );
    assert!(validate_adapter_status_facts(&status_facts).is_ok());

    let capabilities_result = runtime
        .get_adapter_capabilities()
        .expect("capability report should be configured");
    let AdapterResult::Ok(AdapterCapabilityReport {
        contract_version,
        supported_capabilities,
    }) = capabilities_result
    else {
        panic!("expected positive capability report");
    };
    assert_eq!(
        contract_version,
        SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0
        }
    );
    assert_eq!(
        supported_capabilities,
        vec![
            AdapterCapability::SubjectScopeResolution,
            AdapterCapability::PreStateCapture,
            AdapterCapability::PostStateCapture,
            AdapterCapability::TransitionMutationExecution,
            AdapterCapability::StatusFacts,
        ]
    );

    runtime
        .select_capability_error_fixture("mktd03_adapter_negative_capability_not_supported_01_v1")
        .expect("capability-not-supported fixture should exist");
    let capability_error = runtime
        .get_adapter_capabilities()
        .expect("capability error fixture should be selectable");
    let AdapterResult::Err(error) = capability_error else {
        panic!("expected capability_not_supported error");
    };
    assert_eq!(error.code, AdapterErrorCode::CapabilityNotSupported);

    runtime.clear_capability_error_fixture();
    let capability_report = runtime
        .get_adapter_capabilities()
        .expect("capability report should be restored");
    assert!(matches!(capability_report, AdapterResult::Ok(_)));
}

#[test]
fn adapter_reference_runtime_surfaces_current_negative_taxonomy_only() {
    let mut runtime = ReferenceAdapterRuntime::from_fixture_index(
        "docs/test-vectors/fixtures/index.json",
        vec![
            AdapterCapability::SubjectScopeResolution,
            AdapterCapability::PreStateCapture,
            AdapterCapability::PostStateCapture,
            AdapterCapability::TransitionMutationExecution,
            AdapterCapability::StatusFacts,
        ],
    )
    .expect("adapter reference runtime should load from fixture index");

    let invalid_request = SubjectScopeRequest {
        request_material: b"MALFORMED_REQUEST_MATERIAL_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let invalid_request_result = runtime
        .resolve_subject_scope(&invalid_request)
        .expect("invalid_request_material fixture should be configured");
    assert_adapter_error_code(
        invalid_request_result,
        AdapterErrorCode::InvalidRequestMaterial,
    );

    let unsupported_scope_request = SubjectScopeRequest {
        request_material: b"REQUEST_MATERIAL_SCOPE_UNSUPPORTED_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let unsupported_scope_result = runtime
        .resolve_subject_scope(&unsupported_scope_request)
        .expect("scope_not_supported fixture should be configured");
    assert_adapter_error_code(
        unsupported_scope_result,
        AdapterErrorCode::ScopeNotSupported,
    );

    let subject_not_found_request = SubjectScopeRequest {
        request_material: b"SUBJECT_LOOKUP_REQUEST_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let subject_not_found_result = runtime
        .resolve_subject_scope(&subject_not_found_request)
        .expect("subject_not_found fixture should be configured");
    assert_adapter_error_code(subject_not_found_result, AdapterErrorCode::SubjectNotFound);

    let mismatched_subject_lookup_request = SubjectScopeRequest {
        request_material: b"SUBJECT_LOOKUP_REQUEST_PLACEHOLDER".to_vec(),
        operation_context_material: None,
    };
    let mismatched_subject_lookup_result =
        runtime.resolve_subject_scope(&mismatched_subject_lookup_request);
    assert!(matches!(
        mismatched_subject_lookup_result,
        Err(ReferenceAdapterRuntimeError::MissingConfiguration(
            "no fixture-backed resolve_subject_scope success path or matching error is configured"
        ))
    ));

    let subject_scope = SubjectScope {
        subject_reference: b"SUBJECT_REFERENCE_PLACEHOLDER".to_vec(),
        scope_reference: Some(b"SCOPE_REFERENCE_PLACEHOLDER".to_vec()),
    };

    let pre_state_result = runtime
        .capture_pre_state(&subject_scope)
        .expect("pre_state_capture_unavailable fixture should be configured");
    assert_adapter_error_code(
        pre_state_result,
        AdapterErrorCode::PreStateCaptureUnavailable,
    );

    let post_state_result = runtime
        .capture_post_state(&subject_scope)
        .expect("post_state_capture_unavailable fixture should be configured");
    assert_adapter_error_code(
        post_state_result,
        AdapterErrorCode::PostStateCaptureUnavailable,
    );

    let stale_precondition_request = TransitionMutationRequest {
        subject_scope: subject_scope.clone(),
        mutation_material: b"MUTATION_MATERIAL_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"PRESTATE_EXPECTATION_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let stale_precondition_result = runtime
        .execute_transition_mutation(&stale_precondition_request)
        .expect("stale_precondition fixture should be configured");
    assert_adapter_error_code(
        stale_precondition_result,
        AdapterErrorCode::StalePrecondition,
    );

    runtime.clear_transition_mutation_fixture();
    let ambiguous_transition_result =
        runtime.execute_transition_mutation(&TransitionMutationRequest {
            subject_scope: subject_scope.clone(),
            mutation_material: b"MUTATION_MATERIAL_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
        });
    assert!(matches!(
        ambiguous_transition_result,
        Err(ReferenceAdapterRuntimeError::MissingConfiguration(
            "multiple fixture-backed transition-mutation outcomes exist for this request; select one explicitly"
        ))
    ));

    runtime
        .select_transition_mutation_fixture(
            "mktd03_adapter_negative_transition_mutation_rejected_01_v1",
        )
        .expect("transition_mutation_rejected fixture should exist");
    let mutation_rejected_request = TransitionMutationRequest {
        subject_scope: subject_scope.clone(),
        mutation_material: b"MUTATION_MATERIAL_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let mutation_rejected_result = runtime
        .execute_transition_mutation(&mutation_rejected_request)
        .expect("transition_mutation_rejected fixture should be configured");
    assert_adapter_error_code(
        mutation_rejected_result,
        AdapterErrorCode::TransitionMutationRejected,
    );

    runtime
        .select_transition_mutation_fixture(
            "mktd03_adapter_negative_internal_adapter_failure_01_v1",
        )
        .expect("internal_adapter_failure fixture should exist");
    let internal_failure_request = TransitionMutationRequest {
        subject_scope,
        mutation_material: b"MUTATION_MATERIAL_PLACEHOLDER".to_vec(),
        operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
    };
    let internal_failure_result = runtime
        .execute_transition_mutation(&internal_failure_request)
        .expect("internal_adapter_failure fixture should be configured");
    assert_adapter_error_code(
        internal_failure_result,
        AdapterErrorCode::InternalAdapterFailure,
    );
}

#[test]
fn adapter_reference_runtime_surfaces_fixture_backed_positive_resolve_success() {
    let runtime = ReferenceAdapterRuntime::from_fixture_index(
        "docs/test-vectors/fixtures/index.json",
        vec![AdapterCapability::SubjectScopeResolution],
    )
    .expect("adapter reference runtime should load from fixture index");

    let resolve_result = runtime
        .resolve_subject_scope(&SubjectScopeRequest {
            request_material: b"CANONICAL_REQUEST_MATERIAL_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"CANONICAL_OPERATION_CONTEXT_PLACEHOLDER".to_vec()),
        })
        .expect("positive resolve fixture should be configured");

    match resolve_result {
        AdapterResult::Ok(subject_scope) => {
            assert_eq!(
                subject_scope.subject_reference,
                b"SUBJECT_REFERENCE_PLACEHOLDER".to_vec()
            );
            assert_eq!(
                subject_scope.scope_reference,
                Some(b"SCOPE_REFERENCE_PLACEHOLDER".to_vec())
            );
        }
        AdapterResult::Err(error) => {
            panic!("expected positive resolve fixture result, got {error:?}")
        }
    }
}

#[test]
fn adapter_reference_runtime_rejects_duplicate_automatic_lookup_keys() {
    let fixtures = load_all_typed_fixtures_from_index("docs/test-vectors/fixtures/index.json")
        .expect("typed fixtures should load");

    let mut duplicated_resolve = fixtures.clone();
    duplicated_resolve.push(clone_fixture_with_fixture_id(
        duplicated_resolve
            .iter()
            .find(|fixture| {
                fixture.envelope.fixture_id
                    == "mktd03_adapter_negative_invalid_request_material_01_v1"
            })
            .expect("resolve fixture should exist"),
        "mktd03_adapter_negative_invalid_request_material_duplicate_01_v1",
    ));
    let duplicated_resolve_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &duplicated_resolve,
        vec![AdapterCapability::SubjectScopeResolution],
    );
    assert!(matches!(
        duplicated_resolve_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("duplicate resolve_subject_scope outcome key collision")
    ));

    let mut duplicated_pre_capture = fixtures.clone();
    duplicated_pre_capture.push(clone_fixture_with_fixture_id(
        duplicated_pre_capture
            .iter()
            .find(|fixture| {
                fixture.envelope.fixture_id
                    == "mktd03_adapter_negative_pre_state_capture_unavailable_01_v1"
            })
            .expect("pre-state capture fixture should exist"),
        "mktd03_adapter_negative_pre_state_capture_unavailable_duplicate_01_v1",
    ));
    let duplicated_pre_capture_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &duplicated_pre_capture,
        vec![AdapterCapability::PreStateCapture],
    );
    assert!(matches!(
        duplicated_pre_capture_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("duplicate capture_pre_state error key collision")
    ));

    let mut duplicated_status_fixture_id = fixtures.clone();
    let mut duplicated_status_fixture = duplicated_status_fixture_id
        .iter()
        .find(|fixture| {
            fixture.envelope.fixture_id
                == "mktd03_adapter_negative_status_blocked_rebuild_required_01_v1"
        })
        .expect("blocked status fixture should exist")
        .clone();
    duplicated_status_fixture.envelope.input_summary = serde_json::json!({ "method_args": [], "semantic_context": "Duplicated status fixture id for collision testing." });
    duplicated_status_fixture_id.push(duplicated_status_fixture);
    let duplicated_status_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &duplicated_status_fixture_id,
        vec![AdapterCapability::StatusFacts],
    );
    assert!(matches!(
        duplicated_status_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("duplicate adapter status fixture id collision")
    ));

    let mut duplicated_capability_fixture_id = fixtures.clone();
    duplicated_capability_fixture_id.push(
        duplicated_capability_fixture_id
            .iter()
            .find(|fixture| {
                fixture.envelope.fixture_id
                    == "mktd03_adapter_negative_capability_not_supported_01_v1"
            })
            .expect("capability fixture should exist")
            .clone(),
    );
    let duplicated_capability_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &duplicated_capability_fixture_id,
        vec![AdapterCapability::StatusFacts],
    );
    assert!(matches!(
        duplicated_capability_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("duplicate adapter capability fixture id collision")
    ));

    let mut duplicated_transition_fixture_id = fixtures.clone();
    duplicated_transition_fixture_id.push(
        duplicated_transition_fixture_id
            .iter()
            .find(|fixture| {
                fixture.envelope.fixture_id
                    == "mktd03_adapter_negative_transition_mutation_rejected_01_v1"
            })
            .expect("transition fixture should exist")
            .clone(),
    );
    let duplicated_transition_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &duplicated_transition_fixture_id,
        vec![AdapterCapability::TransitionMutationExecution],
    );
    assert!(matches!(
        duplicated_transition_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("duplicate transition_mutation fixture id collision")
    ));
}

#[test]
fn adapter_reference_runtime_rejects_inconsistent_contract_versions() {
    let fixtures = load_all_typed_fixtures_from_index("docs/test-vectors/fixtures/index.json")
        .expect("typed fixtures should load");

    let mut inconsistent_fixtures = fixtures.clone();
    let mut duplicated_status_fixture = clone_fixture_with_fixture_id(
        inconsistent_fixtures
            .iter()
            .find(|fixture| {
                fixture.envelope.fixture_id
                    == "mktd03_adapter_negative_status_blocked_rebuild_required_01_v1"
            })
            .expect("blocked status fixture should exist"),
        "mktd03_adapter_negative_status_blocked_rebuild_required_inconsistent_version_01_v1",
    );
    duplicated_status_fixture.envelope.expected_outcome = serde_json::json!({
        "result_variant": "ok",
        "primary_class": "blocked_boundary_state",
        "status_facts": {
            "contract_version": {
                "major": 9,
                "minor": 0,
                "patch": 0
            },
            "is_blocked": true,
            "blocked_reason": {
                "code": "rebuild_required",
                "description": "BOUNDARY_REBUILD_REQUIRED_PLACEHOLDER"
            },
            "progress_material": "PROGRESS_MATERIAL_PLACEHOLDER"
        }
    });
    if let TypedFixtureCase::AdapterNegativeBlockedStatus(ref mut fixture_case) =
        duplicated_status_fixture.case
    {
        fixture_case.expected.status_facts.contract_version = SemanticVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
    } else {
        panic!("duplicated fixture should remain an adapter blocked status fixture");
    }
    inconsistent_fixtures.push(duplicated_status_fixture);

    let inconsistent_result = ReferenceAdapterRuntime::from_typed_fixtures(
        &inconsistent_fixtures,
        vec![AdapterCapability::StatusFacts],
    );
    assert!(matches!(
        inconsistent_result,
        Err(ReferenceAdapterRuntimeError::FixtureLoad(message))
            if message.contains("inconsistent adapter contract versions observed")
    ));
}

#[test]
fn reference_orchestrator_stops_at_blocked_adapter_boundary() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = ReferenceAdapterRuntime::from_fixture_index(
        "docs/test-vectors/fixtures/index.json",
        vec![
            AdapterCapability::SubjectScopeResolution,
            AdapterCapability::PreStateCapture,
            AdapterCapability::PostStateCapture,
            AdapterCapability::TransitionMutationExecution,
            AdapterCapability::StatusFacts,
        ],
    )
    .expect("adapter reference runtime should load from fixture index");

    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::TransitionMutationExecution],
    );

    let evaluation = orchestrator
        .evaluate_reference_boundary_readiness()
        .expect("blocked adapter boundary should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::BlockedAtAdapterBoundary {
            library_status,
            adapter_status,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(adapter_status.is_blocked);
            assert_eq!(
                adapter_status
                    .blocked_reason
                    .as_ref()
                    .expect("blocked adapter status should carry reason")
                    .code,
                AdapterBlockedReasonCode::RebuildRequired
            );
        }
        other => panic!("expected blocked adapter boundary evaluation, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_propagates_capability_report_error() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let mut adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::PreStateCapture,
        AdapterCapability::PostStateCapture,
        AdapterCapability::TransitionMutationExecution,
        AdapterCapability::StatusFacts,
    ]);
    adapter
        .select_capability_error_fixture("mktd03_adapter_negative_capability_not_supported_01_v1")
        .expect("capability error fixture should exist");

    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::TransitionMutationExecution],
    );

    let evaluation = orchestrator
        .evaluate_reference_boundary_readiness()
        .expect("capability error should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::AdapterCapabilityReportError {
            library_status,
            adapter_status,
            error,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert_eq!(error.code, AdapterErrorCode::CapabilityNotSupported);
        }
        other => panic!("expected adapter capability report error, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_reports_missing_required_capability() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::StatusFacts,
    ]);

    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::TransitionMutationExecution],
    );

    let evaluation = orchestrator
        .evaluate_reference_boundary_readiness()
        .expect("missing capability should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::MissingRequiredCapability {
            library_status,
            adapter_status,
            capability_report,
            missing_capability,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert_eq!(
                missing_capability,
                AdapterCapability::TransitionMutationExecution
            );
            assert!(!capability_report
                .supported_capabilities
                .contains(&AdapterCapability::TransitionMutationExecution));
        }
        other => panic!("expected missing required capability evaluation, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_surfaces_invalid_request_material_at_resolve_step() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::StatusFacts,
    ]);
    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::SubjectScopeResolution],
    );

    let evaluation = orchestrator
        .evaluate_reference_subject_scope_gate(&SubjectScopeRequest {
            request_material: b"MALFORMED_REQUEST_MATERIAL_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
        })
        .expect("resolve negative fixture should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::ResolveSubjectScopeAdapterError {
            library_status,
            adapter_status,
            capability_report,
            error,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert_eq!(error.code, AdapterErrorCode::InvalidRequestMaterial);
            assert!(capability_report
                .supported_capabilities
                .contains(&AdapterCapability::SubjectScopeResolution));
        }
        other => panic!("expected invalid_request_material resolve result, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_surfaces_scope_not_supported_at_resolve_step() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::StatusFacts,
    ]);
    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::SubjectScopeResolution],
    );

    let evaluation = orchestrator
        .evaluate_reference_subject_scope_gate(&SubjectScopeRequest {
            request_material: b"REQUEST_MATERIAL_SCOPE_UNSUPPORTED_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
        })
        .expect("resolve negative fixture should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::ResolveSubjectScopeAdapterError {
            library_status,
            adapter_status,
            capability_report,
            error,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert_eq!(error.code, AdapterErrorCode::ScopeNotSupported);
            assert!(capability_report
                .supported_capabilities
                .contains(&AdapterCapability::SubjectScopeResolution));
        }
        other => panic!("expected scope_not_supported resolve result, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_surfaces_subject_not_found_at_resolve_step() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::StatusFacts,
    ]);
    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::SubjectScopeResolution],
    );

    let evaluation = orchestrator
        .evaluate_reference_subject_scope_gate(&SubjectScopeRequest {
            request_material: b"SUBJECT_LOOKUP_REQUEST_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"BOUNDARY_CONTEXT_PLACEHOLDER".to_vec()),
        })
        .expect("resolve negative fixture should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::ResolveSubjectScopeAdapterError {
            library_status,
            adapter_status,
            capability_report,
            error,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert_eq!(error.code, AdapterErrorCode::SubjectNotFound);
            assert!(capability_report
                .supported_capabilities
                .contains(&AdapterCapability::SubjectScopeResolution));
        }
        other => panic!("expected subject_not_found resolve result, got {other:?}"),
    }
}

#[test]
fn reference_orchestrator_surfaces_resolve_success_with_continuation_deferred() {
    let library =
        ReferenceLibraryRuntime::from_fixture_index("docs/test-vectors/fixtures/index.json")
            .expect("library reference runtime should load from fixture index");
    let adapter = build_unblocked_adapter_runtime(vec![
        AdapterCapability::SubjectScopeResolution,
        AdapterCapability::StatusFacts,
    ]);
    let orchestrator = ReferenceOrchestrator::new(
        library,
        adapter,
        vec![AdapterCapability::SubjectScopeResolution],
    );

    let evaluation = orchestrator
        .evaluate_reference_subject_scope_gate(&SubjectScopeRequest {
            request_material: b"CANONICAL_REQUEST_MATERIAL_PLACEHOLDER".to_vec(),
            operation_context_material: Some(b"CANONICAL_OPERATION_CONTEXT_PLACEHOLDER".to_vec()),
        })
        .expect("positive resolve fixture should evaluate explicitly");

    match evaluation {
        ReferenceBoundaryEvaluation::ResolveSubjectScopeContinuationDeferred {
            library_status,
            adapter_status,
            capability_report,
            subject_scope,
        } => {
            assert_eq!(library_status.lifecycle_state, LifecycleState::Ready);
            assert!(!adapter_status.is_blocked);
            assert!(capability_report
                .supported_capabilities
                .contains(&AdapterCapability::SubjectScopeResolution));
            assert_eq!(
                subject_scope.subject_reference,
                b"SUBJECT_REFERENCE_PLACEHOLDER".to_vec()
            );
            assert_eq!(
                subject_scope.scope_reference,
                Some(b"SCOPE_REFERENCE_PLACEHOLDER".to_vec())
            );
        }
        other => panic!("expected resolve success with continuation deferred, got {other:?}"),
    }
}

#[test]
fn typed_fixture_loading_from_index_is_relative_to_index_parent() {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!(
        "mktd03-fixture-index-relative-loading-{unique_suffix}"
    ));
    let nested_dir = temp_root.join("nested");
    let fixture_dir = nested_dir.join("fixtures");
    fs::create_dir_all(&fixture_dir).expect("temporary fixture directory should be created");

    let fixture_source =
        Path::new("docs/test-vectors/fixtures/library/positive/mktd03_library_positive_status_ready_minimal_01_v1.json");
    let fixture_target =
        fixture_dir.join("mktd03_library_positive_status_ready_minimal_01_v1.json");
    fs::copy(fixture_source, &fixture_target).expect("sample fixture should be copied");

    let index_path = nested_dir.join("index.json");
    fs::write(
        &index_path,
        serde_json::json!({
            "fixture_index_version": "1.0.0",
            "entries": [
                {
                    "filename": "fixtures/mktd03_library_positive_status_ready_minimal_01_v1.json",
                    "surface": "library",
                    "polarity": "positive",
                    "family": "status",
                    "target_method": "get_status",
                    "status": "draft"
                }
            ]
        })
        .to_string(),
    )
    .expect("temporary index should be written");

    let documents = load_all_typed_fixtures_from_index(&index_path)
        .expect("fixtures should load relative to the index parent");
    assert_eq!(documents.len(), 1);

    fs::remove_dir_all(&temp_root).expect("temporary fixture directory should be removed");
}

#[test]
fn scaffold_does_not_claim_cryptographic_validation() {
    let source = std::fs::read_to_string("src/verifier.rs").expect("verifier source should exist");
    assert!(
        source.contains("Deferred") && source.contains("NotImplemented"),
        "current verifier slice must keep tree-proof and full receipt validation explicitly deferred"
    );
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

fn assert_adapter_error_code<T>(result: AdapterResult<T>, expected: AdapterErrorCode) {
    match result {
        AdapterResult::Err(error) => assert_eq!(error.code, expected),
        AdapterResult::Ok(_) => panic!("expected adapter error result {expected:?}"),
    }
}

fn clone_fixture_with_fixture_id(
    fixture: &TypedFixtureDocument,
    fixture_id: &str,
) -> TypedFixtureDocument {
    let mut cloned = fixture.clone();
    cloned.envelope.fixture_id = fixture_id.to_string();
    cloned
}

fn build_unblocked_adapter_runtime(
    supported_capabilities: Vec<AdapterCapability>,
) -> ReferenceAdapterRuntime {
    let mut fixtures = load_all_typed_fixtures_from_index("docs/test-vectors/fixtures/index.json")
        .expect("typed fixtures should load");

    let blocked_status_fixture = fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.envelope.fixture_id
                == "mktd03_adapter_negative_status_blocked_rebuild_required_01_v1"
        })
        .expect("blocked status fixture should exist");

    blocked_status_fixture.envelope.expected_outcome = serde_json::json!({
        "result_variant": "ok",
        "primary_class": "blocked_boundary_state",
        "status_facts": {
            "contract_version": {
                "major": 1,
                "minor": 0,
                "patch": 0
            },
            "is_blocked": false,
            "blocked_reason": null,
            "progress_material": "PROGRESS_MATERIAL_PLACEHOLDER"
        }
    });

    if let TypedFixtureCase::AdapterNegativeBlockedStatus(ref mut fixture_case) =
        blocked_status_fixture.case
    {
        fixture_case.expected.status_facts.is_blocked = false;
        fixture_case.expected.status_facts.blocked_reason = None;
    } else {
        panic!("status fixture should remain an adapter blocked-status fixture");
    }

    ReferenceAdapterRuntime::from_typed_fixtures(&fixtures, supported_capabilities)
        .expect("unblocked adapter reference runtime should construct")
}
