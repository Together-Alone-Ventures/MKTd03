## 2026-03-26 -- MILESTONE: Repo boundary cleanup complete

Decisions made:
  - MKTd03 is confirmed as a dApp-agnostic protocol project repo. It is not the live home of TinyPress application code or TinyPress app-governance docs.
  - TinyPress is confirmed as the first reference target only. It remains relevant to MKTd03 as an integration baseline, but it does not define MKTd03 scope.
  - TinyPress live app materials now belong in the TinyPress repo. Historical TinyPress-origin artifacts retained in MKTd03 are provenance-only and must not be treated as live work surfaces.
  - Standards and cross-project process extraction into TAV-Engineering-Standards remains deferred to a later bounded task.
  - README work is boundary-governance work only. It must not reintroduce TinyPress-specific framing into MKTd03.

Irreversible actions taken:
  - TinyPress_ADR_v1.1.docx moved out of MKTd03/docs/ and restored to the TinyPress repo as the live app architecture record.
  - Repo-local guidance files in both repos were rewritten to reflect the settled two-repo boundary.
  - TinyPress continuity was refreshed in the TinyPress repo to remove stale cross-repo drift.

Do not revisit:
  - Whether TinyPress should remain inside MKTd03 as the live app surface — settled no.
  - Whether MKTd03 should carry TinyPress-local continuity as its active root narrative — settled no.
  - Whether README cleanup and standards extraction should be mixed into implementation work — settled no; keep bounded.

## 2026-03-26 -- MILESTONE: MKTd03 prep gate closed

Decisions made:
  - Codex setup is now verified as an operator baseline, with global and repo-local AGENTS files created and instruction loading confirmed in MKTd03, TinyPress, and TAV-Engineering-Standards.
  - Standards uplift from the MKTd03 prep review is complete and was pushed to TAV-Engineering-Standards main at 8709d20.
  - The MKTd03 prep phase is now closed. The next session should begin the first bounded protocol-library design / ADR step rather than reopening prep work.
  - Repo-boundary cleanup and standards uplift remain closed unless a specific regression is identified.

Irreversible actions taken:
  - Added and verified global `~/.codex/AGENTS.md`.
  - Added repo-local `AGENTS.md` files in MKTd03, TinyPress, and TAV-Engineering-Standards.
  - Patched and pushed TAV-Engineering-Standards with prep-derived Playbook and Design Principles tightenings at commit 8709d20.
  - Updated the standalone Codex setup note to the final reviewed version, including pasteable starter templates and corrected repo-context discovery wording.

Do not revisit:
  - Whether standards uplift must be completed before the first MKTd03 library design step — settled yes, now done.
  - Whether Codex/operator setup remains an open prep dependency — settled no.
  - Whether the next MKTd03 session should resume prep instead of beginning protocol-library design — settled no.

## 2026-03-26 -- SESSION LESSON: standards baseline tightened before ADR start

Decisions made:
  - TAV-Engineering-Standards received one final bounded cleanup pass before the first library ADR session.
  - The standards repo head for the clean starting baseline is now 738c11d, replacing the earlier prep-closeout reference to 8709d20.
  - This was a small baseline-tightening patch, not a reopening of prep or standards work.

Do not revisit:
  - Whether this requires reopening the prep phase — settled no.
  - Whether broader Playbook rewrite work should happen before the first library ADR — settled no; treat as later bounded debt if needed.

## 2026-03-30 -- MILESTONE: Formal-interface close-out checkpoint completed

Decisions made:
  - The current MKTd03 formal-interface/conformance cleanup phase is closed at the Step-4 boundary and does not roll forward into the next phase.
  - The blocking `PreStateCaptured` adapter-contract .did gate was resolved by freezing a named positive pre-state result family without changing payload structure.
  - Minimal positive pre-state semantics for `capture_pre_state` were pinned in the adapter companion rules and aligned explicitly with the .did surface.
  - Golden-vectors authority references were relocated from `docs/spec/` to `docs/test-vectors/`, and dependent fixture-manifest / fixture authority references were aligned accordingly.
  - No machine-readable positive pre-state fixture was created, and no continuation beyond resolve-success orchestration was started.

Irreversible actions taken:
  - Committed `0bf90b9` — `interfaces: freeze named pre-state capture result`
  - Committed `cfacc7f` — `docs: align fixture manifest golden vector paths`
  - Committed `4490f2b` — `docs: pin minimal pre-state capture semantics`
  - Committed `cdfc097` — `docs: relocate golden vectors authority references`

Do not revisit:
  - Whether the current phase should continue into machine-readable pre-state fixture creation — settled no.
  - Whether resolve-success should be extended into a new positive continuation path from this checkpoint — settled no.
  - Whether the path/authority cleanup should be treated as semantic corpus growth — settled no.
  - Whether the repo is being handed over mid-expansion — settled no; this is a clean close checkpoint.

## 2026-04-20 -- MILESTONE: Specification-tightening stream Session 1 landed
Decisions made:

