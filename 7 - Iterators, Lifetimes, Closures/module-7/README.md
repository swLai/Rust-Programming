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

## Modules Overview

### error.rs - Error Handling Foundation

Defines custom error types and Result alias for the text analytics domain.

```rust
enum AnalysisError {
    EmptyInput,
    NoWordsFound,
    WordNotFound(String),
}

type AnalysisResult<T> = Result<T, AnalysisError>;
```

**Concepts**: Enums with data, Result type, Display trait, Type aliases

---

### word.rs - Lifetimes and Borrowing

Core `Word<'a>` struct that borrows from source text (zero-copy extraction).

```rust
struct Word<'a> {
    text: &'a str,    // Borrowed from original text
    position: usize,
    line: usize,
}

fn extract_words<'a>(text: &'a str) -> Vec<Word<'a>>
```

**Data Flow**:
```
"Rust is safe" (original text)
       │
       ▼
Word { text: ──► "Rust" }   // All point INTO original
Word { text: ──► "is" }
Word { text: ──► "safe" }
```

**Concepts**: Struct lifetimes, Function lifetime parameters, Multiple lifetimes, Option combinators (`map_or`, `ok_or_else`)

---

### stats.rs - Traits and Generics

Trait definition, generic functions, and iterator-based computation.

```rust
trait Summarizable {
    fn summarize(&self) -> String;      // Required
    fn item_count(&self) -> usize;      // Required
    fn brief(&self) -> String { ... }   // Default
}

fn find_max<T: Ord, I: Iterator<Item = T>>(iter: I) -> Option<T>
fn count_where<T, I, F>(iter: I, predicate: F) -> usize
```

**Concepts**: Traits with defaults, Generics with trait bounds, Where clauses, Closure parameters (Fn trait)

---

### frequency.rs - HashMap and Entry API

Word frequency analysis using HashMap's Entry API for efficient insert-or-update.

```rust
fn from_words(words: &[Word]) -> WordFrequency {
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry(word.text.to_lowercase()).or_insert(0) += 1;
    }
    WordFrequency { counts }
}
```

**Entry API Flow**:
```
counts.entry("rust")
       ├── Vacant ──► or_insert(0) → &mut 0
       └── Occupied ──► or_insert(0) → &mut existing
```

**Concepts**: HashMap, Entry API, `impl Trait` return type, Trait polymorphism

---

### analyzer.rs - Function Types

Functions as first-class values and the `?` operator for error propagation.

```rust
type Formatter = fn(&str, &str) -> String;

struct TextAnalyzer {
    formatter: Formatter,  // Function pointer stored in struct
}

fn try_analyze(&self, text: &str) -> AnalysisResult<AnalysisReport> {
    let words = try_extract_words(text)?;  // Early return on Err
    Ok(self.build_report(&TextStats::from_words(&words)))
}
```

**Concepts**: Function type aliases, Functions as values, `?` operator, Display trait

---

## main.rs Demonstration Sections

### 1. Word Extraction (Lifetimes)
```rust
let words = extract_words(sample_text);  // Words borrow from sample_text
```

### 2. Traits (Polymorphism)
```rust
fn print_summary(item: &impl Summarizable) {
    println!("{}", item.summarize());
}
print_summary(&stats);  // TextStats
print_summary(&freq);   // WordFrequency - same interface!
```

### 3. Generics
```rust
let max_len = find_max(word_lengths.into_iter());
let count = count_where(words.iter(), |w| w.len() > 6);
```

### 4. Function Types
```rust
let formatters = [simple_format, verbose_format, bracketed_format];
for line in format_with_all("Words", "27", &formatters) { ... }
```

### 5. Error Handling (Result)
```rust
match try_extract_words(sample_text) {
    Ok(w) => println!("Success: {} words", w.len()),
    Err(e) => println!("Error: {}", e),
}
```

### 6. HashMap (Word Frequency)
```rust
println!("Frequency of 'rust': {:?}", freq.get("rust"));
for (word, count) in freq.top_n(5) { ... }
```

### 7. Match Expressions
```rust
match find_word_by_text(&words, search_term) {
    Some(w) if w.line == 1 => println!("Found on first line!"),
    Some(w) => println!("Found on line {}", w.line),
    None => println!("Not found"),
}
```

### 8. Closures (Three Capture Modes)
```rust
// Fn: immutable borrow
let threshold = 7;
let count_long = |words: &[Word]| words.iter().filter(|w| w.len() >= threshold).count();

// FnMut: mutable borrow
let mut total = 0;
let mut accumulator = |len: usize| { total += len; };

// FnOnce (move): takes ownership
let keywords = vec!["rust", "memory", "safe"];
let is_keyword = move |word: &Word| keywords.iter().any(|k| word.text.eq_ignore_ascii_case(k));
// keywords no longer accessible here
```

### 9. Iterators
```rust
let has_long = any_matches(&words, |w| w.len() > 10);
let (caps, lower) = partition_words(&words, |w| w.is_capitalized());
let medium = filter_words(&words, |w| w.len() >= 4 && w.len() <= 7);
let joined = fold_words(&words, String::new(), |mut acc, w| { acc.push_str(w.text); acc });

// Chained: filter -> map -> sum
let long_chars: usize = words.iter()
    .filter(|w| w.len() > 5)
    .map(|w| w.char_count())
    .sum();
```

### 10. Mutable Iteration (iter_mut)
```rust
// Convert word lengths to scores (0-100 scale)
let mut scores: Vec<u8> = words.iter()
    .map(|w| ((w.len() * 100) / max_len) as u8)
    .collect();

// iter_mut: apply minimum threshold in place
for score in scores.iter_mut() {
    if *score < 25 { *score = 25; }
}

// iter_mut with enumerate: boost scores for capitalized words
for (i, score) in scores.iter_mut().enumerate() {
    if words[i].is_capitalized() {
        *score = (*score).saturating_add(10);
    }
}
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

| Module | Topics |
|--------|--------|
| Module 2 | Variables, Vectors, Functions |
| Module 3 | Ownership, References, Borrowing, Dereferencing |
| Module 4 | Match statements, Range patterns, Guards |
| Module 6 | Structs, Traits, Generics, Enums, Option, Result, HashMap |
| Module 7 | Lifetimes, Closures (Fn/FnMut/FnOnce), Function types, Iterators |
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

--- Closures ---
Fn (immutable borrow): 13 words >= 7 chars
FnMut (mutable borrow): first 5 word lengths sum = 25
FnOnce (move): 5 words match keywords

--- Mutable Iteration (iter_mut) ---
Raw scores (first 5): [36, 18, 9, 63, 100]
After min threshold: [36, 25, 25, 63, 100]
After cap bonus:     [46, 25, 25, 63, 100]
```

---

## Key Takeaways

1. **Lifetimes** prevent dangling references at compile time
2. **Traits** enable polymorphism and code reuse
3. **Generics** allow writing flexible, type-safe code
4. **Iterators** provide lazy, chainable data transformations
5. **Closures** capture their environment in three ways (Fn, FnMut, FnOnce)
6. **Result/Option** make error handling explicit and safe
7. **HashMap Entry API** provides efficient insert-or-update operations
8. **iter_mut()** enables in-place modification of collection elements
