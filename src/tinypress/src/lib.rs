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
//   MemoryId(3..5) — reserved for Stage 2 (posts)
//   MemoryId(6..8) — reserved for Stage 3 (comments)

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
// Schema version — increment if data structures change incompatibly
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

    // MemoryId(0): profiles map
    static PROFILES: RefCell<StableBTreeMap<u64, Profile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    // MemoryId(1): principal -> profile_id index (for O(1) caller lookup)
    static PRINCIPAL_INDEX: RefCell<StableBTreeMap<StorablePrincipal, u64, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ));

    // MemoryId(2): monotonic profile ID counter (auto-increment, never reused — ADR-07)
    static PROFILE_COUNTER: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            0u64,
        )
        .expect("Failed to initialise profile counter"),
    );
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

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        StorablePrincipal(Principal::from_slice(bytes.as_ref()))
    }

    // ICP principal: variable length, max 29 bytes
    const BOUND: Bound = Bound::Bounded {
        max_size: 29,
        is_fixed_size: false,
    };
}

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Profile — the PII root record. Hard-deleted on erasure request.
/// All fields are personal data or personal-data-bearing in context (ADR-10).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Profile {
    pub profile_id:   u64,
    pub principal:    Principal,
    pub handle:       String,   // unique; immutable after creation; personal data
    pub display_name: String,   // personal data
    pub bio:          String,   // personal data; may be empty
    pub created_at:   u64,      // nanoseconds since epoch
}

impl Storable for Profile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(
            candid::encode_one(self).expect("Profile serialisation failed"),
        )
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).expect("Profile deserialisation failed")
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// Diagnostic status returned by tinypress_status() (ADR-09).
/// Contains only aggregate counts and schema version — no per-record data.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TinypressStatus {
    pub schema_version: u32,
    pub profile_count:  u64,
    pub post_count:     u64,    // Stage 2
    pub comment_count:  u64,    // Stage 3
    pub status:         String, // "ok" in v1
}

/// Error type (spec §4.1). All variants are explicit — no silent failures (Principle 5).
/// NotFound and Forbidden must remain distinct — conflating them leaks information (spec §4.1 note).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TinyPressError {
    NotFound,               // record does not exist
    Forbidden,              // record exists; caller not authorised
    AlreadyExists,          // duplicate principal or handle
    ProfileNotFound,        // caller has no registered profile
    PostNotFound,           // referenced post_id does not exist (Stage 2+)
    InvalidInput(String),   // malformed or empty required field
    InternalError(String),  // storage/serialisation failure
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Returns true if s is non-empty and not whitespace-only.
/// Used for required text field validation (spec §5).
fn is_valid_required_text(s: &str) -> bool {
    !s.trim().is_empty()
}

/// Increments the profile counter and returns the new ID.
/// profile_id is stable, auto-increment, and never reused (ADR-07).
fn next_profile_id() -> Result<u64, TinyPressError> {
    PROFILE_COUNTER.with(|c| {
        let mut cell = c.borrow_mut();
        let next = cell.get() + 1;
        match cell.set(next) {
            Ok(_) => Ok(next),
            Err(e) => Err(TinyPressError::InternalError(
                format!("Profile counter update failed: {:?}", e),
            )),
        }
    })
}

// ---------------------------------------------------------------------------
// Stage 1 — Profile operations (spec §4.2)
// ---------------------------------------------------------------------------

