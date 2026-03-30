# MKTd03 Negative Cases v1

## Status
Frozen-draft fixture taxonomy note

## Purpose
This note defines the canonical negative-case families that later machine-readable MKTd03 fixtures must cover.

This note is fixture-oriented and normative at the level of:
- named failure family,
- authoritative interface surface,
- required distinction between similar-but-different failure cases,
- minimum interpretation needed for later machine-readable negative fixtures.

This note does not define byte encodings, concrete fixture payloads, or application-shaped examples.

## Authority
This note is subordinate to:
- `RESTART_PACK.md`
- `docs/planning/MKTd03_authority_map_v1.md`
- `docs/planning/ADR-00-evidentiary-scope.md`
- `docs/planning/ADR-01-library-vs-adapter-boundary.md`
- `docs/planning/ADR-02-tree-structure-choice.md`
- `docs/planning/ADR-03-tree-mode-cvdr-structure.md`
- `docs/planning/tree-mode-invariants-note.md`
- `interfaces/mktd03_library.did`
- `interfaces/mktd03_library_interface_rules.md`
- `interfaces/mktd03_adapter_contract.did`
- `interfaces/mktd03_adapter_contract_rules.md`

If this note conflicts with the frozen library or adapter interfaces, the frozen interfaces win.

## 1. Negative-case writing rules

### 1.1 Surface separation rule
Negative cases must be classified under exactly one of:
- library-facing negative cases
- adapter-boundary negative cases
- verifier-input negative cases

A single fixture may reference cross-surface context, but its authoritative expected outcome must belong to one surface only.

### 1.2 Fail-loud rule
Every negative fixture must resolve to a named failure family or a named blocked / non-ready family.
No fixture may rely on null-style ambiguity, silent downgrade, or implied fallback.

### 1.3 dApp-agnostic rule
Negative fixtures must remain protocol-facing.
They must not use TinyPress nouns, route names, schema names, or application-shaped payload examples.

### 1.4 Versioned taxonomy rule
Each fixture family named here is part of the v1 fixture taxonomy.
Later fixture sets may refine payload detail, but must not silently merge distinct negative families defined here.

## 2. Library-facing negative cases

### 2.1 Unsupported version
**Surface:** `interfaces/mktd03_library.did`
**Primary outcome family:** `unsupported_version`
**Applies to:** version check, receipt retrieval, and any library-facing request where the declared version is not processable under the three-class compatibility policy.
**Fixture rule:** unsupported version must remain distinct from malformed request and from conditionally compatible processing.

### 2.2 Invalid subject reference
**Surface:** `interfaces/mktd03_library.did`
**Primary outcome family:** `invalid_subject_reference`
**Applies to:** `get_receipt`
**Fixture rule:** invalid subject reference must remain distinct from `not_found` and `not_yet_issued`.

### 2.3 Receipt not found
**Surface:** `interfaces/mktd03_library.did`
**Primary outcome family:** `not_found`
**Applies to:** `get_receipt`
**Fixture rule:** absence of a retrievable receipt must remain distinct from invalid subject input and from issuance-pending cases.

### 2.4 Receipt not yet issued
**Surface:** `interfaces/mktd03_library.did`
**Primary outcome family:** `not_yet_issued`
**Applies to:** `get_receipt`
**Fixture rule:** issuance-pending absence must remain distinct from clean not-found.

## 3. Verifier-input negative cases

### 3.1 Wrong tree-proof material
**Surface:** verifier-input receipt validation
**Primary outcome family:** `wrong_tree_proof`
**Applies to:** receipt artifacts under validation whose `tree_proof` does not match the canonical structural model, referenced subject position, or commitment transition.
**Fixture rule:** this family must remain distinct from malformed certification/provenance posture and from wrong deletion-state classification.

### 3.2 Wrong pre-state / post-state commitment relationship
**Surface:** verifier-input receipt validation
**Primary outcome family:** `wrong_commitment_relationship`
**Applies to:** receipt artifacts under validation whose `pre_state_commitment` and `post_state_commitment` do not form a coherent protocol-valid transition for the supplied transition evidence.
**Fixture rule:** this family must remain distinct from stale adapter precondition failures, which occur before receipt issuance.

### 3.3 Malformed certification / provenance posture-payload combination
**Surface:** `Receipt.certification_provenance`
**Primary outcome family:** `malformed_certification_provenance`
**Applies to:** receipt artifacts under validation where `posture`, `route`, and optional payload presence are semantically inconsistent under the frozen library interface rules.
**Fixture rule:** this family must cover at least:
- payload required by posture/route but absent,
- payload absence whose meaning is not made explicit by posture/route,
- route-context shape incompatible with the declared posture.

### 3.4 Subject / scope mismatch in receipt evidence
**Surface:** verifier-input receipt validation
**Primary outcome family:** `receipt_subject_scope_mismatch`
**Applies to:** receipt artifacts under validation where `subject_reference` and optional `scope_reference` are inconsistent with the declared transition semantics or are improperly collapsed.
**Fixture rule:** this family must remain distinct from adapter-boundary subject/scope resolution errors.

## 4. Library-facing negative cases

### 4.1 Blocked status: rebuild required
**Surface:** `get_status`
**Primary outcome family:** `blocked_status`
**Applies to:** library status surfaces where evidence cannot be treated as ready because rebuild is required.
**Fixture rule:** blocked rebuild-required status must remain distinct from `get_evidence_readiness = rebuild_required`, even when both are present in the same scenario.

### 4.2 Blocked status: other blocked reason
**Surface:** `get_status`
**Primary outcome family:** `blocked_status`
**Applies to:** library status surfaces where processing is blocked for an explicit named reason other than rebuild-required.
**Fixture rule:** blocked-status fixtures must preserve the distinction between blocked state and lifecycle state.

