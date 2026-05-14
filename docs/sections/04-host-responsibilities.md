## Host Responsibilities

The embedding host must:

- allocate three disjoint `Memory` handles
- retain `pending_id` across Phase A/B/C
- maintain host-owned `module_hash`
- publish `certified_commitment` with `certified_data_set` after Phase A
- call `data_certificate()` in query context before Phase B
- decide how to surface local-demo caveats to downstream consumers
- handle the current single-pending limitation operationally

Corrupt or overlapping host storage is a host integration failure, not a
protocol feature.
