//! Fuzz target for ToolTalk message D-Bus body encoding.
//!
//! Invariants checked:
//!
//! 1. **No panic** — `encode_args()` must never panic for any combination of
//!    `TtArg` variants and arbitrary content.  Panics in a library called
//!    from C code would unwind across the FFI boundary (undefined behaviour).
//!
//! 2. **Roundtrip consistency** — every encoded `(vtype, value)` pair must
//!    have a non-empty `vtype` string (one of "int", "string", or "bytes").
//!
//! 3. **Length preservation** — the number of output pairs equals the number
//!    of input args (no silent dropping or duplication).
//!
//! Run with: `cargo +nightly fuzz run fuzz_message_encoding`

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tt_shim::message::{encode_args, TtArg};

/// Structured input for the fuzzer.
///
/// `#[derive(Arbitrary)]` lets libfuzzer generate type-safe inputs rather
/// than raw bytes, ensuring better coverage of the variant-dispatch logic.
#[derive(Debug, Arbitrary)]
struct FuzzInput {
    args: Vec<FuzzArg>,
}

#[derive(Debug, Arbitrary)]
enum FuzzArg {
    Int(i32),
    Str(String),
    Bytes(Vec<u8>),
}

fuzz_target!(|input: FuzzInput| {
    // Build a Vec<TtArg> from the structured input.
    let args: Vec<TtArg> = input
        .args
        .into_iter()
        .map(|a| match a {
            FuzzArg::Int(i) => TtArg::Int(i),
            FuzzArg::Str(s) => TtArg::String(s),
            FuzzArg::Bytes(b) => TtArg::Bytes(b),
        })
        .collect();

    let n = args.len();

    // Invariant 1: must not panic.
    let encoded = encode_args(&args);

    // Invariant 3: output length equals input length.
    assert_eq!(
        encoded.len(),
        n,
        "encode_args dropped or duplicated args: in={} out={}",
        n,
        encoded.len()
    );

    // Invariant 2: every vtype is one of the known tags.
    for (vtype, _value) in &encoded {
        assert!(
            vtype == "int" || vtype == "string" || vtype == "bytes",
            "encode_args produced unknown vtype: {:?}",
            vtype
        );
    }
});
