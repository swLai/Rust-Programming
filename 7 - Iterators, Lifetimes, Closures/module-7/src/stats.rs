// =============================================================================
// STATS.RS - Traits, Generics, and Iterator Methods
// =============================================================================
//
// CONCEPTS DEMONSTRATED:
// ----------------------
// 1. TRAITS (Module 6 - Traits)
//    - Trait definition with required methods
//    - Default method implementations
//    - Implementing traits for types
//
// 2. GENERICS (Module 6 - Generics)
//    - Generic type parameters <T>
//    - Trait bounds (T: Ord)
//    - Where clauses for complex bounds
//
// 3. ENUMS (Module 6 - Enums)
//    - Simple enum variants
//    - Match with enum values
//
// 4. ITERATORS (Module 7 - Iterators Part 1 & 2)
//    - sum(), max(), min(), filter(), count()
//    - any(), all(), position(), partition()
//    - map(), collect(), fold()
//
// 5. CLOSURES (Module 7 - Closures Part 1 & 2)
//    - Closures as function parameters
//    - Fn trait bound for closures
//
// 6. OPTION HANDLING
//    - unwrap_or() for default values
//    - let-else pattern for early returns
//
// =============================================================================

use crate::word::Word;

// =============================================================================
// TRAIT DEFINITION
// =============================================================================
//
// From Module 6 (Traits and Default Implementations):
//   trait BasicStats {
//       fn mean(&self) -> f32;
//       fn variance(&self) -> f32;
//   }
//
// TRAITS IN RUST:
// - Define shared behavior that types can implement
// - Similar to interfaces in other languages
// - Can have required methods (no body) and default methods (with body)
//
// WHY TRAITS?
// - Polymorphism: Different types can be used interchangeably if they implement the same trait
// - Code reuse: Default implementations reduce duplication
// - Abstraction: Functions can accept "any type that implements X"
// =============================================================================

/// A trait for types that can provide a summary.
///
/// TRAIT SYNTAX:
/// - `fn method(&self)` - required method, implementors must provide body
/// - `fn method(&self) { ... }` - default method, implementors can override
pub trait Summarizable {
    // REQUIRED METHOD
    // No body - every type implementing this trait MUST provide an implementation
    fn summarize(&self) -> String;

    // REQUIRED METHOD
    fn item_count(&self) -> usize;

    // DEFAULT METHOD
    // Has a body - implementors get this for free, but can override if needed
    // This demonstrates code reuse through traits.
    //
    // NOTE: Default methods can call other trait methods (even required ones)
    // because we know any implementing type will have those methods.
    fn brief(&self) -> String {
        format!("{} items", self.item_count())
    }
}

// =============================================================================
// GENERIC FUNCTIONS
// =============================================================================
//
// From Module 6 (Generics):
//   fn square<T>(x: T) -> T
//   where T: std::ops::Mul<Output = T> + Copy {
//       x * x
//   }
//
// GENERICS allow us to write functions that work with any type.
// TRAIT BOUNDS restrict which types can be used.
//
// SYNTAX OPTIONS:
// 1. Inline bounds: fn foo<T: Trait1 + Trait2>(x: T)
// 2. Where clause: fn foo<T>(x: T) where T: Trait1 + Trait2
//
// Where clauses are preferred when bounds are complex or numerous.
// =============================================================================

/// Generic function to find maximum value from any iterator of comparable items.
///
/// TYPE PARAMETERS:
/// - T: The type of items in the iterator (must implement Ord for comparison)
/// - I: The iterator type (must yield items of type T)
///
/// TRAIT BOUNDS:
/// - T: Ord means T must be totally orderable (supports <, >, ==)
/// - I: Iterator<Item = T> means I is an iterator that yields T values
///
/// ASSOCIATED TYPES:
/// Iterator<Item = T> uses an associated type Item to specify what the iterator yields.
pub fn find_max<T, I>(iter: I) -> Option<T>
where
    T: Ord,
    I: Iterator<Item = T>,
{
    // max() is a method on Iterator that returns Option<T>
    // - Some(value) if iterator had elements
    // - None if iterator was empty
    iter.max()
}

