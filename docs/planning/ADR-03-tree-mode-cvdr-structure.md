# ADR-03: Tree-Mode CVDR Structure and Verifier Requirements

## Status
Scaffold — decision pending Session 3

## Date
2026-03-26

## Context
MKTd03 now has:
- ADR-00 approved for baseline evidentiary scope,
- the Tree-mode invariants note committed,
- ADR-02 approved for tree-structure choice,
- ADR-01 approved for library vs adapter boundary,
- the adapter contract concept documented as a conceptual boundary artifact.

The stale-spec inventory showed that the old spreadsheet spec overcommitted or blurred multiple receipt/verifier questions, including:
- receipt field assumptions,
- certification semantics,
- module/build provenance wording,
- tombstone-hash versus tree-proof semantics,
- service-canister/composite orchestration leakage,
- live corroboration versus archival-first verification framing.

ADR-03 exists to define the baseline Tree-mode CVDR structure and verifier requirements without re-opening ADR-00 scope, ADR-02 structure, or ADR-01 boundary decisions.

## Decision
Baseline MKTd03 will define one canonical Tree-mode CVDR structure and one baseline verifier requirement set consistent with ADR-00, ADR-01, and ADR-02.

This ADR must settle:
1. the baseline receipt/CVDR structure,
2. the minimum required evidence components,
3. the relationship between tree proof, tombstone/deletion-state semantics, certification, and build provenance,
4. the baseline verifier expectations,
5. what is explicitly not part of baseline Tree-mode CVDR semantics.

This ADR will not settle:
- service-canister orchestration architecture,
- composite deletion workflows as baseline truth,
- retry/list/recovery service behaviour,
- formal interface file syntax,
- implementation-specific verifier tooling.
- the exact hash preimage layouts and encoding rules for receipt fields, which belong in companion-rule notes or the golden-vectors artifact,

## Candidate Questions to Resolve
- What minimum evidence components must every baseline Tree-mode CVDR contain?
- How should tree proof and deletion-state/tombstone semantics relate within the receipt?
- What certification binding is required for baseline Tree-mode verification?
- What build/module provenance statement is required for an independent verifier?
- Is baseline verification archival-first, live-corroborated, or a defined combination?
- What explicit non-claims must the baseline verifier model state so later readers do not overread it?
- Which old spreadsheet assumptions must be explicitly rejected here rather than silently omitted?
- Has the Phase 2 blocker on tombstone replacement versus deletion-state transition abstraction been resolved, and what does that resolution require of the CVDR structure?
- Is the baseline Tree-mode CVDR self-contained at issuance, or does it require a later fetch step for mandatory verification material?

## Constraints from Earlier Artifacts
- The archival-first versus live-corroborated question flagged in ADR-00 must be resolved as the first decision in this ADR, before receipt structure or verifier requirements are drafted.
- Must stay within ADR-00 evidentiary scope.
- Must remain consistent with ADR-02 structural decisions, including empty-versus-tombstoned distinction and fixed-capacity semantics.
- Must remain consistent with ADR-01 boundary decisions and must not assign host/library responsibilities differently by implication.
- Must not allow service-canister/composite-orchestration assumptions back into baseline CVDR semantics.
- Must not let stale MKTd02 implementation details become authority, but may use MKTd02 lessons as bounded input where explicitly justified.

## Likely Inventory Drivers
- S6, S12, S13, S14, S15, S30, S34, S35, S41, S42, S43, S44, S45, S46, S47, S48, S49, S50, S51, S52, S54, S56, S59, S60, S61, S62, S63, S64, S65, S66
