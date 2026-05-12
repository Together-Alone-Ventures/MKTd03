DATE: 2026-05-13

CURRENT GOAL:
S7-29 is complete locally pending continuity commit/push. S7-28 authority for unsupported `transition_derivation_version` runtime handling has been consumed by S7-29. Next bounded session should start with continuity-close review, then choose the next bounded slice after G/C review; do not automatically continue verifier implementation.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.

S7-23 CLOSE:
- Closed verifier fixture/test-vector symmetry for:
  - unsupported_protocol_version
  - unsupported_receipt_version
- Added both verifier-negative machine-readable fixtures.
- Added both to fixture index.
- Parser/runtime fixture dispatch now handles both families.
- Version fixture families use `primary_class: "unsupported_version"`.
- Existing evidence-invalidity families remain `primary_class: "invalid_evidence"`.
- Fixture-path checks read directly from `FixtureReceipt`; no real-Receipt dual-driving.
- No real runtime verifier semantics changed.
- Gates passed:
  - `cargo fmt --check`
  - `cargo test --offline` — 186 total tests passed
  - `cargo build --offline --target wasm32-unknown-unknown`


S7-24 CLOSE:
- S7-24 attempted certification-provenance fixture real-path parity for the existing `malformed_certification_provenance` verifier-negative fixture.
- Closure criterion B was met: feasibility blocker proven.
- No source, test, fixture, interface, Cargo, or normative-doc changes were made by Codex.
- The fixture parses and materializes into a real `Receipt`.
- The certification-provenance malformation survives materialization.
- The real `validate_receipt(&Receipt)` path rejects earlier on core-transition evidence shape/length before reaching the certification-provenance shape gate.
- Direct observed error:
  - `VerificationFailure::InvalidEvidence("post_state_commitment_unexpected_length")`
- Exact mechanism:
  - `materialize_receipt(...)` converts placeholder fixture strings into raw byte vectors via `.as_bytes().to_vec()`.
  - Placeholder strings in `pre_state_commitment`, `post_state_commitment`, `transition_material`, and `tree_proof` produce byte vectors with incidental lengths.
  - Earlier evidence gates run before S7-22 certification-provenance shape validation.
- Out-of-scope routes rejected:
  - changing existing fixture JSON,
  - adding a sibling fixture plus index/manifest changes,
  - adding harness substitution/normalization,
  - reordering or short-circuiting real verifier gates.
- S7-24 is information-producing only; no code change was appropriate.


S7-25 CLOSE:
- S7-25 examined whether committed authority permits adding a real runtime `validate_receipt(&Receipt)` precheck for `Receipt.core_transition_evidence.transition_derivation_version`.
- Closure criterion B was met: authority blocker proven.
- No source, test, fixture, interface, Cargo, normative-doc, or continuity changes were made by Codex.
- Implementation was authorized only if three decisions were explicitly pinned:
  - runtime rejection principle,
  - exact `VerificationFailure` taxonomy and reason string,
  - verifier gate ordering position.
- All three decisions are unpinned in committed authority.
- What is pinned:
  - `transition_derivation_version : SemanticVersion` exists and is required in the frozen interface.
  - Companion rules say it must not be inferred from other versions.
  - The `missing_transition_derivation_version` verifier fixture family exists.
  - S7-23a recorded that no runtime verifier semantics or new `transition_derivation_version` policy were introduced.
- Current runtime tests explicitly preserve non-inspection:
  - `receipt_validation_does_not_inspect_transition_derivation_version`
  - `receipt_validation_does_not_inspect_transition_derivation_version_after_post_state_check`
- These tests are committed negative authority. A future implementation path must explicitly reverse them after policy is pinned.
- Close-time ADR/policy sweep covered `docs/planning` and `docs/adr` for transition-derivation-version terms to guard against overlooked ADR-level authority.
- S7-25 is information-producing only; no implementation was appropriate.


