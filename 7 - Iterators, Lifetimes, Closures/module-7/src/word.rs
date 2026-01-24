// =============================================================================
// WORD.RS - Core Word Struct with Lifetimes
// =============================================================================
//
// CONCEPTS DEMONSTRATED:
// ----------------------
// 1. LIFETIMES (Module 7 - Lifetimes Part 1 & 2)
//    - Struct lifetime parameters: Word<'a>
//    - Function lifetime parameters
//    - Multiple lifetime parameters
//    - Preventing dangling references
//
// 2. STRUCTS (Module 6 - Structs)
//    - Defining structs with fields
//    - Struct methods via impl blocks
//
// 3. REFERENCES & BORROWING (Module 3 - Ownership)
//    - Borrowing with &str instead of owning String
//    - Memory efficiency through borrowing
//
// 4. OPTION ENUM (Module 6 - Option Enum)
//    - Option<T> for values that might not exist
//    - Option combinators (map_or)
//
// 5. MATCH EXPRESSIONS (Module 4 - Match Statement)
//    - Range patterns (1..=3)
//    - Wildcard pattern (_)
//
// 6. ITERATORS (Module 7 - Iterators Part 1 & 2)
//    - iter(), enumerate(), find(), max_by_key()
//
// 7. CLOSURES (Module 7 - Closures Part 1)
//    - Inline closures for filtering and transformation
//
// =============================================================================

use crate::error::{AnalysisError, AnalysisResult};

// =============================================================================
// STRUCT WITH LIFETIME PARAMETER
// =============================================================================
//
// From Module 7 (Lifetimes Part 2):
//   struct Person<'a> {
//       name: &'a str,
//       age: i32,
//   }
//
// WHY LIFETIMES?
// --------------
// The Word struct borrows text from elsewhere (it holds &str, not String).
// The lifetime 'a tells Rust: "This Word can only exist as long as the
// text it borrows from exists."
//
// This prevents DANGLING REFERENCES (Module 7 - Lifetimes Part 1):
//   let word;
//   {
//       let text = String::from("hello");
//       word = Word::new(&text, 0, 1);  // word borrows from text
//   }  // text is dropped here
//   println!("{}", word.text);  // ERROR! word.text points to freed memory
//
// MEMORY EFFICIENCY:
// - Word holds &str (a reference) instead of String (owned data)
// - Multiple Words can share the same underlying text
// - No memory allocation for each word - just pointers into original text
//
// #[derive(Debug, Clone, Copy)] automatically implements these traits:
// - Debug: Enables {:?} formatting for printing
// - Clone: Enables .clone() method
// - Copy: Enables implicit copying (because &str is Copy, usize is Copy)
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct Word<'a> {
    // BORROWED STRING SLICE
    // &'a str means: "a reference to a string slice that lives at least as long as 'a"
    // This is more memory-efficient than String because no allocation occurs
    pub text: &'a str,

    // Position of word within its line (0-indexed)
    pub position: usize,

    // Line number in source text (1-indexed for human readability)
    pub line: usize,
}

// =============================================================================
// IMPL BLOCK WITH LIFETIME
// =============================================================================
//
// From Module 6 (Generics):
//   impl<T: std::fmt::Debug, U: std::fmt::Debug> Point<T, U> { ... }
//
// For structs with lifetimes, we write impl<'a> StructName<'a>.
// The 'a after impl declares the lifetime parameter.
// The 'a after Word uses that declared lifetime.
// =============================================================================

impl<'a> Word<'a> {
    // -------------------------------------------------------------------------
    // CONSTRUCTOR
    // -------------------------------------------------------------------------
    //
    // The input `text: &'a str` must have lifetime 'a.
    // The return type `Word<'a>` has the same lifetime.
    // This connects input and output: the returned Word lives as long as the input text.
    //
    // STRUCT INITIALIZATION SHORTHAND:
    // When variable names match field names, we can write:
    //   Word { text, position, line }
    // instead of:
    //   Word { text: text, position: position, line: line }
    // -------------------------------------------------------------------------

