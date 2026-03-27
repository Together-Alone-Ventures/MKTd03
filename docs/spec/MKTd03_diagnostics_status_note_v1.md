# MKTd03 Diagnostics / Status Note v1

## Status
Draft

## Date
2026-03-26

## Purpose
Capture the baseline diagnostics and status-surface expectations that follow from the approved MKTd03 ADRs and the completed MKTd02 reuse/generalisation audit.

This note is a design/spec artifact.
It does not replace ADR authority or formal interface artifacts.
Its purpose is to prevent diagnostics and status exposure from being added reactively after implementation or verifier work has already begun.

## Authority
This note is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1
- MKTd03 MKTd02 Reuse / Generalisation Audit v1

Where this note conflicts with an approved ADR, the ADR wins.

## Context
The completed MKTd02 reuse/generalisation audit identified diagnostics and status-surface design as a directly reusable discipline rather than an implementation afterthought.

MKTd03 also already requires:
- externally legible Tree-mode readiness state,
- host-owned operational readiness-state-machine control,
- narrow adapter/status wiring at the protocol boundary,
- verifier-facing evidence/status discipline that should not depend on reactive endpoint growth.

This note therefore exists to state the baseline expectations for diagnostics and status before formal interfaces are frozen.

## Baseline Position

### 1. Diagnostics are part of baseline design
Diagnostics and status surfaces must be designed as part of the baseline protocol/integration surface.
They must not be treated as optional convenience features added only after verifier, lifecycle, or rebuild issues become visible.

### 2. Readiness state must be externally legible
Baseline MKTd03 must make Tree-mode readiness state externally legible.
At minimum, later interface work must provide a way to distinguish:
- uninitialised,
- initialising,
- ready,
- rebuilding,
- blocked.

### 3. Blocked state must not be opaque
A blocked state must not be exposed as a single undifferentiated failure condition.
Later interface/spec work must preserve enough diagnostic distinction to tell whether a blocked state is resumable by the host’s own operational control, or requires external operator intervention beyond the integration boundary.

### 4. Host owns operational status publication
Consistent with ADR-01, the host owns operational status publication and the readiness-state machine.
The library defines protocol predicates; the host exposes or wires the resulting status through the integration boundary.

### 5. Adapter role is narrow
The adapter may carry readiness/status facts needed for protocol predicates and host publication duties, but it must not become a generic monitoring, orchestration, or service-layer surface.

### 6. Diagnostics must support verifier-facing interpretation
Diagnostics/status surfaces should help operators and integrators understand whether Tree mode is capable of producing baseline evidence.
Status reporting must not be designed or labelled in a way that could be interpreted as evidentiary proof of deletion.
The distinction between readiness status and CVDR verification must be explicit in later interface and documentation artifacts.

### 7. Diagnostics must remain dApp-agnostic
Baseline diagnostics/status expectations must not assume TinyPress routes, app-shaped payloads, or application-specific UI conventions, consistent with ADR-00 and the TinyPress containment rule.

## Explicit Non-Goals
This note does not define:
- final endpoint names,
- final Candid/interface syntax,
- service-canister observability APIs,
- application UI/UX,
- monitoring stack integrations,
- verifier tooling behaviour.

## Open Questions
- What minimum status facts should later formal interfaces make mandatory?
- What blocked-state distinctions must later interface artifacts preserve explicitly?
- What diagnostics, if any, should be published for rebuild-compatibility failure versus general blocked state?
- How should diagnostics wording align with ADR-03 so status never gets mistaken for evidentiary proof?
- What diagnostics should be available to confirm rebuild-completion consistency per Tree-mode invariant 5?

## Source Drivers
- Tree-Mode Invariants Note
- ADR-01
- A9, A11, B5, B6, B10 from the reuse/generalisation audit
