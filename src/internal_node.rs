use crate::hashing::hash_with_tag;
use crate::tags::TAG_INTERNAL_NODE;

/// Composes a single internal node per spec §6.1. Empty-subtree recursion
/// (§6.2) and root computation (§6.3) are deferred to future slices.
pub fn hash_internal_node(left_child_hash: &[u8; 32], right_child_hash: &[u8; 32]) -> [u8; 32] {
    hash_with_tag(TAG_INTERNAL_NODE, &[left_child_hash, right_child_hash])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_INTERNAL_NODE_HASH_HEX: &str =
        "35c524e519ede4f60d1ec0f98c21ec915eeab0a6473f38af94351969f16d1bb7";

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn hash_internal_node_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(hash_internal_node(&[0x11; 32], &[0x22; 32])),
            EXPECTED_INTERNAL_NODE_HASH_HEX
        );
    }

    #[test]
    fn left_right_order_matters() {
        assert_ne!(
            hash_internal_node(&[0x11; 32], &[0x22; 32]),
            hash_internal_node(&[0x22; 32], &[0x11; 32])
        );
    }

    #[test]
    fn repeated_calls_produce_identical_digest() {
        assert_eq!(
            hash_internal_node(&[0x11; 32], &[0x22; 32]),
            hash_internal_node(&[0x11; 32], &[0x22; 32])
        );
    }

    #[test]
    fn different_left_child_changes_digest() {
        assert_ne!(
            hash_internal_node(&[0x11; 32], &[0x22; 32]),
            hash_internal_node(&[0x33; 32], &[0x22; 32])
        );
    }

    #[test]
    fn different_right_child_changes_digest() {
        assert_ne!(
            hash_internal_node(&[0x11; 32], &[0x22; 32]),
            hash_internal_node(&[0x11; 32], &[0x33; 32])
        );
    }

    #[test]
    fn identical_children_are_accepted() {
        let left_and_right = [0x44; 32];
        assert_eq!(
            hash_internal_node(&left_and_right, &left_and_right),
            hash_internal_node(&left_and_right, &left_and_right)
        );
    }
}
