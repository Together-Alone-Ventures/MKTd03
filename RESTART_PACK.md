DATE: 2026-04-20
CURRENT GOAL:
Continue the bounded specification-tightening modification stream opened following the Zombie-delete-MKTd03 analytical session. Session 1 (ADR/interface wording tightening) is closed cleanly. Session 2 (transition_derivation_version — frozen-interface change) and Session 3 (documentation additions) remain to be executed in a fresh chat.
IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.
CURRENT STATUS:

Prep is closed.
Repo-boundary cleanup is closed and must not be reopened unless a concrete regression is found.
Standards uplift is complete and pushed.
The formal-interface/conformance phase closed cleanly at cdfc097 is unchanged.
MKTd03_Handover_Pack.md and MKTd03_build_plan.md were added to main on 2026-04-20 at commit d1f6c85 as part of the Antoine handover and build-plan publication.
A specification-tightening modification stream was opened following an analytical comparison with Zombie-delete-MKTd03.
Session 1 of that stream is complete. Session 1 substantive checkpoint is 24db28f.
Sessions 2 and 3 have not begun. They will be run in a fresh chat using the change list agreed with G.

MKTd03 remains dApp-agnostic. TinyPress remains the first reference target only and must not shape protocol truth.
SESSION 1 LANDED CHANGES (COMPLETE):

aeec179 — interfaces: add issuance atomicity and retrieval semantics rules (Change 1.1)

New section 8 "Issuance atomicity and retrieval semantics" in interfaces/mktd03_library_interface_rules.md
Prior section 8 "Non-goals" renumbered to section 9


35fa3fa — ADR-03: add issuance atomicity clause and rejected alternative (Change 1.2)

New clause 9 "Issuance atomicity" in Decision section
New "Query-time reconstruction of evidence-bearing substance" entry in Rejected Alternatives


24db28f — ADR-01: add module-boundary and genericity rules with rejections (Changes 1.3 and 1.4)

New clause 9 "Module-boundary rule" and clause 10 "Genericity rule" in Decision section
New "External witness-canister deployment of the core library" and "Schema-generic library interface" entries in Rejected Alternatives



CROSS-REFERENCE SWEEP:
A sweep was run after Session 1 landing, covering docs/, interfaces/, src/, RESTART_PACK.md, MILESTONE_LOG.md, MKTd03_Handover_Pack.md, and MKTd03_build_plan.md, searching for references to ADR-01/ADR-03 clause numbers, library_interface_rules anchor references, prose references to sections 8 or 9, the strings "Non-goals" and "Rejected Alternatives", and enumerated clause references covering clauses 1-8. The sweep returned no stale references. The only non-benign-pattern matches were 14 fixture-level rules_version_ref anchors pointing to interfaces/mktd03_library_interface_rules.md#v1, which remain valid because Session 1 edits were additive within the existing v1 companion-rules envelope. No housekeeping commit was required.
STANDING CONSTRAINT SURFACED BY THE SWEEP:
14 machine-readable fixtures under docs/test-vectors/fixtures/ reference interfaces/mktd03_library_interface_rules.md#v1 via rules_version_ref. Any future edit that bumps the companion-rules file version (e.g. to #v2 for a semantic change) requires a coordinated update of all 14 fixtures. Current expectation: Session 2 remains within the existing companion-rules v1 envelope, but this must be confirmed during the Session 2 scope-verification sweep.
SESSION 2 PENDING — CHANGE 2.1 (transition_derivation_version):
Frozen-interface change. Adds a new required field transition_derivation_version of type SemanticVersion to CoreTransitionEvidence in interfaces/mktd03_library.did. Per G's decision, this is treated as a breaking frozen-interface change and the library interface_version is to be bumped accordingly. Six coordinated files minimum, plus any fixture-level field enumerations not yet located. Scope-verification sweep is required before editing.
Files named in v2 change list:

interfaces/mktd03_library.did
interfaces/mktd03_library_interface_rules.md
docs/planning/ADR-03-tree-mode-cvdr-structure.md
docs/test-vectors/MKTd03_negative_cases_v1.md
docs/test-vectors/MKTd03_golden_vectors_v1.md
docs/test-vectors/fixtures/manifest.md

