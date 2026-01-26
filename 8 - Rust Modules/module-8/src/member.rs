//! Member module - demonstrates a DIRECTORY-BASED MODULE using MODERN STYLE.
//!
//! # Module Style Comparison
//!
//! When `lib.rs` contains `mod member;`, Rust looks for the module entry point:
//!
//! ## Older Style (pre-Rust 2018):
//! ```text
//! src/
//! └── member/
//!     ├── mod.rs          ← Entry point (this pattern)
//!     └── membership.rs   ← Submodule
//! ```
//!
//! ## Modern Style (Rust 2018+) - USED HERE:
//! ```text
//! src/
//! ├── member.rs           ← Entry point (THIS FILE)
//! └── member/
//!     └── membership.rs   ← Submodule
//! ```
//!
//! # Why Modern Style is Preferred
//!
//! 1. **Better editor experience**: No more multiple `mod.rs` tabs that all look the same
//! 2. **Clearer file naming**: The file name matches the module name (`member.rs` for `mod member`)
//! 3. **Easier navigation**: You can find modules by their actual name in file explorers
//!
//! # How It Works
//!
//! - This file (`src/member.rs`) is the entry point for the `member` module
//! - Submodules are declared with `mod submodule_name;`
//! - Rust looks for submodules in `src/member/` directory
//! - Example: `mod membership;` looks for `src/member/membership.rs`
//!
//! Both styles are fully supported and compile identically. The choice is purely
//! organizational. Most new Rust projects use the modern style.

// =============================================================================
// SUBMODULE DECLARATION
// =============================================================================

// Declare submodule - Rust looks for `src/member/membership.rs`
// This is private by default, but we'll re-export what we need.
mod membership;

// =============================================================================
// RE-EXPORTS FROM SUBMODULE
// =============================================================================

// Re-export `MembershipTier` so users can access it as `member::MembershipTier`
// instead of `member::membership::MembershipTier`.
// The original `membership` module remains private - users can't access it directly.
pub use membership::MembershipTier;

// =============================================================================
// MAIN STRUCT
// =============================================================================

use crate::book::Book;

/// A library member who can borrow books.
///
/// This struct demonstrates:
/// - Using types from sibling modules (`Book` via `crate::book`)
/// - Using types from submodules (`MembershipTier`)
/// - Mixed field visibility
#[derive(Debug)]
pub struct Member {
    // Private fields - controlled via methods
    id: u64,
    borrowed_books: Vec<Book>,

    // Public fields
    pub name: String,
    pub tier: MembershipTier,
}

impl Member {
    /// Creates a new library member.
    ///
    /// # Examples
    ///
    /// ```
    /// use module_8::{Member, MembershipTier};
    /// let member = Member::new(1, "Alice", MembershipTier::Gold);
    /// assert_eq!(member.name, "Alice");
    /// ```
    pub fn new(id: u64, name: &str, tier: MembershipTier) -> Self {
        Member {
            id,
            name: String::from(name),
            tier,
            borrowed_books: Vec::new(),
        }
    }

    /// Returns the member's ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the number of books currently borrowed.
    pub fn borrowed_count(&self) -> usize {
        self.borrowed_books.len()
    }

    /// Returns the maximum books this member can borrow based on their tier.
    pub fn max_books(&self) -> usize {
        // Using a method from the submodule's type
        self.tier.borrow_limit()
    }

    /// Attempts to borrow a book.
    ///
    /// Returns `Ok(())` if successful, `Err` with a message if not.
    pub fn borrow(&mut self, mut book: Book) -> Result<(), &'static str> {
        if self.borrowed_books.len() >= self.max_books() {
            return Err("Borrow limit reached");
        }

        if !book.is_available() {
            return Err("Book is not available");
        }

        book.borrow_book();
        self.borrowed_books.push(book);
        Ok(())
    }

    /// Returns a borrowed book.
    ///
    /// Returns the book if found, or `None` if the member doesn't have it.
    pub fn return_book(&mut self, book_id: u64) -> Option<Book> {
        if let Some(pos) = self.borrowed_books.iter().position(|b| b.id() == book_id) {
            let mut book = self.borrowed_books.remove(pos);
            book.return_book();
            Some(book)
        } else {
            None
        }
    }

    /// Lists all borrowed books (read-only access).
    pub fn borrowed_books(&self) -> &[Book] {
        &self.borrowed_books
    }

    /// Calculates the member's discount based on tier.
    /// Uses `pub(super)` function from membership module.
    pub fn discount_percentage(&self) -> u8 {
        // Access pub(super) function - works because we're in the parent module
        membership::calculate_discount(&self.tier)
    }
}

// =============================================================================
// MODULE-LEVEL FUNCTION
// =============================================================================

/// Creates a guest member with basic tier.
///
/// This is a module-level function (not a method) that demonstrates
/// another way to construct types.
pub fn create_guest(id: u64, name: &str) -> Member {
    Member::new(id, name, MembershipTier::Basic)
}
