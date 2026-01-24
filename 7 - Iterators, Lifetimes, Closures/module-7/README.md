# Module 7: Text Analytics Tool

A cohesive Rust example demonstrating **Iterators**, **Lifetimes**, **Closures**, and concepts from Modules 2-6.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              main.rs                                     │
│                    (Demonstration of all concepts)                       │
└───────────────┬─────────────────────────────────────────────────────────┘
                │ uses
                ▼
┌───────────────┴───────────────┬───────────────────┬─────────────────────┐
│         word.rs               │     stats.rs      │    frequency.rs     │
│  • Word<'a> struct            │  • Summarizable   │  • WordFrequency    │
│  • Lifetimes                  │    trait          │  • HashMap          │
│  • extract_words()            │  • TextStats      │  • Entry API        │
│  • try_extract_words()        │  • Generics       │  • Trait impl       │
└───────────────┬───────────────┴─────────┬─────────┴─────────────────────┘
                │                         │
                │ uses                    │ uses
                ▼                         ▼
        ┌───────────────┐         ┌───────────────┐
        │   error.rs    │         │  analyzer.rs  │
        │ • AnalysisError│        │ • Function    │
        │ • Result type │         │   types       │
        │ • Display     │         │ • ? operator  │
        └───────────────┘         └───────────────┘
```

---

## Step-by-Step Flow

### Step 1: Error Module (`error.rs`)

**Purpose**: Foundation for error handling using enums and Result type.

```rust
// Define what can go wrong
enum AnalysisError {
    EmptyInput,           // No text provided
    NoWordsFound,         // Text has no valid words
    WordNotFound(String), // Searched word doesn't exist
}

// Make it printable (Display trait)
impl Display for AnalysisError { ... }

// Type alias for convenience
type AnalysisResult<T> = Result<T, AnalysisError>;
```

**Concepts Covered**:
- Enums with data (Module 6)
- Result enum (Module 6, Ch 9)
- Traits: Display implementation
- Type aliases

---

### Step 2: Word Module (`word.rs`)

**Purpose**: Core data structure that borrows from source text using lifetimes.

```rust
// Struct with lifetime - borrows from source text
struct Word<'a> {
    text: &'a str,    // Borrowed, not owned (memory efficient)
    position: usize,
    line: usize,
}

// Extract words - returns Vec<Word> that borrow from input
fn extract_words<'a>(text: &'a str) -> Vec<Word<'a>> {
    // Words point INTO the original text (zero-copy)
}
```

**Data Flow**:
```
"Rust is safe" (original text in memory)
       │
       ▼
┌──────────────────────────────────────────┐
│  Word { text: ───► "Rust", ... }         │
│  Word { text: ───► "is", ... }           │  All borrow from original
│  Word { text: ───► "safe", ... }         │
└──────────────────────────────────────────┘
```

**Concepts Covered**:
- Lifetimes in structs (Module 7 - Lifetimes Part 1 & 2)
- Lifetime elision rules (Module 7 - Eliding Lifetimes)
- Multiple lifetime parameters
- Borrowing vs Owning (Module 3)
- Option combinators (`map_or`)
- Match with ranges

---

### Step 3: Stats Module (`stats.rs`)

**Purpose**: Demonstrates traits, generics, and comprehensive iterator usage.

```rust
// TRAIT DEFINITION with default implementation
trait Summarizable {
    fn summarize(&self) -> String;      // Required
    fn item_count(&self) -> usize;      // Required
    fn brief(&self) -> String { ... }   // Default implementation
}

// GENERIC FUNCTION with trait bounds
fn find_max<T: Ord, I: Iterator<Item = T>>(iter: I) -> Option<T> {
    iter.max()
}

// COMPUTING STATS with iterator chains
fn from_words(words: &[Word]) -> TextStats {
    let total_chars = words.iter()
        .map(|w| w.char_count())
        .sum();

    let longest = words.iter()
        .map(|w| w.len())
        .max()
        .unwrap_or(0);
}
```

**Iterator Chain Visualization**:
```
words.iter()              // Iterator over &Word
    │
    ▼
.map(|w| w.len())         // Transform to Iterator over usize
    │
    ▼
.filter(|&l| l > 5)       // Keep only lengths > 5
    │
    ▼
.sum()                    // Add them all up → single usize
```

**Concepts Covered**:
- Traits with default methods (Module 6 - Traits)
- Generics with trait bounds (Module 6 - Generics)
- Where clauses
- Iterator methods: `sum()`, `max()`, `min()`, `filter()`, `count()`, `any()`, `all()`, `position()`, `partition()`, `fold()`
- Closures as function parameters (Module 7 - Closures Part 1)
- `unwrap_or()` for Option handling
- Let-else pattern

---

### Step 4: Frequency Module (`frequency.rs`)

**Purpose**: HashMap-based word frequency analysis with the Entry API.

```rust
struct WordFrequency {
    counts: HashMap<String, usize>,
}

// THE ENTRY API PATTERN
fn from_words(words: &[Word]) -> WordFrequency {
    let mut counts = HashMap::new();
    for word in words {
        // Entry API: insert 0 if not present, then increment
        *counts.entry(word.text.to_lowercase()).or_insert(0) += 1;
    }
    WordFrequency { counts }
}
```

**Entry API Flow**:
```
counts.entry("rust")     // Get Entry enum (Occupied or Vacant)
       │
       ├── Vacant ──► .or_insert(0) inserts 0, returns &mut 0
       │
       └── Occupied ──► .or_insert(0) does nothing, returns &mut existing

