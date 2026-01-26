//! # Library Management System
//!
//! This crate demonstrates the various ways to organize Rust modules.
//! It serves as a learning example for understanding:
//!
//! - File-based modules (`mod module_name;`)
//! - Directory-based modules with submodules (modern `module.rs` style vs older `mod.rs` style)
//! - Inline modules (`mod name { ... }`)
//! - Visibility modifiers (`pub`, `pub(crate)`, `pub(super)`)
//! - Re-exporting with `pub use`
//! - Path resolution (`crate::`, `self::`, `super::`)
//!
//! ## Quick Start
//!
//! ```rust
//! use module_8::{Book, Genre, Member, MembershipTier};
//!
//! let book = Book::new(1, "The Rust Book", Genre::Technical);
//! let member = Member::new(1, "Alice", MembershipTier::Gold);
//! ```

// =============================================================================
// MODULE DECLARATIONS
// =============================================================================

// FILE-BASED MODULE: When you write `mod book;`, Rust looks for either:
//   1. src/book.rs (preferred modern style)
//   2. src/book/mod.rs (older style, still valid)
// The module's contents are defined in that separate file.
// Making it `pub mod` allows external access via `module_8::book::*`
pub mod book;

// DIRECTORY-BASED MODULE WITH SUBMODULES:
// When you write `mod member;` and need submodules, Rust supports two styles:
//
//   OLDER STYLE (pre-2018):        MODERN STYLE (2018+, preferred):
//   src/                           src/
//   └── member/                    ├── member.rs         ← entry point
//       ├── mod.rs  ← entry point  └── member/
//       └── membership.rs              └── membership.rs
//
// We use the MODERN STYLE here because:
//   1. File names match module names (easier to find in editors)
//   2. No more multiple "mod.rs" tabs that look identical
//   3. This is the convention in most new Rust projects
//
// Both styles compile identically - the choice is purely organizational.
// `pub mod` exposes the module's public API to external crates.
pub mod member;

// Another file-based module demonstrating visibility modifiers.
// Made public to allow access to nested modules like `utils::formatting`.
pub mod utils;

// =============================================================================
// INLINE MODULE
// =============================================================================

// INLINE MODULE: Defined directly in this file. Useful for small, closely
// related code that doesn't warrant its own file. Everything inside is
// private by default unless marked `pub`.
// We make the entire module `pub` to expose it to external crates.
pub mod config {
    /// Maximum number of books a member can borrow at once.
    /// This is pub(crate) - visible within this crate but not to external users.
    pub(crate) const MAX_BORROWED_BOOKS: usize = 5;

    /// Library operating hours (internal configuration).
    /// This is completely private - only accessible within this `config` module.
    #[allow(dead_code)]
    const OPENING_HOUR: u8 = 9;

    /// A public constant that external crates can access.
    pub const LIBRARY_NAME: &str = "Rustacean Library";

    // NESTED INLINE MODULE: Modules can be nested to any depth.
    // This demonstrates how child modules can access parent items.
    pub mod fees {
        /// Late fee per day in cents.
        pub const LATE_FEE_PER_DAY: u32 = 25;

        /// Calculate total late fee.
        ///
        /// # Examples
        ///
        /// ```
        /// use module_8::config::fees::calculate_late_fee;
        /// assert_eq!(calculate_late_fee(3), 75);
        /// ```
        pub fn calculate_late_fee(days_overdue: u32) -> u32 {
            days_overdue * LATE_FEE_PER_DAY
        }

        /// Internal helper - uses `super::` to access parent module's items.
        #[allow(dead_code)]
        pub(crate) fn max_fee() -> u32 {
            // `super::` refers to the parent module (config)
            super::MAX_BORROWED_BOOKS as u32 * LATE_FEE_PER_DAY * 30
        }
    }
}

// =============================================================================
// RE-EXPORTING (pub use)
// =============================================================================

// RE-EXPORTING: `pub use` brings items into scope AND makes them publicly
// accessible from this module. This creates a cleaner public API by:
//   1. Hiding internal module structure from users
//   2. Allowing users to import directly: `use module_8::Book;`
//      instead of: `use module_8::book::Book;`

// Re-export main types at the crate root for convenient access
pub use book::{Book, Genre};
pub use member::{Member, MembershipTier};

// Re-export the config module itself (users can access config::LIBRARY_NAME)
pub use config::LIBRARY_NAME;

// Selectively re-export from config::fees
pub use config::fees::calculate_late_fee;

// Re-export utility functions that are part of our public API
pub use utils::format_book_info;

// =============================================================================
// CRATE-LEVEL FUNCTIONALITY
// =============================================================================

/// Represents the library system that manages books and members.
///
/// This struct demonstrates using types from different modules.
pub struct Library {
    name: String,
    books: Vec<Book>,
    members: Vec<Member>,
}

impl Library {
    /// Creates a new library with the default name.
    ///
    /// # Examples
    ///
    /// ```
    /// use module_8::Library;
    /// let lib = Library::new();
    /// ```
    pub fn new() -> Self {
        Library {
            name: String::from(LIBRARY_NAME),
            books: Vec::new(),
            members: Vec::new(),
        }
    }

    /// Adds a book to the library.
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// Registers a new member.
    pub fn register_member(&mut self, member: Member) {
        self.members.push(member);
    }

    /// Returns the library name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of books.
    pub fn book_count(&self) -> usize {
        self.books.len()
    }

    /// Returns the number of members.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Gets the maximum books allowed per member.
    /// Uses a crate-private constant from the config module.
    pub fn max_books_per_member(&self) -> usize {
        // Accessing a pub(crate) item - works within this crate
        config::MAX_BORROWED_BOOKS
    }

    /// Displays all books in the library.
    pub fn display_books(&self) {
        for book in &self.books {
            // Using the re-exported utility function
            println!("{}", format_book_info(book));
        }
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}
