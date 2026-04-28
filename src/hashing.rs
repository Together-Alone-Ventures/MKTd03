use sha2::{Digest, Sha256};

/// Computes SHA-256 over the exact byte concatenation `tag || parts[0] || parts[1] || ...`.
///
/// Tag bytes are consumed exactly as provided.
/// Part bytes are consumed exactly as provided.
/// No implicit separator, null terminator, or length-prefix is added.
/// Integer, principal, and object encoding is not handled here.
pub fn hash_with_tag(tag: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(tag);
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::MKTD03_SCAFFOLD_V1;

    fn sha256_of(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }

    #[test]
    fn known_vector_matches_exact_concatenation() {
        let actual = hash_with_tag(MKTD03_SCAFFOLD_V1, &[b"abc", b"xyz"]);
        let expected = sha256_of(b"MKTD03_SCAFFOLD_V1abcxyz");
        assert_eq!(actual, expected);
    }

    #[test]
    fn tag_change_changes_digest() {
        let left = hash_with_tag(MKTD03_SCAFFOLD_V1, &[b"abc", b"xyz"]);
        let right = hash_with_tag(b"MKTD03_SCAFFOLD_V2", &[b"abc", b"xyz"]);
        assert_ne!(left, right);
    }

    #[test]
    fn part_order_change_changes_digest() {
        let left = hash_with_tag(MKTD03_SCAFFOLD_V1, &[b"abc", b"xyz"]);
        let right = hash_with_tag(MKTD03_SCAFFOLD_V1, &[b"xyz", b"abc"]);
        assert_ne!(left, right);
    }

    #[test]
    fn empty_parts_hashes_exact_tag_only_bytes() {
        let actual = hash_with_tag(MKTD03_SCAFFOLD_V1, &[]);
        let expected = sha256_of(MKTD03_SCAFFOLD_V1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_hidden_null_terminator_between_tag_and_first_part() {
        let tag = b"MKTD03_SCAFFOLD_V1";
        let part = b"payload";
        let actual = hash_with_tag(tag, &[part]);
        let expected = sha256_of(b"MKTD03_SCAFFOLD_V1payload");
        assert_eq!(actual, expected);
    }
}
