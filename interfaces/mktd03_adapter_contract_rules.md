# MKTd03 Adapter Contract Rules v1

## Status
Frozen-draft companion rules

## Purpose
This file defines only the normative companion rules required because `.did` notation alone is insufficient to safely express the MKTd03 adapter-boundary contract.

This file is subordinate to:
- `docs/planning/ADR-01-library-vs-adapter-boundary.md`
- `docs/planning/ADR-02-tree-structure-choice.md`
- `docs/planning/ADR-03-tree-mode-cvdr-structure.md`
- `docs/planning/tree-mode-invariants-note.md`
- `interfaces/mktd03_library.did`
- `interfaces/mktd03_adapter_contract.did`

If this file conflicts with ADR-01 or the frozen library interface baseline, ADR-01 and the frozen library baseline win.

## 1. Adapter obligation interpretation

### 1.1 Narrow-boundary rule
The adapter contract defines only what the adapter must provide to the library boundary for canonical Tree-mode operation.
It must not be interpreted as authorising:
- service-canister APIs
- composite orchestration
- retry/list/recovery workflows
- app-shaped endpoint suites
- host storage ownership by the library

### 1.2 Capability rule
`AdapterCapabilityReport` is declarative support signalling for the boundary surface.
Capabilities must not be inferred from partial success in unrelated methods.

### 1.3 Contract-version rule
`contract_version` identifies the adapter-contract surface version, not the public receipt or status-surface version reported by the library interface.

## 2. Subject vs scope handling at the adapter boundary

### 2.1 Distinct nouns
`subject_reference` and `scope_reference` are distinct at the adapter boundary.
The adapter must not silently collapse scope into subject or subject into scope.

### 2.2 Resolution rule
`resolve_subject_scope` is the boundary operation for turning request material into canonical boundary references.
It is not a general-purpose application lookup API.

### 2.3 Optional scope rule
If `scope_reference` is absent in `SubjectScope`, that means no separate scope payload is being returned by this contract surface.
It does not mean the adapter has silently defaulted scope to subject.

## 3. Pre-state / post-state capture interpretation

### 3.1 Capture role
`capture_pre_state` and `capture_post_state` provide state material needed by the library to derive canonical protocol outputs.
They do not themselves publish commitments, receipts, or verifier-facing artifacts.

### 3.2 No storage-shape leakage by implication
`state_material` and `capture_context_material` are boundary payloads, not permission to expose application storage layout, TinyPress-shaped schema, or host-internal naming.

### 3.3 Pre/post distinction
Pre-state and post-state capture are distinct boundary events.
A consumer must not infer one from the other or reuse one as a default stand-in for the other.

### 3.4 Query-surface interpretation
The use of query methods for capture in this draft expresses boundary read semantics only.
It must not be interpreted as relaxing invariant requirements about freshness, rebuild state, or blocked state.

### 3.5 Minimal positive pre-state success meaning
A successful `capture_pre_state` result means only that, for the supplied `SubjectScope`, the adapter has returned one boundary `StateCapture` object whose `subject_scope` identifies the same boundary subject/scope pair and whose `state_material` is the pre-state material the adapter is asserting for that boundary event.

The returned `StateCapture.subject_scope` must match the requested `SubjectScope` as the authoritative boundary subject/scope identity for that capture result.
The adapter must not return a successful pre-state capture whose `subject_scope` silently changes the requested boundary subject or scope.

At this boundary, the minimum positive structural claim is limited to:
- the supplied `SubjectScope` has been accepted for pre-state capture,
- one corresponding `StateCapture` object has been returned,
- `state_material` is the adapter's asserted pre-state boundary material for that same subject/scope pair,
- `capture_context_material`, if present, is auxiliary boundary context for that capture result only.

A successful `capture_pre_state` result does not by itself imply:
- post-state availability,
- mutation success,
- receipt construction,
- verifier validity,
- commitment correctness,
- proof correctness,
- cryptographic sufficiency beyond the returned boundary object shape.

## 4. Mutation execution semantics at the boundary

