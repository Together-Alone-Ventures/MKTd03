## Host Embedding Model

MKTd03 can now be embedded as a Rust crate by a host canister.

In this guide, "host" means the canister embedding the `mktd03` crate, not an
inter-canister caller. The standalone MKTd03 canister remains a reference
host/wrapper over the same protocol state and host API.

The core host-owned state type is:

```rust
MKTd03State::new(tree_storage, pending_issuance_storage, issued_receipts_storage)
```

The host owns:

- storage allocation
- `module_hash`
- certified-data side effects
- `data_certificate()` retrieval in query context

`MKTd03State` itself makes no `ic_cdk` calls.
