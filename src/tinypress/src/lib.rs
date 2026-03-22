// TinyPress v1 — Stage 1 implementation: profiles map
//
// Together Alone Ventures · MKTd03 Project · March 2026
// Spec: TinyPress ADR + Interface Spec v1.1
//
// HARD CONSTRAINTS (from ADR):
//   - Zero knowledge of MKTd03 tombstoning mechanics (ADR §1)
//   - Author identity derived from caller principal; never caller-supplied (ADR-03)
//   - No deletion-aware terminology anywhere in this file (ADR-06)
//   - handle is immutable after creation (spec §4.2)
//   - delete_profile does NOT cascade to posts/comments — orphaning is intentional (ADR-05)
//
// Memory layout (MemoryManager IDs — do not change without migration):
//   MemoryId(0) — profiles:         StableBTreeMap<u64, Profile>
//   MemoryId(1) — principal_index:  StableBTreeMap<StorablePrincipal, u64>
//   MemoryId(2) — profile_counter:  StableCell<u64>
//   MemoryId(3) — handle_index:     StableBTreeMap<String, u64>  (handle -> profile_id)
//   MemoryId(4..6) — reserved for Stage 2 (posts)
//   MemoryId(7..9) — reserved for Stage 3 (comments)
//
// ic-cdk 0.19.0 notes:
//   - ic_cdk::api::caller() is deprecated but still present; suppressed below
//   - ic-stable-structures 0.7.2: StableCell::init() returns StableCell directly (not Result)
//   - ic-stable-structures 0.7.2: StableCell::set() returns old value (not Result)
//   - ic-stable-structures 0.7.2: Storable requires both to_bytes AND into_bytes

#![allow(deprecated)]

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::{caller, time};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, StableCell, Storable,
};
use std::borrow::Cow;
use std::cell::RefCell;

// ---------------------------------------------------------------------------
// Schema version
// ---------------------------------------------------------------------------

const TINYPRESS_SCHEMA_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Memory type alias
// ---------------------------------------------------------------------------

type Memory = VirtualMemory<DefaultMemoryImpl>;

// ---------------------------------------------------------------------------
// Stable memory initialisation
// ---------------------------------------------------------------------------

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // MemoryId(0): profiles map — primary record store
    static PROFILES: RefCell<StableBTreeMap<u64, Profile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    // MemoryId(1): principal -> profile_id index
    static PRINCIPAL_INDEX: RefCell<StableBTreeMap<StorablePrincipal, u64, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ));

    // MemoryId(2): monotonic profile ID counter (auto-increment, never reused — ADR-07)
    // StableCell::init() returns StableCell directly in 0.7.2 — no Result/expect
    static PROFILE_COUNTER: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            0u64,
        )
    );

    // MemoryId(3): handle -> profile_id index (O(1) uniqueness check; avoids full scan)
    static HANDLE_INDEX: RefCell<StableBTreeMap<String, u64, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        ));
}

// ---------------------------------------------------------------------------
// StorablePrincipal — Storable wrapper for ICP Principal
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct StorablePrincipal(Principal);

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.0.as_slice().to_vec())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0.as_slice().to_vec()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        StorablePrincipal(Principal::from_slice(bytes.as_ref()))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 29,
        is_fixed_size: false,
    };
}

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Profile — the PII root record. Hard-deleted on erasure request.
/// Field named `owner` not `principal` — `principal` is a reserved Candid keyword.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Profile {
    pub profile_id:   u64,
    pub owner:        Principal, // ICP caller identity; personal data (ADR-10)
    pub handle:       String,    // unique; immutable after creation; personal data
    pub display_name: String,    // personal data
    pub bio:          String,    // personal data; may be empty
    pub created_at:   u64,       // nanoseconds since epoch
}

impl Storable for Profile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("Profile serialisation failed"))
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(&self).expect("Profile serialisation failed")
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).expect("Profile deserialisation failed")
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// Diagnostic status (ADR-09). Aggregate counts only — no per-record data.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TinypressStatus {
    pub schema_version: u32,
    pub profile_count:  u64,
    pub post_count:     u64,    // Stage 2
    pub comment_count:  u64,    // Stage 3
    pub status:         String, // "ok" in v1
}

/// Error type (spec §4.1). All variants explicit — no silent failures (Principle 5).
/// NotFound and Forbidden are distinct — conflating them leaks information.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TinyPressError {
    NotFound,
    Forbidden,
    AlreadyExists,
    ProfileNotFound,
    PostNotFound,           // Stage 2+
    InvalidInput(String),
    InternalError(String),
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn is_valid_required_text(s: &str) -> bool {
    !s.trim().is_empty()
}

