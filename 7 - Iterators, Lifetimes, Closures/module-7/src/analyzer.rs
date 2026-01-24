// =============================================================================
// ANALYZER.RS - Function Types and Error Propagation
// =============================================================================
//
// CONCEPTS DEMONSTRATED:
// ----------------------
// 1. FUNCTION TYPES (Module 7 - Function types)
//    - Type aliases for function pointers
//    - Functions as first-class values
//    - Passing functions to other functions
//
// 2. RESULT & ERROR HANDLING (Module 6 - Result Enum, Ch 9)
//    - The ? operator for error propagation
//    - Match on Result variants
//    - Exhaustive error handling
//
// 3. TRAITS (Module 6 - Traits)
//    - Implementing std::fmt::Display
//
// 4. DRY PRINCIPLE
//    - Extracting common code to helper methods
//
// =============================================================================

use std::fmt;

use crate::error::{AnalysisError, AnalysisResult};
use crate::stats::TextStats;
use crate::word::{extract_words, try_extract_words};

// =============================================================================
// FUNCTION TYPE ALIAS
// =============================================================================
//
// From Module 7 (Function types):
//   fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
//       f(some_one);
//       println!(" and my age is {}", age);
//   }
//
// FUNCTION POINTERS:
// - `fn(args) -> return` is the type of a function pointer
// - Unlike closures, function pointers don't capture environment
// - They have a fixed size and can be stored in structs
//
// TYPE ALIAS:
// Creates a name for a type to improve readability.
// `Formatter` is easier to read than `fn(&str, &str) -> String`
//
// FUNCTION TYPES VS CLOSURES:
// - fn(T) -> U : function pointer, no captured state
// - Fn(T) -> U : closure trait, may capture immutably
// - FnMut(T) -> U : closure trait, may capture mutably
// - FnOnce(T) -> U : closure trait, may consume captured
//
// We use `fn` here because our formatters don't need to capture anything.
// =============================================================================

/// Function type for formatting output.
/// Takes a label and value, returns formatted string.
pub type Formatter = fn(&str, &str) -> String;

// =============================================================================
// FORMATTER FUNCTIONS
// =============================================================================
//
// These are regular functions that match the Formatter type signature.
// They can be passed as values, stored in structs, and called dynamically.
//
// From Module 7 (Function types):
//   fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//       f(arg) + f(arg)
//   }
// =============================================================================

/// Simple formatter: "label: value"
pub fn simple_format(label: &str, value: &str) -> String {
    format!("{}: {}", label, value)
}

/// Verbose formatter: "The label is value."
pub fn verbose_format(label: &str, value: &str) -> String {
    format!("The {} is {}.", label, value)
}

/// Bracketed formatter: "[LABEL] value"
pub fn bracketed_format(label: &str, value: &str) -> String {
    format!("[{}] {}", label.to_uppercase(), value)
}

// =============================================================================
// STRUCT WITH FUNCTION FIELD
// =============================================================================
//
// Storing a function in a struct allows for configurable behavior.
// This is a form of the "Strategy Pattern" from OOP.
//
// BENEFITS:
// - Configuration at construction time
// - Same struct, different behaviors
// - Easy to test with mock formatters
// =============================================================================

/// Text analyzer that processes text and produces reports.
/// Uses function types for customizable formatting.
pub struct TextAnalyzer {
    // FUNCTION STORED IN STRUCT:
    // The formatter field holds a function pointer.
    // Different TextAnalyzer instances can have different formatters.
    formatter: Formatter,
}

impl TextAnalyzer {
    // -------------------------------------------------------------------------
    // CONSTRUCTOR TAKING FUNCTION
    // -------------------------------------------------------------------------
    //
    // From Module 7 (Function types):
    //   let mut f = max;  // assign function to variable
    //   println!("Result: {}", f(2, 3));  // call through variable
    // -------------------------------------------------------------------------

    pub fn new(formatter: Formatter) -> TextAnalyzer {
        TextAnalyzer { formatter }
    }

    /// Convenience constructor with simple formatting.
    /// Demonstrates passing a function as a value.
    pub fn with_simple_format() -> TextAnalyzer {
        // FUNCTION AS VALUE:
        // `simple_format` (without parentheses) is the function itself, not a call.
        // This passes the function to new(), which stores it in the struct.
        TextAnalyzer::new(simple_format)
    }

    // -------------------------------------------------------------------------
    // CALLING STORED FUNCTION
    // -------------------------------------------------------------------------
    //
    // To call a function stored in a field, we use (self.field)(args).
    // The outer parentheses are needed because of parsing rules:
    // - self.formatter(x) would look for a method named formatter
    // - (self.formatter)(x) correctly accesses the field and calls it
    // -------------------------------------------------------------------------

    fn format_line(&self, label: &str, value: &str) -> String {
        // CALLING A STORED FUNCTION:
        // (self.formatter) accesses the function
        // (label, value) passes the arguments
        (self.formatter)(label, value)
    }

    // -------------------------------------------------------------------------
    // DRY PRINCIPLE - EXTRACTING COMMON CODE
    // -------------------------------------------------------------------------
    //
    // Both analyze() and try_analyze() need to build a report from stats.
    // Instead of duplicating this code, we extract it to a helper method.
    //
    // This is a key software engineering principle:
    // Don't Repeat Yourself (DRY)
    // -------------------------------------------------------------------------

