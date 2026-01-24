// =============================================================================
// FREQUENCY.RS - HashMap Word Frequency Analysis
// =============================================================================
//
// CONCEPTS DEMONSTRATED:
// ----------------------
// 1. HASHMAP (Module 6 - Hash Maps, Ch 8)
//    - Creating and using HashMap
//    - The entry() API for insert-or-update
//    - Iterating over HashMap
//
// 2. TRAITS (Module 6 - Traits)
//    - Implementing same trait for different types (polymorphism)
//
// 3. ITERATORS (Module 7 - Iterators)
//    - iter(), map(), values(), sum()
//    - into_iter(), take(), collect()
//    - Sorting with sort_by()
//
// 4. RETURN TYPE impl Trait
//    - Returning iterators without specifying concrete type
//
// 5. CLOSURES
//    - Comparison closures for sorting
//
// =============================================================================

use std::collections::HashMap;

use crate::stats::Summarizable;
use crate::word::Word;

// =============================================================================
// HASHMAP STRUCT
// =============================================================================
//
// From Module 6 (Hash Maps):
//   use std::collections::HashMap;
//   let mut person: HashMap<&str, i32> = HashMap::new();
//   person.insert("Nouman", 40);
//
// HashMap<K, V> stores key-value pairs:
// - Keys must implement Eq + Hash traits
// - Values can be any type
// - O(1) average time for insert, lookup, delete
//
// In this struct:
// - Keys are Strings (lowercase word text)
// - Values are usizes (occurrence count)
// =============================================================================

/// Word frequency analysis using HashMap.
pub struct WordFrequency {
    // OWNERSHIP NOTE:
    // We store String (owned) keys, not &str (borrowed).
    // This is because we want WordFrequency to own its data independently.
    // Using &str would require lifetime parameters.
    counts: HashMap<String, usize>,
}

impl WordFrequency {
    // -------------------------------------------------------------------------
    // ENTRY API FOR INSERT-OR-UPDATE
    // -------------------------------------------------------------------------
    //
    // From Module 6 (Hash Maps):
    //   let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
    //   *freq += 1;
    //
    // The entry() API provides efficient insert-or-update:
    //
    // WITHOUT entry API (inefficient - two lookups):
    //   if counts.contains_key(&word) {
    //       *counts.get_mut(&word).unwrap() += 1;
    //   } else {
    //       counts.insert(word, 1);
    //   }
    //
    // WITH entry API (efficient - one lookup):
    //   *counts.entry(word).or_insert(0) += 1;
    //
    // ENTRY VARIANTS:
    // - entry(key).or_insert(default) - insert default if missing
    // - entry(key).or_insert_with(|| compute()) - lazy computation
    // - entry(key).or_default() - uses Default trait
    // - entry(key).and_modify(|v| *v += 1) - modify if present
    // -------------------------------------------------------------------------

    pub fn from_words(words: &[Word]) -> WordFrequency {
        let mut counts = HashMap::new();

        for word in words {
            // ENTRY API PATTERN
            // -----------------
            // 1. word.text.to_lowercase() - create lowercase String
            // 2. counts.entry(...) - get Entry enum (Occupied or Vacant)
            // 3. .or_insert(0) - if Vacant, insert 0 and return &mut
            //                    if Occupied, just return &mut to existing value
            // 4. *count += 1 - dereference and increment
            //
            // The entry API handles both cases (new word / existing word) efficiently.
            let count = counts.entry(word.text.to_lowercase()).or_insert(0);
            *count += 1;

            // DEREFERENCING (Module 3 - Dereferencing):
            // `count` is &mut usize (a mutable reference to the value in HashMap)
            // `*count` dereferences to access/modify the actual usize value
        }

        WordFrequency { counts }
    }

    // -------------------------------------------------------------------------
    // HASHMAP LOOKUP
    // -------------------------------------------------------------------------
    //
    // From Module 6 (Hash Maps):
    //   match person.get("Nouman") {
    //       Some(value) => println!("The value exists {}", value),
    //       None => println!("The value does not exist"),
    //   }
    //
    // get() returns Option<&V>:
    // - Some(&value) if key exists
    // - None if key doesn't exist
    //
    // .copied() converts Option<&usize> to Option<usize>
    // (only works for Copy types)
    // -------------------------------------------------------------------------

    pub fn get(&self, word: &str) -> Option<usize> {
        // Convert to lowercase for case-insensitive lookup
        // .copied() transforms Option<&usize> to Option<usize>
        self.counts.get(&word.to_lowercase()).copied()
    }

    /// Get total unique words (number of distinct keys).
    pub fn unique_count(&self) -> usize {
        // HashMap::len() returns number of key-value pairs
        self.counts.len()
    }

