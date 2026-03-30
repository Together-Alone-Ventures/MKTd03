DATE: 2026-03-26

CURRENT GOAL:
Close the remaining hygiene and conformance gaps so the existing Phase 1-5 baseline can cleanly seed frozen formal-interface drafting.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo’s own RESTART_PACK.md, not this file.

CURRENT STATUS:
- Prep is closed.
- Repo-boundary cleanup is closed and must not be reopened unless a concrete regression is found.
- Standards uplift is complete and pushed.
- MKTd03 main is at commit 505ca1a.
- docs/planning/MKTd03_build_plan.md is present in the live repo.
- MKTd03 remains dApp-agnostic.
- TinyPress remains the first reference target only and must not shape protocol truth.
- Phase 1-5 baseline artifacts exist in repo form, with some path/name drift and draft-status drift still requiring cleanup.
- A conceptual Tree-mode interface artifact exists.
- Companion rules and semantic golden vectors exist as pre-freeze drafting artifacts.
- Frozen formal interface artifacts do not yet exist.

SETTLED CLOSES FOR INTERFACE-PREP WORK:
1. Tree-mode receipt uses a stable two-layer evidence model:
   mandatory core transition evidence + explicit certification/provenance block with route-dependent payload.
2. Versioning uses explicit multi-surface versioning and one authoritative compatibility policy with three classes only:
   compatible, conditionally compatible, unsupported.
3. Diagnostics/status must expose a minimal authoritative status surface including:
   protocol version, status-schema/interface version, build identity, lifecycle state, blocked/not-blocked, structured blocked reason/code, compatibility posture, and operation context when relevant.
   Blocked is first-class and distinct from failed/completed/in-progress.
4. Published terminology is fixed at the policy level:
   subject, scope, pre-state commitment, post-state commitment, transition, tree proof, receipt, certification/provenance, status, compatibility.
   Receipt is the top-level public artifact term; tree proof is a component inside it.
5. Security/privacy constraints on identifier exposure and host-readiness exposure belong in companion rules, not as a .did blocker.

CURRENT NEXT TASK:
- Align continuity, authority map, and conceptual interface artifacts with the settled baseline closes.
- Then use the cleaned conceptual baseline to seed frozen formal-interface drafting.

AUTHORITY ORDER:
1. Live MKTd03 repo state
2. Approved ADR baseline decisions
3. Approved or accepted baseline spec notes and companion rules where they do not conflict with ADRs or settled closes
4. Conceptual interface and vector artifacts as pre-freeze drafting inputs only
5. Old spreadsheet MKTd03 tab as audit input only
6. Prep artifacts (restart pack, milestone log, TinyPress prep notes) as historical context only

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
- The strongest stale-spec inconsistency was the certified-state/evidence model, now closed by the settled two-layer receipt model.
- Current cleanup work is not a reopening of earlier phases; it is a bounded baseline-close pass before frozen interface drafting.

SAFE RESTART PROMPT:
MKTd03 is in the formal-interface pre-freeze cleanup phase.
Current MKTd03 main is 505ca1a.
Use MKTd03 only for dApp-agnostic protocol/spec/ADR/interface/audit work.
Treat the old spreadsheet MKTd03 tab as audit input only.
Treat TinyPress and prep artifacts as historical context only.
Do not reopen the settled interface-prep closes.
Current task is to align continuity, authority mapping, and conceptual interface artifacts so frozen formal-interface drafting can begin from a clean baseline.