An analytical comparison between MKTd03 and the Zombie-delete-MKTd03 monorepo (Antoine's build) confirmed the core MKTd03 architectural decisions without requiring any ADR to be reopened.
A bounded specification-tightening modification stream was opened, organised into three sessions, to close drift windows identified by the comparison:
Session 1 — ADR/interface wording tightening (load-bearing)
Session 2 — transition_derivation_version field (frozen-interface change)
Session 3 — documentation additions (batchable)
Session 1 is complete. Sessions 2 and 3 are pending and will run in fresh chats.
G decided the three open questions before Session 2:
A. transition_derivation_version applies only to core_transition_evidence; no parallel certification/provenance version field.
B. Adding the required field is a breaking frozen-interface change; library interface_version bump is required, not optional.
C. Rhetorical/comparative claim framing stays outside MKTd03 normative spec material; it lives only in a non-normative TAV-Engineering-Standards note.
The RST evaluation lens note will live under docs/analysis/ as a non-normative analytical artefact, not under docs/spec/.
A cross-reference sweep across docs/, interfaces/, src/, and continuity files was run and returned no stale references; no housekeeping commit was required.

Irreversible actions taken:

Committed aeec179 — interfaces: add issuance atomicity and retrieval semantics rules (Change 1.1)
Committed 35fa3fa — ADR-03: add issuance atomicity clause and rejected alternative (Change 1.2)
Committed 24db28f — ADR-01: add module-boundary and genericity rules with rejections (Changes 1.3 and 1.4)
Added MKTd03_Handover_Pack.md and MKTd03_build_plan.md to main at d1f6c85 (handover context for the modification stream).

Do not revisit:

Whether the Zombie-delete-MKTd03 comparison requires any MKTd03 ADR to be reopened — settled no.
Whether the specification-tightening changes should be interleaved rather than session-separated — settled no; Sessions 1, 2, 3 remain distinct commit streams.
Whether module-boundary and genericity should stay implicit in ADR-01 — settled no, both are now explicit rules with paired rejected alternatives.
Whether Change 2.1 is a breaking frozen-interface change — settled yes; interface_version bump required.
Whether the rhetorical claim formulation belongs inside normative MKTd03 spec material — settled no.

Standing constraint surfaced:

14 machine-readable fixtures reference interfaces/mktd03_library_interface_rules.md via rules_version_ref: "...md#v1". Any future bump of that companion-rules file to #v2 requires a coordinated update of all 14 fixtures. Session 2 is currently expected to remain within v1, pending confirmation during the Session 2 scope-verification sweep.

## 2026-04-20 -- MILESTONE: Specification-tightening stream Session 2 landed

Decisions made:
  - Session 2 Change 2.1 is complete and pushed.
  - `transition_derivation_version : SemanticVersion` was added as a required field on `CoreTransitionEvidence`.
  - The breaking frozen-interface change was reflected by bumping library `interface_version` from 1.0.0 to 2.0.0.
  - `interfaces/mktd03_library_interface_rules.md` was bumped from v1 to v2, and fixture `rules_version_ref` anchors were retargeted to `#v2`.
  - Source alignment was brought into Session 2 scope rather than leaving a knowingly inconsistent tree.
  - A new verifier negative family `missing_transition_derivation_version` was added, with verifier dispatch returning `Deferred(...)` rather than `NotImplemented`.
  - Protocol version and receipt version did not change.
  - Session 3 remains open and has not yet begun.

Irreversible actions taken:
  - Committed `e43890f`
  - Committed `4c1d95d`
  - Committed `8a07fb1`
  - Committed `7ddf40c`
  - Committed `6037154`
  - Committed `7b4db16`

Do not revisit:
  - Whether Change 2.1 is a breaking frozen-interface change — settled yes.
  - Whether the companion-rules file remains v1 — settled no; v2 now governs.
  - Whether source alignment should have been deferred to a later session — settled no.
  - Whether the new verifier negative family should fall through to `NotImplemented` — settled no; `Deferred(...)` is the approved posture.
  - Whether protocol version or receipt version changed in Session 2 — settled no.

## 2026-04-20 -- MILESTONE: Specification-tightening stream Session 3 landed

Decisions made:
  - All five Session 3 substantive changes are complete and pushed.
  - ADR-03 §8 "Explicit non-claims" extended with five interpretation-limit bullets covering identifier exhaustion, undeclared-surface inspection, scope-selection completeness, verification-completeness equivalence, and silence-as-proof elevation, plus a closing clause declining both narrowing and verifier-duty expansion.
  - `docs/spec/MKTd03_protocol_refresh_v1.md` §2 ("Evidentiary scope") was rewritten to align with ADR-03 §8 original-bullet wording verbatim where overlap existed, add an inline equivalence non-claim in technical language, and defer the full non-claims set to ADR-03 via an authority pointer with an explicit "ADR-03 wins" tie-breaker. "conservative and archival-first" posture language retained.
  - `docs/spec/MKTd03_security_privacy_note_v1.md` §6 ("Certification and provenance") received one additive paragraph naming representative optional paths for additional certification or provenance layers (certified query routes, reproducible-build attestation chains anchored to the published build, transparency-log or chain anchoring of the ADR-03 stable provenance pointer) without adopting any path.
  - `docs/analysis/MKTd03_rst_evaluation_lens_v1.md` was created as a new non-normative analytical note defining the Residual Trust Statement (RST) evaluation lens. Seven residual-trust categories, four-step application procedure, five reflexive non-claims about the lens itself, and orientation-only cross-references to ADR-00/01/02/03, Protocol Refresh v1, and Security/Privacy Note v1 via heading names rather than section numbers.
  - `docs/analysis/` directory was created in MKTd03 for the first time by this addition.
  - `MKTd03_Residual_Trust_Note.md` was added to the TAV-Engineering-Standards repo at root as a non-normative, project-scoped companion note for operator-facing rhetorical and comparative claim framing about MKTd03. Standards repo CHANGELOG was bumped to v1.1 in the same commit. Standards-repo head is now `cd719a3`. MKTd03-side RST lens and Standards-side companion note are decoupled — neither is authoritative for the other.
  - Commit-order change numbering was used (3.1, 3.2, 3.3, 3.4, 3.5) rather than item-based labelling (which would have produced 3.4a/3.4b).
  - A regulatory/legal-terminology boundary was enforced mid-session: terminology like "legally complete," "regulatorily sufficient," "jurisdiction-specific compliance," and "data-governance completeness" landed in ADR-03 §8 via Change 3.1 in non-claim framing, and was kept bounded to non-claim contexts for the remainder of the session. `protocol_refresh_v1` §2 and the security/privacy note §6 addition contain no such terminology. The RST lens and Standards note use it only in explicit non-claim contexts (RST lens §3.6 and §5; Standards note §3).

Irreversible actions taken:
  - Committed `d2367a8` (MKTd03) — ADR-03: extend section 8 non-claims with interpretation-limit clarifications (Change 3.1)
  - Committed `f37ade9` (MKTd03) — protocol_refresh_v1: align section 2 evidentiary-claim wording with ADR-03 §8 (Change 3.2)
  - Committed `f2cf4c5` (MKTd03) — security_privacy_note_v1: add module-hash strengthening-path paragraph to section 6 (Change 3.3)
  - Committed `0a21274` (MKTd03) — rst_evaluation_lens_v1: add new non-normative residual trust statement analytical lens (Change 3.4)
  - Committed `cd719a3` (TAV-Engineering-Standards) — add non-normative MKTd03 residual trust note and register in CHANGELOG (MKTd03 Change 3.5)

Do not revisit:
  - Whether Session 3 required interface, fixture, or verifier changes — settled no; all five changes are wording-only or new non-normative files.
  - Whether the RST evaluation lens belongs under `docs/spec/` or `docs/planning/` — settled no; `docs/analysis/` is its home per Session 1 decision, now instantiated.
  - Whether the Standards-side residual-trust note should be cross-project rather than MKTd03-scoped — settled no; abstraction to doctrine declined at this time.
  - Whether the Standards-side and MKTd03-side notes should reference each other as authority — settled no; both documents are non-authoritative for the other.
  - Whether regulatory/legal terminology may propagate from ADR-03 §8 into normative artefacts as affirmative claims — settled no; such terminology is restricted to non-claim contexts regardless of whether it appears in ADR-03 §8, non-normative analytical locations, or operator-facing Standards notes.
  - Whether the security/privacy note §6 addition should use the word "strengthening" in the paragraph body — settled no; body text uses "addition," "additional certification or provenance layers," and "additive to" instead. The item label "strengthening-path" is retained in the change list only, for identification.

Standing constraint surfaced:
  - Regulatory/legal terminology remains bounded to non-claim contexts. Any future MKTd03 artefact may discuss legal/regulatory dimensions only as (a) an explicit non-claim inside ADR-03 §8 or an equivalent normative non-claim surface, or (b) material in a non-normative location such as `docs/analysis/` or TAV-Engineering-Standards. Introducing such terminology as an affirmative claim into protocol refresh, interface files, companion rules, or verifier documents requires explicit reopening.

## 2026-04-20 -- SESSION LESSON: stop when a normative-adjacent artefact has an undefined term in its title

What happened:
  - Session 3 Item 2 created `docs/analysis/MKTd03_rst_evaluation_lens_v1.md`. The filename was settled in RESTART_PACK and the Session 1 MILESTONE. When the drafter reached this item, no committed repo artefact explained what "RST" stood for. Three candidate expansions were plausible, each producing a materially different document.
  - The drafter stopped, surfaced the gap explicitly, and escalated rather than guessing. "RST = Residual Trust Statement" was confirmed, and drafting resumed.

The lesson:
  - Before drafting a normative or normative-adjacent artefact whose filename contains a term not defined anywhere in committed repo artefacts, stop and obtain an authoritative expansion. Even a well-reasoned inference can encode a wrong expansion as de facto authority by drift.

Category: review │ process

Apply next time:
  - When a RESTART_PACK or earlier MILESTONE names a new artefact by acronym or novel term, the pre-drafting checklist for that artefact includes: "Has this term been explicitly expanded in a committed repo artefact? If no, stop and escalate before drafting."

## 2026-04-20 -- SESSION LESSON: name standing constraints proactively when approving new terminology in a multi-item session

What happened:
  - Session 3 Item 1 (Change 3.1) approved regulatory/legal terminology into ADR-03 §8 in non-claim framing. During review, C flagged that this terminology shape should not propagate into remaining Session 3 items as affirmative claims, and recommended it as a standing constraint for the rest of the session.
  - The constraint was endorsed. Items 4a, 3, 2, and 4b all honored it without further friction. Where regulatory/legal registers were needed (RST lens §3.6 and §5; Standards note §3), they were scoped as non-claims or non-scope.

The lesson:
  - When approving a new terminology shape in the first item of a multi-item session, proactively name any boundary conditions as standing constraints for remaining items. This prevents silent drift where each individual review looks acceptable but the cumulative effect widens or relocates a register across items.

Category: review │ process

Apply next time:
  - First-item reviews in a multi-item session carry precedent weight. Any new register, terminology, or framing introduced should be accompanied by an explicit "standing constraint for this session" note that remaining items can be checked against.

## 2026-04-23 -- MILESTONE: Authority-block housekeeping landed; specification phase closed

Decisions made:
  - The stale "ADR-03 is currently an intermediate draft" language in the `docs/spec/MKTd03_protocol_refresh_v1.md` Authority block was removed.
  - Removal was bounded to the two-sentence provisional paragraph only. The tiebreaker line ("Where this document and an ADR differ, the ADR wins.") was retained.
  - The file's `## Status` field ("Draft") was explicitly held out of scope and recorded as a separate future open candidate.
  - With this commit, the specification-and-documentation phase of MKTd03 is fully closed. The next bounded session opens the engine build planning phase (build plan Phase 7) but will not write code.

Irreversible actions taken:
  - Committed `16f1124` (MKTd03) — web-editor partial deletion of the first stale sentence only.
  - Committed `44f849b` (MKTd03) — web-editor mistaken re-addition of the second stale sentence.
  - Committed `4ea134a` (MKTd03) — CLI completion: removed the second stale sentence and left the tiebreaker line in place.
  - Committed `1e08238` (MKTd03) — RESTART_PACK updated for the new state and next-session framing.

Do not revisit:
  - Whether the Authority block should retain the "intermediate draft" wording — settled no; ADR-03 is Approved.
  - Whether the `## Status` field should have been updated in the same pass — settled no; explicitly out of scope for this bounded housekeeping commit.
  - Whether the noisy commit history (web-editor partial, web-editor revert, CLI correction) should be rewritten for tidiness — settled no; Playbook rule is no history rewrites.

## 2026-04-23 -- SESSION LESSON: multi-line edits must use CLI, not the GitHub web editor

What happened:
  - A two-sentence stale-language paragraph in `docs/spec/MKTd03_protocol_refresh_v1.md` needed removing. The Playbook rule (web editor for single-line typo fixes only; CLI for anything else) was correctly restated by G at the start of the pass. C then issued instructions that used the phrase "editor of your choice" after G had already closed that choice, and the operator chose the GitHub web editor.
  - The web editor treated the two-sentence paragraph as two separate lines (which they were in the raw markdown, even though they rendered as one paragraph). A first web-editor edit deleted only the first sentence. A second web-editor attempt mistakenly re-added the second sentence. A CLI follow-up was required to complete the removal cleanly.

The lesson:
  - "Multi-line edit" means any edit that deletes or modifies more than one line of raw file text, regardless of how the lines render in the final document. Paragraphs that visually read as one unit may be two or more lines in the source.
  - When the Playbook rule is restated by G, C does not have latitude to re-open the choice. Framings like "editor of your choice" are only appropriate when the Playbook leaves the choice open. C must check whether the Playbook has already closed it.

Category: process │ review

Apply next time:
  - Before issuing edit instructions, C states the edit method explicitly (CLI or web editor) and cites the Playbook clause that governs the choice. No "editor of your choice" language for multi-line edits.
  - If the operator nonetheless uses the web editor for a multi-line edit, the immediate next step is a `git show` on the resulting commit to inspect whether the web editor's line handling produced the intended change, before any follow-up commit is planned.

## 2026-04-24 -- MILESTONE: Authority-map / AGENTS refresh packet landed

Decisions made:
  - `docs/planning/MKTd03_authority_map_v1.md` superseded by `docs/planning/MKTd03_authority_map_v2.md`. Version bump per G's prior-session ruling that the post-Session-3 refresh warrants v2, not v1-with-stamp. v1 file retained in place as historical record; future disposition deferred (G's preferred disposition: relocation to `docs/planning/history/`).
  - `AGENTS.md` refreshed. Three "Current phase" lines rewritten to reflect post-specification-tightening reality and to re-scope the coding-start gate against `docs/planning/MKTd03_first_slice_scope_v1.md` §6 (currently `MKTd03_first_slice_scope_planning_DRAFT_v2.md` §6 until promoted). One new Non-goals bullet added covering the coding-start gate dependencies. Working discipline block preserved unchanged.
  - All 10 authority-map rows flagged for update in the refresh-packet scope (§3.1) absorbed. Row "Refreshed overall protocol narrative" landed at "settled" with citation to housekeeping commit `4ea134a`, per G's ruling D and HEAD evidence.
  - All 5 new authority-map rows enumerated in the refresh-packet scope added.
  - TAV-Engineering-Standards residual-trust note row carries literal "non-normative cross-repo reference; not MKTd03 authority" labelling in its status cell, per G's end-of-prior-session instruction on pointer framing.
  - Per G's secondary review on this packet, authority map v2 gained one additional row ("Coding-start readiness gate") pointing to `docs/planning/MKTd03_first_slice_scope_v1.md` §6 as sole authority, with status "binding; must not be re-authored outside this artifact". Rationale: coding-start is now a first-class phase boundary referenced from AGENTS, RESTART_PACK, and the authority map; elevating it to an authority-map row eliminates future drift where someone "summarises the gate" elsewhere.
  - Per G's secondary review on this packet, AGENTS "Current phase" section gained one additional bullet establishing `docs/planning/MKTd03_first_slice_scope_v1.md` §6 as the sole authoritative definition of the coding-start gate, with explicit statement that AGENTS does not restate or reinterpret it. Rationale: eliminates dual-authority ambiguity between AGENTS (operationally read) and §6 (actually correct).
  - `RESTART_PACK.md` refreshed to correct the stale HEAD self-reference and to update the NEXT BOUNDED SESSION framing to match post-last-session sequencing: §5 reuse-audit close-out → C pre-implementation adversarial review → G's coding-start decision. Refresh-packet scope §2 declined a RESTART_PACK update in principle; G ratified the widen at packet-draft review.

