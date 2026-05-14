# Host Embedding

## Overview

MKTd03 can now be embedded as a Rust crate by a host canister.

In this document, "host" means the canister embedding the `mktd03` crate, not an
inter-canister caller. The standalone MKTd03 canister is now a reference
host/wrapper around the same protocol state and host API.

## Core Design

- Host owns storage allocation.
- Host supplies three disjoint `Memory` handles to `MKTd03State::new(...)`:
  - issuance tree storage
  - pending issuance storage
  - issued receipts storage
- Host owns `module_hash` and supplies it to:
  - `host_begin_phase_a`
  - `host_finalize_phase_c`
- Host owns certified-data side effects:
  - after Phase A, host calls `certified_data_set` / `set_certified_data` with
    `certified_commitment`
  - in Phase B, host calls `data_certificate()` in query context and passes the
    returned bytes as `host_data_certificate`
- `MKTd03State` itself makes no `ic_cdk` calls.

## API Flow

The host-side issuance flow is:

```rust
state.host_begin_phase_a(
    &module_hash,
    HostPhaseAInputs {
        subject_reference,
        scope_reference,
        transition_material,
        deletion_state_material,
    },
)?;

state.host_get_phase_b(HostPhaseBInputs {
    pending_id,
    host_data_certificate,
})?;

state.host_finalize_phase_c(
    &module_hash,
    HostPhaseCInputs {
        pending_id,
        certificate_material,
    },
)?;

state.host_get_receipt(HostReceiptLookupInputs { subject_reference });
```

## Host Responsibilities Checklist

- Allocate disjoint `Memory` handles for tree, pending, and issued-receipt
  storage.
- Persist or otherwise retain `pending_id` between phases.
- Maintain `module_hash` as host-owned state.
- Call `certified_data_set` after Phase A.
- Call `data_certificate()` in query context before Phase B.
- Handle the current single-pending limitation.
- Surface local/demo certificate caveats where applicable.

## Memory Disjointness Warning

The three `Memory` handles supplied to `MKTd03State::new(...)` must be disjoint.
The type system cannot enforce this. Passing overlapping handles may corrupt
MKTd03 protocol state.

## Construction / Panic Note

`MKTd03State::new(...)` follows `ic-stable-structures` behavior. Corrupt stable
memory may panic or fail loud during construction, matching the standalone
canister's fatal-init posture.

## Known Limits

- Single-pending issuance only.
- No cancel/timeout for abandoned pending issuance.
- Stable-cell write-error propagation is not yet production-grade.
- Verifier-side BLS authentication against the IC root key remains post-handoff.
- `StorageUnavailable` is effectively unreachable in well-formed host code,
  except as a stable-storage failure or corruption condition.

## Minimal Worked Example

```rust
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::VectorMemory;
use mktd03::host_api::{
    HostPhaseAInputs, HostPhaseBInputs, HostPhaseCInputs, HostReceiptLookupInputs,
};
use mktd03::transition_material::derive_transition_material;
use mktd03::{library::SemanticVersion, MKTd03State};

let memory_manager = MemoryManager::init(VectorMemory::default());
let mut state = MKTd03State::new(
    memory_manager.get(MemoryId::new(20)),
    memory_manager.get(MemoryId::new(21)),
    memory_manager.get(MemoryId::new(22)),
);

let module_hash = [0x09; 32];
let subject_reference = vec![0x62; 32];
let transition_material = derive_transition_material(
    &SemanticVersion {
        major: 1,
        minor: 0,
        patch: 0,
    },
    b"host-transition-source",
)
.to_vec();

let phase_a = state.host_begin_phase_a(
    &module_hash,
    HostPhaseAInputs {
        subject_reference: subject_reference.clone(),
        scope_reference: None,
        transition_material,
        deletion_state_material: vec![0x01],
    },
)?;

// Host publishes the certified commitment with certified_data_set.
let certified_commitment = phase_a.certified_commitment.clone();

// In a real query path, host calls data_certificate() and forwards the bytes.
let phase_b = state.host_get_phase_b(HostPhaseBInputs {
    pending_id: phase_a.pending_id.clone(),
    host_data_certificate: vec![0xCA, 0xFE],
})?;

let phase_c = state.host_finalize_phase_c(
    &module_hash,
    HostPhaseCInputs {
        pending_id: phase_a.pending_id,
        certificate_material: phase_b.certificate_material,
    },
)?;

let receipt = state.host_get_receipt(HostReceiptLookupInputs {
    subject_reference,
});
```

This example is intentionally generic. It shows host-owned storage, host-owned
`module_hash`, host-owned certified-data responsibilities, and subject-based
receipt lookup without any TinyPress-specific naming.

## Testing Reference

`src/state.rs` contains non-canister host API tests using `VectorMemory`. Those
tests prove the host API path does not require `dfx`, a running replica,
`ic_cdk`, or the standalone canister wrappers.