fn next_profile_id() -> u64 {
    PROFILE_COUNTER.with(|c| {
        let mut cell = c.borrow_mut();
        let next = cell.get() + 1;
        // StableCell::set() returns old value in 0.7.2 — not a Result
        let _ = cell.set(next);
        next
    })
}

// ---------------------------------------------------------------------------
// Stage 1 — Profile operations (spec §4.2)
// ---------------------------------------------------------------------------

/// create_profile — registers a new profile for the calling principal.
/// Not idempotent: duplicate principal or handle returns AlreadyExists (ADR-08).
#[ic_cdk::update]
fn create_profile(
    handle: String,
    display_name: String,
    bio: String,
) -> Result<u64, TinyPressError> {
    let caller = caller();

    if !is_valid_required_text(&handle) {
        return Err(TinyPressError::InvalidInput(
            "handle must be non-empty and non-whitespace-only".to_string(),
        ));
    }
    if !is_valid_required_text(&display_name) {
        return Err(TinyPressError::InvalidInput(
            "display_name must be non-empty and non-whitespace-only".to_string(),
        ));
    }

    // One profile per principal
    if PRINCIPAL_INDEX.with(|idx| idx.borrow().contains_key(&StorablePrincipal(caller))) {
        return Err(TinyPressError::AlreadyExists);
    }

    // Handle uniqueness — O(1) via handle_index
    if HANDLE_INDEX.with(|h| h.borrow().contains_key(&handle)) {
        return Err(TinyPressError::AlreadyExists);
    }

    let profile_id = next_profile_id();

    let profile = Profile {
        profile_id,
        owner: caller,
        handle: handle.clone(),
        display_name,
        bio,
        created_at: time(),
    };

    PROFILES.with(|p| p.borrow_mut().insert(profile_id, profile));
    PRINCIPAL_INDEX.with(|idx| idx.borrow_mut().insert(StorablePrincipal(caller), profile_id));
    HANDLE_INDEX.with(|h| h.borrow_mut().insert(handle, profile_id));

    Ok(profile_id)
}

/// get_profile — NotFound if profile_id does not exist.
#[ic_cdk::query]
fn get_profile(profile_id: u64) -> Result<Profile, TinyPressError> {
    PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::NotFound)
}

/// update_profile — updates display_name and bio only.
/// handle is immutable. creator_handle on existing posts NOT updated (ADR-04).
/// Idempotent (ADR-08).
#[ic_cdk::update]
fn update_profile(display_name: String, bio: String) -> Result<(), TinyPressError> {
    let caller = caller();

    let profile_id = PRINCIPAL_INDEX
        .with(|idx| idx.borrow().get(&StorablePrincipal(caller)))
        .ok_or(TinyPressError::ProfileNotFound)?;

    let mut profile = PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::InternalError(
            "Principal index references missing profile".to_string(),
        ))?;

    profile.display_name = display_name;
    profile.bio = bio;
    // handle unchanged — immutable after creation (spec §4.2)

    PROFILES.with(|p| p.borrow_mut().insert(profile_id, profile));

    Ok(())
}

/// delete_profile — hard-deletes profile. Posts/comments NOT cascaded (ADR-05).
/// Orphaned records are residual live application data — not anonymised or de-identified.
/// NotFound if absent; Forbidden if caller != owner (distinct — spec §4.1).
/// Idempotent after first success (ADR-08).
#[ic_cdk::update]
fn delete_profile(profile_id: u64) -> Result<(), TinyPressError> {
    let caller = caller();

    let profile = PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::NotFound)?;

    if profile.owner != caller {
        return Err(TinyPressError::Forbidden);
    }

    PROFILES.with(|p| p.borrow_mut().remove(&profile_id));
    PRINCIPAL_INDEX.with(|idx| idx.borrow_mut().remove(&StorablePrincipal(caller)));
    HANDLE_INDEX.with(|h| h.borrow_mut().remove(&profile.handle));

    Ok(())
}

// ---------------------------------------------------------------------------
// Diagnostic query (ADR-09)
// ---------------------------------------------------------------------------

/// tinypress_status — aggregate counts + schema version. No per-record data.
#[ic_cdk::query]
fn tinypress_status() -> TinypressStatus {
    TinypressStatus {
        schema_version: TINYPRESS_SCHEMA_VERSION,
        profile_count:  PROFILES.with(|p| p.borrow().len()),
        post_count:     0, // Stage 2
        comment_count:  0, // Stage 3
        status:         "ok".to_string(),
    }
}