*... += 1                // Dereference and increment
```

**Concepts Covered**:
- HashMap creation and usage (Module 6 - Hash Maps, Ch 8)
- Entry API for insert-or-update pattern
- HashMap iteration
- `impl Trait` return type
- Sorting with closures (`sort_by`)
- Trait polymorphism (same trait for different types)

---

### Step 5: Analyzer Module (`analyzer.rs`)

**Purpose**: Function types as first-class values and error propagation.

```rust
// FUNCTION TYPE ALIAS
type Formatter = fn(&str, &str) -> String;

// STRUCT STORING A FUNCTION
struct TextAnalyzer {
    formatter: Formatter,  // Function pointer stored in struct
}

// CALLING STORED FUNCTION
fn format_line(&self, label: &str, value: &str) -> String {
    (self.formatter)(label, value)  // Parentheses needed!
}

// THE ? OPERATOR FOR ERROR PROPAGATION
fn try_analyze(&self, text: &str) -> AnalysisResult<AnalysisReport> {
    let words = try_extract_words(text)?;  // Returns early on Err
    // ... continues on Ok
}
```

**? Operator Flow**:
```
try_extract_words(text)?
         │
         ├── Err(e) ──────► return Err(e) immediately
         │
         └── Ok(words) ───► extract words value, continue execution
```

**Concepts Covered**:
- Function types (Module 7 - Function types)
- Type aliases for function pointers
- Functions as first-class values
- The ? operator (Ch 9)
- Display trait implementation
- DRY principle (extracting common code)

---

### Step 6: Main Module (`main.rs`)

**Purpose**: Demonstrates all concepts working together.

#### 6.1 Lifetimes in Action

```rust
let sample_text = "Rust is...";           // Text lives here
let words = extract_words(sample_text);   // Words borrow from sample_text
// words are valid as long as sample_text is valid
```

#### 6.2 Trait Polymorphism

```rust
// Both types implement Summarizable
let stats = TextStats::from_words(&words);
let freq = WordFrequency::from_words(&words);

// Same function works with both!
fn print_summary(item: &impl Summarizable) {
    println!("{}", item.summarize());
}
print_summary(&stats);  // Works!
print_summary(&freq);   // Also works!
```

#### 6.3 Three Closure Capture Modes

```rust
// Fn (immutable borrow) - can be called multiple times
let threshold = 7;
let count_long = |words: &[Word]| {
    words.iter().filter(|w| w.len() >= threshold).count()
};

// FnMut (mutable borrow) - modifies captured variable
let mut total = 0;
let mut accumulator = |len: usize| {
    total += len;  // Mutable capture
};

// FnOnce (move) - takes ownership of captured value
let keywords = vec!["rust", "safe"];
let check = move |s: &str| {
    keywords.iter().any(|k| s.contains(k))  // keywords moved in
};
// keywords is no longer accessible here
```

#### 6.4 Mutable Iteration

```rust
// iter_mut() allows modifying elements in place
let mut values = vec![1, 2, 3, 4, 5];
for val in values.iter_mut() {
    *val *= 2;  // Dereference and modify
}
// values is now [2, 4, 6, 8, 10]
```

---

## Concept Coverage Matrix

| File | Lifetimes | Closures | Iterators | Traits | Generics | HashMap | Result |
|------|:---------:|:--------:|:---------:|:------:|:--------:|:-------:|:------:|
| error.rs | | | | ✓ | | | ✓ |
| word.rs | ✓ | ✓ | ✓ | | | | ✓ |
| stats.rs | ✓ | ✓ | ✓ | ✓ | ✓ | | |
| frequency.rs | | ✓ | ✓ | ✓ | | ✓ | |
| analyzer.rs | | | ✓ | ✓ | | | ✓ |
| main.rs | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

## Learning Materials Reference

This example integrates concepts from:

| Module | Topics |
|--------|--------|
| Module 2 | Variables, Vectors, Functions |
| Module 3 | Ownership, References, Borrowing, Dereferencing |
| Module 4 | Match statements, Range patterns, Guards |
| Module 6 | Structs, Traits, Generics, Enums, Option, Result, HashMap |
| Module 7 | Lifetimes, Closures (Fn/FnMut/move), Function types, Iterators |
| Ch 8 | HashMap Entry API |
| Ch 9 | Error handling, ? operator |

---

## Running the Example

```bash
cd module-7
cargo run
```

**Expected Output**:
```
=== Text Analytics Tool ===

Sample text:
Rust is a systems programming language...

--- Word Extraction (Lifetimes) ---
Extracted 27 words from the text.
First word 'Rust' at position 0 on line 1, categorized as: medium

--- Traits (Polymorphism) ---
TextStats: Text: 27 words, 168 chars, 6.2 avg len
WordFrequency: Frequency: 26 unique words, 27 total occurrences
...
```

---

## Key Takeaways

1. **Lifetimes** prevent dangling references at compile time
2. **Traits** enable polymorphism and code reuse
3. **Generics** allow writing flexible, type-safe code
4. **Iterators** provide functional-style data processing
5. **Closures** capture their environment in three ways (Fn, FnMut, FnOnce)
6. **Result/Option** make error handling explicit and safe
7. **HashMap Entry API** provides efficient insert-or-update operations
