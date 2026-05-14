## Stable Memory Layout

Embedded MKTd03 requires three disjoint storage handles:

- issuance tree storage
- pending issuance storage
- issued receipts storage

The type system does not enforce disjointness.
If the host passes overlapping handles to `MKTd03State::new(...)`, protocol
state may be corrupted.

`MKTd03State::new(...)` follows `ic-stable-structures` behavior.
Corrupt memory may panic or fail loud during construction, matching the
standalone canister's fatal-init posture.
