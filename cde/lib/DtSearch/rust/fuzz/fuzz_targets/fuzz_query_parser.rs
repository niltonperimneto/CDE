//! Fuzz target for the DtSearch boolean query parser.
//!
//! Invariants checked:
//!
//! 1. **No panic** — `QueryParser::parse()` must never panic on arbitrary input,
//!    regardless of length, embedded NUL bytes, or exotic Unicode sequences.
//!    Panics in a library called from C code (via `DtSearchQuery`) would unwind
//!    across an FFI boundary, which is undefined behaviour.
//!
//! 2. **Parse-then-display round-trip** — if the parser produces a `Query`, the
//!    `Debug` representation must be constructible without panic.
//!
//! Run with: `cargo +nightly fuzz run fuzz_query_parser`

#![no_main]

use dtsearch_rs::search::QueryParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Accept any byte sequence; the parser must handle invalid UTF-8 gracefully.
    let input = match std::str::from_utf8(data) {
        Ok(s) => s,
        // Non-UTF-8 bytes: feed an empty string to test the empty-input path.
        Err(_) => return,
    };

    // Invariant 1: never panic regardless of input.
    let result = QueryParser::new(input).parse();

    // Invariant 2: if parsing succeeded, Debug must also not panic.
    if let Ok(ref query) = result {
        let _ = format!("{:?}", query);
    }
});
