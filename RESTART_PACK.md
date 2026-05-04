DATE: 2026-05-04

CURRENT GOAL:
S7-19 implementation is complete and pushed. Continuity was repaired after a heredoc/paste-damaged close packet. Next bounded session should scope the next verifier/protocol slice; do not begin new implementation without a fresh scope packet and review.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.

CURRENT REPO STATE:
- `b2c7be1` (HEAD -> main, origin/main) continuity: repair S7-19 close packet
- `9404117` continuity: record S7-19 close
- `fe5627f` implementation: wire S7-19 receipt pre-state commitment validation
- `c8082fa` continuity: record S7-18 close
- `2e1489b` chore: remove accidental uploaded spec file
- `e5fee37` implementation: wire S7-18 receipt post-state commitment validation
- `47b57a6` Add files via upload
- `0d64a42` continuity: record S7-17 close

NOTE:
`9404117` was pushed but the terminal paste showed a heredoc/paste glitch that truncated/damaged the continuity packet. Forward repair commit `b2c7be1` is the clean S7-19 continuity close.

S7-19 FINAL GATES:
- `cargo fmt --check` passed.
- `cargo test --offline --lib` passed: 149 tests.
- `cargo build --offline --target wasm32-unknown-unknown` passed.
- Implementation commit `fe5627f` pushed to `origin/main`.

WHAT S7-19 DID:
- Wired pre-state commitment relationship validation into `src/verifier.rs` `validate_receipt(receipt: &Receipt)`.
- `validate_receipt` order is now:
  1. S7-16 `CoreTransitionEvidence` structural pre-check.
  2. S7-17 non-trapping `record_position_key` derivation.
  3. S7-17 defensive proof-envelope parse.
  4. S7-17 `validate_proof_directions`.
  5. S7-19 non-trapping `compute_occupied_leaf`.
  6. S7-19 pre-state root reconstruction from parsed `ProofEnvelope`.
  7. S7-19 `pre_state_commitment` wrapper comparison.
  8. S7-18 tombstoned-position extraction.
  9. S7-18 non-trapping `compute_tombstoned_leaf`.
  10. S7-18 post-state root reconstruction from parsed `ProofEnvelope`.
  11. S7-18 `post_state_commitment` wrapper comparison.
  12. Existing downstream `VerificationFailure::NotImplemented(...)` posture.

S7-19 DETAILS:
- `compute_occupied_leaf` already existed with signature:
  `fn compute_occupied_leaf(subject_reference: &[u8], scope_reference: Option<&[u8]>, transition_material: &[u8; 32]) -> Result<[u8; 32], LeafHashError>`
- S7-19 widened `compute_occupied_leaf` to `pub(crate)`, not `pub`.
- The trap-returning public occupied-leaf wrapper remains intact.
- Occupied-leaf inputs are:
  - `subject_reference`
  - `scope_reference`
  - `transition_material`
- Using `transition_material` as an occupied-leaf input is not transition-material relationship validation.
- Pre-state root reconstruction uses the same `reconstruct_root_from_proof` helper/path convention as S7-18.
- `pre_state_commitment` mismatch maps to:
  - `VerificationFailure::InvalidEvidence("pre_state_commitment_mismatch")`
- Occupied-leaf / pre-state reconstruction defensive failure maps to:
  - `VerificationFailure::InvalidEvidence("pre_state_root_reconstruction_invalid")`
- `pre_state_root_reconstruction_invalid` is currently defensive/unreachable through `validate_receipt`, because S7-16 structural validation pre-empts:
  - empty `subject_reference`
  - empty `scope_reference`
  - non-32-byte `transition_material`
- A comment was added in `src/verifier.rs` documenting that defensive mapping.

TESTING / REVIEW NOTES:
- Final lib test count is 149.
- The obsolete S7-18 test `receipt_validation_does_not_validate_pre_state_commitment_yet` was rewritten as `validate_receipt_rejects_pre_state_commitment_mismatch`.
- Misleading intermediate tests for `pre_state_root_reconstruction_invalid` were removed because they observed structural-gate failures, not the S7-19 failure family.
- Existing structural-gate coverage exists for:
  - `empty_subject_reference`
  - `empty_scope_reference`
  - `transition_material_unexpected_length`
- Existing S7-18 verifier-path tests were adjusted only to provide valid pre-state material where needed so their original post-state assertions remain reachable.
- Direction mismatch remains upstream of pre-state and post-state commitment validation.
- One redundant pre-state override remains in the direction-mismatch test. It is harmless because the test exits before pre-state validation. Candidate for next-touch cleanup only.

BOUNDARIES STILL IN FORCE:
- No transition-material relationship validation.
- No `transition_derivation_version` value semantics.
- No pre/post transition semantic validation.
- No protocol-version / receipt-version / interface-version compatibility checks.
- No certification material validation.
- No BLS certificate validation.
- No certified-data commitment validation.
- No receipt issuance or receipt storage.
- No Phase A/B/C issuance/finalization logic.
- No fixture-path validation.
- No fixture-to-library bridge.
- No string-to-bytes bridge.
- No `.did` changes.
- No docs/test-vectors or fixture changes.
- No Cargo changes.
- No public canister API or public Rust API expansion; only `pub(crate)` internal visibility.
- No new hashing primitives, new tags, or new manual preimage semantics.
- No `empty_subtree.rs` changes.
- No `record_position` semantics changes.
- No proof-direction semantics changes.
- No proof-envelope serialization changes.
- No MKTd02, zombie-core, TinyPress, or `canisters/mktd-store/**` consumption.

PROCESS LESSON FROM S7-19:
Gate A / Gate B reporting gates must be real review turns before wiring. In S7-19 the gates and wiring effectively collapsed into one Codex pass, which produced two misleading intermediate tests. The cleanup succeeded, but future slices should enforce: inspect/report first, G/C review second, implementation third.

S7-20A TAXONOMY CLOSE:
- S7-20A landed at `aae057b`.
- Added verifier-input taxonomy authority for unsupported `receipt.protocol_version`:
  - `docs/test-vectors/MKTd03_negative_cases_v1.md` §3.6 `unsupported_protocol_version`
  - §6.5 separating library-facing `unsupported_version` from verifier-input `unsupported_protocol_version`
- No code, fixtures, interfaces, Cargo files, or version constants were changed.
- Reserved for S7-20B:
  - future verifier failure mapping `VerificationFailure::UnsupportedVersion("unsupported_protocol_version")`
  - protocol-version-only verifier precheck
  - receipt_version, interface_version, and transition_derivation_version remain out of scope
- Adjacent drift not fixed:
  - `docs/test-vectors/MKTd03_negative_cases_v1.md` has pre-existing heading-order oddity: `## 5. Non-goals` appears after `## 6. Cross-surface distinction rules for fixtures`.

NEXT BOUNDED SESSION:
Scope S7-20B: protocol-version-only verifier precheck consuming the `unsupported_protocol_version` taxonomy family. Do not include receipt_version, interface_version, or transition_derivation_version without separate re-gating.
