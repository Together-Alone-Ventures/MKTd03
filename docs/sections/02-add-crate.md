## Add the Crate

Add `mktd03` as a normal Rust dependency of the host canister:

```toml
[dependencies]
mktd03 = { path = "../MKTd03" }
```

The crate is designed to run in-process inside the host canister.
There is no required inter-canister call path, no separate receipt canister,
and no generated MKTd03 client/declaration requirement for embedded use.