    fn build_report(&self, stats: &TextStats) -> AnalysisReport {
        let lines = vec![
            self.format_line("Total words", &stats.total_words.to_string()),
            self.format_line("Total characters", &stats.total_chars.to_string()),
            self.format_line(
                "Average word length",
                &format!("{:.2}", stats.avg_word_length),
            ),
            self.format_line("Longest word", &stats.longest_word_len.to_string()),
            self.format_line("Shortest word", &stats.shortest_word_len.to_string()),
            self.format_line("Capitalized words", &stats.capitalized_count.to_string()),
            // {:?} uses Debug formatting for the enum
            self.format_line("Reading level", &format!("{:?}", stats.reading_level)),
        ];
        AnalysisReport { lines }
    }

    /// Analyze text and produce a formatted report.
    /// This version never fails (returns AnalysisReport directly).
    pub fn analyze(&self, text: &str) -> AnalysisReport {
        let words = extract_words(text);
        let stats = TextStats::from_words(&words);
        self.build_report(&stats)
    }

    // -------------------------------------------------------------------------
    // THE ? OPERATOR FOR ERROR PROPAGATION
    // -------------------------------------------------------------------------
    //
    // From Module 6 (Result Enum) / Ch 9:
    //
    // The ? operator is syntactic sugar for error propagation:
    //
    // LONG FORM:
    //   let words = match try_extract_words(text) {
    //       Ok(w) => w,
    //       Err(e) => return Err(e),
    //   };
    //
    // SHORT FORM (using ?):
    //   let words = try_extract_words(text)?;
    //
    // HOW ? WORKS:
    // 1. If the Result is Ok(value), extract value and continue
    // 2. If the Result is Err(e), immediately return Err(e) from this function
    //
    // REQUIREMENTS:
    // - Function must return a Result (or Option)
    // - Error types must be compatible (via From trait)
    // -------------------------------------------------------------------------

    pub fn try_analyze(&self, text: &str) -> AnalysisResult<AnalysisReport> {
        // THE ? OPERATOR:
        // If try_extract_words returns Err, this function returns that Err immediately.
        // If it returns Ok(words), we get the words and continue.
        let words = try_extract_words(text)?;

        let stats = TextStats::from_words(&words);

        // Wrap successful result in Ok
        Ok(self.build_report(&stats))
    }
}

// =============================================================================
// REPORT STRUCT
// =============================================================================

/// Result of text analysis containing formatted lines.
pub struct AnalysisReport {
    pub lines: Vec<String>,
}

// =============================================================================
// IMPLEMENTING DISPLAY TRAIT
// =============================================================================
//
// std::fmt::Display provides human-readable string representation.
// Implementing Display allows using {} in println! and format!.
//
// This is more idiomatic than having a custom print() method.
// Why? Because Display integrates with Rust's formatting infrastructure:
// - Works with format!(), println!(), write!()
// - Can be used in string interpolation
// - Follows Rust conventions
// =============================================================================

impl fmt::Display for AnalysisReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ENUMERATE for index tracking:
        // enumerate() yields (index, value) pairs
        for (i, line) in self.lines.iter().enumerate() {
            // Don't print newline before first line
            if i > 0 {
                // writeln! writes to the formatter with a newline
                // ? propagates any formatting error
                writeln!(f)?;
            }
            // write! writes to the formatter without a newline
            write!(f, "{}", line)?;
        }
        // Return Ok(()) to indicate successful formatting
        Ok(())
    }
}

// =============================================================================
// FUNCTIONS IN COLLECTIONS
// =============================================================================
//
// Function pointers can be stored in arrays and slices.
// This allows iterating over multiple functions and applying them.
// =============================================================================

/// Apply multiple formatters to the same data.
///
/// PARAMETER: &[Formatter]
/// This is a slice of function pointers.
/// We can pass an array: &[simple_format, verbose_format]
///
/// RETURNS: Vec<String>
/// One formatted string per formatter.
pub fn format_with_all(label: &str, value: &str, formatters: &[Formatter]) -> Vec<String> {
    // ITERATOR OVER FUNCTIONS:
    // iter() yields &Formatter (references to function pointers)
    // map(|f| f(...)) calls each function
    // collect() gathers results into Vec
    formatters.iter().map(|f| f(label, value)).collect()
}

// =============================================================================
// EXHAUSTIVE ERROR HANDLING
// =============================================================================
//
// When matching on Result<T, E>, we should handle all error variants.
// This provides informative error messages for each case.
//
// From Module 6 (Result Enum):
//   match result {
//       Ok(value) => { /* use value */ },
//       Err(e) => { /* handle error */ },
//   }
//
// ENUM VARIANT PATTERNS:
// We can match on specific enum variants for different handling:
// - Err(AnalysisError::EmptyInput) - specific variant
// - Err(e) - catch-all for other errors
// =============================================================================

/// Handle analysis result with match expression.
pub fn handle_analysis_result(result: AnalysisResult<AnalysisReport>) {
    match result {
        // SUCCESS CASE:
        // Extract the report and print it using Display trait
        Ok(report) => {
            println!("Analysis successful:");
            // Uses the Display impl we defined above
            println!("{}", report);
        }

        // SPECIFIC ERROR CASES:
        // Match on specific enum variants for custom messages
        Err(AnalysisError::EmptyInput) => {
            println!("Error: Cannot analyze empty text.");
        }

        Err(AnalysisError::NoWordsFound) => {
            println!("Error: Text contains no valid words.");
        }

        // CATCH-ALL ERROR CASE:
        // For any other error, use the Display impl of the error
        // This handles new error variants we might add later
        Err(e) => {
            // e implements Display, so {} works
            println!("Error: {}", e);
        }
    }
}
