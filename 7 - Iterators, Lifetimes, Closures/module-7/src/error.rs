// =============================================================================
// ERROR.RS - Custom Error Handling
// =============================================================================
//
// CONCEPTS DEMONSTRATED:
// ----------------------
// 1. ENUMS (Module 6 - Enums)
//    - Defining custom enum types with variants
//    - Enum variants can hold data (like WordNotFound(String))
//
// 2. RESULT ENUM (Module 6 - Result Enum, Ch 9)
//    - Result<T, E> is Rust's standard way to handle recoverable errors
//    - Ok(T) represents success with value T
//    - Err(E) represents failure with error E
//
// 3. TRAITS (Module 6 - Traits)
//    - Implementing std::fmt::Display for human-readable error messages
//    - Implementing std::error::Error for integration with Rust's error ecosystem
//
// 4. TYPE ALIASES
//    - Creating shorthand for commonly used types
//    - Reduces boilerplate when using Result<T, AnalysisError>
//
// =============================================================================

use std::fmt;

// -----------------------------------------------------------------------------
// CUSTOM ERROR ENUM
// -----------------------------------------------------------------------------
//
// In Rust, we define custom error types as enums. Each variant represents
// a different kind of error that can occur.
//
// From Module 6 (Result Enum):
//   enum Result<T, E> {
//       Ok(T),   // Success case with value of type T
//       Err(E),  // Error case with error of type E
//   }
//
// We create our own error type E to use with Result.
//
// WHY ENUMS FOR ERRORS?
// - Exhaustive matching: The compiler ensures all error cases are handled
// - Type safety: Can't confuse different error types
// - Data attachment: Variants can carry relevant context (like the missing word)
//
// #[derive(Debug)] automatically implements the Debug trait, allowing us to
// print the error with {:?} format specifier for debugging purposes.
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum AnalysisError {
    // Simple variant with no associated data
    // Used when the input text is completely empty
    EmptyInput,

    // Simple variant with no associated data
    // Used when text exists but contains no valid words (e.g., only punctuation)
    NoWordsFound,

    // Variant with associated data (a String)
    // This is like a tuple struct variant - it holds the word that wasn't found
    // From Module 6 (Enums): Enum variants can hold data of any type
    WordNotFound(String),
}

// -----------------------------------------------------------------------------
// IMPLEMENTING THE DISPLAY TRAIT
// -----------------------------------------------------------------------------
//
// From Module 6 (Traits):
//   - Traits define shared behavior (like interfaces in other languages)
//   - We implement traits for our types using `impl TraitName for TypeName`
//
// std::fmt::Display is a standard library trait that:
//   - Provides human-readable string representation
//   - Enables using {} in println! and format! macros
//   - Is the "user-facing" output (vs Debug which is for developers)
//
// The fmt method signature:
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
//
// LIFETIME ANNOTATION '_:
//   - The '_ is an anonymous/elided lifetime
//   - The Formatter borrows something, but we don't need to name that lifetime
//   - From Module 7 (Lifetime Elision): Rust can infer some lifetimes
// -----------------------------------------------------------------------------

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // MATCH EXPRESSION (Module 4 - Match Statement)
        // ---------------------------------------------
        // Match must be exhaustive - every possible variant must be handled.
        // The compiler will error if we forget a case.
        //
        // Pattern matching on `self` to determine which variant we have:
        // - AnalysisError::EmptyInput => matches the EmptyInput variant
        // - AnalysisError::NoWordsFound => matches the NoWordsFound variant
        // - AnalysisError::WordNotFound(word) => destructures to extract the String
        //
        // write! macro writes formatted text to the Formatter.
        // It returns fmt::Result (Ok(()) on success, Err on failure).

        match self {
            AnalysisError::EmptyInput => write!(f, "Input text is empty"),
            AnalysisError::NoWordsFound => write!(f, "No words found in text"),

            // DESTRUCTURING (Module 6 - Enums)
            // Here we extract the String from the WordNotFound variant
            // `word` becomes a reference to the String inside
            AnalysisError::WordNotFound(word) => write!(f, "Word not found: {}", word),
        }
    }
}

// -----------------------------------------------------------------------------
// IMPLEMENTING THE ERROR TRAIT
// -----------------------------------------------------------------------------
//
// std::error::Error is a marker trait that identifies a type as an error.
// It requires Display to be implemented (which we did above).
//
// Benefits of implementing Error:
// - Integration with ? operator for error propagation
// - Works with Box<dyn Error> for heterogeneous error handling
// - Compatibility with error handling crates (anyhow, thiserror)
//
// Empty implementation {} uses default trait methods.
// The Error trait has optional methods like source() for error chains,
// but we don't need them for this simple example.
// -----------------------------------------------------------------------------

impl std::error::Error for AnalysisError {}

// -----------------------------------------------------------------------------
// TYPE ALIAS
// -----------------------------------------------------------------------------
//
// Type aliases create a new name for an existing type.
// They don't create a new type - just a shorthand.
//
// Instead of writing:
//   fn try_extract_words(text: &str) -> Result<Vec<Word>, AnalysisError>
//
// We can write:
//   fn try_extract_words(text: &str) -> AnalysisResult<Vec<Word>>
//
// The <T> makes this a generic type alias:
// - AnalysisResult<i32> expands to Result<i32, AnalysisError>
// - AnalysisResult<Vec<Word>> expands to Result<Vec<Word>, AnalysisError>
//
// This is a common pattern in Rust:
// - std::io::Result<T> is type alias for Result<T, std::io::Error>
// - std::fmt::Result is type alias for Result<(), std::fmt::Error>
// -----------------------------------------------------------------------------

pub type AnalysisResult<T> = Result<T, AnalysisError>;
