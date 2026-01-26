//! Utilities module - demonstrates PATH RESOLUTION and VISIBILITY.
//!
//! This module shows different ways to reference items from other modules:
//! - `crate::` - absolute path from crate root
//! - `self::` - relative path from current module
//! - `super::` - relative path from parent module
//! - `use` statements to bring items into scope

// =============================================================================
// IMPORTING FROM OTHER MODULES
// =============================================================================

// ABSOLUTE PATH: Start from the crate root using `crate::`
// This is unambiguous and works from anywhere in the crate.
use crate::book::{Book, Genre};

// We can also use multiple items from the same module with nested paths:
// use crate::member::{Member, MembershipTier};

// =============================================================================
// PUBLIC UTILITY FUNCTIONS
// =============================================================================

/// Formats book information for display.
///
/// This function is re-exported at the crate root via `pub use` in lib.rs,
/// so users can call it as `module_8::format_book_info()`.
///
/// # Examples
///
/// ```
/// use module_8::{Book, Genre, format_book_info};
/// let book = Book::new(1, "Rust Basics", Genre::Technical);
/// let info = format_book_info(&book);
/// assert!(info.contains("Rust Basics"));
/// ```
pub fn format_book_info(book: &Book) -> String {
    let availability = if book.is_available() {
        "Available"
    } else {
        "Borrowed"
    };

    format!(
        "[#{}] \"{}\" ({:?}) - {} | Borrowed {} times",
        book.id(),
        book.title,
        book.genre,
        availability,
        book.times_borrowed()
    )
}

/// Formats a genre for display.
pub fn format_genre(genre: &Genre) -> &'static str {
    match genre {
        Genre::Fiction => "Fiction",
        Genre::NonFiction => "Non-Fiction",
        Genre::Technical => "Technical",
        Genre::Mystery => "Mystery",
        Genre::SciFi => "Science Fiction",
    }
}

// =============================================================================
// CRATE-INTERNAL UTILITIES
// =============================================================================

/// Validates a book title.
///
/// `pub(crate)` means this is accessible anywhere in the crate,
/// but NOT by external users of the library.
#[allow(dead_code)]
pub(crate) fn validate_title(title: &str) -> bool {
    !title.is_empty() && title.len() <= 200
}

/// Generates a unique identifier.
///
/// Completely private - only accessible within this `utils` module.
#[allow(dead_code)]
fn generate_id() -> u64 {
    // In a real app, this would use a proper ID generation strategy
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

// =============================================================================
// NESTED MODULE DEMONSTRATING `self::` AND `super::`
// =============================================================================

/// Submodule for formatting helpers.
///
/// This demonstrates inline nested modules and path resolution.
pub mod formatting {
    // `super::` refers to the parent module (utils)
    // This imports the Genre type that `utils` imported from `crate::book`
    use super::Genre;

    /// Formats genre as an emoji.
    pub fn genre_emoji(genre: &Genre) -> &'static str {
        match genre {
            Genre::Fiction => "📖",
            Genre::NonFiction => "📚",
            Genre::Technical => "💻",
            Genre::Mystery => "🔍",
            Genre::SciFi => "🚀",
        }
    }

    /// A helper that uses `self::` to reference items in the current module.
    pub fn genre_with_emoji(genre: &Genre) -> String {
        // `self::genre_emoji` is equivalent to just `genre_emoji` here,
        // but `self::` makes it explicit that we're calling a function
        // from the current module.
        format!("{} {}", self::genre_emoji(genre), super::format_genre(genre))
    }

    // Private nested module
    mod internal {
        /// This function can access its parent (`formatting`) via `super::`
        /// and the grandparent (`utils`) via `super::super::`
        #[allow(dead_code)]
        pub(super) fn validate_emoji(emoji: &str) -> bool {
            !emoji.is_empty()
        }
    }

    /// Uses the private internal module.
    #[allow(dead_code)]
    pub(crate) fn is_valid_genre_emoji(genre: &Genre) -> bool {
        let emoji = genre_emoji(genre);
        // Access pub(super) function from child module
        internal::validate_emoji(emoji)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_book_info() {
        let book = Book::new(42, "Test Book", Genre::Fiction);
        let info = format_book_info(&book);

        assert!(info.contains("42"));
        assert!(info.contains("Test Book"));
        assert!(info.contains("Fiction"));
        assert!(info.contains("Available"));
    }

    #[test]
    fn test_validate_title() {
        assert!(validate_title("Valid Title"));
        assert!(!validate_title(""));
        assert!(!validate_title(&"x".repeat(201)));
    }

    #[test]
    fn test_formatting_submodule() {
        let genre = Genre::Technical;
        assert_eq!(formatting::genre_emoji(&genre), "💻");
        assert!(formatting::genre_with_emoji(&genre).contains("Technical"));
    }
}
