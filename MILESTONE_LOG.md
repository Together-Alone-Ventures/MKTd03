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
