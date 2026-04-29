#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ScopeEncodingError {
    EmptyScopeReference,
}

pub(crate) fn encode_scope_reference(
    scope_reference: Option<&[u8]>,
) -> Result<Vec<u8>, ScopeEncodingError> {
    match scope_reference {
        None => Ok(vec![0x00]),
        Some(bytes) if bytes.is_empty() => Err(ScopeEncodingError::EmptyScopeReference),
        Some(bytes) => {
            let mut encoded = Vec::with_capacity(bytes.len() + 1);
            encoded.push(0x01);
            encoded.extend_from_slice(bytes);
            Ok(encoded)
        }
    }
}
