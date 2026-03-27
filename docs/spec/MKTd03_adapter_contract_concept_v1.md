# MKTd03 Adapter Contract Concept v1

## Status
Draft

## Date
2026-03-26

## Purpose
Describe the conceptual minimum adapter contract implied by ADR-01, without yet freezing formal interface files.

This document is a boundary/spec artifact. ADR-01 remains authoritative for ownership split and out-of-scope decisions.

## Context
ADR-01 approves a narrow protocol-boundary adapter seam between the MKTd03 library and the integrating host.

The adapter exists to let the library obtain the protocol-relevant host interactions required for canonical Tree-mode operation, while keeping application-specific storage models, orchestration concerns, retry/list behaviours, and service-layer responsibilities outside the baseline library boundary.

## Contract Intent
The baseline adapter contract is conceptual, narrow, and protocol-scoped.

It must support only the interactions needed for:
1. deriving canonical Tree-mode inputs from host state,
2. executing host-side mutations required by canonical Tree-mode state transitions,
3. making progress/state facts available for protocol-critical readiness and rebuild checks,
4. providing readiness and status facts to the integration boundary so the host can fulfil its operational status obligations per ADR-01.

## Conceptual Contract Categories
### 1. Read access
The adapter must provide the library with access to the host facts needed to:
- identify the target record in canonical protocol terms,
- derive the canonical pre-state inputs required for Tree-mode processing,
- inspect host-owned progress/state facts that affect readiness, rebuild, or blocked-state predicates.

### 2. Mutation execution
The adapter must provide a way for host-side mutations required by canonical Tree-mode transitions to be executed under host control, without making the library the owner of application storage layout or caller policy.

### 3. Progress and rebuild facts
The adapter must make available the progress/state facts needed for the library to evaluate:
- whether Tree mode is uninitialised, initialising, ready, rebuilding, or blocked,
- whether a blocked state is resumable or requires intervention,
- whether rebuild is required,
- whether protocol predicates for evidence readiness are satisfied.
These facts are for library predicate evaluation, not for defining the host’s external status surface.

### 4. Status wiring
The adapter must support the integration boundary’s need to provide readiness and status facts in a way consistent with ADR-01 and the Tree-mode invariants note, so the host can meet its external status obligations without expanding the adapter into a service-layer surface.

## Explicit Non-Goals
This conceptual contract does not define:
- a service-canister API,
- retry/list/recovery operations,
- composite-deletion orchestration,
- application-specific endpoint suites,
- final formal interface files,
- verifier procedure or receipt schema.
- application-specific storage layout or storage ownership,

## Open Questions
- What exact conceptual operations should be named in the minimum adapter contract?
- Which progress/state facts must be mandatory versus optional at the boundary?
- How should the contract describe mutation execution without implying storage ownership by the library?
- ADR-01 has already settled the core meanings of “host,” “library,” “adapter,” and “integration seam.” What exact notation should later interface/spec artifacts use when mapping those settled terms into formal interface language?

## Source Artifacts
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- Tree-Mode Invariants Note