S7-26 CLOSE:
- S7-26 was the first larger bounded packet-slice for verifier receipt-validation work.
- Phase 1 audited and classified remaining verifier/receipt-validation gaps.
- Phase 2 did not open because there was no implementation-ready group under current committed authority and slice boundaries.
- No source, test, fixture, interface, Cargo, normative-doc, or continuity changes were made by Codex.
- Zero-implementation outcome is a successful packet close, not a failed slice.
- Classification:
  - authority-blocked: `transition_derivation_version` runtime handling and `missing_transition_derivation_version` concrete runtime semantics;
  - fixture/materialization-blocked: downstream verifier-negative real-path parity under current placeholder evidence materialization;
  - out of scope: broader tree-proof semantics, certification/provenance cryptography, transition-material semantics, success path, and precheck refactors;
  - later carry-forward: helper consolidation and Playbook/doctrine promotion.
- Close-time sweeps confirmed:
  - `Deferred(...)` surfaces in `src/verifier.rs` remain carry-forward / out-of-scope, not implementation-ready;
  - current `transition_derivation_version` non-inspection tests remain committed negative authority;
  - downstream fixture parity remains blocked by placeholder-string materialization unless a future fixture/materialization strategy is approved.
- S7-26 did not perform a comprehensive test-coverage audit for already-pinned gates; that remains a candidate future packet.

S7-26 FORWARD PATHWAYS:
- More verifier implementation now requires one of three doors to be opened first:
  1. ADR/spec authority for `transition_derivation_version` runtime treatment;
  2. an approved fixture/materialization strategy for real-path parity of downstream verifier negatives;
  3. explicit re-gating of broader tree-proof semantics beyond the current non-cryptographic slice.
- Do not start another verifier implementation packet until one of these doors is deliberately opened.

PACKET-SLICE DISCIPLINE NOW IN FORCE:
- Bigger slices are allowed only when candidates share the same committed authority citation, error taxonomy, ordering zone, local code area, and no-interface/no-doc boundary.
- Packet slices use an internal two-phase structure:
  - audit/classify with citations;
  - G/C checkpoint;
  - implementation only for the approved subset.
- Zero-implementation packet closes are valid when no implementation-ready group exists.
- Commits remain per substantive change plus a separate continuity-close commit.


S7-27 CLOSE:
- S7-27 audited coverage gaps for already-pinned verifier gates.
- Phase 1 found no implementation-ready test batch.
- Phase 2 did not open.
- No source, test, fixture, interface, Cargo, normative-doc, or continuity changes were made by Codex.
- Zero-implementation outcome is a successful packet close, not a failed slice.
- Already sufficiently covered:
  - real runtime `protocol_version` precheck rejection and ordering;
  - real runtime `receipt_version` precheck rejection and ordering;
  - real runtime certification-provenance shape gate failure and ordering;
  - fixture integration loop coverage for currently materialized verifier families.
- Existing-test-only / proposed-test-as-pinning candidates were not escalated:
  - direct fixture-dispatch unit tests for `malformed_certification_provenance`;
  - direct fixture-dispatch unit tests for `wrong_commitment_relationship`;
  - direct fixture-dispatch unit tests for `receipt_subject_scope_mismatch`;
  - direct `Deferred(...)` dispatch tests for `missing_transition_derivation_version`;
  - direct `Deferred(...)` dispatch tests for `wrong_tree_proof`.
- Fixture/materialization real-path parity remains blocked by S7-24.
- S7-27 did not perform test maintenance, dispatch fuzzing, or unrecognized-input negative coverage.

S7-27 AUDIT LESSONS:
- Variant-only fixture-dispatch tests are less contract-hardening than exact-reason-string tests, but both remain non-auto-eligible without independent authority or explicit G/C escalation.
- Future audits should distinguish normative-spec pinning from committed-authority-record pinning. MILESTONE_LOG and RESTART_PACK record decisions, but they are not equivalent to ADR/spec/interface-rule authority.
- Existing tests may describe current behaviour without independently authorizing more tests that harden it.
- Tests pinning exact reason strings require stronger authority than tests pinning only the `VerificationFailure` variant.

S7-28 CLOSE:
- S7-28 landed the authority/spec policy required for later runtime handling of unsupported `Receipt.core_transition_evidence.transition_derivation_version`.
- The supported `transition_derivation_version` set is now pinned to exactly `1.0.0`.
- A real `validate_receipt(&Receipt)` implementation must reject any other value with:
  - `VerificationFailure::UnsupportedVersion("unsupported_transition_derivation_version")`
