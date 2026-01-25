mod analyzer;
mod error;
mod frequency;
mod stats;
mod word;

use analyzer::{
    bracketed_format, format_with_all, handle_analysis_result, simple_format, verbose_format,
    TextAnalyzer,
};
use frequency::{frequency_distribution, WordFrequency};
use stats::{
    any_matches, count_where, filter_words, find_max, fold_words, partition_words, Summarizable,
    TextStats,
};
use word::{extract_words, find_longest, find_word_by_text, try_extract_words, try_find_word};

fn main() {
    let sample_text = "Rust is a systems programming language.
It provides memory safety without garbage collection.
The ownership system ensures safe concurrency.
Many developers find Rust both challenging and rewarding.";

    println!("=== Text Analytics Tool ===\n");
    println!("Sample text:\n{}\n", sample_text);

    // =========================================================================
    // LIFETIMES: Words borrow from sample_text (Module 7)
    // =========================================================================
    println!("--- Word Extraction (Lifetimes) ---");
    let words = extract_words(sample_text);
    println!("Extracted {} words from the text.", words.len());

    if let Some(first) = words.first() {
        println!(
            "First word '{}' at position {} on line {}, categorized as: {}",
            first.text,
            first.position,
            first.line,
            first.length_category()
        );
        debug_assert!(!first.is_empty(), "Words should never be empty");
    }
    println!();

    // =========================================================================
    // TRAITS: Polymorphism - same interface, different types (Module 6)
    // =========================================================================
    println!("--- Traits (Polymorphism) ---");
    let stats = TextStats::from_words(&words);
    let freq = WordFrequency::from_words(&words);

    // Both types implement Summarizable trait
    println!("TextStats: {}", stats.summarize());
    println!("WordFrequency: {}", freq.summarize());

    // Default trait method
    println!("Stats brief: {}", stats.brief());

    // Function accepting any Summarizable (impl Trait syntax)
    fn print_summary(item: &impl Summarizable) {
        println!("  Summary: {} (count: {})", item.summarize(), item.item_count());
    }
    print_summary(&stats);
    print_summary(&freq);
    println!();

    // =========================================================================
    // GENERICS: Type-agnostic functions with trait bounds (Module 6)
    // =========================================================================
    println!("--- Generics ---");

    // find_max<T: Ord> works with any ordered type
    let word_lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    let max_len = find_max(word_lengths.into_iter());
    println!("Max word length: {:?}", max_len);

    // count_where<T, P: Fn(&T) -> bool> works with any iterator and predicate
    let long_word_count = count_where(words.iter(), |w| w.len() > 6);
    println!("Words longer than 6 chars: {}", long_word_count);
    println!();

    // =========================================================================
    // FUNCTION TYPES: Functions as first-class values (Module 7)
    // =========================================================================
    println!("--- Function Types ---");
    let analyzer = TextAnalyzer::with_simple_format();
    let report = analyzer.analyze(sample_text);
    println!("{}", report);
    println!();

    // Array of function pointers - same signature, different behavior
    let formatters = [simple_format, verbose_format, bracketed_format];
    println!("Same data, different formatters:");
    for line in format_with_all("Word count", &stats.total_words.to_string(), &formatters) {
        println!("  {}", line);
    }
    println!();

    // =========================================================================
    // RESULT & ERROR HANDLING: Explicit error propagation (Ch 9)
    // =========================================================================
    println!("--- Error Handling (Result) ---");

    // Success case
    match try_extract_words(sample_text) {
        Ok(w) => println!("Success: extracted {} words", w.len()),
        Err(e) => println!("Error: {}", e),
    }

    // Error cases with exhaustive matching
    handle_analysis_result(analyzer.try_analyze(""));
    handle_analysis_result(analyzer.try_analyze("!!!"));

    // Word search with Result
    match try_find_word(&words, "rust") {
        Ok(word) => println!("Found 'rust' on line {}", word.line),
        Err(e) => println!("{}", e),
    }
    println!();

    // =========================================================================
    // HASHMAP: Word frequency with Entry API (Ch 8)
    // =========================================================================
    println!("--- HashMap (Word Frequency) ---");
    println!("Unique: {}, Total: {}", freq.unique_count(), freq.total_occurrences());
    println!("Frequency of 'rust': {:?}", freq.get("rust"));

    println!("Top 5 words:");
    for (word, count) in freq.top_n(5) {
        println!("  '{}': {}", word, count);
    }

    // Frequency distribution using Entry API
    let dist = frequency_distribution(&freq);
    println!("Distribution: {:?}", dist);
    println!();

    // =========================================================================
    // MATCH EXPRESSIONS: Pattern matching with guards (Ch 6)
    // =========================================================================
    println!("--- Match Expressions ---");

    // Match on Option with method call
    match find_longest(&words) {
        Some(word) => println!("Longest: '{}' ({} chars)", word.text, word.len()),
        None => println!("No words found"),
    }

    // Match with guard clause
    let search_term = "programming";
    match find_word_by_text(&words, search_term) {
        Some(w) if w.line == 1 => println!("'{}' found on first line!", search_term),
        Some(w) => println!("'{}' found on line {}", search_term, w.line),
        None => println!("'{}' not found", search_term),
    }
    println!();

    // =========================================================================
    // CLOSURES: Three capture modes - Fn, FnMut, FnOnce (Module 7)
    // =========================================================================
    println!("--- Closures ---");

    // Fn: immutable borrow of captured variable
    let threshold = 7;
    let count_long = |words: &[word::Word]| -> usize {
        words.iter().filter(|w| w.len() >= threshold).count()
    };
    println!("Fn (immutable borrow): {} words >= {} chars", count_long(&words), threshold);

    // FnMut: mutable borrow of captured variable
    let mut running_total = 0;
    let mut accumulator = |len: usize| {
        running_total += len;
    };
    for w in words.iter().take(5) {
        accumulator(w.len());
    }
    println!("FnMut (mutable borrow): first 5 word lengths sum = {}", running_total);

    // FnOnce (move): takes ownership of captured variable
    let keywords = vec!["rust", "memory", "safe", "ownership"];
    let is_keyword = move |word: &word::Word| -> bool {
        keywords.iter().any(|k| word.text.eq_ignore_ascii_case(k))
    };
    let keyword_count = words.iter().filter(|w| is_keyword(w)).count();
    println!("FnOnce (move): {} words match keywords", keyword_count);
    // keywords is no longer accessible here
    println!();

    // =========================================================================
    // ITERATORS: Lazy, chainable data transformations (Module 7)
    // =========================================================================
    println!("--- Iterators ---");

    // any() and filter()
    let has_long = any_matches(&words, |w| w.len() > 10);
    println!("Has word > 10 chars: {}", has_long);

    // partition() - split by predicate
    let (caps, lower) = partition_words(&words, |w| w.is_capitalized());
    println!("Capitalized: {}, Lowercase: {}", caps.len(), lower.len());

    // filter() with complex predicate
    let medium = filter_words(&words, |w| w.len() >= 4 && w.len() <= 7);
    println!("Medium words (4-7 chars): {}", medium.iter().map(|w| w.text).collect::<Vec<_>>().join(", "));

    // fold() - accumulate into single value
    let joined = fold_words(&words, String::new(), |mut acc, w| {
        if !acc.is_empty() { acc.push('-'); }
        acc.push_str(w.text);
        acc
    });
    println!("Joined: {}...", &joined[..50.min(joined.len())]);

    // Chained iterator: filter -> map -> sum
    let long_chars: usize = words.iter()
        .filter(|w| w.len() > 5)
        .map(|w| w.char_count())
        .sum();
    println!("Total chars in words > 5 letters: {}", long_chars);
    println!();

    // =========================================================================
    // MUTABLE ITERATION: iter_mut() for in-place modification
    // =========================================================================
    println!("--- Mutable Iteration (iter_mut) ---");

    // Collect word lengths and normalize to scores (0-100 scale)
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(1);
    let mut scores: Vec<u8> = words.iter()
        .map(|w| ((w.len() * 100) / max_len) as u8)
        .collect();

    println!("Raw scores (first 5): {:?}", &scores[..5.min(scores.len())]);

    // iter_mut: modify scores in place - apply minimum threshold of 25
    for score in scores.iter_mut() {
        if *score < 25 {
            *score = 25;
        }
    }
    println!("After min threshold: {:?}", &scores[..5.min(scores.len())]);

    // iter_mut with enumerate: boost scores for capitalized words
    for (i, score) in scores.iter_mut().enumerate() {
        if words[i].is_capitalized() {
            *score = (*score).saturating_add(10); // Cap at 255
        }
    }
    println!("After cap bonus:     {:?}", &scores[..5.min(scores.len())]);
}
