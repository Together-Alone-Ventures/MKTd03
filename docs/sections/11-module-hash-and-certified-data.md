## Module Hash and Certified Data

The host owns `module_hash` and supplies it to:

- `host_begin_phase_a`
- `host_finalize_phase_c`

The host also owns certified-data side effects:

- after Phase A, publish `certified_commitment`
- before Phase B, call `data_certificate()` in query context and pass the bytes
  as `host_data_certificate`

As with MKTd02, a canister should not try to derive its own deployed module hash
at runtime. The clean model is host/deployer ownership of the hash input.

The trust model is therefore:

- protocol logic proves how module hash and certificate material are embedded
- host/deployer process determines whether the supplied module hash and
  certified-data publication are operationally trustworthy