- The ordering is now pinned as:
  - `protocol_version`
  - `receipt_version`
  - `transition_derivation_version`
  - core-transition evidence structural gates
  - commitment gates
  - certification-provenance shape gate
  - final `NotImplemented(...)` scaffold
- This is a version-support failure, not malformed evidence, even though the field is nested under `core_transition_evidence`.
- `interfaces/mktd03_library_interface_rules.md` §1.4 remains the cross-reference that makes the field independently meaningful; S7-28 pins the runtime consequence of that independence.
- S7-28 does not resolve concrete runtime semantics for `missing_transition_derivation_version`.
- The current non-inspection tests remain unchanged for now, but are now scheduled for reversal in the later implementation slice that uses this authority.
- No source, test, fixture, interface, `.did`, or Cargo changes were made.
- Gates passed:
  - `cargo fmt --check`
  - `cargo test --offline`
  - `cargo build --offline --target wasm32-unknown-unknown`

STRATEGIC STATE AFTER S7-27:
- S7-26 and S7-27 are consecutive zero-implementation packet closes.
- This means the verifier implementation surface is saturated under current authority.
- Do not start another verifier implementation/coverage packet by default.
- Next substantive work should deliberately open one of:
  1. `transition_derivation_version` ADR/spec authority;
  2. fixture/materialization strategy for downstream real-path parity;
  3. broader tree-proof semantics re-gating;
  4. TAV-Engineering-Standards Playbook promotion for accumulated process doctrine.

STRATEGIC STATE AFTER S7-29:
- The S7-28 unsupported-TDV authority door has now been consumed by S7-29.
- Runtime `validate_receipt(&Receipt)` rejects unsupported `transition_derivation_version` values as `UnsupportedVersion("unsupported_transition_derivation_version")`.
- The two former TDV non-inspection tests were deliberately removed/replaced in S7-29 and no longer stand as committed negative authority.
- Do not automatically continue verifier implementation. The remaining upstream doors are separate:
  1. missing-TDV runtime semantics, requiring separate authority;
  2. fixture/materialization strategy for downstream real-path parity;
  3. broader tree-proof semantics re-gating;
  4. TAV-Engineering-Standards Playbook promotion for accumulated process doctrine.

BOUNDARIES STILL IN FORCE:
- No `.did` changes.
- No Cargo changes.
- No interface changes.
- No fixture manifest changes.
- No normative docs changes.
- No policy reopening for protocol_version, receipt_version, interface_version, or transition_derivation_version.

OPEN CANDIDATES / CARRY-FORWARD:
1. S7-22 certification-provenance runtime gate still lacks real-path fixture coverage. S7-24 proved the current fixture cannot reach that gate because placeholder-derived core-transition evidence fails earlier.
2. Future real-path parity for downstream-gate verifier negatives needs a separately scoped design, likely involving either structurally valid negative fixtures or an explicitly approved fixture/materialization strategy.
3. Placeholder-string semantics in `materialize_receipt(...)` are a structural impediment to downstream-gate real-path parity whenever earlier evidence-length gates run first.
4. Missing-TDV runtime semantics remain unresolved and require separate authority; do not conflate missing-TDV with unsupported-TDV.
5. The former `transition_derivation_version` non-inspection tests were deliberately removed/replaced in S7-29; do not restore them.
6. Broader `wrong_tree_proof` semantics remain explicitly out of scope until tree-proof validation is deliberately re-gated.
7. Future coverage audits should distinguish normative-spec authority, committed-authority-record pinning, existing-test-only behaviour, and proposed-test-as-pinning.
8. Tests pinning exact reason strings require stronger authority than tests pinning only `VerificationFailure` variants.
9. The three-part authority-pinning requirement should be considered for TAV-Engineering-Standards Playbook promotion: rejection principle, error taxonomy/reason string, and ordering position must be pinned before runtime semantics are added for typed evidence fields.
10. Packet-slice discipline and zero-implementation outcome normalization should be promoted to TAV-Engineering-Standards Playbook.
11. Possible shared helper consolidation for version-support predicates if duplication becomes material.
12. Promote full-suite gate rule and full-diff pre-commit review rule to TAV-Engineering-Standards Playbook.

