//! Fuzz target for ToolTalk message D-Bus body encoding.
//!
//! Invariants checked:
//!
//! 1. **No panic** — `encode_args()` must never panic for any combination of
//!    `TtArg` variants and arbitrary content.  Panics in a library called
//!    from C code would unwind across the FFI boundary (undefined behaviour).
//!
//! 2. **Roundtrip consistency** — every encoded `(mode, vtype, value)` triple
//!    must have a valid `mode` ("in", "out", or "inout") and a valid `vtype`
//!    ("int", "string", or "bytes").
//!
//! 3. **Length preservation** — the number of output triples equals the number
//!    of input args (no silent dropping or duplication).
//!
//! 4. **mode_str coverage** — the fuzzer exercises all Tt_mode boundary values
//!    (0=undefined, 1=TT_IN, 2=TT_OUT, 3=TT_INOUT, and arbitrary integers) to
//!    confirm mode_str() always returns one of the three valid wire strings.
//!
//! Run with: `cargo +nightly fuzz run fuzz_message_encoding`

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tt_shim::message::{encode_args, TtArg};

/// Top-level structured input for the fuzzer.
#[derive(Debug, Arbitrary)]
struct FuzzInput {
    args: Vec<FuzzArg>,
}

/// A single fuzzed argument.  `mode` is an arbitrary `i32` so libfuzzer
/// explores values outside the canonical {0,1,2,3} range and confirms
/// that `mode_str()` never panics and always returns a valid wire string.
#[derive(Debug, Arbitrary)]
struct FuzzArg {
    mode: i32,
    value: FuzzArgValue,
}

#[derive(Debug, Arbitrary)]
enum FuzzArgValue {
    Int(i32),
    Str(String),
    Bytes(Vec<u8>),
}

fuzz_target!(|input: FuzzInput| {
    // Build a Vec<TtArg> from the structured input.
    let args: Vec<TtArg> = input
        .args
        .into_iter()
        .map(|a| match a.value {
            FuzzArgValue::Int(i) => TtArg::int(a.mode, i),
            FuzzArgValue::Str(s) => TtArg::string(a.mode, s),
            FuzzArgValue::Bytes(b) => TtArg::bytes(a.mode, b),
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

    // Invariant 2: every mode and vtype is one of the known wire strings.
    for (mode, vtype, _value) in &encoded {
        assert!(
            mode == "in" || mode == "out" || mode == "inout",
            "encode_args produced unknown mode: {:?}",
            mode
        );
        assert!(
            vtype == "int" || vtype == "string" || vtype == "bytes",
            "encode_args produced unknown vtype: {:?}",
            vtype
        );
    }
});
