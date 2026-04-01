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