/// create_profile — registers a new profile for the calling principal.
///
/// Failure semantics (spec §5):
///   - InvalidInput  if handle or display_name is empty/whitespace-only
///   - AlreadyExists if caller already has a profile
///   - AlreadyExists if handle is already taken
///
/// Not idempotent: duplicate principal returns AlreadyExists (ADR-08).
#[ic_cdk::update]
fn create_profile(
    handle: String,
    display_name: String,
    bio: String,
) -> Result<u64, TinyPressError> {
    let caller = caller();

    // Validate required fields — bio may be empty
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

    // Handle uniqueness — O(n) iteration is acceptable at toy app data volumes
    if PROFILES.with(|p| p.borrow().iter().any(|(_, prof)| prof.handle == handle)) {
        return Err(TinyPressError::AlreadyExists);
    }

    let profile_id = next_profile_id()?;

    let profile = Profile {
        profile_id,
        principal: caller,
        handle,
        display_name,
        bio,
        created_at: time(),
    };

    PROFILES.with(|p| p.borrow_mut().insert(profile_id, profile));
    PRINCIPAL_INDEX.with(|idx| {
        idx.borrow_mut().insert(StorablePrincipal(caller), profile_id)
    });

    Ok(profile_id)
}

/// get_profile — returns the profile record for the given profile_id.
///
/// Failure semantics:
///   - NotFound if profile_id does not exist
#[ic_cdk::query]
fn get_profile(profile_id: u64) -> Result<Profile, TinyPressError> {
    PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::NotFound)
}

/// update_profile — updates display_name and bio for the calling principal's profile.
///
/// handle is immutable after creation — not accepted here (spec §4.2).
/// creator_handle on existing posts is NOT updated — historical copy by design (ADR-04).
///
/// Failure semantics:
///   - ProfileNotFound if caller has no registered profile
///
/// Idempotent: same update applied twice produces the same result (ADR-08).
#[ic_cdk::update]
fn update_profile(display_name: String, bio: String) -> Result<(), TinyPressError> {
    let caller = caller();

    let profile_id = PRINCIPAL_INDEX
        .with(|idx| idx.borrow().get(&StorablePrincipal(caller)))
        .ok_or(TinyPressError::ProfileNotFound)?;

    let mut profile = PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::InternalError(
            "Principal index references missing profile record".to_string(),
        ))?;

    profile.display_name = display_name;
    profile.bio = bio;
    // profile.handle unchanged — immutable (spec §4.2)

    PROFILES.with(|p| p.borrow_mut().insert(profile_id, profile));

    Ok(())
}

/// delete_profile — hard-deletes the profile record for the given profile_id.
///
/// Posts and comments authored by this profile are NOT deleted.
/// Orphaned records with dangling author references are the intended post-deletion
/// state for MKTd03 integration testing (ADR-05). They are residual live application
/// data — not anonymised, pseudonymised, or de-identified.
///
/// Failure semantics:
///   - NotFound   if profile_id does not exist
///   - Forbidden  if caller principal != profile.principal (distinct from NotFound — spec §4.1)
///
/// Idempotent after first success: second call returns NotFound, no side effects (ADR-08).
#[ic_cdk::update]
fn delete_profile(profile_id: u64) -> Result<(), TinyPressError> {
    let caller = caller();

    let profile = PROFILES
        .with(|p| p.borrow().get(&profile_id))
        .ok_or(TinyPressError::NotFound)?;

    if profile.principal != caller {
        return Err(TinyPressError::Forbidden);
    }

    PROFILES.with(|p| p.borrow_mut().remove(&profile_id));
    PRINCIPAL_INDEX.with(|idx| idx.borrow_mut().remove(&StorablePrincipal(caller)));

    Ok(())
}

// ---------------------------------------------------------------------------
// Diagnostic query (ADR-09)
// ---------------------------------------------------------------------------

/// tinypress_status — read-only aggregate status. No per-record data exposed.
///
/// Excluded per ADR-09: cycle balance, memory usage, mutation timestamps,
/// per-record identifiers, and any deletion-related counts (ADR-06).
#[ic_cdk::query]
fn tinypress_status() -> TinypressStatus {
    TinypressStatus {
        schema_version: TINYPRESS_SCHEMA_VERSION,
        profile_count:  PROFILES.with(|p| p.borrow().len()),
        post_count:     0, // Stage 2 — not yet implemented
        comment_count:  0, // Stage 3 — not yet implemented
        status:         "ok".to_string(),
    }
}
