use crate::hashing::hash_with_tag;
use crate::tags::{TAG_POST_STATE_COMMITMENT, TAG_PRE_STATE_COMMITMENT};

/// Composes the §7.2 pre_state_commitment wrapper over an already-computed
/// pre-state root. Root computation (§6.3) is deferred to a future slice;
/// subject, scope, transition, deletion-state, proof, and provenance material
/// are not part of this commitment preimage.
pub fn pre_state_commitment(pre_state_root: &[u8; 32]) -> [u8; 32] {
    hash_with_tag(TAG_PRE_STATE_COMMITMENT, &[pre_state_root])
}

/// Composes the §8.2 post_state_commitment wrapper over an already-computed
/// post-state root. Root computation (§6.3) is deferred to a future slice;
/// subject, scope, transition, deletion-state, proof, and provenance material
/// are not part of this commitment preimage.
pub fn post_state_commitment(post_state_root: &[u8; 32]) -> [u8; 32] {
    hash_with_tag(TAG_POST_STATE_COMMITMENT, &[post_state_root])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_PRE_STATE_COMMITMENT_HEX: &str =
        "e6e175f66f287635a7f7046af9323a194c768d2a182d33c6ff268d9d2861927c";
    const EXPECTED_POST_STATE_COMMITMENT_HEX: &str =
        "c7921736df695ea54e13f7ea69f9407797531bb422f044dba9f448679203720b";

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn pre_state_commitment_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(pre_state_commitment(&[0x11; 32])),
            EXPECTED_PRE_STATE_COMMITMENT_HEX
        );
    }

    #[test]
    fn pre_state_commitment_is_deterministic() {
        assert_eq!(
            pre_state_commitment(&[0x11; 32]),
            pre_state_commitment(&[0x11; 32])
        );
    }

    #[test]
    fn pre_state_commitment_changes_when_root_changes() {
        assert_ne!(
            pre_state_commitment(&[0x11; 32]),
            pre_state_commitment(&[0x22; 32])
        );
    }

    #[test]
    fn post_state_commitment_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(post_state_commitment(&[0x11; 32])),
            EXPECTED_POST_STATE_COMMITMENT_HEX
        );
    }

    #[test]
    fn post_state_commitment_is_deterministic() {
        assert_eq!(
            post_state_commitment(&[0x11; 32]),
            post_state_commitment(&[0x11; 32])
        );
    }

    #[test]
    fn post_state_commitment_changes_when_root_changes() {
        assert_ne!(
            post_state_commitment(&[0x11; 32]),
            post_state_commitment(&[0x22; 32])
        );
    }

    #[test]
    fn tag_domain_separation_is_preserved() {
        assert_ne!(
            pre_state_commitment(&[0x11; 32]),
            post_state_commitment(&[0x11; 32])
        );
    }

    #[test]
    fn all_zero_roots_are_accepted() {
        assert_eq!(
            pre_state_commitment(&[0x00; 32]),
            pre_state_commitment(&[0x00; 32])
        );
        assert_eq!(
            post_state_commitment(&[0x00; 32]),
            post_state_commitment(&[0x00; 32])
        );
    }
}
