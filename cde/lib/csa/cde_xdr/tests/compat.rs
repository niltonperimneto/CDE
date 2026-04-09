// Phase F: Wire-format compatibility tests.
//
// These tests verify that cde_xdr produces byte-for-byte identical output to
// what the C libtirpc XDR routines produce.  The expected byte sequences are
// derived from RFC 4506 (obsoletes RFC 1832) and cross-checked by hand against
// libtirpc xdr_int(), xdr_string(), xdr_bytes(), xdr_vector(), xdr_pointer().
//
// If any assertion here fails after a change to primitives.rs it means a
// wire-format regression — the network peer (dtcm) will reject our encoding.

use cde_xdr::{pack, unpack};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// §4.1 — Integer (big-endian, 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn golden_i32_zero() {
    // xdr_int(xdrs, 0) → [00 00 00 00]
    let mut buf = Vec::new();
    pack(&0i32, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_i32_one() {
    // xdr_int(xdrs, 1) → [00 00 00 01]
    let mut buf = Vec::new();
    pack(&1i32, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn golden_i32_minus_one() {
    // -1 in two's complement big-endian → [FF FF FF FF]
    let mut buf = Vec::new();
    pack(&(-1i32), &mut buf).unwrap();
    assert_eq!(buf, [0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn golden_i32_max() {
    // i32::MAX = 0x7FFF_FFFF
    let mut buf = Vec::new();
    pack(&i32::MAX, &mut buf).unwrap();
    assert_eq!(buf, [0x7F, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn golden_i32_min() {
    // i32::MIN = 0x8000_0000
    let mut buf = Vec::new();
    pack(&i32::MIN, &mut buf).unwrap();
    assert_eq!(buf, [0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_u32_deadbeef() {
    // xdr_u_int(xdrs, 0xDEADBEEF)
    let mut buf = Vec::new();
    pack(&0xDEAD_BEEFu32, &mut buf).unwrap();
    assert_eq!(buf, [0xDE, 0xAD, 0xBE, 0xEF]);
}

// ---------------------------------------------------------------------------
// §4.5 — Hyper integers (big-endian, 8 bytes)
// ---------------------------------------------------------------------------

#[test]
fn golden_i64_one() {
    // xdr_hyper(xdrs, 1) → [00 00 00 00  00 00 00 01]
    let mut buf = Vec::new();
    pack(&1i64, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn golden_i64_minus_one() {
    let mut buf = Vec::new();
    pack(&(-1i64), &mut buf).unwrap();
    assert_eq!(buf, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn golden_u64_boundary() {
    // u64 with high half = 0x01020304, low half = 0x05060708
    let v: u64 = 0x0102_0304_0506_0708;
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(buf, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
}

// ---------------------------------------------------------------------------
// §4.4 — Boolean (XDR bool is int32 0 or 1)
// ---------------------------------------------------------------------------

#[test]
fn golden_bool_false() {
    let mut buf = Vec::new();
    pack(&false, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_bool_true() {
    let mut buf = Vec::new();
    pack(&true, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x01]);
}

// Bool decode is tolerant of any non-zero value per libtirpc xdr_bool().
#[test]
fn golden_bool_any_nonzero_is_true() {
    // libtirpc xdr_bool decodes non-zero int32 as TRUE
    let buf = [0x00, 0x00, 0x00, 0x02u8];
    let (v, _) = unpack::<bool, _>(&mut Cursor::new(&buf)).unwrap();
    assert!(v);
}

// ---------------------------------------------------------------------------
// §4.6 — Float (IEEE 754 single, 4 bytes, big-endian)
// ---------------------------------------------------------------------------

#[test]
fn golden_f32_zero() {
    let mut buf = Vec::new();
    pack(&0.0f32, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_f32_one() {
    // 1.0f32 in IEEE 754: sign=0, exp=127 (0x7F), mantissa=0 → 0x3F80_0000
    let mut buf = Vec::new();
    pack(&1.0f32, &mut buf).unwrap();
    assert_eq!(buf, [0x3F, 0x80, 0x00, 0x00]);
}

#[test]
fn golden_f64_one() {
    // 1.0f64 in IEEE 754: sign=0, exp=1023 (0x3FF), mantissa=0 → 0x3FF0_0000_0000_0000
    let mut buf = Vec::new();
    pack(&1.0f64, &mut buf).unwrap();
    assert_eq!(buf, [0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

// ---------------------------------------------------------------------------
// §4.11 — String (4-byte length + UTF-8 bytes + NUL padding to 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn golden_string_empty() {
    // xdr_string(xdrs, "", ~0) → [00 00 00 00] (just the length)
    let mut buf = Vec::new();
    pack(&String::new(), &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_string_hi() {
    // "hi" (2 bytes) → length=2, data=[68 69], pad=[00 00]
    // Total: 8 bytes
    let mut buf = Vec::new();
    pack(&"hi".to_string(), &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x02, b'h', b'i', 0x00, 0x00]);
    assert_eq!(buf.len(), 8);
}

#[test]
fn golden_string_abc() {
    // "abc" (3 bytes) → length=3, data=[61 62 63], pad=[00]
    // Total: 8 bytes
    let mut buf = Vec::new();
    pack(&"abc".to_string(), &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x03, b'a', b'b', b'c', 0x00]);
    assert_eq!(buf.len(), 8);
}

#[test]
fn golden_string_four_bytes_no_pad() {
    // "abcd" (4 bytes) → length=4, data=[61 62 63 64], pad=[]
    // Total: 8 bytes (4 length + 4 data, already aligned)
    let mut buf = Vec::new();
    pack(&"abcd".to_string(), &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x04, b'a', b'b', b'c', b'd']);
    assert_eq!(buf.len(), 8);
}

#[test]
fn golden_string_five_bytes() {
    // "hello" (5 bytes) → length=5, data=[68 65 6c 6c 6f], pad=[00 00 00]
    // Total: 12 bytes
    let s = "hello".to_string();
    let mut buf = Vec::new();
    pack(&s, &mut buf).unwrap();
    assert_eq!(
        buf,
        [0x00, 0x00, 0x00, 0x05, b'h', b'e', b'l', b'l', b'o', 0x00, 0x00, 0x00]
    );
    assert_eq!(buf.len(), 12);
}

#[test]
fn golden_string_roundtrip_preserves_content() {
    let original = "CDE calendar service".to_string();
    let mut buf = Vec::new();
    pack(&original, &mut buf).unwrap();
    // Length must be 4-byte-aligned
    assert_eq!(buf.len() % 4, 0);
    let (decoded, consumed) = unpack::<String, _>(&mut Cursor::new(&buf)).unwrap();
    assert_eq!(decoded, original);
    assert_eq!(consumed, buf.len());
}

// ---------------------------------------------------------------------------
// §4.10 — Opaque variable-length data (4-byte length + bytes + NUL padding)
// ---------------------------------------------------------------------------

#[test]
fn golden_opaque_empty() {
    let mut buf = Vec::new();
    pack(&Vec::<u8>::new(), &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_opaque_3_bytes() {
    // 3 bytes → length=3, data=[CA FE BA], pad=[00]
    let data: Vec<u8> = vec![0xCA, 0xFE, 0xBA];
    let mut buf = Vec::new();
    pack(&data, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x03, 0xCA, 0xFE, 0xBA, 0x00]);
    assert_eq!(buf.len(), 8);
}

#[test]
fn golden_opaque_4_bytes_no_pad() {
    let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
    let mut buf = Vec::new();
    pack(&data, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04]);
}

// ---------------------------------------------------------------------------
// §4.13 — Variable-length array (4-byte count + elements)
// ---------------------------------------------------------------------------

#[test]
fn golden_array_empty() {
    // xdr_array with 0 elements → [00 00 00 00]
    let v: Vec<i32> = vec![];
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_array_two_i32s() {
    // Vec<i32> [1, 2] → count=2, elem0=1, elem1=2
    // [00 00 00 02  00 00 00 01  00 00 00 02]  (12 bytes)
    let v: Vec<i32> = vec![1, 2];
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(
        buf,
        [0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]
    );
}

#[test]
fn golden_array_roundtrip_strings() {
    // Ensure Vec<String> serialises correctly — typical for cms_name arrays
    let v = vec!["dt".to_string(), "cde".to_string()];
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(buf.len() % 4, 0, "array encoding must be 4-byte aligned");
    let (decoded, _) = unpack::<Vec<String>, _>(&mut Cursor::new(&buf)).unwrap();
    assert_eq!(decoded, v);
}

// ---------------------------------------------------------------------------
// §4.19 — Optional data (XDR "pointer" — 4-byte bool + optional T)
// ---------------------------------------------------------------------------

#[test]
fn golden_option_none() {
    // xdr_pointer, absent → [00 00 00 00]
    let v: Option<i32> = None;
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn golden_option_some_one() {
    // present=1, value=1 → [00 00 00 01  00 00 00 01]
    let v: Option<i32> = Some(1);
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    assert_eq!(buf, [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn golden_option_some_string() {
    // present=1, string="ok" → [00 00 00 01] + string encoding
    let v: Option<String> = Some("ok".to_string());
    let mut buf = Vec::new();
    pack(&v, &mut buf).unwrap();
    // present flag (4) + length=2 (4) + "ok" (2) + pad (2) = 12 bytes
    assert_eq!(buf.len(), 12);
    assert_eq!(buf[0..4], [0x00, 0x00, 0x00, 0x01]); // present
    assert_eq!(buf[4..8], [0x00, 0x00, 0x00, 0x02]); // length = 2
    assert_eq!(buf[8..10], [b'o', b'k']);              // data
    assert_eq!(buf[10..12], [0x00, 0x00]);             // padding
}

// ---------------------------------------------------------------------------
// §4.16 — Void (zero bytes)
// ---------------------------------------------------------------------------

#[test]
fn golden_void_zero_bytes() {
    let mut buf = Vec::new();
    pack(&(), &mut buf).unwrap();
    assert_eq!(buf, [] as [u8; 0]);
}

// ---------------------------------------------------------------------------
// Decode-only: verify decode rejects malformed input without panicking
// ---------------------------------------------------------------------------

#[test]
fn decode_truncated_i32_returns_err() {
    let buf = [0x00, 0x01]; // only 2 bytes, need 4
    assert!(unpack::<i32, _>(&mut Cursor::new(&buf)).is_err());
}

#[test]
fn decode_truncated_string_body_returns_err() {
    // Length says 10, but only 2 bytes of body follow
    let buf = [0x00, 0x00, 0x00, 0x0A, 0x61, 0x62];
    assert!(unpack::<String, _>(&mut Cursor::new(&buf)).is_err());
}

#[test]
fn decode_invalid_utf8_returns_err() {
    // Valid XDR wrapper, but body is 0xFF (not valid UTF-8)
    let buf = [0x00, 0x00, 0x00, 0x01, 0xFF, 0x00, 0x00, 0x00];
    assert!(unpack::<String, _>(&mut Cursor::new(&buf)).is_err());
}

#[test]
fn decode_size_limit_string_returns_err() {
    // Claim a 4 GiB string — should be rejected by the size cap without OOM
    let buf = [0xFF, 0xFF, 0xFF, 0xFF]; // length = 4,294,967,295
    assert!(unpack::<String, _>(&mut Cursor::new(&buf)).is_err());
}

#[test]
fn decode_size_limit_array_returns_err() {
    // Claim 4 billion elements — rejected before any allocation
    let buf = [0xFF, 0xFF, 0xFF, 0xFF];
    assert!(unpack::<Vec<i32>, _>(&mut Cursor::new(&buf)).is_err());
}

// ---------------------------------------------------------------------------
// Composite: multi-field struct layout
// Simulates a simplified cms_key { u32 id; String name; bool recurring; }
// ---------------------------------------------------------------------------

#[test]
fn golden_composite_struct_layout() {
    // Encode the three fields independently and verify concatenation
    let id: u32 = 42;
    let name = "daily".to_string();
    let recurring = true;

    let mut buf = Vec::new();
    let n1 = pack(&id, &mut buf).unwrap();
    let n2 = pack(&name, &mut buf).unwrap();
    let n3 = pack(&recurring, &mut buf).unwrap();

    // id: 4 bytes
    // name "daily" (5 chars): 4 (length) + 5 (data) + 3 (pad) = 12 bytes
    // bool: 4 bytes
    // Total: 20 bytes
    assert_eq!(n1, 4);
    assert_eq!(n2, 12);
    assert_eq!(n3, 4);
    assert_eq!(buf.len(), 20);
    assert_eq!(buf.len() % 4, 0);

    // Decode and verify
    let mut cur = Cursor::new(&buf);
    let (did, _) = unpack::<u32, _>(&mut cur).unwrap();
    let (dname, _) = unpack::<String, _>(&mut cur).unwrap();
    let (drec, _) = unpack::<bool, _>(&mut cur).unwrap();
    assert_eq!(did, 42);
    assert_eq!(dname, "daily");
    assert!(drec);
}