NEXT BOUNDED SESSION:
Continuity-close review first. Then choose the next bounded slice after G/C review. Do not automatically continue verifier implementation. Do not reopen S7-24, S7-25, S7-26, S7-27, S7-28, or S7-29 reactively.

---

## S7-29 CLOSE — transition-derivation runtime rejection

Status:
  - S7-29 complete locally at substantive commit `e8b3374`.
  - Continuity close in progress.
  - Branch should be one substantive commit ahead of `origin/main` until the continuity commit and push complete.

Substantive commit:
  - `e8b3374` — `verifier: reject unsupported transition derivation version`

What changed:
  - `src/verifier.rs` now has a runtime TDV support precheck in `validate_receipt(&Receipt)`.
  - Supported TDV is exactly `SemanticVersion { major: 1, minor: 0, patch: 0 }`.
  - Unsupported TDV rejects as `VerificationFailure::UnsupportedVersion("unsupported_transition_derivation_version")`.
  - The TDV gate is ordered after receipt_version and before core-transition evidence structural validation.
  - The prior non-inspection tests were deliberately removed/replaced.
  - New tests cover rejection, ordering above and below the TDV gate, and supported-`1.0.0` passage to the existing final `NotImplemented(...)` scaffold.

Validation before substantive commit:
  - `cargo fmt --check` passed.
  - `cargo test --offline` passed: 164 unit tests and 24 fixture tests.
  - `cargo build --offline --target wasm32-unknown-unknown` passed.

Boundaries preserved:
  - No docs/spec/ADR/interface/.did changes.
  - No fixture, fixture-manifest, or index changes.
  - No changes to `validate_fixture_receipt_semantics` dispatch arms.
  - `missing_transition_derivation_version` remains deferred/authority-blocked.
  - No transition_material semantic validation.
  - No broader tree-proof semantics.
  - No certification crypto.
  - No receipt-success path.
  - Final `NotImplemented(...)` scaffold preserved.

Still open after S7-29:
  - Missing-TDV runtime semantics require separate authority.
  - S7-24 downstream verifier-negative real-path parity remains blocked by fixture/materialization strategy.
  - Broader tree-proof semantics remain separately gated.
  - Certification/provenance crypto remains out of scope.
  - Success-path behavior remains unauthorised.

Next candidate slice:
  - Do not automatically continue verifier implementation.
  - Next step should be a bounded selection review after continuity push, using current `MILESTONE_LOG.md`, `RESTART_PACK.md`, and git history.

---

## S7-30 CLOSE — missing-TDV classification

Status:
  - S7-30 closes as a zero-implementation classification packet.
  - No code, test, fixture, interface, `.did`, spec, ADR, or Cargo changes were made.
  - Continuity close in progress.

Classification:
  - Missing `transition_derivation_version` is a required-field absence, not an unsupported-value case.
  - It is not representable at the typed runtime `Receipt` boundary because:
    - `interfaces/mktd03_library.did` declares `transition_derivation_version : SemanticVersion` as required/non-`opt`;
    - `src/library.rs` declares runtime `CoreTransitionEvidence.transition_derivation_version: SemanticVersion` as non-`Option`.
  - Therefore `validate_receipt(&Receipt)` cannot and should not grow a missing-TDV check.
  - Candid/API intake structurally rejects missing required fields before typed receipt validation.
  - Fixture JSON represents missing TDV by omitted key only because the fixture-layer type in `src/fixtures.rs` uses `Option<SemanticVersion>` for negative-fixture classification.
  - The missing-TDV fixture path is accommodated structurally, then punted semantically via `VerificationFailure::Deferred(...)`.
  - The fixture dispatch arm is not semantic validation.

S7-29 protection:
  - `docs/spec/MKTd03_versioning_compatibility_note_v1.md` §9.1 remains unchanged and remains the sole authority for unsupported-TDV runtime handling.
  - S7-30 does not extend the verifier ordering chain.
  - No missing-TDV runtime gate is authorized.
  - S7-29 tests/helper/precheck position remain untouched.

