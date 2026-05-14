## Known Limits

- Single-pending issuance only.
- No cancel/timeout for abandoned pending issuance.
- Stable-cell write-error propagation is not yet production-grade.
- `StorageUnavailable` should be treated as a stable-storage failure/corruption
  condition in host-embedding context.
- Verifier-side BLS authentication against the IC root key remains post-handoff.
- The current package-readiness state is local-demo oriented; production/handoff
  hardening still requires additional storage, provenance, and verifier work.
