# ADR-01: Library vs Adapter Boundary

## Status
Scaffold — decision pending Session 3

## Date
2026-03-26

## Context
MKTd03 now has an approved baseline evidentiary scope (ADR-00), an approved Tree-mode structural baseline (ADR-02), and a committed Tree-mode invariants note.

The stale-spec inventory showed that the old spreadsheet spec mixed true core-boundary questions with non-core architecture spillover, including service-canister assumptions, orchestration flows, additive endpoint shapes, retry/list models, and implementation-mechanism details.

ADR-01 exists to decide only the core library boundary for baseline MKTd03:
- what the library must own,
- what the host canister must own,
- what the adapter seam is,
- what is explicitly out of scope for the core library.

ADR-01 must not become a catch-all ADR for orchestration, composite receipts, service-canister dependencies, verifier semantics, or tree-structure details already settled elsewhere.

## Decision
Baseline MKTd03 will use a library-plus-host integration model with an explicit adapter seam.

The decision to settle in this ADR is:
1. which responsibilities belong to the core MKTd03 library,
2. which responsibilities remain with the integrating host canister,
3. what minimum adapter contract is required between them,
4. which concerns are explicitly not part of the baseline library boundary.

This ADR will not settle:
- final receipt schema or verifier procedure,
- tree structure or tree terminology already decided by ADR-02,
- orchestration or service-canister architecture,
- retry/list/recovery workflows,
- formal interface file contents beyond boundary implications.

## Candidate Boundary Questions to Resolve
- What state transitions and checks must the library own directly?
- What host-specific storage and data-access responsibilities must remain outside the library?
- What minimum adapter contract must exist at the protocol boundary, and does baseline MKTd03 adopt, reject, or replace the old S18-style data-source model?
- Which freeze/rebuild/readiness responsibilities belong to the host, the library, or the integration seam?
- Which additive interface surfaces are consequences of the boundary, and which are out of scope for ADR-01?
- What explicit out-of-scope list is needed to stop later spillover from orchestration or service layers?
- Who owns the Tree-mode readiness state machine: the library, the host, or a defined seam between them?
- Is the initialisation/resume scheduling mechanism a library responsibility, a host responsibility, or explicitly out of scope for the baseline library boundary?

## Constraints from Earlier Artifacts
- Must stay within ADR-00 evidentiary scope.
- Must preserve all Tree-mode invariants.
- Must not reopen ADR-02 structural decisions by implication.
- Must not assign capacity-setting authority to the library or the host; that question is deferred until the fixed-capacity parameter definition is finalised from ADR-02 follow-up.
- Must not let TinyPress, app-shaped examples, or stale MKTd02 implementation history become baseline authority.
- Must not assign orchestration/service-canister responsibilities to the baseline library.
- Must not decide formal interface contents prematurely, but may state what kinds of interfaces the boundary implies.

## Likely Inventory Drivers
- S2, S17, S18, S19, S20, S25, S29, S30, S32, S53, S55, S57, S58, S67
