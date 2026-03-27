# ADR-01: Library vs Adapter Boundary

## Status
Draft decision — pending Session 3 completion

## Date
2026-03-26

## Context
MKTd03 now has an approved baseline evidentiary scope (ADR-00), an approved Tree-mode structural baseline (ADR-02), and a committed Tree-mode invariants note.

The stale-spec inventory showed that the old spreadsheet spec mixed true core-boundary questions with non-core architecture spillover, including service-canister assumptions, orchestration flows, additive endpoint shapes, retry/list models, and implementation-mechanism details.

ADR-01 exists to decide only the core library boundary for baseline MKTd03:
- what the library must own,
- what the host canister must own,
- what the adapter seam is,
- what is explicitly out of scope for the core library.

ADR-01 must not become a catch-all ADR for orchestration, composite receipts, service-canister dependencies, verifier semantics, or tree-structure details already settled elsewhere.

## Decision
Baseline MKTd03 will use a library-plus-host integration model with a narrow explicit adapter seam.

The baseline boundary decision is:

1. **Library-owned responsibilities**  
   The core MKTd03 library owns protocol-critical Tree-mode logic, including:
   - canonical Tree-mode state-transition rules required by ADR-00 and ADR-02,
   - protocol-critical checks needed to preserve Tree-mode invariants,
   - canonical tree-state update logic,
   - rebuild-compatibility predicates and structural validity checks,
   - protocol-level interpretation of empty versus tombstoned positions,
   - protocol-level predicates for readiness gating, the enforcement of which is assigned by point 4.

2. **Host-owned responsibilities**  
   The integrating host canister owns:
   - application-specific storage layout,
   - application-specific authority and caller policy,
   - record discovery and host-state access outside the library’s canonical inputs,
   - scheduling/execution mechanics for initialisation or resume across messages,
   - external endpoint exposure and interface wiring,
   - operational status publication required by the integration surface.

3. **Adapter seam**  
   Baseline MKTd03 uses a narrow adapter seam between library and host. The adapter provides the library with the host-state access and mutation execution required for canonical Tree-mode operation, and nothing beyond that. It must not import application-specific storage models, orchestration assumptions, or service-layer concerns into the baseline library boundary.

4. **Readiness state machine ownership**  
   The host owns the operational readiness state machine, including transition triggering, progress persistence wiring, and scheduling mechanics. The library defines the protocol predicates that determine whether Tree-mode state is evidence-ready, rebuild-required, or blocked, but the host wires those predicates into its own operational control and scheduling. The library does not own host scheduling policy or application operational control.

5. **Scheduling ownership**  
   Initialisation/resume scheduling mechanics are host responsibilities or explicitly out of scope for the baseline library. Baseline MKTd03 does not require heartbeat-driven scheduling or any other single operational mechanism as part of the core library boundary.

6. **Adapter contract shape**  
   ADR-01 rejects a broad orchestration-style adapter. The baseline adapter contract must remain narrow — limited to the protocol-boundary data and control interactions required for canonical Tree-mode operation. It must not become a service-layer, retry-layer, listing-layer, or composite-deletion orchestration surface.

7. **Explicit out-of-scope boundary exclusions**  
   The baseline library boundary does not include:
   - service-canister registration or topology management,
   - composite deletion orchestration,
   - retry/list/recovery workflows,
   - app-shaped endpoint suites,
   - verifier procedure or receipt-schema decisions,
   - host-specific storage ownership decisions beyond the adapter contract.

This ADR does not yet finalise the exact adapter method set or formal interface file contents. Those later artifacts must remain consistent with this boundary and must not broaden the library scope by implication.

## Remaining Questions to Resolve Within This ADR
- What exact minimum protocol-level adapter contract should baseline MKTd03 publish?
- What exact host/library split should baseline MKTd03 publish for progress persistence wiring and status exposure?
- What exact terminology should ADR-01 use for “host,” “integration seam,” and “adapter” so later interface artifacts stay consistent? (Must resolve before ADR-03 drafting begins.)

## Constraints from Earlier Artifacts
- Must stay within ADR-00 evidentiary scope.
- Must preserve all Tree-mode invariants.
- Must not reopen ADR-02 structural decisions by implication.
- Must not assign capacity-setting authority to the library or the host; that question is deferred until the fixed-capacity parameter definition is finalised from ADR-02 follow-up.
- Must not let TinyPress, app-shaped examples, or stale MKTd02 implementation history become baseline authority.
- Must not assign orchestration/service-canister responsibilities to the baseline library.
- Must not decide formal interface contents prematurely, but may state what kinds of interfaces the boundary implies.

## Rejected Alternatives
- **Broad orchestration adapter**
  Rejected because it would pull service-layer, retry-layer, and composite-deletion concerns into the baseline library boundary.

- **Library-owned operational scheduling**
  Rejected because scheduling mechanics are host-operational concerns and would force the baseline library to assume app/runtime control it should not own.

- **Library-owned application authority policy**
  Rejected because caller/authority policy is application-specific and must remain with the integrating host.

- **Host-only enforcement of protocol invariants with a thin passive library**
  Rejected because baseline MKTd03 requires protocol-critical checks and canonical Tree-mode logic to remain inside the library boundary rather than being left entirely to host discipline.

- **S18-style broad data-source adapter**
  Rejected in favour of a narrow protocol-boundary seam, because a broader data-source model would risk importing storage-shape assumptions and non-baseline orchestration concerns into the library boundary.

## Likely Inventory Drivers
- S2, S17, S18, S19, S20, S25, S29, S30, S32, S53, S55, S57, S58, S67
