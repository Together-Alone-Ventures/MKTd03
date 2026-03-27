# ADR-03: Tree-Mode CVDR Structure and Verifier Requirements

## Status
Approved

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

The baseline Tree-mode CVDR decision is:

1. **Verification model priority**
   Baseline Tree-mode verification is archival-first. The baseline CVDR must be structured so that an independent verifier can evaluate the receipt using the receipt-contained evidence plus published protocol/build artifacts, without requiring a later live fetch as a mandatory first step.

2. **Receipt self-containment**
   Baseline Tree-mode CVDR is self-contained at issuance for baseline verification purposes. Live corroboration may exist as an optional supplementary route, but baseline verification must not depend on a later fetch step for mandatory evidence components.

3. **Deletion-state semantics**
   Baseline Tree-mode CVDR will express deletion evidence in terms of a deletion-state transition. Where baseline semantics require tombstone-specific meaning, the receipt must preserve that meaning explicitly and must not collapse tombstoned state into the canonical empty-position representation defined by ADR-02.

4. **Minimum mandatory evidence components**
   Every baseline Tree-mode CVDR must contain, in canonical form, evidence sufficient to bind together:
   - the target record identifier or canonical record-position reference used by the protocol,
   - the relevant pre-state and post-state tree commitments,
   - the proof material required to verify the relevant tree-position transition under the canonical structural model defined by ADR-02, without requiring re-implementation of host storage layout,
   - the deletion-state or tombstone-related semantics required by the protocol,
   - the ICP certification material required by ADR-00 baseline scope,
   - the build/module provenance statement required for an independent verifier.

5. **Tree proof and deletion-state relationship**
   Tree proof material and deletion-state semantics are both required parts of baseline Tree-mode CVDR meaning. Tree proof alone is not sufficient if it does not preserve the protocol’s deletion-state semantics, and deletion-state/tombstone semantics alone are not sufficient without the corresponding tree-bound transition evidence.

6. **Certification requirement**
   Baseline Tree-mode CVDR must contain or embed the certification material required for archival-first baseline verification. Live corroboration may supplement this, but certification must not be defined as a later optional fetch for otherwise incomplete baseline receipts.

7. **Build/module provenance requirement**
   Baseline Tree-mode CVDR must carry or unambiguously reference the provenance information required for an independent verifier to connect the verified evidence to a published, reproducibly buildable source version, consistent with ADR-00.

8. **Explicit non-claims**
   Baseline Tree-mode CVDR does not, by default, claim:
   - whole-application deletion completeness,
   - cross-canister completion beyond the single baseline receipt scope,
   - service-canister orchestration truth,
   - retry/list/recovery workflow truth,
   - live-network availability as a prerequisite for baseline verification.

This ADR does not yet finalise exact receipt field names, exact encoding rules, or golden-vector layouts. Those later artifacts must remain consistent with these semantic decisions and must not narrow or expand baseline CVDR meaning by implication.

## Remaining Questions to Resolve Within This ADR
- What exact minimum evidence components should baseline MKTd03 name explicitly in the CVDR structure?
- What exact wording should baseline MKTd03 use for the deletion-state transition so that tombstone-specific meaning is preserved where required?
- What exact certification material should baseline MKTd03 require for self-contained archival-first verification?
- What exact provenance statement should baseline MKTd03 require for independent verifier use?
- What exact explicit non-claims should baseline Tree-mode verifier guidance publish to prevent overreading?
- Should certification material and build/module provenance be inline in the receipt or referenced by stable published pointer, and what are the archival-first implications of each?
- What exact field names and canonical ordering should baseline MKTd03 publish for the minimum mandatory evidence categories named in this ADR?

## Constraints from Earlier Artifacts
- The archival-first versus live-corroborated question flagged in ADR-00 is resolved here by making archival-first the baseline verification priority; later ADR-03 details must remain consistent with that decision.
- Must stay within ADR-00 evidentiary scope.
- Must remain consistent with ADR-02 structural decisions, including empty-versus-tombstoned distinction and fixed-capacity semantics.
- Must remain consistent with ADR-01 boundary decisions and must not assign host/library responsibilities differently by implication.
- Must not allow service-canister/composite-orchestration assumptions back into baseline CVDR semantics.
- Must not let stale MKTd02 implementation details become authority, but may use MKTd02 lessons as bounded input where explicitly justified.

## Rejected Alternatives
- **Live-fetch-first baseline verification**
  Rejected because ADR-00 requires archival-first baseline evidentiary scope and because a later mandatory fetch step would make baseline verification depend on live availability rather than receipt-contained evidence.

- **Receipt incomplete at issuance**
  Rejected because an incomplete receipt at issuance would require a mandatory later fetch, which would contradict archival-first verification and make baseline receipt validity depend on continued live endpoint availability.

- **Tree proof alone as sufficient baseline meaning**
  Rejected because baseline Tree-mode deletion evidence depends on preserving deletion-state semantics in addition to tree-transition proof.

- **Deletion-state or tombstone semantics alone as sufficient baseline meaning**
  Rejected because deletion-state semantics without corresponding tree-bound transition evidence would not satisfy baseline Tree-mode evidentiary scope.

- **Service-canister or composite-orchestration semantics as part of baseline CVDR truth**
  Rejected because ADR-00 excludes those claims from baseline protocol truth.

## Likely Inventory Drivers
- S6, S12, S13, S14, S15, S30, S34, S35, S41, S42, S43, S44, S45, S46, S47, S48, S49, S50, S51, S52, S54, S56, S59, S60, S61, S62, S63, S64, S65, S66
