DATE: 2026-03-26

CURRENT GOAL:
Begin MKTd03 build Session 1 — Phase 1 stale-spec inventory from the clean post-prep baseline.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo’s own RESTART_PACK.md, not this file.

CURRENT STATUS:
- Prep is closed.
- Repo-boundary cleanup is closed and must not be reopened unless a concrete regression is found.
- Standards uplift is complete and pushed.
- MKTd03 main is at commit 654b5122a590db292d9b31a7099546ec18eaee47.
- docs/planning/MKTd03_build_plan.md is present in the live repo.
- TinyPress no longer lives in MKTd03 as an active app work surface.
- MKTd03 remains dApp-agnostic.
- TinyPress remains the first reference target only.
- Codex setup review completed.
- Global and repo AGENTS files were created and verified for MKTd03, TinyPress, and TAV-Engineering-Standards.

AUTHORITY ORDER:
1. Live MKTd03 repo state
2. New build-phase ADR/spec/interface artifacts created in this phase
3. Old spreadsheet MKTd03 tab as audit input only
4. Prep artifacts (restart pack, milestone log, TinyPress prep notes) as historical context only

SESSION 1 OPERATING CONSTRAINTS:
- ADR-01 scoping guardrail: ADR-01 must not become a catch-all. It should decide only the core library boundary, host-owned responsibilities, adapter seam, and explicit out-of-scope items. Service-canister architecture, orchestration flows, retry/list/recovery models, and other non-baseline concerns must be deferred or excluded unless explicitly re-gated.
- Tree-mode terminology is provisional until ADR-02 settles structure/nouns.
- Reuse audit findings cannot change ADR-01 or ADR-02 without explicit G sign-off and phase re-gate.
- Formal interface files must live in MKTd03, not TinyPress.
- Do not let TinyPress leak into protocol truth, examples, payloads, routes, schemas, fixtures, or interface names.
- Do not let MKTd02 implementation history become authority; use it only for bounded reuse analysis.
- Do not begin code before the Phase 6 gate is satisfied.

SESSION 1 FINDINGS:
- The stale-spec inventory surfaced four dominant clusters: evidentiary/verifier scope, tree-structure/terminology, core library boundary, and service-canister/orchestration overreach.
- ADR-01 remains tightly scoped by guardrail and must not absorb orchestration/service-canister architecture.
- The strongest stale-spec drops from baseline are the TAV-operated service-canister assumptions.
- The strongest stale-spec inconsistency is the certified-state/evidence model, which should be settled through ADR-00 and later ADR-03.
- Recommended drafting order from inventory findings: ADR-00, Tree-mode invariants note, ADR-02, then ADR-01.

READY TO START:
This is the first actual MKTd03 protocol build phase.
Prior work was prep, including TinyPress readiness as a later reference target.
The first content task is to audit the old spreadsheet spec and classify its items before any new protocol truth is written.

NEXT LIKELY TASKS:
1. Create the stale-spec inventory from the old MKTd03 spreadsheet tab.
2. Create the authority map for this build phase.
3. Keep protocol work dApp-agnostic and upstream of TinyPress-local implementation.
4. Do not reopen prep, cleanup, or standards uplift unless a specific regression is found.

SAFE RESTART PROMPT:
MKTd03 is in its first actual protocol-build phase.
Current MKTd03 main is 654b5122a590db292d9b31a7099546ec18eaee47.
Use MKTd03 only for dApp-agnostic protocol/spec/ADR/interface work.
Treat the old spreadsheet MKTd03 tab as audit input only.
Treat prep artifacts as historical context only.
Begin with Phase 1 stale-spec inventory before any coding.