    pub fn new(text: &'a str, position: usize, line: usize) -> Word<'a> {
        Word {
            text,
            position,
            line,
        }
    }

    // -------------------------------------------------------------------------
    // LENGTH METHODS
    // -------------------------------------------------------------------------
    //
    // len() returns byte length of the UTF-8 string.
    // is_empty() is the companion method - Rust convention (Clippy lint).
    //
    // SELF REFERENCE:
    // &self is shorthand for self: &Self, which is self: &Word<'a>
    // We borrow self immutably, so we can call these methods without moving ownership.
    // -------------------------------------------------------------------------

    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Companion to len() - Rust best practice (Clippy lint).
    /// Any type with len() should also have is_empty().
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    // -------------------------------------------------------------------------
    // OPTION COMBINATORS
    // -------------------------------------------------------------------------
    //
    // From Module 6 (Option Enum):
    //   enum Option<T> {
    //       None,
    //       Some(T),
    //   }
    //
    // Option::map_or(default, f) is a combinator that:
    // - Returns `default` if Option is None
    // - Applies function `f` to the value if Option is Some
    //
    // Equivalent match expression:
    //   match self.text.chars().next() {
    //       Some(c) => c.is_uppercase(),
    //       None => false,
    //   }
    //
    // The combinator version is more idiomatic and concise.
    //
    // ITERATOR CHAIN:
    // self.text.chars() - creates iterator over characters
    // .next() - gets first element as Option<char>
    // .map_or(false, |c| ...) - handles the Option
    // -------------------------------------------------------------------------

    pub fn is_capitalized(&self) -> bool {
        // CLOSURE: |c| c.is_uppercase()
        // This is an inline function that takes c and returns whether it's uppercase.
        // From Module 7 (Closures Part 1): |inputs| expression
        self.text.chars().next().map_or(false, |c| c.is_uppercase())
    }

    // -------------------------------------------------------------------------
    // CHARACTER COUNT
    // -------------------------------------------------------------------------
    //
    // IMPORTANT DISTINCTION:
    // - len() returns BYTES (UTF-8 encoded length)
    // - char_count() returns CHARACTERS (Unicode scalar values)
    //
    // For ASCII text, these are the same.
    // For Unicode text like "café", len() = 5 bytes, char_count() = 4 characters.
    //
    // chars().count() iterates through all characters to count them.
    // -------------------------------------------------------------------------

    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    // -------------------------------------------------------------------------
    // MATCH WITH RANGES
    // -------------------------------------------------------------------------
    //
    // From Module 4 (Match Statement):
    //   match marks {
    //       90..=100 => 'A',
    //       80..=89  => 'B',
    //       _ => 'F',
    //   }
    //
    // RANGE PATTERNS:
    // - 0 => matches exactly 0
    // - 1..=3 => matches 1, 2, or 3 (inclusive range)
    // - _ => wildcard, matches anything else
    //
    // RETURN TYPE &'static str:
    // 'static lifetime means these strings live for the entire program duration.
    // String literals like "short" are stored in the program binary.
    // -------------------------------------------------------------------------

    pub fn length_category(&self) -> &'static str {
        match self.len() {
            0 => "empty",
            1..=3 => "short",     // 1, 2, or 3 characters
            4..=6 => "medium",    // 4, 5, or 6 characters
            7..=10 => "long",     // 7 through 10 characters
            _ => "very long",     // anything longer
        }
    }
}

// =============================================================================
// FUNCTION WITH LIFETIME PARAMETERS
// =============================================================================
//
// From Module 7 (Lifetimes Part 2):
//   fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str { ... }
//
// When a function takes references and returns a reference, Rust needs to know
// how the output lifetime relates to the input lifetimes.
//
// LIFETIME ELISION RULES (Module 7 - Eliding Lifetimes):
// 1. Each reference parameter gets its own lifetime
// 2. If exactly one input lifetime, it's assigned to all output lifetimes
// 3. If &self or &mut self exists, self's lifetime is assigned to outputs
//
// When elision rules don't apply, we must annotate explicitly.
// =============================================================================

/// Extracts words from text, returning Word structs that borrow from the source.
///
/// LIFETIME EXPLANATION:
/// - Input: text with lifetime 'a
/// - Output: Vec<Word<'a>> where each Word borrows from text
///
/// The returned Words are only valid as long as `text` is valid.
/// This is enforced at compile time by the lifetime parameter.
pub fn extract_words<'a>(text: &'a str) -> Vec<Word<'a>> {
    // Create empty vector to collect words
    // Vec::new() creates a vector with no heap allocation until first push
    let mut words = Vec::new();

    // ITERATOR: lines() + enumerate()
    // --------------------------------
    // text.lines() - iterator over lines (splits on \n)
    // .enumerate() - wraps iterator to yield (index, value) tuples
    //
    // From Module 7 (Iterators Part 1): for (line_num, line) in text.lines().enumerate()
    for (line_num, line) in text.lines().enumerate() {
        let mut position = 0;

        // split_whitespace() splits on any whitespace and skips empty strings
        for word_text in line.split_whitespace() {
            // CLOSURE FOR TRIMMING
            // trim_matches takes a closure that returns true for chars to remove
            // |c: char| !c.is_alphanumeric() removes non-alphanumeric chars from edges
            let cleaned = word_text.trim_matches(|c: char| !c.is_alphanumeric());

            if !cleaned.is_empty() {
                // IMPORTANT: `cleaned` is a slice INTO `text`
                // No new allocation occurs - cleaned points to bytes in original text
                // This is why Word can borrow with lifetime 'a
                words.push(Word::new(cleaned, position, line_num + 1));
            }
            position += 1;
        }
    }

    words
}