### 4.3 Evidence readiness: rebuild required
**Surface:** `get_evidence_readiness`
**Primary outcome family:** `rebuild_required`
**Applies to:** narrow readiness predicate reporting.
**Fixture rule:** readiness fixtures must not duplicate structured blocked diagnosis that belongs to `get_status`.

### 4.4 Evidence readiness: not evidence ready
**Surface:** `get_evidence_readiness`
**Primary outcome family:** `not_evidence_ready`
**Applies to:** non-ready library predicate states that do not resolve to the narrower rebuild-required case.
**Fixture rule:** this family must remain distinct from status-surface blocked reasoning.

## 5. Adapter-boundary negative cases

### 5.1 Invalid request material
**Surface:** `interfaces/mktd03_adapter_contract.did`
**Primary outcome family:** `invalid_request_material`
**Applies to:** `resolve_subject_scope` and other adapter requests carrying malformed boundary input material.
**Fixture rule:** invalid request material must remain distinct from subject-not-found and from scope-not-supported.

### 5.2 Subject not found
**Surface:** adapter boundary
**Primary outcome family:** `subject_not_found`
**Applies to:** `resolve_subject_scope`
**Fixture rule:** subject absence must remain distinct from invalid request encoding and from stale precondition.

### 5.3 Scope not supported
**Surface:** adapter boundary
**Primary outcome family:** `scope_not_supported`
**Applies to:** subject/scope resolution or capture/mutation requests that ask for a scope the adapter boundary does not support.
**Fixture rule:** unsupported scope must remain distinct from malformed request and from subject/scope mismatch inside a library receipt.

### 5.4 Subject / scope mismatch at the adapter boundary
**Surface:** adapter boundary
**Primary outcome family:** semantic note-level grouping only
**Applies to:** requests where supplied subject/scope material is internally inconsistent at the adapter seam.
**Fixture rule:** this family must not become its own machine-readable adapter family label in the first fixture pass.
Concrete fixtures in this group must map to existing explicit adapter outcomes only:
- `invalid_request_material`, or
- `scope_not_supported`

### 5.5 Pre-state capture unavailable
**Surface:** adapter boundary
**Primary outcome family:** `pre_state_capture_unavailable`
**Applies to:** `capture_pre_state`
**Fixture rule:** unavailable pre-state capture must remain distinct from stale precondition, which is detected at mutation execution time.

### 5.6 Post-state capture unavailable
**Surface:** adapter boundary
**Primary outcome family:** `post_state_capture_unavailable`
**Applies to:** `capture_post_state`
**Fixture rule:** unavailable post-state capture must remain distinct from mutation rejection.

### 5.7 Blocked / rebuild-required boundary state
**Surface:** `get_adapter_status_facts`
**Primary outcome family:** `blocked_boundary_state`
**Applies to:** adapter status facts where mutation/capture cannot proceed because the boundary is blocked.
**Fixture rule:** rebuild-required boundary state must be represented only through:
- `is_blocked = true`
- `blocked_reason.code = rebuild_required`

No fixture may introduce or imply a separate rebuild-required flag in `AdapterStatusFacts`.

### 5.8 Stale precondition
**Surface:** `execute_transition_mutation`
**Primary outcome family:** `stale_precondition`
**Applies to:** mutation requests whose expected pre-state or boundary precondition no longer matches current host state at execution time.
**Fixture rule:** this family must remain distinct from:
- blocked boundary state
- general transition mutation rejection
- pre-state capture unavailable

### 5.9 Transition mutation rejected for non-stale reasons
**Surface:** `execute_transition_mutation`
**Primary outcome family:** `transition_mutation_rejected`
**Applies to:** non-stale mutation rejection cases at the adapter boundary.
**Fixture rule:** fixtures in this family must not be used to represent stale-precondition cases.

### 5.10 Capability not supported
**Surface:** adapter boundary
**Primary outcome family:** `capability_not_supported`
**Applies to:** boundary operations the adapter does not support even though the contract shape exists.
**Fixture rule:** capability absence must remain distinct from transient boundary unavailability.

### 5.11 Internal adapter failure
**Surface:** adapter boundary
**Primary outcome family:** `internal_adapter_failure`
**Applies to:** explicit fail-loud internal boundary failures.
**Fixture rule:** internal adapter failure must not be used as a catch-all substitute for stale precondition, blocked, or unsupported capability.

## 6. Cross-surface distinction rules for fixtures

### 6.1 Receipt retrieval vs verifier-input invalidity
Receipt retrieval fixtures and verifier-input invalidity fixtures are not interchangeable.
If the negative outcome is a retrieval/status/version result from the frozen library interface, it belongs to the library-facing family.
If the negative outcome is that a returned receipt artifact fails validation under the published receipt rules, it belongs to the verifier-input family.

### 6.2 Stale precondition vs wrong pre/post commitment relationship
`stale_precondition` is an adapter execution-time failure before a valid receipt is produced.
`wrong_commitment_relationship` is a verifier-input invalid receipt evidence case.
Fixtures must not collapse them.

### 6.3 Blocked status vs unsupported version
Blocked status is an operational/reporting state.
Unsupported version is a compatibility-processing outcome.
Fixtures must preserve the distinction even if both appear in a larger scenario.

### 6.4 Subject/scope mismatch split
Subject/scope mismatch at the adapter boundary concerns boundary input or resolution failure.
Subject/scope mismatch in a receipt concerns verifier-input evidence inconsistency.
Fixtures must assign each case to the correct surface.

## 5. Non-goals

This note does not define:
- concrete fixture bytes
- final serialization encodings
- application-specific identifiers
- TinyPress-shaped examples
- recovery workflow modelling
- machine-readable fixture filenames beyond the manifest naming rules
