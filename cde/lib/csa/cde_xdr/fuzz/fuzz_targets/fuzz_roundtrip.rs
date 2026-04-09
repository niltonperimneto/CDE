// Phase E: structured roundtrip fuzzer.
//
// Uses libfuzzer-sys's Arbitrary derive to generate well-typed Rust values,
// encodes them with cde_xdr::pack, then decodes with cde_xdr::unpack and
// asserts structural equality.  Any divergence between encode and decode is a bug.
//
// Run with:
//   cargo fuzz run fuzz_roundtrip -- -max_total_time=300
#![no_main]

use cde_xdr::{pack, unpack};
use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Structured fuzz input — all fields have Arbitrary implementations from
// std types.  We wrap them to keep the fuzzer corpus focused.
// ---------------------------------------------------------------------------

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    i32_val: i32,
    u32_val: u32,
    i64_val: i64,
    u64_val: u64,
    bool_val: bool,
    // Limit string length to avoid slow corpus entries.
    // libfuzzer will still explore lengths 0..MAX_STRING and edge cases.
    #[arbitrary(with = arb_short_string)]
    string_val: String,
    #[arbitrary(with = arb_short_bytes)]
    bytes_val: Vec<u8>,
    #[arbitrary(with = arb_small_vec_i32)]
    i32_vec: Vec<i32>,
    opt_i32: Option<i32>,
    opt_string: Option<String>,
    nested_opt: Option<Option<i32>>,
    box_i32: Box<i32>,
}

// Arbitrary "with" helpers — keep generated data small enough that the fuzzer
// can explore the state space quickly.

fn arb_short_string(
    u: &mut libfuzzer_sys::arbitrary::Unstructured<'_>,
) -> libfuzzer_sys::arbitrary::Result<String> {
    let len: u8 = u.arbitrary()?; // 0–255 bytes
    let bytes: Vec<u8> = (0..len as usize)
        .map(|_| u.arbitrary::<u8>())
        .collect::<Result<_, _>>()?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn arb_short_bytes(
    u: &mut libfuzzer_sys::arbitrary::Unstructured<'_>,
) -> libfuzzer_sys::arbitrary::Result<Vec<u8>> {
    let len: u8 = u.arbitrary()?;
    (0..len as usize)
        .map(|_| u.arbitrary::<u8>())
        .collect()
}

fn arb_small_vec_i32(
    u: &mut libfuzzer_sys::arbitrary::Unstructured<'_>,
) -> libfuzzer_sys::arbitrary::Result<Vec<i32>> {
    let len: u8 = u.arbitrary()?;
    (0..len as usize)
        .map(|_| u.arbitrary::<i32>())
        .collect()
}

// ---------------------------------------------------------------------------
// Roundtrip assertion helper
// ---------------------------------------------------------------------------

fn roundtrip<T>(value: &T)
where
    T: cde_xdr::Pack + cde_xdr::Unpack + std::fmt::Debug + PartialEq,
{
    let mut buf = Vec::new();
    let n = match pack(value, &mut buf) {
        Ok(n) => n,
        Err(_) => return, // pack errors are acceptable (e.g., I/O failure on Vec)
    };
    assert_eq!(n, buf.len(), "pack returned wrong byte count for {:?}", value);

    let mut cur = Cursor::new(&buf);
    let (decoded, consumed) = match unpack::<T, _>(&mut cur) {
        Ok(pair) => pair,
        Err(e) => panic!("unpack failed after successful pack: {:?} — value was {:?}", e, value),
    };
    assert_eq!(consumed, buf.len(), "unpack consumed wrong count for {:?}", value);
    assert_eq!(&decoded, value, "roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Fuzz target
// ---------------------------------------------------------------------------

fuzz_target!(|input: FuzzInput| {
    roundtrip(&input.i32_val);
    roundtrip(&input.u32_val);
    roundtrip(&input.i64_val);
    roundtrip(&input.u64_val);
    roundtrip(&input.bool_val);
    roundtrip(&input.string_val);
    roundtrip(&input.bytes_val);
    roundtrip(&input.i32_vec);
    roundtrip(&input.opt_i32);
    roundtrip(&input.opt_string);
    roundtrip(&input.nested_opt);
    roundtrip(&input.box_i32);
});