Irreversible actions taken:
  - Committed f0c1ed497f8af779f13f4799782f405a36e37f1e (MKTd03) — authority_map: add v2 reflecting post-Session-3 reality; v1 retained as historical.
  - Committed 274f6015b44977ad7338996d620a75d3074ea517 (MKTd03) — agents: refresh current-phase framing and re-scope coding-start gate.
  - Committed 8038f79d59eb91a3496a7b3f8cd2a734c713c8fc (MKTd03) — restart_pack: update HEAD self-reference and next-session framing for post-refresh state.

Do not revisit:
  - Whether the authority map bumps to v2 or stays at v1-with-stamp — settled v2.
  - Whether the refresh-packet scope itself required promotion from DRAFT — settled no; operating-brief status sufficient.
  - Whether `docs/spec/MKTd03_protocol_refresh_v1.md` authority-block housekeeping is closed — settled yes at `4ea134a`, confirmed by HEAD evidence.
  - Whether the TAV-Engineering-Standards residual-trust note is MKTd03 authority — settled no; listed as non-normative cross-repo reference with explicit labelling.
  - Whether the refresh session should also touch `interfaces/mktd03_tree_mode_conceptual_interface_v1.did` itself — settled no per refresh-packet scope §8.
  - Whether `docs/spec/MKTd03_adapter_contract_concept_v1.md` should be deleted rather than marked reference-only — settled no; retained in place with reference-only status, per minimum-change principle.
  - Whether the coding-start gate should be defined or restated in AGENTS or the authority map — settled no; first-slice scope §6 is the sole authoritative definition; AGENTS and the authority map only point to it.