### 4.1 Mutation role
`execute_transition_mutation` is the boundary operation by which the adapter performs the host-side mutation required for the canonical Tree-mode transition.
It is not a general application mutation API.

### 4.2 Host-control rule
Mutation execution remains under host control, consistent with ADR-01.
This contract does not transfer application authority policy or storage ownership to the library.

### 4.3 No silent success rule
If a requested mutation is not applied, the adapter must not return `ok` with semantics that silently imply success.
`mutation_applied` must truthfully represent whether the requested boundary mutation was actually applied.

### 4.4 Result-material rule
`result_material` is optional boundary feedback only.
Absence of `result_material` does not imply mutation failure or success by itself; the authoritative success indicator is the result variant plus `mutation_applied`.

## 5. Adapter capability / error interpretation

### 5.1 Fail-loud rule
Unsupported or unavailable boundary behaviour must be expressed through explicit capability or error signaling.
The adapter must not silently downgrade, fabricate substitute state, or pretend support for unsupported capabilities.

### 5.2 Error-code identity
`AdapterError.code` is the programmatic identity.
`AdapterError.description` is explanatory only.

### 5.3 Boundary-specific error meaning
`subject_not_found`, `scope_not_supported`, `pre_state_capture_unavailable`, `post_state_capture_unavailable`, and `transition_mutation_rejected` are boundary meanings only.
They must not be reinterpreted as receipt-verification outcomes or public-library proof semantics.

### 5.4 Blocked / rebuild signalling
`initialisation_incomplete`, `rebuild_required`, and `blocked` at the adapter boundary indicate host-side boundary conditions relevant to library predicates.
They do not themselves constitute public-library status or evidentiary proof.

### 5.5 Stale-precondition rule
`stale_precondition` means the requested transition depended on a pre-state or boundary precondition that no longer matched current host state at execution time.
It is distinct from `blocked`, which signals a broader boundary condition, and distinct from `transition_mutation_rejected`, which remains the general mutation-rejection category for non-stale-precondition cases.
A stale-precondition outcome must fail loud and must not be collapsed into generic rejection or blocked-state signaling.

## 6. Fail-loud and no-silent-default rules

### 6.1 No hidden defaults
No omitted optional field in the adapter contract may be given an unstated semantic default.
Any default interpretation must be explicitly documented in approved companion rules.

### 6.2 No empty-bytes absence rule
Optional payload absence means absence.
Consumers must not reinterpret empty byte vectors as equivalent to omitted optional fields.

### 6.3 No synthetic scope or state rule
The adapter must not synthesize scope, state material, capture context, progress material, or blocked reasons merely to satisfy shape expectations when the underlying boundary facts are unavailable.

## 7. Status-facts interpretation

### 7.1 Boundary status only
`get_adapter_status_facts` reports adapter-boundary facts needed for library predicate evaluation.
It is not the public library status surface and must not be treated as a substitute for `get_status` from the frozen library interface.

### 7.2 Blocked reason rule
If `is_blocked = true`, `blocked_reason` must be present.
If `is_blocked = false`, `blocked_reason` must be absent.

### 7.3 Rebuild-required signaling rule
Rebuild-required signaling must appear only through blocked-state signaling at this boundary.
If rebuild is required, `is_blocked` must be `true` and `blocked_reason.code` must be `rebuild_required`.
Consumers must not look for or infer a separate rebuild-required flag in `AdapterStatusFacts`.

### 7.4 Progress-material rule
`progress_material` is boundary material only.
Its presence must not be interpreted as a public progress API, orchestration API, or host-internal scheduler disclosure surface.

## 8. TinyPress containment

No rule in this file authorises TinyPress-specific nouns, payloads, routes, schemas, fixtures, or examples in the adapter contract.
The adapter contract must remain dApp-agnostic and subordinate to ADR-01 boundary discipline.

## 9. Non-goals

This file does not define:
- receipt schema
- verifier procedure
- public-library status surface
- service-canister architecture
- retry/list/recovery models
- machine-readable fixtures
- application storage schema
- host scheduling implementation