    // -------------------------------------------------------------------------
    // SORTING WITH CLOSURES
    // -------------------------------------------------------------------------
    //
    // sort_by() takes a comparison closure that returns Ordering.
    //
    // Ordering enum:
    // - Ordering::Less means a < b
    // - Ordering::Equal means a == b
    // - Ordering::Greater means a > b
    //
    // cmp() method returns Ordering for Ord types.
    //
    // For descending order: b.cmp(a) instead of a.cmp(b)
    // -------------------------------------------------------------------------

    pub fn top_n(&self, n: usize) -> Vec<(&str, usize)> {
        // STEP 1: Collect all entries into a vector
        // iter() yields (&String, &usize) pairs
        let mut entries: Vec<_> = self.counts.iter().collect();

        // STEP 2: Sort using a comparison closure
        // sort_by takes |a, b| -> Ordering
        //
        // COMPLEX SORTING:
        // Primary sort: by count descending (b.1.cmp(a.1))
        // Secondary sort: alphabetically for ties (a.0.cmp(b.0))
        //
        // MATCH ON ORDERING:
        // If counts are equal, use alphabetical order.
        // Otherwise, use the count comparison result.
        entries.sort_by(|a, b| match b.1.cmp(a.1) {
            std::cmp::Ordering::Equal => a.0.cmp(b.0),
            other => other,
        });

        // STEP 3: Take first n elements and transform
        // into_iter() - consumes vector, yields owned tuples
        // take(n) - limits to first n elements
        // map() - transforms (&String, &usize) to (&str, usize)
        // collect() - gathers into Vec
        entries
            .into_iter()
            .take(n)
            .map(|(word, &count)| (word.as_str(), count))
            .collect()
    }

    // -------------------------------------------------------------------------
    // RETURNING impl Trait
    // -------------------------------------------------------------------------
    //
    // `impl Iterator<Item = X>` means:
    // "I return some type that implements Iterator, yielding X values"
    //
    // Benefits:
    // - Hides the concrete iterator type (which can be complex)
    // - Allows callers to chain iterator methods
    // - Compiler still knows the exact type (zero-cost abstraction)
    //
    // Limitation:
    // - Can only return ONE concrete type (not conditional returns)
    // -------------------------------------------------------------------------

    pub fn iter(&self) -> impl Iterator<Item = (&str, usize)> {
        // HASHMAP ITERATION:
        // iter() yields (&K, &V) pairs, i.e., (&String, &usize)
        //
        // We map to (&str, usize):
        // - w.as_str() converts &String to &str
        // - &c is a pattern that dereferences the &usize to usize
        self.counts.iter().map(|(w, &c)| (w.as_str(), c))
    }

    // -------------------------------------------------------------------------
    // HASHMAP values() ITERATOR
    // -------------------------------------------------------------------------
    //
    // values() returns an iterator over just the values (&V).
    // Useful when you don't need the keys.
    // -------------------------------------------------------------------------

    pub fn total_occurrences(&self) -> usize {
        // values() yields &usize references
        // sum() adds them all up
        self.counts.values().sum()
    }
}

// =============================================================================
// TRAIT POLYMORPHISM
// =============================================================================
//
// The SAME trait (Summarizable) is implemented for DIFFERENT types
// (TextStats and WordFrequency).
//
// This enables POLYMORPHIC code:
//   fn print_summary(item: &impl Summarizable) {
//       println!("{}", item.summarize());
//   }
//   print_summary(&text_stats);   // Works!
//   print_summary(&word_freq);    // Also works!
//
// DRY PRINCIPLE:
// We call unique_count() and total_occurrences() instead of
// recalculating. This avoids code duplication and ensures consistency.
// =============================================================================

impl Summarizable for WordFrequency {
    fn summarize(&self) -> String {
        format!(
            "Frequency: {} unique words, {} total occurrences",
            self.unique_count(),       // Reuse existing method
            self.total_occurrences()   // Reuse existing method
        )
    }

    fn item_count(&self) -> usize {
        self.unique_count()  // Delegate to existing method
    }

    // brief() uses the default implementation from the trait
}

// =============================================================================
// FREQUENCY DISTRIBUTION
// =============================================================================
//
// Creates a "meta-frequency" map:
// - Key: how many times a word appears (1, 2, 3, ...)
// - Value: how many words have that frequency
//
// Example: If 10 words appear once and 3 words appear twice:
//   {1: 10, 2: 3}
//
// This uses the same entry() API pattern we saw above.
// =============================================================================

pub fn frequency_distribution(freq: &WordFrequency) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();

    // DESTRUCTURING IN FOR LOOP:
    // for (_, count) iterates over (word, count) pairs
    // _ discards the word since we don't need it
    for (_, count) in freq.iter() {
        // Same entry() pattern:
        // Get entry for this count, insert 0 if new, then increment
        *distribution.entry(count).or_insert(0) += 1;
    }

    distribution
}
