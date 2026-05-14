## Prerequisites

MKTd03 currently assumes:

- Rust canister development with the `wasm32-unknown-unknown` target.
- `ic-stable-structures`-backed host storage.
- A host canister that can allocate three disjoint stable-memory handles.
- A host-controlled `module_hash` pipeline.
- A host-controlled certified-data flow:
  - publish `certified_commitment` after Phase A
  - call `data_certificate()` in query context before Phase B

MKTd03 is local-demo ready after S7-38.
Verifier-side BLS/root-key authentication remains post-handoff work.
