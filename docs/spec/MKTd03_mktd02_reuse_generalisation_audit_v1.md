# MKTd03 MKTd02 Reuse / Generalisation Audit v1

## Status
Draft

## Date
2026-03-26

## Purpose
Audit MKTd02 as a bounded reuse input for MKTd03, identifying what may be reused, what must be generalised, and what must not be carried forward.

This document is an audit artifact, not protocol authority.
Approved MKTd03 ADRs remain authoritative.

## Authority and Guardrails
This audit is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1

Guardrails:
- MKTd02 implementation history must not become authority by drift.
- Reuse is permitted only as bounded analysis input, not as automatic inheritance of fields, formulas, or flows.
- Findings in this audit must not silently revise ADR-01 or ADR-02.
- If a finding pressures ADR-01 or ADR-02, that requires explicit re-gate and sign-off.
- Leaf-mode-specific assumptions must not be carried into Tree-mode baseline without explicit justification.
- The candidate list produced by this audit does not authorise edits to MKTd02, creation of MKTd02 tasks, or parallel back-port work during this phase.

## Audit Scope
- **MKTd02 repo URL:** To be pinned before audit rows are added.
- **Exact commit SHA or tag under review:** Mandatory before audit rows are added.
- **Modules included in scope:** To be listed explicitly before audit rows are added.
- **Modules explicitly excluded from scope:** To be listed explicitly before audit rows are added.

This audit is not reproducible until those four scope items are filled in.

## Audit Questions
1. Which MKTd02 concepts remain valid at the level of evidentiary logic?
2. Which MKTd02 mechanisms are leaf-mode-specific and therefore non-reusable?
3. Which MKTd02 concepts are reusable only after generalisation for Tree mode?
4. Which MKTd02 field names, receipt assumptions, or verification patterns would create drift if copied directly?
5. What reusable lessons from MKTd02 should explicitly shape MKTd03 implementation and documentation work?

## Audit Categories
### A. Reusable without major change
Use this only for concepts that survive into MKTd03 with little or no conceptual rewriting.

### B. Reusable only after Tree-mode generalisation
Use this for concepts that remain valuable but must be rewritten for Tree-mode structure, boundary, or receipt semantics.

### C. Do not carry forward
Use this for MKTd02-specific assumptions, mechanisms, or vocabulary that would distort MKTd03 if inherited.

### D. Candidate for future generalisation — not authorising back-port
Use this for concepts or modules that are too MKTd02-specific to reuse directly now, but which may justify future abstraction work. Listing an item here does not authorise back-port or parallel work in MKTd02 during this phase.

## Initial Candidate Areas
- archival-first evidentiary posture
- certification material embedded for baseline verification
- build/module provenance expectations
- explicit non-claims in verifier semantics
- receipt self-containment expectations
- readiness/finalisation discipline lessons
- field naming and receipt-ID derivation caution
- leaf-mode tombstone assumptions
- MKTd02-specific implementation sequencing
- verifier decomposition patterns, if still meaningful after Tree-mode generalisation
- diagnostics/status surface
- versioning and compatibility dispatch

## Module Taxonomy Note
The build plan requires a module taxonomy view before or alongside this audit.
If a separate taxonomy artifact is not created first, this audit must include taxonomy notes as rows are added, so each item can be classified as:
- receipt/artifact identity,
- hashing/tagging,
- sequencing/lifecycle,
- certification/provenance,
- verifier-facing semantics,
- diagnostics/status surface,
- versioning/compatibility dispatch,
- integration-specific or Leaf-mode-specific behaviour.

The taxonomy requirement is not satisfied by concept-level observations alone.

## Audit Table
| Item ID | MKTd02 concept / mechanism / assumption | Taxonomy category | Category | Encodes protocol truth or only MKTd02 integration truth? | Assumes single-record Leaf model? | Reuse decision | Why | Follow-up artifact |
|---|---|---|---|---|---|---|---|---|
