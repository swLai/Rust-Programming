# Rust Module System - Library Management Example

This project demonstrates all the ways an experienced Rust developer organizes their code using the module system.

## Quick Start

```bash
cargo run      # Run the example
cargo test     # Run all tests (unit + doc tests)
cargo doc --open  # Generate and view documentation
```

## Project Structure

```
module-8/
├── Cargo.toml              # Package manifest with external dependencies
├── src/
│   ├── lib.rs              # Library crate root (re-exports + inline modules)
│   ├── main.rs             # Binary crate (demonstrates using the library)
│   ├── book.rs             # File-based module (no submodules)
│   ├── utils.rs            # File-based module with inline nested submodule
│   ├── member.rs           # Directory module entry point (MODERN STYLE)
│   └── member/             # Directory for member's submodules
│       └── membership.rs   # Submodule
```

## Module Organization Patterns

### 1. File-Based Modules

When you write `mod book;` in `lib.rs`, Rust looks for:
- `src/book.rs` (modern, preferred style)
- `src/book/mod.rs` (older style)

**Example:** [book.rs](src/book.rs)

```rust
// In lib.rs
mod book;  // Looks for src/book.rs
```

### 2. Directory-Based Modules (with Submodules)

When a module needs submodules, Rust supports **two styles**:

#### Older Style (pre-Rust 2018)
```
src/
└── member/
    ├── mod.rs          ← Entry point
    └── membership.rs   ← Submodule
```

#### Modern Style (Rust 2018+) - PREFERRED
```
src/
├── member.rs           ← Entry point (THIS PROJECT USES THIS)
└── member/
    └── membership.rs   ← Submodule
```

**Why we chose modern style:**
1. **File names match module names** - `member.rs` for `mod member`
2. **No duplicate `mod.rs` tabs** - easier to navigate in editors
3. **Industry standard** - most new Rust projects use this style

**Example:** [member.rs](src/member.rs) + [member/](src/member/)

```rust
// In lib.rs
mod member;  // Looks for src/member.rs (modern) or src/member/mod.rs (older)

// In member.rs (the entry point)
mod membership;  // Looks for src/member/membership.rs
```

Both styles compile identically - the choice is purely organizational.

### 3. Inline Modules

Small, closely related code can be defined directly in a file:

**Example:** `config` module in [lib.rs](src/lib.rs)

```rust
mod config {
    pub const LIBRARY_NAME: &str = "Rustacean Library";

    pub mod fees {  // Nested inline module
        pub const LATE_FEE_PER_DAY: u32 = 25;
    }
}
```

## Visibility Modifiers

| Modifier | Visibility |
|----------|------------|
| (none) | Private to current module only |
| `pub` | Public to all |
| `pub(crate)` | Public within the crate only |
| `pub(super)` | Public to parent module only |
| `pub(in path)` | Public within specified path |

**Examples from this project:**

```rust
// In lib.rs - config module
pub(crate) const MAX_BORROWED_BOOKS: usize = 5;  // Crate-internal
pub const LIBRARY_NAME: &str = "...";             // Fully public

// In membership.rs
pub(super) fn calculate_discount(...) { }         // Only parent (member) can see
```

## Re-Exporting with `pub use`

Re-exporting creates a cleaner public API:

```rust
// In lib.rs
pub use book::{Book, Genre};        // Users can write: use module_8::Book;
pub use member::MembershipTier;     // Instead of: use module_8::book::Book;
```

## Path Resolution

| Path | Description |
|------|-------------|
| `crate::` | Absolute path from crate root |
| `self::` | Relative to current module |
| `super::` | Relative to parent module |

**Examples:**

```rust
// In utils.rs
use crate::book::Book;              // Absolute path

// In member/membership.rs
pub(super) fn calculate_discount()  // Visible to super (member module)

// In utils.rs - formatting submodule
use super::Genre;                   // Access parent's imports
```

## Struct vs Enum Visibility

**Structs:** Each field's visibility must be specified individually.

```rust
pub struct Book {
    id: u64,           // Private - can't be set directly
    pub title: String, // Public - can be read/modified
}
```

**Enums:** If the enum is `pub`, ALL variants are automatically public.

```rust
pub enum Genre {
    Fiction,    // Automatically public
    Technical,  // Automatically public
}
```

## Using External Crates

1. Add to `Cargo.toml`:
```toml
[dependencies]
chrono = "0.4"
```

2. Use in your code:
```rust
use chrono::Local;
let now = Local::now();
```

## Import Styles

```rust
// Grouped imports
use module_8::{Book, Genre, Library};

// Individual imports
use module_8::LIBRARY_NAME;

// Nested path imports
use module_8::config::fees::LATE_FEE_PER_DAY;

// Aliased imports
use module_8::utils::formatting::genre_emoji as get_emoji;

// Wildcard (use sparingly)
use module_8::book::*;
```

## Module Hierarchy

```
crate (module_8)
├── book                    [pub mod - file: book.rs]
│   ├── Genre              [pub enum]
│   ├── Book               [pub struct]
│   └── tests              [private, #[cfg(test)]]
│
├── member                  [pub mod - file: member.rs + dir: member/]
│   ├── membership         [private submod in member/membership.rs]
│   │   ├── MembershipTier [pub enum, re-exported]
│   │   └── calculate_discount [pub(super) fn]
│   ├── Member             [pub struct]
│   └── create_guest       [pub fn]
│
├── utils                   [pub mod - file: utils.rs]
│   ├── format_book_info   [pub fn, re-exported at root]
│   ├── validate_title     [pub(crate) fn]
│   └── formatting         [pub mod - inline]
│       ├── genre_emoji    [pub fn]
│       └── genre_with_emoji [pub fn]
│
├── config                  [pub mod - inline in lib.rs]
│   ├── MAX_BORROWED_BOOKS [pub(crate) const]
│   ├── LIBRARY_NAME       [pub const, re-exported at root]
│   └── fees               [pub mod - nested inline]
│       ├── LATE_FEE_PER_DAY [pub const]
│       └── calculate_late_fee [pub fn, re-exported at root]
│
└── Library                 [pub struct - defined in lib.rs]
```

## Key Takeaways

1. **Use file-based modules** for most code organization
2. **Use directory-based modules** when you need submodules
3. **Prefer modern style** (`module.rs` + `module/`) over older style (`module/mod.rs`)
4. **Use inline modules** for small, closely related code
5. **Re-export with `pub use`** to create a clean public API
6. **Use visibility modifiers** to encapsulate implementation details
7. **Prefer `crate::`** for absolute paths within your crate
8. **Tests go in `#[cfg(test)] mod tests`** at the bottom of each file

## Running the Example

```bash
$ cargo run

╔════════════════════════════════════════════════════════════╗
║     RUST MODULES DEMONSTRATION - Library Management        ║
╚════════════════════════════════════════════════════════════╝

📚 CREATING LIBRARY
─────────────────────────────────────────────────────────────
Welcome to: Rustacean Library
...
```

## Further Reading

- [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust Reference - Visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