Standing constraint surfaced:
  - The coding-start gate is specified by `docs/planning/MKTd03_first_slice_scope_v1.md` §6 readiness checklist (currently the draft `MKTd03_first_slice_scope_planning_DRAFT_v2.md` §6 until promoted). The AGENTS "Current phase" coding-start line and the authority map v2 both reference this. Any future change to the readiness checklist must land in first-slice scope §6 and must not be silently re-authored in AGENTS or the authority map.
  - RESTART_PACK updates are normally out-of-scope for bounded packets. This instance is an exception due to multi-field staleness (HEAD reference + session framing).

Adjacent drift flagged but not acted on (for a future pass):
  - AGENTS Working discipline bullet ("…binding for interface-prep work…") carries a pre-Phase-6 descriptor that is now linguistically stale. Scope brief §3.2 instructed preserve-unchanged; followed literally.
  - AGENTS Non-goals bullet 3 ("No premature library implementation before the formal-interface gate is complete") is now a historical marker; bullet 4 supersedes it operationally. Both coexist per scope brief §3.2 preserve-unchanged instruction. G's preferred remediation (from secondary review of this packet): prefix with "(Historical — pre-Phase-6)".
  - Authority map row ("Conceptual interface seed artifact") retains "pre-freeze draft" status language, which is linguistically stale. Scope brief §3.1 did not enumerate this row for update. G's preferred remediation: explicit "non-authoritative and must not diverge from frozen interface on overlapping types".

