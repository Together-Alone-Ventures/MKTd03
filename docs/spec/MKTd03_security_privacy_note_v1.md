# MKTd03 Security / Privacy Note v1

## Status
Draft

## Date
2026-03-26

## Purpose
Capture the baseline security and privacy expectations that follow from the approved MKTd03 ADRs, the protocol refresh, and the completed MKTd02 reuse/generalisation audit.

This note is a design/spec artifact.
It does not replace ADR authority or formal interface artifacts.
Its purpose is to prevent security and privacy assumptions from being left implicit or deferred until after interface and implementation work is already underway.

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
MKTd03 is a dApp-agnostic Tree-mode protocol line with an archival-first evidentiary posture, a narrow library/adapter boundary, and an explicit requirement that diagnostics, versioning, and compatibility be designed intentionally rather than reactively.

The completed MKTd02 reuse/generalisation audit also showed that several security/privacy disciplines survive reuse at the principle level, while exact MKTd02 mechanisms must not be copied directly into Tree mode without restatement.

This note therefore exists to state the baseline security and privacy posture before later formal interfaces, companion rules, and implementation work are frozen.

## Baseline Position

### 1. Security and privacy are baseline design requirements
Security and privacy must be treated as baseline protocol/integration requirements, not implementation afterthoughts.

### 2. Evidence must not overexpose application data
Baseline Tree-mode CVDR and related status surfaces must carry only the evidence needed for independent verification and protocol interpretation.
They must not expose plaintext record identifiers or application-specific payload data beyond what ADR-03 requires for baseline protocol meaning.

### 3. Structural distinction must not leak into unnecessary disclosure
ADR-02 requires structural distinction between empty and tombstoned positions where deletion evidence depends on it.
The receipt and interface surface must not expose host-state detail beyond what ADR-03 defines as mandatory evidence components.

### 4. Boundary discipline is a security control
Consistent with ADR-01, the narrow adapter seam is part of the security posture.
The adapter must not expand into a broad data-source, orchestration, or service-layer surface that increases exposure or weakens ownership boundaries.

### 5. Convenience must not weaken exposure boundaries
Later interface and status work must not trade away security or privacy boundaries for convenience.
Reactive endpoint growth or broad adapter expansion creates unplanned disclosure surfaces that may expose host-state detail beyond baseline evidentiary scope.

### 6. Certification and provenance are part of trust, not optional extras
Certification material and build/module provenance are part of the baseline trust model.
They must not be treated as optional convenience features for “better” verification; they are part of baseline evidentiary trust.

### 7. Replay, stale-proof, and fake-proof risks must be addressed
Baseline MKTd03 must address replay risk, stale-proof risk, and fake-proof risk before formal interface files are frozen.
Later companion rules and interface/spec artifacts must state what makes a receipt unreplayable in a different context and what constitutes a stale, invalidated, or non-genuine proof.

### 8. Unknown versions and unsupported interpretations must fail loud
Consistent with the versioning/compatibility note, unsupported versions, unknown compatibility states, or unrecognised evidence forms must fail loud rather than degrade into fallback interpretation.

### 9. Diagnostics must not be mistaken for proof
Status and diagnostics surfaces may help operators understand readiness or blocked state, but they must not be designed or labelled in a way that could be mistaken for evidentiary proof of deletion.

### 10. dApp-agnostic privacy posture still applies
Security/privacy expectations must remain dApp-agnostic and must not assume TinyPress-specific data models, payload shapes, UI conventions, or operator workflows.

## Explicit Non-Goals
This note does not define:
- final cryptographic primitive choices beyond what approved ADRs already settle,
- application-specific privacy policies,
- deployment hardening checklists,
- monitoring product choices,
- legal/compliance advice,
- final interface syntax.

## Open Questions
- What minimum privacy-preserving identifier rules should later interface/spec artifacts require?
- What host-state details must be explicitly excluded from baseline receipt and status surfaces?
- What security-sensitive distinctions must later interface artifacts preserve between blocked, rebuilding, and ready states?
- How should later artifacts describe certification/provenance failure versus general verification failure without leaking unnecessary operational detail?
- What security/privacy warnings should later verifier guidance publish explicitly?
- How should later artifacts distinguish security-relevant blocked/rebuild states without exposing unnecessary host internals?
- What prevents a host from asserting readiness when protocol predicates have not been satisfied, and how should later interface artifacts constrain this?

## Source Drivers
- ADR-00
- ADR-01
- ADR-02
- ADR-03
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1
- MKTd03 Diagnostics / Status Note v1
- MKTd03 Versioning / Compatibility Note v1
- MKTd03 MKTd02 Reuse / Generalisation Audit v1

