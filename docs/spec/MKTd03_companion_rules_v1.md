# MKTd03 Companion Rules v1

## Status
Draft

## Date
2026-03-26

## Purpose
Capture the companion-rule layer that sits between the approved ADR/spec baseline and later frozen interface/vector artifacts.

This document exists to define rule-level conventions that are too specific for the ADRs, but too important to leave implicit before golden vectors and later formal interface freezing.

## Authority
This note is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1
- MKTd03 Diagnostics / Status Note v1
- MKTd03 Versioning / Compatibility Note v1
- MKTd03 Security / Privacy Note v1
- interfaces/mktd03_tree_mode_conceptual_interface_v1.did (conceptual interface artifact; subordinate to the ADR/spec baseline)

Where this note conflicts with an approved ADR, the ADR wins.
Where this note conflicts with the conceptual interface artifact, the ADR/spec baseline wins until the interface is explicitly revised.

## Context
The approved ADRs and follow-on notes now define the MKTd03 baseline at the level of evidentiary scope, structure, boundary, CVDR semantics, diagnostics, versioning, and security/privacy posture.

The conceptual interface artifact introduces a first typed surface for:
- readiness/status reporting,
- version/compatibility signalling,
- CVDR retrieval,
- blocked-state distinction,
- deletion-state distinction.

Before golden vectors are drafted, MKTd03 needs a companion-rule layer that states how those conceptual elements should be interpreted and constrained.

## Baseline Rule Areas

### 1. Version identity rule
Version-bearing artifacts must use explicit published version identity.
Free-form or undocumented version strings are not permitted where structured version semantics are defined by the versioning/compatibility note.

### 2. Unsupported-version rule
Unsupported or unrecognised versions must produce loud, named error outcomes.
No companion rule may permit fallback interpretation, nearest-known coercion, or silent downgrade.

### 3. Blocked-code rule
Blocked-state signalling must preserve programmatic reason identity.
Human-readable descriptions may accompany blocked codes, but must not replace them.

### 4. Readiness/status separation rule
Operational status and protocol evidence readiness must remain distinct concepts.
Companion rules must not allow status surfaces to be mistaken for proof surfaces.

### 5. Deletion-state distinction rule
Companion rules must preserve the ADR-02 distinction between tombstoned-position state and empty-position state.
No rule may collapse those two meanings into a single opaque interpretation.

### 6. CVDR field-meaning rule
Companion rules must define field meaning before they define field encoding.
Field names, ordering, and vectors must remain subordinate to the semantic roles already established by ADR-03.
A companion rule that specifies encoding without first establishing field meaning is out of order and must be revised before it can be treated as authoritative.

### 7. Reference-vs-inline rule
Certification material and build/module provenance must be carried inline in the baseline CVDR.
Reference-based treatment is not permitted as baseline where it would require a later live fetch for mandatory verification material.
Later artifacts may define optional reference-based treatments only if they do not weaken archival-first baseline verification.

### 8. Hash-role separation rule
Companion rules and golden vectors must preserve the ADR-02 canonical role separation for hashing.
Each distinct structural role must use a distinct, published hash-role identifier.
No companion rule may permit role identifiers to be shared or left implicit.

### 9. No semantic drift through companion rules
Companion rules must not be used to smuggle structural, evidentiary, or boundary changes under naming, ordering, or encoding refinements.
If a rule would materially alter ADR-02 structure, ADR-03 CVDR meaning, or ADR-01 boundary meaning, it requires an ADR update or explicit re-gate rather than a companion-rule change.

### 10. Fail-loud retrieval rule
CVDR retrieval outcomes must distinguish at least:
- not found,
- not yet issued,
- invalid record reference,
- unsupported version.

Later artifacts may refine these, but must not collapse them into null/none-style ambiguity.

### 11. dApp-agnostic rule
Companion rules must remain dApp-agnostic and must not import TinyPress-specific naming, payload conventions, or UI assumptions.

## Explicit Non-Goals
This note does not define:
- final byte-level encodings,
- final golden-vector data,
- final endpoint names,
- implementation code paths,
- deployment workflows.

## Open Questions
- Which companion rules should be frozen before golden vectors are drafted?
- Which conceptual interface names are stable enough to preserve versus still provisional?
- What exact baseline rule should govern inline-versus-reference treatment for certification and provenance material?
- What exact blocked codes should later artifacts standardise on?
- What exact field-ordering rule should later frozen artifacts adopt for CVDR structures?
- Should fixed-capacity semantics be expressed as a named parameter in companion rules and vectors, and if so, where?

## Source Drivers
- ADR-02
- ADR-03
- MKTd03 Protocol Refresh v1
- MKTd03 Diagnostics / Status Note v1
- MKTd03 Versioning / Compatibility Note v1
- MKTd03 Security / Privacy Note v1
- interfaces/mktd03_tree_mode_conceptual_interface_v1.did
