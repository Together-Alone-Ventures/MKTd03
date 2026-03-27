# MKTd03 MKTd02 Reuse / Generalisation Audit v1

## Status
Draft

## Date
2026-03-26

## Purpose
Audit MKTd02 as a bounded reuse input for MKTd03, identifying what may be reused, what must be generalised, and what must not be carried forward.

This document is an audit artifact, not protocol authority.
Approved MKTd03 ADRs remain authoritative.

## Authority and Guardrails
This audit is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1

Guardrails:
- MKTd02 implementation history must not become authority by drift.
- Reuse is permitted only as bounded analysis input, not as automatic inheritance of fields, formulas, or flows.
- Findings in this audit must not silently revise ADR-01 or ADR-02.
- If a finding pressures ADR-01 or ADR-02, that requires explicit re-gate and sign-off.
- Leaf-mode-specific assumptions must not be carried into Tree-mode baseline without explicit justification.
- The candidate list produced by this audit does not authorise edits to MKTd02, creation of MKTd02 tasks, or parallel back-port work during this phase.

## Audit Scope
- **MKTd02 repo URL:** https://github.com/Together-Alone-Ventures/MKTd02
- **Exact commit SHA or tag under review:** 54f1e2dc24dd0b79705a66894b2f25138e28a9ad
- **Modules included in scope:** mktd02/src/certified.rs; mktd02/src/engine.rs; mktd02/src/export.rs; mktd02/src/finalization.rs; mktd02/src/guard.rs; mktd02/src/lib.rs; mktd02/src/nonce.rs; mktd02/src/state.rs; mktd02/src/storage.rs; mktd02/src/trait_def.rs; README.md; MKTd02_Integration_Guide.md; docs/architecture/finalization-flow.md; docs/sections/03-adapter.md; docs/sections/05-api-endpoints.md; docs/sections/06-stable-memory.md; docs/sections/07-receipt-export.md; docs/sections/10-verification.md; docs/sections/11-deterministic-encoding.md; docs/sections/module-hash-pipeline.md
- **Modules explicitly excluded from scope:** local Cargo.lock modification; docs/compose.yaml; docs/scripts/compose.py; docs/sections/00-header.md; docs/sections/01-prereqs-product.md; docs/sections/02-add-crate.md; docs/sections/04-lifecycle-hooks.md; docs/sections/08-ref-adapters.md; docs/sections/09-assumptions-product.md; LICENSE; RELEASES.md; shell-script behaviour outside the committed module/doc set under review

This audit is reproducible only for the exact repo state and module scope named above.

## Audit Questions
1. Which MKTd02 concepts remain valid at the level of evidentiary logic?
2. Which MKTd02 mechanisms are leaf-mode-specific and therefore non-reusable?
3. Which MKTd02 concepts are reusable only after generalisation for Tree mode?
4. Which MKTd02 field names, receipt assumptions, or verification patterns would create drift if copied directly?
5. What reusable lessons from MKTd02 should explicitly shape MKTd03 implementation and documentation work?

## Audit Categories
### A. Reusable without major change
Use this only for concepts that survive into MKTd03 with little or no conceptual rewriting.

### B. Reusable only after Tree-mode generalisation
Use this for concepts that remain valuable but must be rewritten for Tree-mode structure, boundary, or receipt semantics.

### C. Do not carry forward
Use this for MKTd02-specific assumptions, mechanisms, or vocabulary that would distort MKTd03 if inherited.

### D. Candidate for future generalisation — not authorising back-port
Use this for concepts or modules that are too MKTd02-specific to reuse directly now, but which may justify future abstraction work. Listing an item here does not authorise back-port or parallel work in MKTd02 during this phase.

## Initial Candidate Areas
- archival-first evidentiary posture
- certification material embedded for baseline verification
- build/module provenance expectations
- explicit non-claims in verifier semantics
- receipt self-containment expectations
- readiness/finalisation discipline lessons
- field naming and receipt-ID derivation caution
- leaf-mode tombstone assumptions
- MKTd02-specific implementation sequencing
- verifier decomposition patterns, if still meaningful after Tree-mode generalisation
- diagnostics/status surface
- versioning and compatibility dispatch