## 2026-04-24 -- MILESTONE: §5 reuse-audit close-out landed

Decisions made:
  - Three §5 analysis artifacts promoted from v2 draft to approved status at `docs/analysis/`: `MKTd02_module_taxonomy_for_MKTd03.md`, `MKTd02_module_reuse_audit_v1.md`, `MKTd02_generalise_now_backport_later_candidates_v1.md`. G review 2026-04-24 approved all three with one directed edit (taxonomy §3.4 explicit Category E "absent in MKTd02" row added); reuse audit and candidate list approved without substantive changes.
  - MKTd02 primary anchor pinned: repo `Together-Alone-Ventures/ICP-Delete-Leaf` (formerly `MKTd02`) at commit `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`. Same SHA as the Phase-4 reuse-generalisation audit v1 at `docs/spec/MKTd03_mktd02_reuse_generalisation_audit_v1.md`; the SHA resolves transparently under the renamed repo.
  - `zombie-core` secondary anchor pinned: repo `Together-Alone-Ventures/zombie-core` at tag `zombie-core-v0.3.1` commit `508f2f8bb88f4395293168c6ef25c92a67dee894`. Dependency pin confirmed in `mktd02/Cargo.toml`.
  - Eight open questions across the three artifacts resolved; rulings recorded per-artifact in their "Resolved questions" sections. Cross-indexed here:
    - Taxonomy Q1 (Category E representation): explicit §3.4 "absent in MKTd02" row added.
    - Reuse audit Q2 (`zombie-core::nns_keys` assumption promotion): audit-local; do not promote until certification/trust-root slice.
    - Reuse audit Q3 (Finalization-lock shape, §5.3): Phase-7 planning input; not an ADR-03 reopening.
    - Reuse audit Q4 (tag-namespace strategy, §5.4): decision deferred to first-slice scope proposal; option (c) — MKTd03-owned tag constants — recorded as preferred default.
    - Candidate list Q1 (Candidate 6): Phase-7 planning input; not an ADR-03 reopening.
    - Candidate list Q2 (Candidate 3): precondition before any hashing/preimage slice (S7-3 or later); not a precondition for S7-1.
    - Candidate list Q3 (Candidate 7): left analytical; do not promote to normative companion-rule text until a later slice issues receipts.
    - Candidate list Q4 (missing candidates): none.
  - Session role map: C drafts, G reviews (role-swap allowance per session open; same as the allowance under which the v2 drafts were originally written).
  - `RESTART_PACK.md` refreshed for post-§5 state: CURRENT GOAL updated; new §5 REUSE-AUDIT CLOSE-OUT section added; SESSION LESSONS entries for 2026-04-24 appended; NEXT BOUNDED SESSION framing updated to C pre-implementation adversarial review per G's four-step sequence step 3.

Irreversible actions taken:
  - Committed 936b873fd1c511cd8125a8143a25e588b1dcfa7f (MKTd03) — analysis: promote section 5 reuse-audit drafts to approved at docs/analysis/ (G review 2026-04-24; anchors pinned). Commit 1 of this session's sequence; amended once to correct file mode from 100755 to 100644.
  - Committed 23b70c6bd3ee07faa5841c9fb1cfa17b2c3ea66f (MKTd03) — restart_pack: update for post-section-5 state; next session is C pre-implementation adversarial review. Commit 2 of this session's sequence; amended once to resolve three 1e485402a96e2efc84951408ab8949b56dff92bf self-reference placeholders with the commit's own SHA.

Do not revisit:
  - Whether the v2 drafts graduate cleanly — settled yes; G approved all three on 2026-04-24.
  - Whether the MKTd02 SHA from Phase-4 reuse-generalisation audit v1 carries to the §5 artifacts — settled yes; same SHA `54f1e2dc…` resolves under the renamed `ICP-Delete-Leaf` repo.
  - Whether Candidate 6, Candidate 7, or the Finalization-lock-shape finding reopens ADR-03 — settled no; all three are Phase-7 planning inputs only.
  - Whether the tag-namespace strategy is decided now — settled no; option (c) preferred, final decision deferred to the first-slice scope proposal.
  - Whether the `zombie-core::nns_keys` carry-over assumption graduates to normative text now — settled no; stays audit-local until the certification/trust-root slice.
  - Whether the `zombie-core::receipt` split classification is restored — settled no; v2 single-bucket (Leaf-mode-specific) stands; derivation discipline is carried as Candidate 7 narrative only.

