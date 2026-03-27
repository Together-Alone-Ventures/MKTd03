DATE: 2026-03-26

CURRENT GOAL:
Continue Session 4 audit work from the approved MKTd03 Phase 1–3 baseline, starting with population of the pinned MKTd02 reuse/generalisation audit.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo’s own RESTART_PACK.md, not this file.

CURRENT STATUS:
- Prep is closed.
- Repo-boundary cleanup is closed and must not be reopened unless a concrete regression is found.
- Standards uplift is complete and pushed.
- MKTd03 main is at commit 80be0ea.
- docs/planning/MKTd03_build_plan.md is present in the live repo.
- MKTd03 remains dApp-agnostic.
- TinyPress remains the first reference target only and must not shape protocol truth.
- ADR-00 evidentiary scope is approved.
- Tree-mode invariants note is committed.
- ADR-02 tree-structure choice is approved.
- ADR-01 library-vs-adapter boundary is approved.
- ADR-03 Tree-mode CVDR structure is an approved intermediate draft, with completion still pending its remaining questions.
- Adapter contract concept v1 is committed.
- Protocol Refresh v1 is present as a reviewed intermediate draft.
- MKTd02 reuse/generalisation audit scaffold is present and its audit scope is pinned to a specific MKTd02 repo state and module set.
- Session 4 audit work is now underway.

CURRENT NEXT TASK:
- Continue populating the MKTd02 reuse/generalisation audit beyond the initial candidate set, then move into diagnostics/versioning/security/privacy note drafting.

AUTHORITY ORDER:
1. Live MKTd03 repo state
2. Approved build-phase ADR/spec/interface artifacts created in this phase
3. Draft build-phase artifacts created in this phase
4. Old spreadsheet MKTd03 tab as audit input only
5. Prep artifacts (restart pack, milestone log, TinyPress prep notes) as historical context only

OPERATING CONSTRAINTS:
- ADR-01 scoping guardrail: ADR-01 must not become a catch-all. It should decide only the core library boundary, host-owned responsibilities, adapter seam, and explicit out-of-scope items. Service-canister architecture, orchestration flows, retry/list/recovery models, and other non-baseline concerns must be deferred or excluded unless explicitly re-gated.
- Reuse audit findings cannot change ADR-01 or ADR-02 without explicit G sign-off and phase re-gate.
- Formal interface files must live in MKTd03, not TinyPress.
- Do not let TinyPress leak into protocol truth, examples, payloads, routes, schemas, fixtures, or interface names.
- Do not let MKTd02 implementation history become authority; use it only for bounded reuse analysis.
- Do not begin code before the Phase 6 gate is satisfied.

DURABLE FINDINGS:
- The stale-spec inventory surfaced four dominant clusters: evidentiary/verifier scope, tree-structure/terminology, core library boundary, and service-canister/orchestration overreach.
- ADR-01 remains tightly scoped and must not absorb orchestration/service-canister architecture.
- The strongest stale-spec drops from baseline are the TAV-operated service-canister assumptions.
- The strongest stale-spec inconsistency was the certified-state/evidence model, settled through ADR-00 and ADR-03.
- Approved MKTd03 baseline now consists of ADR-00, ADR-01, ADR-02, ADR-03, the Tree-mode invariants note, and the protocol refresh draft.

SAFE RESTART PROMPT:
MKTd03 is in Session 4 audit work with Phase 1–3 baseline ADRs approved.
Current MKTd03 main is 80be0ea.
Use MKTd03 only for dApp-agnostic protocol/spec/ADR/interface/audit work.
Treat the old spreadsheet MKTd03 tab as audit input only.
Treat TinyPress and prep artifacts as historical context only.
Current task is to continue the pinned MKTd02 reuse/generalisation audit, then draft diagnostics/versioning/security/privacy notes.
