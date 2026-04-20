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