Standing constraint surfaced:
  - Tag-namespace option (c) — MKTd03-owned tag constants, never using `zombie-core`-declared constants directly — is the preferred default for any MKTd03 slice that touches hashing or preimage composition. First-slice scope proposal is the authoritative place to finalise the decision; any deviation from option (c) at slice-planning time must be explicit and re-gated.
  - Candidate 3 (tag-discipline extraction) becomes a precondition before any MKTd03 slice that uses hashing or preimage composition. S7-1 does not hit this; S7-3 or later slices do. Slice scope proposals from S7-3 onward must confirm Candidate-3 precondition status before opening.

Adjacent drift flagged but not acted on (for a future pass):
  - Phase-4 reuse-generalisation audit v1 at `docs/spec/MKTd03_mktd02_reuse_generalisation_audit_v1.md` still references the old repo URL `https://github.com/Together-Alone-Ventures/MKTd02` in its `Audit Scope` block. The SHA it pins is canonical under the rename so the reference remains correct-in-fact, but the URL is now a redirect. Candidate for a bounded hygiene pass: update the Phase-4 audit's `MKTd02 repo URL` field to the renamed repo. Not in §5 close-out scope.

## 2026-04-24 -- SESSION LESSON: repo-rename SHA carry-over

What happened:
  - The §5 close-out required pinning an MKTd02 audited-commit SHA. The Phase-4 reuse-generalisation audit v1 had previously pinned `54f1e2dc24dd0b79705a66894b2f25138e28a9ad` under the old repo name `Together-Alone-Ventures/MKTd02`. The repository had since been renamed to `Together-Alone-Ventures/ICP-Delete-Leaf` with no history rewrite.
  - G's ruling 2026-04-24 was to keep the same SHA; it resolves transparently under the renamed repo. No re-audit of the source tree was required. The anchor description in each of the three §5 artifacts flags the rename explicitly: "formerly `MKTd02`; internal source name still `mktd02`".

The lesson:
  - Pure repository renames do not invalidate previously-pinned commit SHAs. When a repo is renamed without history rewrite, SHAs remain canonical across the rename; subsequent artifacts may pin the same SHA under the new repo name. The audit-target identity travels with the commit, not with the URL.
  - The URL of the repo at the old name is now a redirect. Any artifact that embeds a full repo URL alongside a SHA is correct-in-fact until the redirect is retired, but should be updated on any subsequent touch of that artifact.

Category: process │ repository-hygiene

Apply next time:
  - When an anchor SHA has been pinned in a prior artifact and the repo has since been renamed, the default is to carry the SHA under the new repo name. No re-pinning is required. Flag the rename explicitly in the new artifact's anchor description.
  - When any prior-artifact URL field is visible in the scope of a later bounded session, note stale-but-functional URLs in the Adjacent drift flagged list, to be corrected in a future hygiene pass.
  - Playbook uplift candidate: "Anchor-SHA carry across repo rename" as a named pattern.

## 2026-04-24 -- SESSION LESSON: Dropbox → WSL file-transfer carries NTFS executable-bit

What happened:
  - Commit 1 of this session's sequence staged three markdown files copied from Dropbox (`/mnt/c/Users/Stef/Dropbox/…/MKTd03/`) to `docs/analysis/` via `cp`. The resulting `git commit` output showed `create mode 100755` on all three — the NTFS executable-bit from the Dropbox-side copy carried through to the working tree on `git add`.
  - Inconsistent with the existing `docs/analysis/` convention (the pre-existing `MKTd03_rst_evaluation_lens_v1.md` in the same directory was at mode 100644). Required a `chmod 644` + `git commit --amend --no-edit` cycle to correct. No correctness impact; markdown at 100755 is functionally identical to 100644. Repo-convention issue only.

The lesson:
  - Any file copied from `/mnt/c/` into the WSL checkout inherits NTFS file-mode semantics at `git add` time. For non-executable assets (markdown, JSON, text), this produces `mode 100755` on the first commit, which deviates from repo convention.
  - The correct pattern for Dropbox → WSL → git transfer of non-executable assets is: `cp` → `chmod 644 <files>` → `git add` → `git commit`. The `chmod` step is not optional; it is a required part of the transfer pattern.

Category: process │ file-transfer

Apply next time:
  - C's file-transfer instructions for Dropbox → WSL → git of non-executable assets include a `chmod 644` step between `cp` and `git add`, not as a post-commit correction. Verify mode with `ls -la` before staging.
  - If the first `git commit` on a transferred file shows `create mode 100755` and the file is non-executable, the correct response is immediate `chmod 644` + amend before any downstream commit lands in the sequence.
  - Playbook uplift candidate: standing `chmod 644` step in the Dropbox → WSL → git transfer pattern for non-executable assets.

## 2026-04-24 -- MILESTONE: C pre-implementation adversarial review complete

