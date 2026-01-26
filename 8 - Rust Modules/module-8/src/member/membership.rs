//! Membership submodule - demonstrates SUBMODULES within a directory module.
//!
//! This file is loaded because `member/mod.rs` contains `mod membership;`.
//! It's a submodule of `member`, so its full path is `crate::member::membership`.

// =============================================================================
// MEMBERSHIP TIER ENUM
// =============================================================================

/// Library membership tiers with different privileges.
///
/// This enum is re-exported by the parent module (`member/mod.rs`),
/// so users can access it as `module_8::MembershipTier` or `module_8::member::MembershipTier`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MembershipTier {
    /// Basic membership - limited privileges
    Basic,
    /// Silver membership - moderate privileges
    Silver,
    /// Gold membership - premium privileges
    Gold,
}

impl MembershipTier {
    /// Returns the maximum number of books this tier can borrow.
    pub fn borrow_limit(&self) -> usize {
        match self {
            MembershipTier::Basic => 2,
            MembershipTier::Silver => 5,
            MembershipTier::Gold => 10,
        }
    }

    /// Returns the loan period in days for this tier.
    pub fn loan_days(&self) -> u32 {
        match self {
            MembershipTier::Basic => 14,
            MembershipTier::Silver => 21,
            MembershipTier::Gold => 30,
        }
    }
}

// =============================================================================
// VISIBILITY MODIFIERS DEMONSTRATION
// =============================================================================

/// Calculates discount percentage for a membership tier.
///
/// This function is `pub(super)` - visible to the parent module (`member`)
/// but NOT to modules outside of `member`. This allows `Member` to use it
/// internally while keeping it hidden from the public API.
///
/// # Visibility Levels:
/// - `pub`: visible everywhere the parent module is visible
/// - `pub(crate)`: visible anywhere in the current crate
/// - `pub(super)`: visible to the parent module only
/// - `pub(in path)`: visible within the specified path
/// - (no modifier): private to this module only
pub(super) fn calculate_discount(tier: &MembershipTier) -> u8 {
    match tier {
        MembershipTier::Basic => 0,
        MembershipTier::Silver => 10,
        MembershipTier::Gold => 20,
    }
}

/// Internal function - completely private to this module.
/// Not even the parent module (`member`) can access this.
#[allow(dead_code)]
fn tier_rank(tier: &MembershipTier) -> u8 {
    match tier {
        MembershipTier::Basic => 1,
        MembershipTier::Silver => 2,
        MembershipTier::Gold => 3,
    }
}

/// Crate-visible function - accessible anywhere in this crate.
///
/// This demonstrates `pub(crate)` - useful for internal utilities
/// that multiple modules need but shouldn't be part of the public API.
#[allow(dead_code)]
pub(crate) fn is_premium_tier(tier: &MembershipTier) -> bool {
    matches!(tier, MembershipTier::Gold)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_limits() {
        assert_eq!(MembershipTier::Basic.borrow_limit(), 2);
        assert_eq!(MembershipTier::Silver.borrow_limit(), 5);
        assert_eq!(MembershipTier::Gold.borrow_limit(), 10);
    }

    #[test]
    fn test_discounts() {
        assert_eq!(calculate_discount(&MembershipTier::Basic), 0);
        assert_eq!(calculate_discount(&MembershipTier::Gold), 20);
    }

    #[test]
    fn test_private_function() {
        // Private functions are accessible within the same module's tests
        assert_eq!(tier_rank(&MembershipTier::Basic), 1);
        assert_eq!(tier_rank(&MembershipTier::Gold), 3);
    }
}
