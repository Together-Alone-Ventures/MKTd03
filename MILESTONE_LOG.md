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
