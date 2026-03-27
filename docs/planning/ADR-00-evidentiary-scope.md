# ADR-00: Evidentiary Scope

## Status
Draft

## Date
2026-03-26

## Context
MKTd03 is entering its first actual protocol-build phase after prep closure.

The stale-spec inventory showed that the old spreadsheet spec is not safe to treat as evidentiary authority. It contains stale or over-committed assumptions around receipt contents, certification scope, verifier expectations, module-hash provenance, and service-canister/composite orchestration.

Before tree structure, library boundary, receipt schema, or formal interfaces are settled, MKTd03 needs a conservative baseline statement of what its evidence is intended to support and what it is not intended to prove. Without that boundary, later ADRs risk silently overclaiming the meaning of a Tree-mode CVDR.

## Decision
MKTd03 baseline evidentiary scope will be defined conservatively and dApp-agnostically.

The baseline protocol will aim to support evidence of:
1. a target record identifier or target tree position, as defined by the protocol, within a canonical published tree model,
2. a pre-state to post-state transition affecting the relevant position,
3. tombstone replacement or deletion-state transition, the form of which is to be settled in Phase 2 per the open question flagged below,
4. certified binding on ICP for the committed state relied on by the receipt or proof (receipt form is subject to ADR-03; this point constrains only the certification mechanism, not the receipt schema),
5. build/module provenance sufficient for an independent verifier to confirm that the running canister corresponds to a published, reproducibly buildable source version.

The following are explicit non-claims of the MKTd03 baseline protocol. They are deliberate scope boundaries, not implementation limitations, and later architecture layers may address them separately only through explicit re-gating.
The baseline protocol will not, by default, claim that:
- a whole business workflow is complete across multiple canisters,
- every related PII-bearing canister in an application ecosystem has completed deletion,
- all relevant user data everywhere in an application or canister group has been deleted,
- a service-canister orchestration layer exists,
- live-network corroboration is always available or required,
- composite-orchestration models are not baseline protocol truth; any adoption requires an explicit re-gate with G sign-off before they can appear in normative artifacts.

## Consequences
- ADR-03 must define receipt and verifier requirements that do not exceed this scope without explicit re-gate.
- ADR-02 may settle tree structure and terminology, but not expand evidentiary claims by implication.
- ADR-01 may settle library/host responsibilities, but not import orchestration or service-canister assumptions into baseline scope.
- Composite receipts are deferred from baseline scope by this ADR; any later adoption requires explicit re-gate rather than implicit carry-forward through later drafting.
- Composite-receipt, service-canister, and cross-canister completeness concepts are deferred architecture unless later adopted by explicit decision.
- Baseline MKTd03 wording should remain dApp-agnostic and avoid application-shaped claims about global deletion completeness.

## In Scope
- What MKTd03 baseline evidence is intended to support
- What MKTd03 baseline evidence is not intended to prove
- Guardrails against overclaiming verifier or orchestration scope

## Out of Scope
- Final receipt field schema
- Final tree structure choice
- Final library/adapter boundary
- Formal interface files
- Detailed verifier procedure
- Optional composite/orchestration architecture beyond baseline scoping

## Open Questions
- How should baseline evidentiary scope describe record identity without leaking application-shaped semantics?
- What minimum provenance statement is required for module/build trust in MKTd03 baseline?
- Should baseline language commit specifically to tombstone replacement, or use a slightly broader deletion-state transition abstraction? (Must resolve in Phase 2; affects ADR-02 terminology and ADR-03 receipt semantics.)
- How should archival-first versus live corroboration be framed at ADR level before ADR-03? (Must resolve before ADR-03 drafting begins.)

## Inventory Drivers
- S6, S12–S15, S30, S34–S35, S41–S66
