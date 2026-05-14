## Receipt Retrieval

MKTd03 receipt lookup is subject-reference based:

```rust
state.host_get_receipt(HostReceiptLookupInputs { subject_reference })
```

The current host lookup result uses `library::ReceiptResult` semantics:

- `Ok { receipt }`
- `Err { error_code: InvalidSubjectReference }`
- `Err { error_code: NotYetIssued }`
- `Err { error_code: NotFound }`

There is no separate receipt-id scheme in the current host API.
