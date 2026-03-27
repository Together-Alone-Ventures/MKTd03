# MKTd03 Authority Map v1

## Current authority order
1. Live MKTd03 repo state
2. New build-phase ADR/spec/interface artifacts created in this phase
3. Old spreadsheet MKTd03 tab as audit input only
4. Prep artifacts (restart pack, milestone log, TinyPress prep notes) as historical context only

## Settled boundary conditions
- MKTd03 is the dApp-agnostic protocol repo.
- TinyPress is a later reference target only.
- Prep, cleanup, and standards uplift are treated as closed unless a concrete regression is found.

## Session 1 operating constraints
- Tree-mode terminology is provisional until ADR-02 settles structure/nouns.
- Reuse audit findings cannot change ADR-01 or ADR-02 without explicit G sign-off and phase re-gate.
- Formal interface files must live in MKTd03, not TinyPress.
- Do not let TinyPress leak into protocol truth, examples, payloads, routes, schemas, fixtures, or interface names.
- Do not let MKTd02 implementation history become authority; use it only for bounded reuse analysis.
- Do not begin code before the Phase 6 gate is satisfied.
