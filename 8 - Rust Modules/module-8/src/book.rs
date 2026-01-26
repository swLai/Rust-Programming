//! Book module - demonstrates a FILE-BASED MODULE.
//!
//! This file is loaded because `lib.rs` contains `mod book;`.
//! Rust automatically looks for `src/book.rs` or `src/book/mod.rs`.

// =============================================================================
// ENUM WITH PUBLIC VARIANTS
// =============================================================================

/// Book genre classification.
///
/// When an enum is marked `pub`, ALL its variants are automatically public.
/// This is different from structs, where each field's visibility must be
/// specified individually.
#[derive(Debug, Clone, PartialEq)]
pub enum Genre {
    Fiction,
    NonFiction,
    Technical,
    Mystery,
    SciFi,
}

// =============================================================================
// STRUCT WITH MIXED FIELD VISIBILITY
// =============================================================================

/// Represents a book in the library.
///
/// # Field Visibility
///
/// - `id`: private - can only be set via `new()`, prevents external modification
/// - `title`: public - can be read and modified externally
/// - `genre`: public - can be read and modified externally
/// - `is_available`: private - controlled via methods to maintain invariants
///
/// This demonstrates how Rust lets you control access at the field level.
#[derive(Debug, Clone)]
pub struct Book {
    // Private field: only accessible within this module
    id: u64,

    // Public fields: accessible from anywhere the struct is visible
    pub title: String,
    pub genre: Genre,

    // Private field: we control availability through methods
    is_available: bool,

    // Private field: internal tracking
    times_borrowed: u32,
}

impl Book {
    /// Creates a new book.
    ///
    /// Since `id` and `is_available` are private, users MUST use this
    /// constructor - they cannot create a Book using struct literal syntax
    /// like `Book { id: 1, ... }`.
    ///
    /// # Examples
    ///
    /// ```
    /// use module_8::{Book, Genre};
    /// let book = Book::new(1, "Rust Programming", Genre::Technical);
    /// assert!(book.is_available());
    /// ```
    pub fn new(id: u64, title: &str, genre: Genre) -> Self {
        Book {
            id,
            title: String::from(title),
            genre,
            is_available: true,
            times_borrowed: 0,
        }
    }

    /// Returns the book's ID (read-only access to private field).
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Checks if the book is available for borrowing.
    pub fn is_available(&self) -> bool {
        self.is_available
    }

    /// Returns how many times this book has been borrowed.
    pub fn times_borrowed(&self) -> u32 {
        self.times_borrowed
    }

    /// Marks the book as borrowed.
    ///
    /// Returns `true` if successful, `false` if already borrowed.
    pub fn borrow_book(&mut self) -> bool {
        if self.is_available {
            self.is_available = false;
            self.times_borrowed += 1;
            true
        } else {
            false
        }
    }

    /// Returns the book to the library.
    pub fn return_book(&mut self) {
        self.is_available = true;
    }
}

// =============================================================================
// MODULE-PRIVATE HELPER (not visible outside this module)
// =============================================================================

/// Internal helper function - not marked `pub`, so it's private to this module.
/// Even though `book.rs` is a module file, items without `pub` are still private.
#[allow(dead_code)]
fn generate_isbn(id: u64) -> String {
    format!("ISBN-{:010}", id)
}

// =============================================================================
// TESTS SUBMODULE
// =============================================================================

// The `#[cfg(test)]` attribute means this module is only compiled during testing.
// This is a common pattern for unit tests in Rust.
#[cfg(test)]
mod tests {
    // `super::*` imports everything from the parent module (book).
    // This is how test modules access the code they're testing.
    use super::*;

    #[test]
    fn test_new_book_is_available() {
        let book = Book::new(1, "Test Book", Genre::Fiction);
        assert!(book.is_available());
        assert_eq!(book.times_borrowed(), 0);
    }

    #[test]
    fn test_borrow_and_return() {
        let mut book = Book::new(1, "Test Book", Genre::Fiction);

        assert!(book.borrow_book()); // First borrow succeeds
        assert!(!book.is_available());
        assert!(!book.borrow_book()); // Second borrow fails

        book.return_book();
        assert!(book.is_available());
        assert_eq!(book.times_borrowed(), 1);
    }

    #[test]
    fn test_private_function_accessible_in_tests() {
        // We can test private functions from within the same module
        let isbn = generate_isbn(42);
        assert_eq!(isbn, "ISBN-0000000042");
    }
}
