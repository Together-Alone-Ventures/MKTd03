# Tree-Mode Invariants Note

## Status
Draft

## Date
2026-03-26

## Purpose
Record the non-negotiable operational invariants for MKTd03 Tree mode before ADR-02 and ADR-01 settle structure and boundary details.

This note exists to prevent multi-message initialisation or rebuild mechanics from silently weakening deletion evidence, pre-state/post-state meaning, or tree consistency guarantees.

## Context
Tree mode introduces an initialisation and rebuild hazard: deletion evidence becomes unreliable if the tree is allowed to serve as the basis for receipts before it has been fully initialised or rebuilt against the relevant host state.

If a canister begins serving Tree-mode deletion operations while the tree is incomplete, transitional, or structurally stale, later receipts may be tied to ambiguous or chimera roots. The protocol therefore needs explicit invariants for:
- first-time tree initialisation,
- rebuild after manifest-shape or structural incompatibility,
- resumable progress across multiple messages,
- operational freeze conditions during incomplete tree construction.

## Invariants
1. **No deletion evidence from incomplete tree state.**  
   MKTd03 must not produce deletion evidence derived from a tree that is known to be incomplete, mid-initialisation, or mid-rebuild.

2. **Freeze during incomplete initialisation or rebuild.**
   While tree initialisation or rebuild is incomplete, PII-modifying operations that would affect Tree-mode state must be rejected. Read-only queries may continue only if they do not misrepresent tree-backed deletion state as final.

3. **Resumable progress must be explicit.**  
   If tree construction requires multiple messages, progress state must be persisted explicitly so resume logic cannot invent, skip, or silently rewrite covered state.

4. **Rebuild trigger must be explicit.**  
   A manifest-shape mismatch, structural incompatibility, or equivalent rebuild condition must force explicit rebuild mode rather than permitting incremental reuse of an incompatible tree.

5. **Rebuild completion must be state-consistent.**
   A completed rebuild must produce a tree state and resulting root that are verifiably consistent with the current live host state. If rebuild logic cannot verify that its completed output matches the state it is meant to represent, the canister must not transition to ready.

6. **No silent queueing in baseline scope.**
   Baseline MKTd03 should not silently queue deletion requests during incomplete initialisation or rebuild, because queueing weakens the meaning of pre-state and pre-root.

7. **Operational state must remain externally legible.**
   Tree-mode readiness state must be externally visible so callers and operators do not mistake transitional state for steady-state evidence readiness. Baseline design must therefore include an explicit way to distinguish uninitialised, initialising, ready, rebuilding, and blocked states.

## Consequences
- ADR-02 may choose tree structure and terminology, but must preserve these invariants. Any ADR-02 tree structure choice that cannot preserve invariants 1–7 is disqualified.
- ADR-01 may assign host/library responsibility for freeze logic, progress persistence, and status exposure, but must not relax these invariants.
- Baseline interface work will likely require explicit status/reporting surfaces for Tree-mode readiness.
- Any future queueing or deferred-operation design would require explicit re-gate rather than implicit carry-forward.

## In Scope
- Initialisation and rebuild safety invariants
- Freeze/resume expectations
- Rebuild-trigger discipline
- Operational legibility of readiness state

## Out of Scope
- Final tree algorithm choice
- Final library/adapter boundary
- Final interface schema
- Detailed implementation mechanism for scheduling/resume
- Optional queued/deferred deletion architecture

## Inventory Drivers
- S22, S23, S24, S27, S28
