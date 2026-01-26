//! # Library Management System - Main Entry Point
//!
//! This binary crate demonstrates how to USE a library crate and its modules.
//!
//! ## Key Concepts Demonstrated:
//! - Importing from your own library crate
//! - Using re-exported items (cleaner imports)
//! - Using external crates from crates.io
//! - Different import styles (`use` with braces, aliases, wildcards)

// =============================================================================
// IMPORTING FROM OUR LIBRARY CRATE
// =============================================================================

// When you have both lib.rs and main.rs, they form separate crates:
// - lib.rs = library crate (named after the package, here "module_8")
// - main.rs = binary crate
//
// main.rs accesses lib.rs content using the package name, just like
// an external user would.

// GROUPED IMPORTS: Import multiple items from the same crate using braces.
// These work because lib.rs re-exports them with `pub use`.
use module_8::{Book, Genre, Library, Member, MembershipTier};

// INDIVIDUAL IMPORTS: You can also import items one by one.
use module_8::calculate_late_fee;
use module_8::format_book_info;
use module_8::LIBRARY_NAME;

// NESTED PATH IMPORTS: Access items from nested modules.
// Even though `config` is a module inside lib.rs, we can access its
// public submodules.
use module_8::config::fees::LATE_FEE_PER_DAY;

// ALIAS IMPORT: Rename an import to avoid conflicts or improve clarity.
use module_8::utils::formatting::genre_emoji as get_emoji;

// =============================================================================
// IMPORTING EXTERNAL CRATE
// =============================================================================

// External crates are added to Cargo.toml under [dependencies].
// After adding `chrono = "0.4"` to Cargo.toml, we can use it here.
// This demonstrates using crates from crates.io.
use chrono::Local;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     RUST MODULES DEMONSTRATION - Library Management        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // -------------------------------------------------------------------------
    // Using re-exported types from library root
    // -------------------------------------------------------------------------
    println!("📚 CREATING LIBRARY");
    println!("─────────────────────────────────────────────────────────────");

    let mut library = Library::new();
    println!("Welcome to: {}", library.name());
    println!("Library constant: {}", LIBRARY_NAME);
    println!("Max books per member: {}", library.max_books_per_member());
    println!();

    // -------------------------------------------------------------------------
    // Using Book and Genre types
    // -------------------------------------------------------------------------
    println!("📖 ADDING BOOKS");
    println!("─────────────────────────────────────────────────────────────");

    // Create books using the re-exported types
    let book1 = Book::new(1, "The Rust Programming Language", Genre::Technical);
    let book2 = Book::new(2, "Clean Code", Genre::Technical);
    let book3 = Book::new(3, "Foundation", Genre::SciFi);
    let book4 = Book::new(4, "Murder on the Orient Express", Genre::Mystery);

    // Using the utility function (re-exported at crate root)
    println!("{}", format_book_info(&book1));
    println!("{}", format_book_info(&book2));
    println!("{}", format_book_info(&book3));
    println!("{}", format_book_info(&book4));

    // Using the aliased import for emoji
    println!(
        "\nGenre icons: {} Technical, {} SciFi, {} Mystery",
        get_emoji(&Genre::Technical),
        get_emoji(&Genre::SciFi),
        get_emoji(&Genre::Mystery)
    );

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);
    library.add_book(book4);
    println!("\nTotal books in library: {}", library.book_count());
    println!();

    // -------------------------------------------------------------------------
    // Using Member and MembershipTier types
    // -------------------------------------------------------------------------
    println!("👥 REGISTERING MEMBERS");
    println!("─────────────────────────────────────────────────────────────");

    let member1 = Member::new(1, "Alice", MembershipTier::Gold);
    let member2 = Member::new(2, "Bob", MembershipTier::Silver);
    let member3 = Member::new(3, "Charlie", MembershipTier::Basic);

    // Using module function (not re-exported, accessed via full path)
    let guest = module_8::member::create_guest(4, "Guest User");

    for member in [&member1, &member2, &member3, &guest] {
        println!(
            "Member: {} | Tier: {:?} | Max Books: {} | Discount: {}%",
            member.name,
            member.tier,
            member.max_books(),
            member.discount_percentage()
        );
    }

    library.register_member(member1);
    library.register_member(member2);
    library.register_member(member3);
    library.register_member(guest);
    println!("\nTotal members: {}", library.member_count());
    println!();

    // -------------------------------------------------------------------------
    // Using config module items
    // -------------------------------------------------------------------------
    println!("💰 FEE CALCULATIONS");
    println!("─────────────────────────────────────────────────────────────");

    println!("Late fee per day: {} cents", LATE_FEE_PER_DAY);
    println!("Late fee for 3 days: {} cents", calculate_late_fee(3));
    println!("Late fee for 7 days: {} cents", calculate_late_fee(7));
    println!();

    // -------------------------------------------------------------------------
    // Using external crate (chrono)
    // -------------------------------------------------------------------------
    println!("📅 EXTERNAL CRATE DEMO (chrono)");
    println!("─────────────────────────────────────────────────────────────");

    let now = Local::now();
    println!("Current date/time: {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("Today is: {}", now.format("%A, %B %d, %Y"));
    println!();

    // -------------------------------------------------------------------------
    // Demonstrating borrowing workflow
    // -------------------------------------------------------------------------
    println!("📚 BORROWING WORKFLOW");
    println!("─────────────────────────────────────────────────────────────");

    let mut book = Book::new(100, "Demo Book", Genre::Fiction);

    println!("Before borrowing:");
    println!("  Book available: {}", book.is_available());
    println!("  Times borrowed: {}", book.times_borrowed());

    // Borrow the book
    if book.borrow_book() {
        println!("\nBook borrowed successfully!");
    }

    println!("\nAfter borrowing:");
    println!("  Book available: {}", book.is_available());
    println!("  Times borrowed: {}", book.times_borrowed());

    // Return the book
    book.return_book();
    println!("\nAfter returning:");
    println!("  Book available: {}", book.is_available());
    println!();

    // -------------------------------------------------------------------------
    // Summary
    // -------------------------------------------------------------------------
    println!("═══════════════════════════════════════════════════════════");
    println!("  Module System Concepts Demonstrated:");
    println!("═══════════════════════════════════════════════════════════");
    println!("  ✓ File-based modules (book.rs)");
    println!("  ✓ Directory-based modules (member.rs + member/ - modern style)");
    println!("  ✓ Inline modules (config in lib.rs)");
    println!("  ✓ Re-exporting with pub use");
    println!("  ✓ Visibility modifiers (pub, pub(crate), pub(super))");
    println!("  ✓ Path resolution (crate::, self::, super::)");
    println!("  ✓ External crates (chrono)");
    println!("  ✓ Various import styles (grouped, aliased, wildcard)");
    println!("═══════════════════════════════════════════════════════════");
}