S7-24 relationship:
  - S7-30 does not resolve the S7-24 fixture/materialization blocker.
  - It sharpens the dependency: any future concrete missing-TDV implementation at custom intake or fixture-to-runtime materialization depends on a separately scoped fixture/materialization or intake-authority strategy.
  - Do not treat missing-TDV as an independent implementation candidate until that dependency is resolved or explicitly re-gated.

Still open after S7-30:
  - S7-24 downstream verifier-negative real-path parity / fixture-materialization strategy.
  - Missing-TDV concrete implementation only after S7-24-related strategy or separate intake authority.
  - Broader tree-proof semantics.
  - Certification/provenance crypto.
  - Success-path behavior.
  - TAV-Engineering-Standards Playbook promotion for accumulated process doctrine.

Next bounded session:
  - Start with continuity-close review.
  - Then choose the next bounded slice after G/C review.
  - Do not automatically continue verifier implementation.

---

## S7-31 CLOSE — fixture/materialization strategy audit

Status:
  - S7-31 closes as a zero-implementation fixture/materialization strategy audit.
  - No code, tests, fixtures, fixture schema, fixture index/manifest, interfaces, `.did`, specs, ADRs, or Cargo files changed.
  - Continuity close recorded in `MILESTONE_LOG.md` and this restart pack.

Core finding:
  - Current verifier-negative JSON fixtures are taxonomy/intake fixtures, not runtime-ready verifier fixtures.
  - The current fixture corpus uses placeholder-string material in evidence-bearing fields.
  - Placeholder material is not systematically valid for the real typed `validate_receipt(&Receipt)` path.
  - The systemic blocker is not limited to S7-24's `malformed_certification_provenance` case.
  - The positive library receipt fixture is a retrieval-surface fixture, not evidence of verifier-real-path readiness.

Fixture-surface distinction:
  - Fixture-level semantic dispatch and typed runtime verifier validation are distinct parallel surfaces.
  - `validate_fixture_receipt_semantics(...)` dispatch does not imply real-path parity through `validate_receipt(&Receipt)`.
  - Fixture-level semantic dispatch currently exists for:
    - `malformed_certification_provenance`
    - `wrong_commitment_relationship`
    - `receipt_subject_scope_mismatch`
  - Fixture-level deferral currently remains for:
    - `wrong_tree_proof`
    - `missing_transition_derivation_version`

Strategy posture:
  - Strategy A remains the current posture: taxonomy/intake fixtures only.
  - Strategy I is the recommended candidate for a future test-helper authority packet: test-only materialization/helpers for runtime gate coverage.
  - Strategy I is not authorized by S7-31.
  - Strategy B, C, D, and G remain deferred future candidates:
    - B: separate runtime-ready verifier-negative fixture profile
    - C: deterministic byte-material fixture decoding
    - D: raw-artifact intake validation layer
    - G: layered fixtures with separate views
  - Any selection among B/C/D/G requires a separately scoped authority packet.

S7-24 relationship:
  - S7-24 blocker remains active.
  - S7-31 maps the fixture/materialization strategy landscape but does not resolve S7-24.
  - Any future verifier-negative real-path parity, missing-TDV implementation, or wrong-tree-proof promotion remains downstream of future authority work.

Candidate follow-ups:
  - Confirm whether a verifier-positive fixture file is intentionally absent:
    - `docs/test-vectors/fixtures/verifier/positive/mktd03_verifier_positive_receipt_inline_certification_01_v1.json`
  - Future test-helper authority packet for Strategy I, if runtime-gate coverage is prioritized.
  - Future fixture-schema/materialization authority packet for Strategy B or G, if fixture-level real-path parity becomes necessary.
  - Future ADR/spec/interface authority packet for Strategy D, if raw-artifact intake validation becomes necessary.

Next bounded session:
  - Playbook promotion should be next.
  - Candidate topics include:
    - full-suite gate rule
    - full unified diff review rule
    - packet-slice discipline
    - authority/spec slice rhythm
    - normative-spec vs committed-authority-record distinction
    - cross-path classification synthesis
    - read-only audit discipline
    - strategy-vs-authority distinction
    - per-family inspection / heuristic-blocker audit pattern