## Module Taxonomy Note
The build plan requires a module taxonomy view before or alongside this audit.
If a separate taxonomy artifact is not created first, this audit must include taxonomy notes as rows are added, so each item can be classified as:
- receipt/artifact identity,
- hashing/tagging,
- sequencing/lifecycle,
- certification/provenance,
- verifier-facing semantics,
- diagnostics/status surface,
- versioning/compatibility dispatch,
- integration-specific or Leaf-mode-specific behaviour.

The taxonomy requirement is not satisfied by concept-level observations alone.

## Audit Table
| Item ID | MKTd02 concept / mechanism / assumption | Taxonomy category | Category | Encodes protocol truth or only MKTd02 integration truth? | Assumes single-record Leaf model? | Reuse decision | Why | Follow-up artifact |
|---|---|---|---|---|---|---|---|---|
| A1 | Archival-first evidentiary posture | verifier-facing semantics | A. Reusable without major change | Protocol truth | No | Reuse | MKTd02’s move toward archival-first verification survives cleanly into MKTd03 and aligns with ADR-00 / ADR-03 baseline scope. | ADR-03 / protocol refresh |
| A2 | Embedded certification material for baseline verification | certification/provenance | B. Reusable only after Tree-mode generalisation | Protocol truth | No | Reuse after generalisation | The MKTd02 lesson that baseline verification should not depend on a mandatory later live fetch is reusable. Generalisation requires restating which evidence components are mandatory for Tree-mode self-containment per ADR-03, without inheriting Leaf-mode field shapes. | ADR-03 |
| A3 | Build/module provenance as baseline verifier input | certification/provenance | A. Reusable without major change | Protocol truth | No | Reuse | The requirement to connect verified evidence to a published reproducible build chain remains valid across MKTd02 and MKTd03. | ADR-00 / ADR-03 |
| A4 | Explicit verifier non-claims to prevent overreading | verifier-facing semantics | A. Reusable without major change | Protocol truth | No | Reuse | MKTd02’s clearer non-claim discipline generalises directly and is now part of the MKTd03 baseline posture. | ADR-00 / ADR-03 / protocol refresh |
| A5 | Receipt self-containment expectation at issuance | receipt/artifact identity | B. Reusable only after Tree-mode generalisation | Protocol truth | No | Reuse after generalisation | The self-contained-at-issuance lesson is reusable, but the Tree-mode generalisation is to restate self-containment in terms of ADR-03’s mandatory evidence categories rather than Leaf-mode receipt fields. | ADR-03 |
| A6 | Receipt field set copied directly from MKTd02 leaf-mode receipts | receipt/artifact identity | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | Direct field carry-forward would import Leaf-mode assumptions and stale receipt semantics into Tree mode. | ADR-03 |
| A7 | Receipt-ID derivation patterns from current MKTd02 implementation | receipt/artifact identity | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | Yes | Reuse after generalisation | The reusable protocol-truth component is derivation discipline, including domain separation and stable identity construction. The non-reusable MKTd02 integration-truth component is the exact set of Leaf-mode receipt fields used as derivation inputs. | ADR-03 |
| A8 | Leaf-mode tombstone assumptions as baseline deletion model | hashing/tagging | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | Tree mode has approved structural distinction rules and deletion-state semantics that cannot be reduced to Leaf-mode assumptions. | ADR-02 / ADR-03 |
| A9 | Finalisation/readiness discipline lessons from MKTd02 | sequencing/lifecycle; diagnostics/status surface | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The discipline around pending/finalised evidence states is useful, but Tree-mode readiness and rebuild mechanics differ and must be restated through MKTd03 invariants and ADR-03. The diagnostics/status lesson should also carry forward into the later diagnostics note. | Tree-mode invariants note / ADR-03 / diagnostics note |
| A10 | Verifier decomposition into distinct check classes | verifier-facing semantics | B. Reusable only after Tree-mode generalisation | Protocol truth | No | Reuse after generalisation | The decomposition pattern remains useful, but the check set and evidence relationships must be rewritten for Tree-mode structure and CVDR semantics. | ADR-03 / protocol refresh |
| A11 | Diagnostics/status surface should be designed in early, not added reactively | diagnostics/status surface | A. Reusable without major change | Protocol truth | No | Reuse | The MKTd02 lesson that evidence/status queries and readiness reporting should be designed proactively survives cleanly into MKTd03 and aligns with the Tree-mode invariants requirement for externally legible readiness state. | diagnostics note / protocol refresh |
| A12 | Versioning and compatibility dispatch discipline | versioning/compatibility dispatch | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that verifier-facing artifacts need explicit versioning and compatibility dispatch discipline. The non-reusable part is the exact MKTd02 V2/V3 dispatch mechanism, which must not be copied directly into Tree-mode semantics without restatement. | versioning note / ADR-03 |

