DATE: 2026-05-13

CURRENT GOAL:
S7-28 is closed as the transition-derivation-version policy authority packet. Next bounded session should start with continuity-close review, then decide whether to open the follow-on runtime implementation slice for unsupported `transition_derivation_version`, or another upstream-door slice, after G/C review.

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

STRATEGIC STATE AFTER S7-28:
- The first upstream door from S7-26/S7-27 is now open: ADR/spec authority for unsupported `transition_derivation_version` runtime handling has been pinned.
- The verifier implementation surface is no longer fully saturated; one narrow follow-on runtime slice is now authorized in principle, but has not yet been opened in code.
- The next substantive slice may implement the unsupported `transition_derivation_version` precheck, reverse the two current non-inspection tests, and add focused taxonomy/ordering coverage under the newly pinned authority.
- The other upstream doors remain separate:
  1. fixture/materialization strategy for downstream real-path parity;
  2. broader tree-proof semantics re-gating;
  3. TAV-Engineering-Standards Playbook promotion for accumulated process doctrine.

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
4. Follow-on runtime slice: implement the now-pinned unsupported `transition_derivation_version` precheck in `validate_receipt(&Receipt)`, reverse the two current non-inspection tests, and add focused ordering/taxonomy coverage under S7-28 authority.
5. The two current non-inspection tests for `transition_derivation_version` remain committed negative authority until the follow-on runtime implementation slice deliberately reverses them; they must not be removed as incidental cleanup before then.
6. Broader `wrong_tree_proof` semantics remain explicitly out of scope until tree-proof validation is deliberately re-gated.
7. Future coverage audits should distinguish normative-spec authority, committed-authority-record pinning, existing-test-only behaviour, and proposed-test-as-pinning.
8. Tests pinning exact reason strings require stronger authority than tests pinning only `VerificationFailure` variants.
9. The three-part authority-pinning requirement should be considered for TAV-Engineering-Standards Playbook promotion: rejection principle, error taxonomy/reason string, and ordering position must be pinned before runtime semantics are added for typed evidence fields.
10. Packet-slice discipline and zero-implementation outcome normalization should be promoted to TAV-Engineering-Standards Playbook.
11. Possible shared helper consolidation for version-support predicates if duplication becomes material.
12. Promote full-suite gate rule and full-diff pre-commit review rule to TAV-Engineering-Standards Playbook.

NEXT BOUNDED SESSION:
Continuity-close review first. Then either open the follow-on runtime implementation slice for unsupported `transition_derivation_version` under S7-28 authority, or choose another upstream-door slice after G/C review. Do not treat S7-28 itself as runtime implementation. Do not reopen S7-24, S7-25, S7-26, or S7-27 reactively.