Decisions made:
  - C adversarial review of the Phase 6 artifact set as a post-Session-3 whole executed per G's four-step sequence step 3; ruled Approve-with-flags.
  - Review anchored to HEAD = `776aff7394dc3fb7e16cebd7bf081c1489fdb621` (§5 close-out MILESTONE_LOG commit).
  - No flag blocks step 4 (G's explicit coding-start decision) from opening.
  - Role-swap from §5 close-out carried into this session per Stef's ruling 2026-04-24: C drafted the review brief; G reviewed the brief (one gap identified: HEAD SHA pinning) and executed the adversarial review.
  - Stress-test result: the expensive failure pattern — CVDR-as-declaration rather than certified record of execution — is not structurally present in the Phase 6 set. Lazy receipt construction is blocked at the spec layer by the absence of receipt-producing paths in S7-1 and by A→B→C discipline preserved as later-slice input.
  - Per-artifact rulings: three Flags, one Approve-with-flag, all remaining Phase 6 artifacts Approve.
    - Flag: `AGENTS.md` + `RESTART_PACK.md` — adjacent drift (pre-Phase-6 "formal-interface gate" / interface-prep wording). Folds into existing open-candidates queue item (AGENTS adjacent-drift cleanup pass); scope expanded to include RESTART_PACK.md.
    - Flag: `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` still reads "Draft". Existing open-candidates queue item confirmed as active.
    - Flag: Authority map v2 row 15 ("Conceptual interface seed artifact") retains "pre-freeze draft" descriptor. Existing open-candidates queue item confirmed as active.
    - Approve-with-flag: First-slice scope draft v2 — update context anchor from old `3319c3f` at promotion time. Attached to existing open-candidates queue item (first-slice promotion) as promotion-time detail.
  - Approve rulings (no flag): stale spec inventory (no evidence of stale spreadsheet authority reassertion); ADR-00 evidentiary scope; Tree-mode invariants note; ADR-02 tree structure; ADR-01 boundary; ADR-03 receipt/verifier semantics (Session 1/3 tightening blocks declaration-based and lazy receipt semantics); diagnostics/status note; versioning/compatibility note (post-Session-2 `interface_version = 2.0.0` reflected in authority map); security/privacy note (certification/provenance strengthening remains optional/additive, not adopted as proof claim); published terminology policy; conceptual adapter contract (reference-only status preserved, superseded by frozen adapter contract); companion-rule layer (v2 governs, no v1/v2 partial state); golden vectors / negative cases (no unsupported positive extension); frozen `.did` interfaces (`interface_version = 2.0.0`, `PreStateCaptured` frozen, coherent post-Session-2 state); machine-readable fixtures (`#v2` anchors and `missing_transition_derivation_version` reflected); `transition_derivation_version` row (correctly scoped to `core_transition_evidence` only); library interface version row (correctly binds `.did` + rules v2); `docs/analysis/` convention / RST lens / Standards note (non-normative boundary explicit); coding-start readiness gate (correctly points to first-slice scope §6 and prevents gate re-authoring).
  - No flag identified in the review is net-new to the open-candidates queue. All four fold into existing items. Queue updated accordingly in RESTART_PACK at Commit 1 of this session's sequence.

Irreversible actions taken:
  - Committed 1dcdee10c7ce49b07e45b4eddfedbf99904ea555 (MKTd03) — restart_pack: record pre-implementation review outcome; next bounded session is G coding-start decision. Commit 1 of this session's sequence.
  - Committed 1e485402a96e2efc84951408ab8949b56dff92bf (MKTd03) — milestone_log: record pre-implementation adversarial review ruling and session lesson. Commit 2 of this session's sequence.

Do not revisit:
  - Whether the Phase 6 artifact set holds as a post-Session-3 whole — settled yes, ruled Approve-with-flags.
  - Whether any flag blocks step 4 — settled no.
  - Whether C drafts the pre-implementation review brief or G does — settled C drafts (role-swap confirmed for this session by Stef's ruling; reverts to standard role map after step 3 close).
  - Whether the pre-implementation review session includes any artifact edits beyond continuity — settled no; findings only, with continuity bounded to this session's RESTART_PACK and MILESTONE_LOG updates.
  - Whether any flag identified in the review is net-new to the queue — settled no; all fold into existing open-candidates items.
  - Whether the CVDR-as-declaration failure pattern is structurally present in the Phase 6 set — settled no.
  - Whether lazy receipt construction is permitted by the current spec — settled no; blocked by absence of receipt-producing paths in S7-1 and by A→B→C discipline preserved as later-slice input.

## 2026-04-24 -- SESSION LESSON: pin the HEAD SHA explicitly in review briefs anchored at "HEAD"

What happened:
  - C drafted the pre-implementation review brief under role-swap. The scope clause read "`docs/planning/MKTd03_authority_map_v2.md` at HEAD" without specifying a commit SHA. G's brief-review step flagged this as the only gap in the brief: "HEAD SHA must be filled before C runs the review, because 'authority map v2 at HEAD' is otherwise ambiguous." The SHA (`776aff7394dc3fb7e16cebd7bf081c1489fdb621`) was resolved via `git log -1` and pinned before the review executed.

The lesson:
  - "HEAD" is a moving reference. In a review brief that declares what artifact set is being reviewed, "at HEAD" is insufficient as an anchor — it must be "at HEAD [SHA]". The SHA is the real anchor; "HEAD" is only a convenience shorthand for "as of the moment of this brief". Without the SHA pinned, the review record is ambiguous the first time HEAD moves, and the review may end up evaluating a different tree than intended.
  - The brief-review step caught this cleanly. But the drafter (C under role-swap) should not rely on the brief-review step to catch it; pinning HEAD SHA in the brief itself is draft-time discipline, not review-time correction.

Category: process │ review

Apply next time:
  - Any review brief that anchors against a live repo state must pin the commit SHA explicitly at draft time. Template clause: "anchored to HEAD at [SHA]". If the SHA is not known at brief-draft time (rare — `git log -1` is always available), use a placeholder and fill it as a mandatory step before the review executes.
  - Candidate for the Playbook-uplift queue under review-brief drafting patterns; not yet promoted.

## 2026-04-24 — MILESTONE: S7-1 coding-start opened; first-slice scope promoted

Decisions made:
  - First-slice scope document promoted from draft to authoritative:
    `docs/planning/MKTd03_first_slice_scope_v1.md`.
  - Promotion commit: 8204b2c (parent c284c59).
  - Four mechanical edits applied during promotion:
    1. Title desuffixed (removed "Draft v2").
    2. Status line updated to authoritative coding-start state.
    3. Intended-path line deleted.
    4. §1 gating paragraph replaced with state statement ("all gates closed; authorised for S7-1 coding-start").
  - Context anchor inside the document updated separately from 3319c3f → c284c59.
    (This is a document context anchor, not the promotion commit SHA.)
  - Coding-start decision: OPEN under G ruling, with five binding constraints:
    strict slice containment; module_hash status-only discipline; `.did` zero-divergence gate;
    observable fail-loud behaviour; no forward-semantics leakage.
  - Standard role map restored (G drafter, C adversarial reviewer, Codex executor).

Irreversible actions taken:
  - 8204b2c — planning: add first-slice scope v1 (S7-1) for coding-start

Do not revisit:
  - Whether S7-1 is open for implementation — settled OPEN under G ruling.
  - Whether the draft status is still in effect — settled no; document is authoritative.
  - Whether the context anchor equals the commit SHA — settled no; they are distinct.

Standing constraint surfaced:
  - S7-1 must remain a closed minimal slice; any introduction of deletion, receipt, proof,
    certification, or provenance semantics is a hard stop, not an extension.

## 2026-04-27 -- MILESTONE: S7-1 status-surface implementation landed

Decisions made:
  - S7-1 implementation landed as a status-surface skeleton with lifecycle scaffold only.
  - Exposed canister surface is limited to init, post_upgrade, and get_tree_mode_status.
  - Stable storage introduced by this slice is limited to lifecycle_state and module_hash.
  - module_hash is used only for StatusSurface.build_identity.module_hash, not for hash preimages, receipts, evidence, certification, provenance, or verifier semantics.
  - The S7-1 .did zero-divergence gate is interpreted as Candid-canonical equivalence: blob/vec nat8 aliasing, record-field order, and whitespace are non-semantic and normalized before comparison. Type names, field names, variants, field types, and method signatures are not normalized away.
  - cargo fmt --check remains red at parent a476baf due pre-existing non-S7-1 rustfmt drift.
  - full cargo test --offline remains red due pre-existing verifier fixture-loader issue for missing_transition_derivation_version.
  - dfx/local deploy was not run because the repo has no dfx.json; wasm32 build plus S7-1 lib tests are accepted for this slice.
  - The verifier fixture-loader issue is queued as a blocker before the first verifier-touching slice.

Validation evidence:
  - cargo test --offline --lib passed.
  - cargo build --offline --target wasm32-unknown-unknown passed.
  - Current uncommitted implementation files before commit were Cargo.toml, Cargo.lock, and src/lib.rs only.

## 2026-04-27 -- MILESTONE: S7-2 version-support surface landed

Decisions made:
  - S7-2 implemented check_version_support(SemanticVersion) -> VersionCheckResult per frozen .did.
  - VersionInfo introduced carrying protocol_version, interface_version, and compatibility.
  - protocol_version field always reports the library-supported PROTOCOL_VERSION.
  - supported(...) returned only for exact protocol version match.
  - unsupported_version(...) returned for major-version mismatch.
  - same-major but different version triggers explicit fail-loud:
    "S7-2 conditionally_compatible policy not yet defined"
  - conditionally_compatible classification intentionally not implemented in this slice.
  - .did zero-divergence gate extended using Candid-canonical equivalence for:
    VersionInfo, VersionCheckResult, and check_version_support.
  - no malformed-input trap required for typed SemanticVersion input.
  - full cargo test --offline remains red due pre-existing verifier fixture-loader issue.
  - cargo fmt --check remains red due pre-existing repo-wide formatting drift.

Validation evidence:
  - cargo test --offline --lib passed.
  - cargo build --offline --target wasm32-unknown-unknown passed.
  - implementation confined to src/lib.rs only.

## 2026-04-27 -- MILESTONE: S7-3 tag-discipline scaffold landed

Decisions made:
  - S7-3 establishes the MKTd03-owned tag namespace scaffold.
  - Exactly one sentinel tag was added: MKTD03_SCAFFOLD_V1.
  - The sentinel is not used in any hash preimage and exists only to validate tag-discipline tests.
  - No receipt, tombstone, commitment, tree-node, or evidence tags were declared.
  - Hashing and preimage construction remain deferred to a later slice.
  - No public canister method, Candid type, .did file, Cargo dependency, storage, adapter, verifier, or orchestration change was introduced.

Validation evidence:
  - cargo test --offline --lib passed.
  - cargo build --offline --target wasm32-unknown-unknown passed.
  - generated .did remains unchanged.
  - diff confined to src/lib.rs and src/tags.rs.

## 2026-04-27 -- MILESTONE: S7-4 generic hashing helper landed

Decisions made:
  - S7-4 introduced the protocol-owned generic hashing helper hash_with_tag(tag, parts).
  - The helper computes SHA-256 over the exact byte concatenation tag || parts[0] || parts[1] || ...
  - Tag bytes are consumed exactly as supplied.
  - Part bytes are consumed exactly as supplied.
  - No separator, null terminator, length-prefix, Unicode normalization, or object encoding is added implicitly.
  - Integer, principal, and protocol-object encoding remain caller responsibilities for later slices.
  - No commitment object, receipt logic, tree logic, proof logic, certification logic, storage change, adapter/orchestration/verifier change, or .did change was introduced.
  - sha2 was added as a direct exact-pinned dependency at =0.10.9, matching the version already present transitively.

Validation evidence:
  - cargo test --offline --lib passed.
  - cargo build --offline --target wasm32-unknown-unknown passed.
  - generated .did remained unchanged.
  - diff was confined to Cargo.toml, Cargo.lock, src/lib.rs, and src/hashing.rs.