| B1 | Adapter trait discipline from trait_def.rs and docs/sections/03-adapter.md | integration-specific or Leaf-mode-specific behaviour | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | Yes for adapter shape; No for boundary-contract discipline | Reuse after generalisation | The reusable part is the discipline of expressing host/library interaction through an explicit boundary contract. The non-reusable part is the exact Leaf-mode adapter shape, which must not be copied into MKTd03 now that ADR-01 has approved a narrower Tree-mode adapter seam. | ADR-01 / adapter contract concept |
| B2 | Broad data-source style adapter assumptions in MKTd02 integration material | integration-specific or Leaf-mode-specific behaviour | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | Carrying forward a broad data-source adapter model would conflict with ADR-01’s explicit rejection of broad orchestration/data-source seams in favour of a narrow protocol-boundary adapter. | ADR-01 |
| B3 | Stable-memory slot discipline and explicit persistence layout thinking from storage.rs/state.rs/docs/sections/06-stable-memory.md | sequencing/lifecycle | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that persistence layout, explicit slot discipline, and stable-state evolution need to be designed deliberately. The non-reusable part is the exact MKTd02 Leaf-mode storage layout and slot model. | diagnostics note / formal interface phase |
| B4 | Exact MKTd02 stable-memory slot map and Leaf-mode storage schema | integration-specific or Leaf-mode-specific behaviour | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | The exact MKTd02 slot map is implementation-specific to the current Leaf-mode crate and would create false structural authority if copied into MKTd03 Tree mode. | ADR-01 / later implementation planning |
| B5 | Finalization lock / guard discipline from finalization.rs and guard.rs | sequencing/lifecycle | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that evidence-producing transitions need explicit guard discipline and anti-double-finalisation control. In Tree mode, that generalises to preventing evidence from being produced for an in-progress or already-transitioned tree position. The non-reusable part is the exact Leaf-mode finalisation sequence and lock placement. | ADR-03 / diagnostics note |
| B6 | Reactive API-surface growth lesson from export.rs and docs/sections/05-api-endpoints.md | diagnostics/status surface | A. Reusable without major change | Protocol truth | No | Reuse | MKTd02 shows that status/export/query surfaces should be designed as part of the protocol boundary early, not bolted on reactively after verifier and lifecycle needs become visible. | diagnostics note / formal interface phase |
| B7 | Exact MKTd02 endpoint set for Leaf-mode integration | integration-specific or Leaf-mode-specific behaviour | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | The exact endpoint suite in MKTd02 reflects Leaf-mode integration choices and must not be mistaken for baseline MKTd03 protocol surface authority. | ADR-01 / formal interface phase |
| B8 | Receipt export discipline from export.rs and docs/sections/07-receipt-export.md | receipt/artifact identity | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that export paths and artifact emission should be explicit and verifier-facing. The non-reusable part is the exact Leaf-mode export shape and receipt serialisation assumptions. | ADR-03 / formal interface phase |
| B9 | Finalization-flow documentation discipline from docs/architecture/finalization-flow.md | sequencing/lifecycle | A. Reusable without major change | Protocol truth | No | Reuse | MKTd02 demonstrates the value of documenting evidence lifecycle flow explicitly. That discipline transfers directly even though the Tree-mode lifecycle differs in detail. | ADR-03 / protocol refresh |
| B10 | Readiness / blocked / rebuild status should be externally legible, not implicit in internal state only | diagnostics/status surface | A. Reusable without major change | Protocol truth | No | Reuse | This lesson survives directly into MKTd03 and reinforces the Tree-mode invariants note plus ADR-01’s host-owned readiness-state-machine decision. | Tree-mode invariants note / ADR-01 / diagnostics note |
| B11 | Deterministic encoding discipline from docs/sections/11-deterministic-encoding.md | hashing/tagging | B. Reusable only after Tree-mode generalisation | Protocol truth | No | Reuse after generalisation | The reusable lesson is that verifier-facing artifacts need deterministic encoding rules and canonical ordering discipline. The non-reusable part is any exact MKTd02 encoding profile that was chosen around Leaf-mode receipt structure rather than ADR-02 / ADR-03 Tree-mode requirements. | ADR-02 / ADR-03 / formal interface phase |
| B12 | Certified-state binding discipline from mktd02/src/certified.rs | certification/provenance | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that certified-state binding must be explicit, verifier-facing, and tied to the evidence model rather than treated as an implementation afterthought. The non-reusable part is the exact Leaf-mode binding shape and any assumptions tied to MKTd02’s current certified-data arrangement. | ADR-03 / protocol refresh |
| B13 | Exact MKTd02 certified-data arrangement and Leaf-mode commitment shape | certification/provenance | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | Carrying forward the exact MKTd02 certified-data arrangement would import Leaf-mode commitment semantics into Tree mode and risk repeating the stale-spec inconsistency already corrected through ADR-00 and ADR-03. | ADR-03 |
| B14 | Nonce / sequencing discipline from mktd02/src/nonce.rs | sequencing/lifecycle | B. Reusable only after Tree-mode generalisation | Protocol truth mixed with MKTd02 integration truth | No | Reuse after generalisation | The reusable lesson is that evidence artifacts need explicit sequencing or anti-replay discipline. The non-reusable part is the exact MKTd02 nonce/deletion-seq mechanism and any direct coupling to Leaf-mode receipt identity assumptions. | ADR-03 / versioning note |
| B15 | Exact current MKTd02 nonce/deletion-seq implementation as Tree-mode identity mechanism | receipt/artifact identity | C. Do not carry forward | Only MKTd02 integration truth | Yes | Do not reuse | Directly inheriting the current MKTd02 sequencing implementation would blur the distinction between reusable anti-replay discipline and Leaf-mode-specific identity construction. | ADR-03 |
| B16 | Module-hash pipeline discipline from docs/sections/module-hash-pipeline.md | certification/provenance | B. Reusable only after Tree-mode generalisation | Protocol truth | No | Reuse after generalisation | The reusable lesson is that module/build provenance must be attached through an explicit published pipeline rather than assumed from platform magic. The non-reusable part is any MKTd02-specific pipeline wording that presumes Leaf-mode receipt composition or current tooling shape. | ADR-00 / ADR-03 |
| B17 | Exact current MKTd02 module-hash capture mechanism and documentation phrasing | integration-specific or Leaf-mode-specific behaviour | C. Do not carry forward | Only MKTd02 integration truth | No | Do not reuse | Exact mechanism wording from current MKTd02 docs may encode present-tooling assumptions that should not silently become baseline MKTd03 authority, especially where the stale-spec audit already flagged module-hash capture phrasing as a revision point. | ADR-03 / formal interface phase |
