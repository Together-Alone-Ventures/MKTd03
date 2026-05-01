use crate::hashing::hash_with_tag;
use crate::scope_encoding::{encode_scope_reference, ScopeEncodingError};
use crate::tags::TAG_RECORD_POSITION_KEY;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum RecordPositionError {
    EmptySubjectReference,
    EmptyScopeReference,
}

impl RecordPositionError {
    fn message(&self) -> String {
        match self {
            Self::EmptySubjectReference => {
                "S7-6 record position subject_reference must not be empty.".to_string()
            }
            Self::EmptyScopeReference => {
                "S7-6 record position scope_reference must not be Some(empty bytes).".to_string()
            }
        }
    }
}

fn trap_on_error<T>(result: Result<T, RecordPositionError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => ic_cdk::trap(&error.message()),
    }
}

pub(crate) fn compute_record_position_key(
    subject_reference: &[u8],
    scope_reference: Option<&[u8]>,
) -> Result<[u8; 32], RecordPositionError> {
    if subject_reference.is_empty() {
        return Err(RecordPositionError::EmptySubjectReference);
    }

    let encoded_scope = encode_scope_reference(scope_reference).map_err(|error| match error {
        ScopeEncodingError::EmptyScopeReference => RecordPositionError::EmptyScopeReference,
    })?;

    Ok(hash_with_tag(
        TAG_RECORD_POSITION_KEY,
        &[subject_reference, &encoded_scope],
    ))
}

pub fn record_position_key(subject_reference: &[u8], scope_reference: Option<&[u8]>) -> [u8; 32] {
    trap_on_error(compute_record_position_key(
        subject_reference,
        scope_reference,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_RECORD_POSITION_KEY_HEX: &str =
        "2f415ad5cf2d6e4eb95c1694e60f4b843b37db933e7e41616ee15307bbe74e23";

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn record_position_key_matches_pinned_golden_vector() {
        let subject_reference = [0x42; 32];
        assert_eq!(
            to_hex(record_position_key(&subject_reference, None)),
            EXPECTED_RECORD_POSITION_KEY_HEX
        );
    }

    #[test]
    fn same_subject_and_scope_are_deterministic() {
        let subject_reference = [0x42; 32];
        assert_eq!(
            record_position_key(&subject_reference, None),
            record_position_key(&subject_reference, None)
        );
    }

    #[test]
    fn different_subjects_produce_different_keys() {
        assert_ne!(
            record_position_key(&[0x42; 32], None),
            record_position_key(&[0x43; 32], None)
        );
    }

    #[test]
    fn different_scopes_produce_different_keys() {
        let subject_reference = [0x42; 32];
        assert_ne!(
            record_position_key(&subject_reference, None),
            record_position_key(&subject_reference, Some(b"x"))
        );
    }

    #[test]
    fn empty_subject_reference_fails_loud() {
        let error =
            compute_record_position_key(b"", None).expect_err("empty subject should fail loud");
        assert_eq!(
            error.message(),
            "S7-6 record position subject_reference must not be empty."
        );
    }

    #[test]
    fn empty_scope_reference_fails_loud() {
        let error = compute_record_position_key(&[0x42; 32], Some(b""))
            .expect_err("empty scope should fail loud");
        assert_eq!(
            error.message(),
            "S7-6 record position scope_reference must not be Some(empty bytes)."
        );
    }
}
