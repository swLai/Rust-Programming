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
    all_match, any_matches, count_where, filter_words, find_max, find_position, fold_words,
    length_distribution, partition_words, transform_texts, Summarizable, TextStats,
};
use word::{extract_words, find_longest, find_word_by_text, try_extract_words, try_find_word};

fn main() {
    let sample_text = "Rust is a systems programming language.
It provides memory safety without garbage collection.
The ownership system ensures safe concurrency.
Many developers find Rust both challenging and rewarding.";

    println!("=== Text Analytics Tool ===\n");
    println!("Sample text:\n{}\n", sample_text);

    // =========================================
    // LIFETIMES: Words borrow from sample_text
    // =========================================
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
        // Demonstrate is_empty() - companion to len() (Clippy best practice)
        debug_assert!(!first.is_empty(), "Words should never be empty");
    }
    println!();

    // =========================================
    // TRAITS: Polymorphism (Module 6)
    // =========================================
    println!("--- Traits (Polymorphism) ---");
    let stats = TextStats::from_words(&words);
    let freq = WordFrequency::from_words(&words);

    // Both types implement Summarizable trait
    println!("TextStats: {}", stats.summarize());
    println!("WordFrequency: {}", freq.summarize());

    // Use default trait method
    println!("Stats brief: {}", stats.brief());
    println!("Freq brief: {}", freq.brief());

    // Function that accepts any Summarizable
    fn print_summary(item: &impl Summarizable) {
        println!("  -> {} (count: {})", item.summarize(), item.item_count());
    }
    print!("Via trait: ");
    print_summary(&stats);
    print!("Via trait: ");
    print_summary(&freq);
    println!();

    // =========================================
    // GENERICS (Module 6)
    // =========================================
    println!("--- Generics ---");

    // Generic find_max works with any Ord iterator
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let max_num = find_max(numbers.into_iter());
    println!("Max of numbers: {:?}", max_num);

    let word_lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    let max_len = find_max(word_lengths.into_iter());
    println!("Max word length: {:?}", max_len);

    // Generic count_where works with any iterator and predicate
    let even_count = count_where(vec![1, 2, 3, 4, 5, 6].into_iter(), |n| n % 2 == 0);
    println!("Count of even numbers: {}", even_count);
    println!();

    // =========================================
    // FUNCTION TYPES: Pluggable formatters
    // =========================================
    println!("--- Function Types (Formatters) ---");
    let analyzer = TextAnalyzer::with_simple_format();
    let report = analyzer.analyze(sample_text);
    println!("Simple format:");
    println!("{}", report); // Uses Display trait instead of custom print()
    println!();

    let formatters = [simple_format, verbose_format, bracketed_format];
    let formatted = format_with_all("Words", "27", &formatters);
    println!("Same data, different formatters:");
    for line in formatted {
        println!("  {}", line);
    }
    println!();

    // =========================================
    // RESULT & ERROR HANDLING (Ch 9)
    // =========================================
    println!("--- Result-based Error Handling ---");

    match try_extract_words(sample_text) {
        Ok(words) => println!("Successfully extracted {} words", words.len()),
        Err(e) => println!("Error: {}", e),
    }

    let empty_result = analyzer.try_analyze("");
    handle_analysis_result(empty_result);

    let punct_result = analyzer.try_analyze("... !!! ???");
    handle_analysis_result(punct_result);

    match try_find_word(&words, "rust") {
        Ok(word) => println!("Found 'rust' on line {}", word.line),
        Err(e) => println!("Search failed: {}", e),
    }

    match try_find_word(&words, "python") {
        Ok(word) => println!("Found 'python' on line {}", word.line),
        Err(e) => println!("Search failed: {}", e),
    }
    println!();

    // =========================================
    // HASHMAP: Word Frequency (Ch 8)
    // =========================================
    println!("--- HashMap Word Frequency ---");
    println!("Unique words: {}", freq.unique_count());
    println!("Total occurrences: {}", freq.total_occurrences());
    println!("Frequency of 'rust': {:?}", freq.get("rust"));

    println!("\nTop 5 most frequent words:");
    for (word, count) in freq.top_n(5) {
        println!("  '{}': {} times", word, count);
    }

    let dist = frequency_distribution(&freq);
    println!("\nFrequency distribution:");
    let mut dist_sorted: Vec<_> = dist.iter().collect();
    dist_sorted.sort_by_key(|(k, _)| *k);
    for (count, num_words) in dist_sorted {
        println!("  {} occurrence(s): {} words", count, num_words);
    }
    println!();

    // =========================================
    // MATCH EXPRESSIONS (Ch 6)
    // =========================================
    println!("--- Match Expressions ---");
    println!("Reading level: {:?}", stats.reading_level);
    println!("Summary: {}", stats.summary());

    let longest = find_longest(&words);
    match longest {
        Some(word) => println!("Longest word: '{}' ({} chars)", word.text, word.len()),
        None => println!("No words found"),
    }

    let search_term = "programming";
    match find_word_by_text(&words, search_term) {
        Some(w) if w.line == 1 => println!("'{}' found on first line!", search_term),
        Some(w) => println!("'{}' found on line {}", search_term, w.line),
        None => println!("'{}' not found", search_term),
    }
    println!();

    // =========================================
    // CLOSURES: Borrow Modes (Closures Part 2)
    // =========================================
    println!("--- Closures: Borrow Modes ---");

    // 1. Immutable borrow (Fn) - closure borrows by reference
    let threshold = 7;
    let count_long = |words: &[word::Word]| -> usize {
        words.iter().filter(|w| w.len() >= threshold).count()
    };
    println!("Long words (immutable borrow): {}", count_long(&words));

    // 2. Mutable borrow (FnMut) - closure modifies captured variable
    let mut running_total = 0;
    let mut accumulator = |len: usize| {
        running_total += len; // Mutable capture
    };
    for w in words.iter().take(5) {
        accumulator(w.len());
    }
    println!("Running total of first 5 word lengths: {}", running_total);

    // 3. Mutable closure with iter_mut() - modifying data in place
    let mut scores = vec![80, 90, 75, 85, 95];
    println!("Original scores: {:?}", scores);
    scores.iter_mut().for_each(|s| *s += 5); // Add 5 to each score
    println!("After +5 bonus: {:?}", scores);

    // 4. Move closure - takes ownership of captured value
    let keywords = vec!["rust", "memory", "safe"];
    let contains_keyword = move |text: &str| -> bool {
        // keywords is moved into the closure
        keywords.iter().any(|k| text.to_lowercase().contains(k))
    };
    println!(
        "'Rust is safe' contains keyword: {}",
        contains_keyword("Rust is safe")
    );
    println!(
        "'Python is dynamic' contains keyword: {}",
        contains_keyword("Python is dynamic")
    );
    // keywords is no longer accessible here (moved into closure)
    println!();

    // =========================================
    // CLOSURES: Various Patterns
    // =========================================
    println!("--- Closures: Iterator Patterns ---");

    let has_very_long = any_matches(&words, |w| w.len() > 10);
    let all_short = all_match(&words, |w| w.len() <= 15);
    println!("Has word > 10 chars: {}", has_very_long);
    println!("All words <= 15 chars: {}", all_short);

    if let Some(pos) = find_position(&words, |w| w.text.starts_with('s')) {
        println!("First word starting with 's' at position: {}", pos);
    }
    println!();

    // =========================================
    // ITERATORS: Advanced Patterns
    // =========================================
    println!("--- Iterator Advanced Patterns ---");

    let (caps, lower) = partition_words(&words, |w| w.is_capitalized());
    println!("Capitalized: {}, Lowercase: {}", caps.len(), lower.len());

    let medium_words = filter_words(&words, |w| w.len() >= 4 && w.len() <= 7);
    println!(
        "Medium-length words (4-7 chars): {}",
        medium_words
            .iter()
            .map(|w| w.text)
            .collect::<Vec<_>>()
            .join(", ")
    );

    let shouted = transform_texts(&words, |t| format!("{}!", t.to_uppercase()));
    println!("First 3 shouted: {:?}", &shouted[..3.min(shouted.len())]);

    let concatenated = fold_words(&words, String::new(), |mut acc, w| {
        if !acc.is_empty() {
            acc.push('-');
        }
        acc.push_str(w.text);
        acc
    });
    println!(
        "All words joined: {}...",
        &concatenated[..50.min(concatenated.len())]
    );
    println!();

    // =========================================
    // MUTABLE ITERATION (iter_mut)
    // =========================================
    println!("--- Mutable Iteration (iter_mut) ---");

    let mut values = vec![1, 2, 3, 4, 5];
    println!("Before doubling: {:?}", values);

    // Using iter_mut() to modify in place
    for val in values.iter_mut() {
        *val *= 2;
    }
    println!("After doubling: {:?}", values);

    // iter_mut with enumerate
    let mut letters = vec!['a', 'b', 'c', 'd'];
    for (i, letter) in letters.iter_mut().enumerate() {
        if i % 2 == 0 {
            *letter = letter.to_ascii_uppercase();
        }
    }
    println!("Alternating case: {:?}", letters);
    println!();

    // =========================================
    // LENGTH DISTRIBUTION
    // =========================================
    println!("--- Word Length Distribution ---");
    let dist = length_distribution(&words);
    for (len, count) in dist {
        let bar: String = (0..count).map(|_| '#').collect();
        println!("Length {:2}: {} {}", len, bar, count);
    }
    println!();

    // =========================================
    // CHAINED ITERATORS
    // =========================================
    println!("--- Chained Iterator Methods ---");

    let total_chars_long: usize = words
        .iter()
        .filter(|w| w.len() > 5)
        .map(|w| w.char_count())
        .sum();
    println!("Total chars in words > 5 letters: {}", total_chars_long);

    let first_three_long: Vec<_> = words
        .iter()
        .enumerate()
        .filter(|(_, w)| w.len() > 6)
        .take(3)
        .map(|(i, w)| format!("#{}: {}", i, w.text))
        .collect();
    println!("First 3 long words with index: {:?}", first_three_long);

    let last_word = words.iter().rev().next();
    match last_word {
        Some(w) => println!("Last word: '{}'", w.text),
        None => println!("No words"),
    }
}
