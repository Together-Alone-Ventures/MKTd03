## Host API Flow

The host-side issuance flow is:

```rust
let phase_a = state.host_begin_phase_a(
    &module_hash,
    HostPhaseAInputs {
        subject_reference,
        scope_reference,
        transition_material,
        deletion_state_material,
    },
)?;

let phase_b = state.host_get_phase_b(HostPhaseBInputs {
    pending_id: phase_a.pending_id.clone(),
    host_data_certificate,
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

Semantically:

- Phase A creates pending issuance state and returns `pending_id` plus
  `certified_commitment`.
- Phase B binds host-supplied certificate bytes to the pending issuance.
- Phase C finalizes the receipt and clears pending state on success.
- Receipt lookup remains subject-reference based.