/// Generic function to count items matching a predicate.
///
/// TYPE PARAMETERS:
/// - T: Type of items being counted
/// - I: Iterator type
/// - F: Closure/function type for the predicate
///
/// CLOSURE TRAIT BOUND:
/// F: Fn(&T) -> bool means F is a closure that:
/// - Takes a reference to T (borrows, doesn't consume)
/// - Returns a boolean
///
/// WHY Fn AND NOT FnMut OR FnOnce?
/// - Fn: borrows captured variables immutably, can be called multiple times
/// - FnMut: borrows captured variables mutably
/// - FnOnce: takes ownership of captured variables, can only be called once
///
/// We use Fn because filter() needs to call the predicate multiple times.
pub fn count_where<T, I, F>(iter: I, predicate: F) -> usize
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    // ITERATOR CHAIN:
    // filter() - keeps only elements where predicate returns true
    // count() - consumes iterator and counts elements
    //
    // From Module 7 (Iterators Part 2):
    //   let filtered = a.iter().filter(|&x| *x >= 5).collect::<Vec<_>>();
    iter.filter(|item| predicate(item)).count()
}

// =============================================================================
// ENUM FOR CATEGORIZATION
// =============================================================================
//
// From Module 6 (Enums):
// Enums define a type that can be one of several variants.
//
// #[derive(...)] attributes:
// - Debug: Enables {:?} formatting
// - Clone: Enables .clone() method
// - Copy: Enables implicit copying (only for simple enums without data)
// - PartialEq: Enables == and != comparison
// =============================================================================

/// Reading level based on average word length.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadingLevel {
    Elementary,    // Simple, short words (avg < 4 chars)
    Intermediate,  // Standard vocabulary (avg 4-6 chars)
    Advanced,      // Complex vocabulary (avg 6-8 chars)
    Expert,        // Technical content (avg > 8 chars)
}

impl ReadingLevel {
    // -------------------------------------------------------------------------
    // MATCH WITH GUARDS
    // -------------------------------------------------------------------------
    //
    // From Module 4 (Match Statement):
    //   match marks {
    //       90..=100 => 'A',
    //       _ => 'F',
    //   }
    //
    // MATCH GUARDS (if conditions):
    // - `x if x < 4.0` binds the value to x AND checks the condition
    // - The guard is evaluated only if the pattern matches
    //
    // This is different from range patterns because f64 doesn't support ranges.
    // -------------------------------------------------------------------------

    pub fn from_avg_length(avg: f64) -> ReadingLevel {
        match avg {
            // MATCH GUARD: `x if condition` binds value and checks condition
            x if x < 4.0 => ReadingLevel::Elementary,
            x if x < 6.0 => ReadingLevel::Intermediate,
            x if x < 8.0 => ReadingLevel::Advanced,
            _ => ReadingLevel::Expert,  // Wildcard catches all remaining cases
        }
    }

    // -------------------------------------------------------------------------
    // EXHAUSTIVE MATCHING
    // -------------------------------------------------------------------------
    //
    // When matching on an enum, Rust requires ALL variants to be handled.
    // If we add a new variant to ReadingLevel, this match will fail to compile
    // until we add a case for the new variant - this is a compile-time safety feature!
    // -------------------------------------------------------------------------

    pub fn description(&self) -> &'static str {
        match self {
            ReadingLevel::Elementary => "Simple vocabulary, short words",
            ReadingLevel::Intermediate => "Standard vocabulary",
            ReadingLevel::Advanced => "Complex vocabulary",
            ReadingLevel::Expert => "Technical or specialized content",
            // No _ wildcard needed - we've covered all variants
        }
    }
}

// =============================================================================
// STRUCT FOR COMPUTED STATISTICS
// =============================================================================

/// Text statistics computed from a collection of words.
#[derive(Debug)]
pub struct TextStats {
    pub total_words: usize,
    pub total_chars: usize,
    pub avg_word_length: f64,
    pub longest_word_len: usize,
    pub shortest_word_len: usize,
    pub capitalized_count: usize,
    pub reading_level: ReadingLevel,
}

impl TextStats {
    // -------------------------------------------------------------------------
    // COMPUTING STATS WITH ITERATORS
    // -------------------------------------------------------------------------
    //
    // This method demonstrates many iterator methods from Module 7:
    // - sum(): Adds up all values
    // - max(), min(): Find extreme values
    // - map(): Transform each element
    // - filter(): Keep elements matching condition
    // - count(): Count number of elements
    // -------------------------------------------------------------------------