// =============================================================================
// RESULT-BASED FUNCTION
// =============================================================================
//
// From Module 6 (Result Enum):
//   match some_vec.get(15) {
//       Some(a) => Ok(a),
//       None => Err("The value does not exist"),
//   }
//
// We return Result to indicate success or failure with specific error types.
// The caller must handle both cases (or use ? to propagate errors).
// =============================================================================

/// Extract words with validation, returning Result.
///
/// Unlike extract_words(), this function:
/// - Returns Err(EmptyInput) if text is empty
/// - Returns Err(NoWordsFound) if no valid words exist
/// - Returns Ok(Vec<Word>) on success
pub fn try_extract_words<'a>(text: &'a str) -> AnalysisResult<Vec<Word<'a>>> {
    // Early return on empty input
    if text.is_empty() {
        return Err(AnalysisError::EmptyInput);
    }

    // Reuse the non-Result version for actual extraction
    let words = extract_words(text);

    // Check if any words were found
    if words.is_empty() {
        return Err(AnalysisError::NoWordsFound);
    }

    // Wrap successful result in Ok
    Ok(words)
}

// =============================================================================
// COMPLEX LIFETIME ANNOTATIONS
// =============================================================================
//
// When dealing with structs that have lifetimes AND functions that return
// references, lifetime annotations become more complex.
//
// RULE: The returned reference must live at least as long as the inputs
// it could potentially be derived from.
// =============================================================================

/// Finds the longest word from a slice of words.
///
/// LIFETIME BREAKDOWN:
/// - words: &'a [Word<'a>]
///   - Outer 'a: lifetime of the slice reference
///   - Inner 'a: lifetime of text inside each Word
/// - Return: Option<&'a Word<'a>>
///   - We return a reference with the same lifetime as the slice
///
/// max_by_key finds the maximum element by a key function.
/// |w| w.len() is a closure that extracts the comparison key.
pub fn find_longest<'a>(words: &'a [Word<'a>]) -> Option<&'a Word<'a>> {
    // ITERATOR METHOD: max_by_key
    // Returns the element with the maximum value of the key function
    // Returns None if the iterator is empty
    words.iter().max_by_key(|w| w.len())
}

// =============================================================================
// MULTIPLE LIFETIME PARAMETERS
// =============================================================================
//
// From Module 7 (Lifetimes Part 2):
//   fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str
//
// Sometimes we need different lifetimes for different references.
// This allows more flexibility in how the function can be called.
// =============================================================================

/// Returns the first word matching a condition.
///
/// TWO LIFETIME PARAMETERS:
/// - 'a: lifetime of the slice reference (how long we borrow the slice)
/// - 'b: lifetime of the text inside Words (from original text)
///
/// The return type &'a Word<'b> means:
/// - The reference to Word lives as long as the slice borrow ('a)
/// - The Word's internal text lives as long as 'b
///
/// This allows the slice and the original text to have independent lifetimes.
pub fn find_word_by_text<'a, 'b>(words: &'a [Word<'b>], target: &str) -> Option<&'a Word<'b>> {
    // ITERATOR METHOD: find
    // Returns the first element matching the predicate, wrapped in Some
    // Returns None if no element matches
    //
    // CLOSURE: |w| w.text.eq_ignore_ascii_case(target)
    // Compares word text to target, ignoring ASCII case differences
    words.iter().find(|w| w.text.eq_ignore_ascii_case(target))
}

// =============================================================================
// OPTION TO RESULT CONVERSION
// =============================================================================
//
// A common pattern: convert Option<T> to Result<T, E>
// - Some(value) -> Ok(value)
// - None -> Err(error)
//
// Option::ok_or_else(|| error) does this idiomatically.
// The closure is only called if Option is None (lazy evaluation).
// =============================================================================

/// Find word by text with Result-based error handling.
///
/// IDIOMATIC PATTERN:
/// Instead of:
///   match find_word_by_text(words, target) {
///       Some(word) => Ok(word),
///       None => Err(AnalysisError::WordNotFound(target.to_string())),
///   }
///
/// We use:
///   find_word_by_text(words, target)
///       .ok_or_else(|| AnalysisError::WordNotFound(target.to_string()))
///
/// ok_or_else takes a closure that produces the error.
/// The closure is only called if the Option is None.
pub fn try_find_word<'a, 'b>(words: &'a [Word<'b>], target: &str) -> AnalysisResult<&'a Word<'b>> {
    find_word_by_text(words, target)
        .ok_or_else(|| AnalysisError::WordNotFound(target.to_string()))
}
