/// Sentinel only. Not used in any hash preimage. Exists to validate tag-discipline tests.
pub const MKTD03_SCAFFOLD_V1: &[u8] = b"MKTD03_SCAFFOLD_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_RECORD_POSITION_KEY: &[u8] = b"MKTD03_RECORD_POSITION_KEY_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_LEAF: &[u8] = b"MKTD03_LEAF_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_INTERNAL_NODE: &[u8] = b"MKTD03_INTERNAL_NODE_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_PRE_STATE_COMMITMENT: &[u8] = b"MKTD03_PRE_STATE_COMMITMENT_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_POST_STATE_COMMITMENT: &[u8] = b"MKTD03_POST_STATE_COMMITMENT_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_TRANSITION_MATERIAL: &[u8] = b"MKTD03_TRANSITION_MATERIAL_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_CERTIFIED_COMMITMENT: &[u8] = b"MKTD03_CERTIFIED_COMMITMENT_V1";
/// Protocol tag defined by the commitment/preimage spec v1.
pub const TAG_RECEIPT_ID: &[u8] = b"MKTD03_RECEIPT_ID_V1";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const ALL_TAGS: [&[u8]; 9] = [
        MKTD03_SCAFFOLD_V1,
        TAG_RECORD_POSITION_KEY,
        TAG_LEAF,
        TAG_INTERNAL_NODE,
        TAG_PRE_STATE_COMMITMENT,
        TAG_POST_STATE_COMMITMENT,
        TAG_TRANSITION_MATERIAL,
        TAG_CERTIFIED_COMMITMENT,
        TAG_RECEIPT_ID,
    ];

    #[test]
    fn all_tag_constants_begin_with_mktd03_prefix() {
        for tag in ALL_TAGS {
            assert!(
                tag.starts_with(b"MKTD03_"),
                "tag missing MKTD03_ prefix: {tag:?}"
            );
        }
    }

    #[test]
    fn no_tag_constant_contains_mktd02_substring() {
        for tag in ALL_TAGS {
            let text = std::str::from_utf8(tag).expect("tag constants must be valid ASCII");
            assert!(
                !text.contains("MKTD02"),
                "tag must not contain MKTD02 substring: {text}"
            );
        }
    }

    #[test]
    fn no_tag_constant_is_imported_from_zombie_core() {
        let source = include_str!("tags.rs");
        assert!(
            !source.lines().map(str::trim).any(|line| {
                line.starts_with("use zombie_core")
                    || line.starts_with("pub use zombie_core")
                    || line.starts_with("use zombie::")
                    || line.starts_with("pub use zombie::")
            }),
            "tags module must not import tag constants from zombie-core"
        );
    }

    #[test]
    fn all_tag_constants_are_valid_ascii() {
        for tag in ALL_TAGS {
            assert!(tag.is_ascii(), "tag must be valid ASCII: {tag:?}");
        }
    }

    #[test]
    fn all_tag_constants_are_unique() {
        let unique = ALL_TAGS.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique.len(), ALL_TAGS.len(), "tag constants must be unique");
    }

    #[test]
    fn no_tag_constant_contains_null_byte() {
        for tag in ALL_TAGS {
            assert!(
                !tag.contains(&0),
                "tag must not contain a null byte: {tag:?}"
            );
        }
    }
}
