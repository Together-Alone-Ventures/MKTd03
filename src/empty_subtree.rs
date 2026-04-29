use crate::internal_node::hash_internal_node;
use crate::leaf_hash::hash_empty_leaf;

const MAXIMUM_EMPTY_SUBTREE_HEIGHT: usize = 256;

#[derive(Debug, Eq, PartialEq)]
enum EmptySubtreeError {
    HeightAboveMaximum { height: usize, maximum: usize },
}

impl EmptySubtreeError {
    fn message(&self) -> String {
        match self {
            Self::HeightAboveMaximum { height, maximum } => {
                format!("S7-8 empty-subtree height {height} exceeds maximum {maximum}.")
            }
        }
    }
}

fn trap_on_error<T>(result: Result<T, EmptySubtreeError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => ic_cdk::trap(&error.message()),
    }
}

fn compute_empty_subtree_root(height: usize) -> Result<[u8; 32], EmptySubtreeError> {
    if height > MAXIMUM_EMPTY_SUBTREE_HEIGHT {
        return Err(EmptySubtreeError::HeightAboveMaximum {
            height,
            maximum: MAXIMUM_EMPTY_SUBTREE_HEIGHT,
        });
    }

    let mut current = hash_empty_leaf();
    for _ in 0..height {
        current = hash_internal_node(&current, &current);
    }
    Ok(current)
}

/// Computes the canonical empty-subtree root at the given height per spec §6.2,
/// where height = 0 is the empty-leaf hash and each successive height composes
/// two copies of the previous height via §6.1. Heights are accepted in
/// 0..=256 inclusive; the upper bound is project policy derived from §4.7's
/// baseline SMT depth. Root computation over occupied/tombstoned leaves (§6.3)
/// is deferred to future slices.
pub fn empty_subtree_root(height: usize) -> [u8; 32] {
    trap_on_error(compute_empty_subtree_root(height))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_HEIGHT_ZERO_HEX: &str =
        "c1e1013414a35a60a28d73920ee54dd84a1ae14356498411fb3bfa38d341a747";
    const EXPECTED_HEIGHT_ONE_HEX: &str =
        "0127a5adab605fa037cabe9ca63ab644ec0ed57d28b8a2cb68b13506867d918f";
    const EXPECTED_HEIGHT_256_HEX: &str =
        "3f26b46bc81298e15ba1b1b495f44c6410fd627e705d8243ac20259e77427aaa";

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn height_zero_matches_hash_empty_leaf() {
        assert_eq!(empty_subtree_root(0), hash_empty_leaf());
    }

    #[test]
    fn height_one_matches_internal_node_of_empty_leaf() {
        let h0 = hash_empty_leaf();
        assert_eq!(empty_subtree_root(1), hash_internal_node(&h0, &h0));
    }

    #[test]
    fn height_two_matches_recurrence() {
        let h1 = empty_subtree_root(1);
        assert_eq!(empty_subtree_root(2), hash_internal_node(&h1, &h1));
    }

    #[test]
    fn height_zero_matches_pinned_golden_vector() {
        assert_eq!(to_hex(empty_subtree_root(0)), EXPECTED_HEIGHT_ZERO_HEX);
    }

    #[test]
    fn height_one_matches_pinned_golden_vector() {
        assert_eq!(to_hex(empty_subtree_root(1)), EXPECTED_HEIGHT_ONE_HEX);
    }

    #[test]
    fn height_256_matches_pinned_golden_vector() {
        assert_eq!(to_hex(empty_subtree_root(256)), EXPECTED_HEIGHT_256_HEX);
    }

    #[test]
    fn deterministic_repeat_for_non_zero_height() {
        assert_eq!(empty_subtree_root(7), empty_subtree_root(7));
    }

    #[test]
    fn increasing_heights_produce_distinct_roots() {
        let h0 = empty_subtree_root(0);
        let h1 = empty_subtree_root(1);
        let h2 = empty_subtree_root(2);
        assert_ne!(h0, h1);
        assert_ne!(h1, h2);
        assert_ne!(h0, h2);
    }

    #[test]
    fn maximum_height_succeeds() {
        assert_eq!(to_hex(empty_subtree_root(256)), EXPECTED_HEIGHT_256_HEX);
    }

    #[test]
    fn height_above_maximum_fails_loud() {
        let error = compute_empty_subtree_root(257).expect_err("height 257 must fail loud");
        assert_eq!(
            error,
            EmptySubtreeError::HeightAboveMaximum {
                height: 257,
                maximum: 256,
            }
        );
        assert_eq!(
            error.message(),
            "S7-8 empty-subtree height 257 exceeds maximum 256."
        );
    }
}