Plus one new machine-readable verifier negative fixture under docs/test-vectors/fixtures/verifier/negative/.
G-DECIDED OPEN QUESTIONS (resolved before Session 2):
A. transition_derivation_version applies only to core_transition_evidence. No parallel certification/provenance version field unless an independent derivation-rule evolution case appears.
B. This IS a breaking frozen-interface change. Bump the library interface_version accordingly. Do not hand-wave that part.
C. Rhetorical/comparative framing ("certified record of an execution, not a declaration") lives outside normative MKTd03 spec material, in a non-normative TAV-Engineering-Standards communications/documentation-pattern note. Only the evidentiary formulation appears in MKTd03 protocol docs.
G'S FORWARD-LOOKING CAUTION FOR SESSION 2:
If the receipt field set is enumerated anywhere beyond the six files already named — especially in fixture indexes or manifest-style field enumerations — that surface must move in the same session. Do not let transition_derivation_version become a "mostly coordinated" interface change. Scope-verification sweep is required before the Session 2 edit stream begins.
SESSION 3 PENDING — DOCUMENTATION ADDITIONS:
Batchable. No interface changes. Changes 3.1 through 3.4 per the agreed v2 change list:

3.1: Extend non-claims list in ADR-03 (forensic byte erasure, semantic resurrection, undeclared residues)
3.2: New docs/analysis/MKTd03_rst_evaluation_lens_v1.md (non-normative)
3.3: Add certified module-hash strengthening-path note to docs/spec/MKTd03_security_privacy_note_v1.md
3.4: Adopt evidentiary claim formulation in docs/spec/MKTd03_protocol_refresh_v1.md; separate TAV-Engineering-Standards non-normative note per G's decision (C above)

AUTHORITY ORDER:

Live MKTd03 repo state
Committed close-out sequence ending at cdfc097, followed by handover-pack commit d1f6c85, followed by Session 1 commits aeec179 / 35fa3fa / 24db28f
Approved ADR baseline decisions (including Session 1 additions)
Approved or accepted baseline spec notes and companion rules where they do not conflict with ADRs or committed close-out state (including Session 1 additions to companion rules)
Conceptual interface and vector artifacts as pre-freeze / pre-next-phase inputs only
Old spreadsheet MKTd03 tab as audit input only
Prep artifacts and TinyPress materials as historical context only

OPERATING CONSTRAINTS:

ADR-01 scoping guardrail remains in force: ADR-01 must not become a catch-all.
Do not let TinyPress leak into protocol truth, examples, payloads, routes, schemas, fixtures, or interface names.
Do not let MKTd02 implementation history become authority; use it only for bounded reuse analysis.
Do not begin code before the relevant future gate is explicitly opened.
Session 2 requires scope-verification sweep before any file edits.
Session 2 treats the library interface_version bump as non-optional.
Session 3 keeps the rhetorical/comparative claim framing out of MKTd03 spec material.

DURABLE FINDINGS:

The formal-interface phase remains closed at the Step-4 boundary (cdfc097). The Session 1 specification-tightening is additive to that baseline, not a reopening.
The Zombie-delete-MKTd03 comparison confirmed the core MKTd03 architectural decisions (ADR-01, ADR-02, ADR-03) without requiring any of them to be reopened.
"Certified record of an execution, not a declaration" is the working summary of what MKTd03 provides that a two-canister witness alternative cannot; the precise normative claim is "cryptographic proof of a deletion transition of authoritative application state."
Fixture-version coupling (14 rules_version_ref anchors) is a known constraint for any future companion-rules version bump.

OPEN QUESTIONS (deferred, not active in this phase):

Whether a machine-readable GV-05A fixture should be created in a later bounded phase.
Whether any future corpus growth is justified beyond the current resolved baseline.

SAFE RESTART PROMPT:
MKTd03 is resuming from a Session 1 substantive checkpoint at commit 24db28f (Session 1 of the specification-tightening stream). The formal-interface phase remains closed at cdfc097. Session 1 is complete: atomicity of CVDR issuance is now explicitly locked in both the library interface rules and ADR-03, and ADR-01 now explicitly names the module-boundary rule (separate module != separate canister) and the genericity rule (protocol-generic, not schema-generic), each with paired rejected alternatives. Sessions 2 and 3 of the modification stream remain. Session 2 is Change 2.1 — adding a required transition_derivation_version field to CoreTransitionEvidence, treated as a breaking frozen-interface change requiring a library interface_version bump. Session 2 must begin with a scope-verification sweep for any fixture-level or manifest-level field enumerations beyond the six files already named in the v2 change list. Session 3 is documentation additions and is batchable. The full v2 change list, G's review, and G's decisions on open questions A/B/C are captured above. The repo is dApp-agnostic. TinyPress is a reference target only. MKTd02 is bounded-analysis input only. No new code, fixture corpus growth, or orchestration expansion is authorised outside the stated Session 2 and Session 3 scopes.
