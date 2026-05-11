DATE: 2026-05-11

CURRENT GOAL:
S7-23a is closed. The next step is to restore the stashed S7-23 WIP, re-run the full gate set, and inspect the actual S7-23 unified diff before any S7-23 commit. S7-23 itself is still open and uncommitted.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.

RECENT REPO STATE:
- `91e3817` fixtures: allow missing transition derivation version fixture
- `8f77272` continuity: repair S7-22 restart date
- `1be168b` continuity: record S7-22 close
- `8a02077` verifier: wire certification-provenance shape validation
- `257dae1` continuity: record S7-21B close
- `8590f89471058e6440ac3dba7633f3a93d029aa6` verifier: reject unsupported receipt version
- `d2941cc` continuity: record S7-21A close
- `1b65e4124d74bf182b429a6d219f2686550e829a` docs: settle receipt version policy

S7-23A CLOSE:
- S7-23a repaired the pre-existing verifier fixture integration failure rooted in `missing_transition_derivation_version`.
- `FixtureCoreTransitionEvidence.transition_derivation_version` now parses as `Option<SemanticVersion>`.
- Typed fixture parsing now enforces:
  - `is_none()` for `missing_transition_derivation_version`
  - `is_some()` for every other verifier fixture family
- Positive library receipt fixtures must include `transition_derivation_version`.
- `materialize_receipt(...)` now fails loudly if the field is missing, as a defensive guard against future parser drift or manually constructed invalid typed fixtures.
- The fixture integration target is now green:
  - `cargo test --offline --test fixtures` — 24 passed; 0 failed; 0 ignored

GATE RULE FROM NOW ON:
- Every slice must run:
  - `cargo fmt --check`
  - `cargo test --offline`
  - `cargo build --offline --target wasm32-unknown-unknown`
- The old `cargo test --offline --lib`-only close gate is no longer sufficient.
- Narrower tests may still be run for diagnosis, but slice-close review must use the full offline test suite.

REVIEW RULE FROM NOW ON:
- Summary-only pre-commit review is not acceptable.
- Pre-commit review must inspect the full unified diff and changed-file contents.
- This repeats the S7-12 standing constraint and should be promoted into the next TAV-Engineering-Standards Playbook/doctrine session.

NEXT STEPS:
1. Confirm the stash state before restoring S7-23 WIP.
2. Restore the single S7-23 WIP stash entry.
3. Re-run the full gate set:
   - `cargo fmt --check`
   - `cargo test --offline`
   - `cargo build --offline --target wasm32-unknown-unknown`
4. Inspect the actual S7-23 unified diff and changed-file contents before any S7-23 commit.
5. Do not commit S7-23 until G/C review of the full diff pack.

BOUNDARIES STILL IN FORCE:
- No real-Receipt dual-driving for S7-23.
- No `validate_receipt(&Receipt)` semantic expansion in S7-23.
- No `.did`, Cargo, interface, protocol_version policy, receipt_version policy, interface_version policy, or transition_derivation_version policy changes in S7-23.