    pub fn from_words(words: &[Word]) -> TextStats {
        // EARLY RETURN for empty input
        // This is a common pattern to handle edge cases
        if words.is_empty() {
            return TextStats {
                total_words: 0,
                total_chars: 0,
                avg_word_length: 0.0,
                longest_word_len: 0,
                shortest_word_len: 0,
                capitalized_count: 0,
                reading_level: ReadingLevel::Elementary,
            };
        }

        let total_words = words.len();

        // ITERATOR: map() + sum()
        // -----------------------
        // words.iter() - borrows each word
        // .map(|w| w.char_count()) - transforms Word to usize
        // .sum() - adds all the usizes together
        //
        // From Module 7 (Iterators Part 1): let check: u32 = a.iter().sum();
        let total_chars: usize = words.iter().map(|w| w.char_count()).sum();

        // Type casting: usize to f64 for floating-point division
        let avg_word_length = total_chars as f64 / total_words as f64;

        // ITERATOR: max() and min() with unwrap_or()
        // -------------------------------------------
        // max() and min() return Option<T> (None if iterator is empty)
        //
        // unwrap_or(0) is idiomatic shorthand for:
        //   match result {
        //       Some(val) => val,
        //       None => 0,
        //   }
        //
        // We know the iterator isn't empty (checked above), but unwrap_or
        // is safer than unwrap() and documents our default value.
        let longest_word_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let shortest_word_len = words.iter().map(|w| w.len()).min().unwrap_or(0);

        // ITERATOR: filter() + count()
        // ----------------------------
        // filter() takes a closure returning bool
        // count() consumes iterator and counts remaining elements
        let capitalized_count = words.iter().filter(|w| w.is_capitalized()).count();

        let reading_level = ReadingLevel::from_avg_length(avg_word_length);

        // STRUCT INITIALIZATION SHORTHAND
        // When variable name matches field name, we can omit the field name
        TextStats {
            total_words,
            total_chars,
            avg_word_length,
            longest_word_len,
            shortest_word_len,
            capitalized_count,
            reading_level,
        }
    }

    // -------------------------------------------------------------------------
    // MATCH ON SPECIFIC VALUES
    // -------------------------------------------------------------------------

    pub fn summary(&self) -> String {
        match self.total_words {
            0 => String::from("No text to analyze."),
            1 => format!("Single word text: {} characters.", self.total_chars),
            // `n` binds the matched value for use in the expression
            n => format!(
                "{} words, avg {:.1} chars/word. {}",
                n,
                self.avg_word_length,
                self.reading_level.description()
            ),
        }
    }
}

// =============================================================================
// IMPLEMENTING A TRAIT FOR A TYPE
// =============================================================================
//
// From Module 6 (Functions within a Trait):
//   impl BasicStats for Data {
//       fn mean(&self) -> f32 { ... }
//       fn variance(&self) -> f32 { ... }
//   }
//
// SYNTAX: impl TraitName for TypeName { ... }
//
// After implementation, TextStats values can be used anywhere that requires
// a Summarizable type (e.g., fn process(item: &impl Summarizable)).
// =============================================================================

impl Summarizable for TextStats {
    // Provide implementation for required method
    fn summarize(&self) -> String {
        format!(
            "Text: {} words, {} chars, {:.1} avg len",
            self.total_words, self.total_chars, self.avg_word_length
        )
    }

    // Provide implementation for required method
    fn item_count(&self) -> usize {
        self.total_words
    }

    // We don't override brief() - we get the default implementation for free!
}

// =============================================================================
// FUNCTIONS WITH CLOSURE PARAMETERS
// =============================================================================
//
// From Module 7 (Closures Part 1):
//   fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) { ... }
//
// These functions demonstrate higher-order programming:
// - Take closures as parameters
// - Apply those closures to data
// - Return computed results
//
// This pattern enables flexible, reusable code.
// =============================================================================

/// Check if any word matches a predicate.
///
/// ITERATOR METHOD: any()
/// Returns true if ANY element satisfies the predicate.
/// Short-circuits: stops as soon as it finds a match.
///
/// From Module 7 (Iterators Part 1):
///   let check = a.iter().any(|&x| x > 0);
pub fn any_matches<F>(words: &[Word], predicate: F) -> bool
where
    F: Fn(&Word) -> bool,
{
    words.iter().any(|w| predicate(w))
}

/// Check if all words match a predicate.
///
/// ITERATOR METHOD: all()
/// Returns true if ALL elements satisfy the predicate.
/// Short-circuits: stops as soon as it finds a non-match.
///
/// From Module 7 (Iterators Part 1):
///   let check = a.iter().all(|&x| x > 0);
pub fn all_match<F>(words: &[Word], predicate: F) -> bool
where
    F: Fn(&Word) -> bool,
{
    words.iter().all(|w| predicate(w))
}

