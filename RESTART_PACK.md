DATE: 2026-03-26

CURRENT GOAL:
Prepare MKTd03 as the clean starting point for dApp-agnostic protocol-library design on ICP.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo’s own RESTART_PACK.md, not this file.

CURRENT STATUS:
- Repo-boundary cleanup checkpoint completed.
- MKTd03 main is at commit 52b3fc2 — docs: reset root continuity files for dApp-agnostic boundary
- TinyPress main is at commit 195890e — docs: narrow README to app-local boundary
- TinyPress no longer lives in MKTd03 as an active app work surface.
- MKTd03 remains dApp-agnostic.
- TinyPress remains the first reference target only.

WHAT MKTd03 NOW CONTAINS:
- Protocol/project framing
- Deletion-boundary doctrine
- Integration and adapter thinking
- Enterprise/protocol targeting material
- Historical TinyPress artifacts only where retained for provenance

WHAT MKTd03 DOES NOT NOW CONTAIN:
- Live TinyPress application continuity
- Live TinyPress ADR ownership
- Active TinyPress app build surface at repo root

NOT YET STARTED:
The dApp-agnostic MKTd03 protocol library has not yet begun.
Do not describe the library as underway until the repo-cleanup gates are green.

NEXT LIKELY TASKS:
1. Finalise root continuity files and README so they all tell the same boundary story.
2. Complete the operator cleanliness gate for both repos.
3. Separately schedule standards/process extraction into TAV-Engineering-Standards.
4. Only then begin bounded library design work.

SAFE RESTART PROMPT:
MKTd03 is now a dApp-agnostic protocol repo, not the live home of TinyPress app work.
Current MKTd03 main is 52b3fc2 and TinyPress main is 195890e.
Repo-boundary cleanup is complete: TinyPress ADR ownership and TinyPress continuity now live in the TinyPress repo.
Use MKTd03 only for protocol/integration/library-start work.
The dApp-agnostic MKTd03 protocol library has not yet begun.
