// Phase E: arbitrary-input decode fuzzer.
//
// Feeds unstructured bytes into every Unpack impl and verifies:
//   • No panics — all decode errors are returned as Err(_), never unwrapped.
//   • No infinite loops — every call consumes a bounded prefix of the input.
//
// Run with:
//   cargo fuzz run fuzz_decode_arbitrary -- -max_total_time=300
#![no_main]

use cde_xdr::unpack;
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    macro_rules! try_unpack {
        ($T:ty) => {
            let _ = unpack::<$T, _>(&mut Cursor::new(data));
        };
    }

    // Scalar primitives
    try_unpack!(i32);
    try_unpack!(u32);
    try_unpack!(i64);
    try_unpack!(u64);
    try_unpack!(bool);
    try_unpack!(f32);
    try_unpack!(f64);
    try_unpack!(());

    // String (variable-length with UTF-8 validation)
    try_unpack!(String);

    // Opaque bytes (variable-length, no UTF-8 check)
    try_unpack!(Vec<u8>);

    // Variable arrays of primitives
    try_unpack!(Vec<i32>);
    try_unpack!(Vec<u32>);
    try_unpack!(Vec<String>);

    // Optional fields
    try_unpack!(Option<i32>);
    try_unpack!(Option<u32>);
    try_unpack!(Option<String>);
    try_unpack!(Option<Vec<u8>>);

    // Boxed (used by xdrgen for recursive pointer types)
    try_unpack!(Box<i32>);
    try_unpack!(Box<String>);

    // Nested optional — covers multi-level pointer chains in cm.x
    try_unpack!(Option<Option<i32>>);

    // Vec of optional — covers optional arrays in rtable types
    try_unpack!(Vec<Option<i32>>);
});
