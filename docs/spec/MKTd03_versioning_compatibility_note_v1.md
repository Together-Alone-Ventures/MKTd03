# MKTd03 Versioning / Compatibility Note v1

## Status
Draft

## Date
2026-03-26

## Purpose
Capture the baseline versioning and compatibility expectations that follow from the approved MKTd03 ADRs and the completed MKTd02 reuse/generalisation audit.

This note is a design/spec artifact.
It does not replace ADR authority or formal interface artifacts.
Its purpose is to prevent versioning and compatibility handling from being introduced reactively after receipt/schema/interface drift has already occurred.

## Authority
This note is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- MKTd03 Protocol Refresh v1
- MKTd03 MKTd02 Reuse / Generalisation Audit v1

Where this note conflicts with an approved ADR, the ADR wins.

## Context
The completed MKTd02 reuse/generalisation audit identified versioning and compatibility dispatch discipline as reusable, while also showing that exact MKTd02 dispatch mechanisms must not be copied directly into MKTd03 without Tree-mode restatement.

MKTd03 therefore needs a baseline compatibility posture before formal interfaces, companion rules, and vectors are frozen.

## Baseline Position

### 1. Versioning must be explicit
Verifier-facing artifacts must carry explicit version identity.
Baseline MKTd03 must not rely on undocumented implicit interpretation of receipt, proof, or interface semantics.

### 2. Compatibility policy must be designed, not inferred
Compatibility handling must be designed as part of the protocol surface.
Later artifacts must state what is versioned, what is stable across versions, and what requires explicit compatibility dispatch.

### 3. Dispatch discipline is required
Where multiple artifact versions or interpretation paths may exist, later verifier/interface work must use explicit compatibility dispatch rather than best-effort guesswork.
Unknown or unsupported versions must not be silently coerced into the nearest known interpretation.
Unknown or unsupported versions must produce an explicit, named error that identifies the version as unrecognised rather than attempting interpretation.

### 4. Versioning must not smuggle structural change
Version labels must not be applied to changes that materially alter ADR-02 structural meaning, ADR-03 evidentiary meaning, or ADR-01 boundary meaning without an explicit decision update.
Such changes require an ADR revision or explicit re-gate, not a version bump alone.

### 5. Compatibility must remain verifier-facing
Compatibility policy exists to preserve correct interpretation by independent verifiers and integrators.
It must therefore be stated in a way that is externally legible, not merely as an internal implementation convenience.

### 6. Backward-compatibility claims must be scoped
Later artifacts must be precise about whether compatibility is:
- full semantic compatibility,
- partial interpretation compatibility,
- migration-only compatibility,
- or no compatibility.

Migration-only compatibility means old artifacts can be interpreted only through an explicit migration path; new artifacts are not produced in the old format.

### 7. Version mismatch must fail loud
Baseline MKTd03 must produce loud, named errors on unsupported or mismatched versions.
Version rejection must be explicit and diagnosable; it must not degrade into fallback interpretation, nearest-known dispatch, or silent coercion.

### 8. dApp-agnostic rule still applies
Versioning and compatibility rules must remain dApp-agnostic and must not assume TinyPress-specific lifecycle or payload conventions.

## Explicit Non-Goals
This note does not define:
- final receipt field syntax,
- final Candid/interface syntax,
- exact dispatch code paths,
- migration tooling,
- release process mechanics.

## Open Questions
- What exact artifacts must carry explicit version identifiers?
- What compatibility classes should later interface/spec artifacts standardise on?
- Which MKTd03 artifacts require formal dispatch rules versus simple rejection of unknown versions?
- How should compatibility wording distinguish semantic compatibility from migration-only support?
- What minimum versioning guarantees should be visible to independent verifiers?
- How should later artifacts relate version identity to the approved ADR baseline so version labels cannot drift away from protocol meaning?
- How should a version transition that changes structural parameters interact with Tree-mode rebuild requirements?

## Source Drivers
- A12, B14, B15 from the reuse/generalisation audit
- ADR-02
- ADR-03
- MKTd03 Protocol Refresh v1