/// Find the first word matching a predicate and return its position.
///
/// ITERATOR METHOD: position()
/// Returns Some(index) of first matching element, or None.
///
/// From Module 7 (Iterators Part 1):
///   let check = a.iter().position(|&x| x > 4);
pub fn find_position<F>(words: &[Word], predicate: F) -> Option<usize>
where
    F: Fn(&Word) -> bool,
{
    words.iter().position(|w| predicate(w))
}

/// Collect words matching a predicate into a new Vec.
///
/// ITERATOR CHAIN: filter() + collect()
/// - filter() creates an iterator that only yields matching elements
/// - collect() consumes iterator into a collection
///
/// LIFETIME ANNOTATIONS:
/// The returned references have the same lifetime as the input slice.
pub fn filter_words<'a, F>(words: &'a [Word<'a>], predicate: F) -> Vec<&'a Word<'a>>
where
    F: Fn(&Word) -> bool,
{
    words.iter().filter(|w| predicate(w)).collect()
}

/// Transform word texts using a closure, collecting results.
///
/// ITERATOR CHAIN: map() + collect()
/// - map() transforms each element using the closure
/// - collect() gathers results into a Vec<String>
///
/// The closure takes &str and returns String (owned data).
pub fn transform_texts<F>(words: &[Word], transformer: F) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    words.iter().map(|w| transformer(w.text)).collect()
}

/// Partition words into two groups based on a predicate.
///
/// ITERATOR METHOD: partition()
/// Splits elements into two collections:
/// - First: elements where predicate returns true
/// - Second: elements where predicate returns false
///
/// Returns a tuple (matching, non_matching).
pub fn partition_words<'a, F>(
    words: &'a [Word<'a>],
    predicate: F,
) -> (Vec<&'a Word<'a>>, Vec<&'a Word<'a>>)
where
    F: Fn(&Word) -> bool,
{
    words.iter().partition(|w| predicate(w))
}

// =============================================================================
// LET-ELSE PATTERN
// =============================================================================
//
// The `let ... else { return }` pattern is a concise way to:
// 1. Try to destructure/match a value
// 2. If it fails, execute the else block (which must diverge: return, break, etc.)
//
// This is cleaner than:
//   let max_len = match words.iter().map(|w| w.len()).max() {
//       Some(len) => len,
//       None => return Vec::new(),
//   };
// =============================================================================

/// Calculate word frequency by length, returns (length, count) pairs.
pub fn length_distribution(words: &[Word]) -> Vec<(usize, usize)> {
    // LET-ELSE PATTERN
    // If max() returns None (empty iterator), execute the else block
    let Some(max_len) = words.iter().map(|w| w.len()).max() else {
        return Vec::new();
    };

    // RANGE ITERATOR + ITERATOR CHAIN
    // (1..=max_len) creates an iterator from 1 to max_len (inclusive)
    // .map() transforms each length to a (length, count) tuple
    // .filter() keeps only tuples where count > 0
    // .collect() gathers into a Vec
    (1..=max_len)
        .map(|len| {
            let count = words.iter().filter(|w| w.len() == len).count();
            (len, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect()
}

// =============================================================================
// FOLD PATTERN
// =============================================================================
//
// fold() is the most general iterator method - all other methods can be
// implemented using fold.
//
// SIGNATURE: fold(initial_value, |accumulator, element| new_accumulator)
//
// It works by:
// 1. Starting with initial_value as the accumulator
// 2. For each element, calling the closure with (accumulator, element)
// 3. Using the closure's return value as the new accumulator
// 4. Returning the final accumulator
// =============================================================================

/// Fold words into a single value using an accumulator.
///
/// GENERIC PARAMETERS:
/// - T: Type of the accumulator (and final result)
/// - F: Closure type that takes (T, &Word) and returns T
///
/// EXAMPLE USAGE:
///   let total_len = fold_words(&words, 0, |acc, w| acc + w.len());
///   let all_texts = fold_words(&words, String::new(), |mut acc, w| {
///       acc.push_str(w.text);
///       acc
///   });
pub fn fold_words<T, F>(words: &[Word], init: T, folder: F) -> T
where
    F: Fn(T, &Word) -> T,
{
    // iter() creates an iterator over references
    // fold() accumulates a result
    words.iter().fold(init, |acc, w| folder(acc, w))
}
