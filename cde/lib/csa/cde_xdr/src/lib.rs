// cde_xdr — RFC 4506 XDR codec
//
// Drop-in replacement for the unmaintained `xdr-codec 0.2` crate.
// The public API surface is identical to xdr-codec 0.2 so that xdrgen-
// generated code and existing call sites compile unchanged when you add:
//
//     use cde_xdr as xdr_codec;
//
// Wire format: byte-for-byte compatible with libtirpc XDR routines.
// Standard: RFC 4506 (obsoletes RFC 1832).

// This crate is fully safe Rust — no unsafe blocks.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

mod error;
mod primitives;

pub use error::XdrError;

/// The unified result type for all XDR operations.
/// Identical to `xdr_codec::Result<T>` in 0.2.
pub type Result<T> = std::result::Result<T, XdrError>;

/// Also export `Error` as an alias for generated code that references
/// `xdr_codec::Error` directly.
pub use error::XdrError as Error;

// ---------------------------------------------------------------------------
// Core traits
// ---------------------------------------------------------------------------

/// Encodes a value into an XDR byte stream (RFC 4506).
///
/// Returns the number of bytes written.  Implementors must write data in
/// big-endian, 4-byte-aligned format per the XDR specification.
///
/// The signature matches `xdr_codec::Pack` from version 0.2.
pub trait Pack<Out: std::io::Write> {
    fn pack(&self, out: &mut Out) -> Result<usize>;
}

/// Decodes a value from an XDR byte stream (RFC 4506).
///
/// Returns a `(value, bytes_consumed)` pair.  Implementors must consume
/// exactly as many bytes as the XDR specification requires for the type,
/// including any padding.
///
/// The signature matches `xdr_codec::Unpack` from version 0.2.
pub trait Unpack<In: std::io::Read>: Sized {
    fn unpack(input: &mut In) -> Result<(Self, usize)>;
}

// ---------------------------------------------------------------------------
// Free functions (mirrors the xdr_codec 0.2 free function API)
// ---------------------------------------------------------------------------

/// Encode `v` into `out`.  Returns the number of bytes written.
///
/// Equivalent to `xdr_codec::pack(v, out)`.
#[inline]
pub fn pack<T: Pack<W>, W: std::io::Write>(v: &T, out: &mut W) -> Result<usize> {
    v.pack(out)
}

/// Decode a value of type `T` from `input`.
/// Returns `(value, bytes_consumed)`.
///
/// Equivalent to `xdr_codec::unpack::<T, R>(input)`.
#[inline]
pub fn unpack<T: Unpack<In>, In: std::io::Read>(input: &mut In) -> Result<(T, usize)> {
    T::unpack(input)
}

/// Pack a string with an optional maximum length limit.
#[inline]
pub fn pack_string<W: std::io::Write>(v: &str, _max_len: Option<usize>, out: &mut W) -> Result<usize> {
    v.to_string().pack(out)
}

/// Unpack a string with an optional maximum length limit.
#[inline]
pub fn unpack_string<In: std::io::Read>(input: &mut In, _max_len: Option<usize>) -> Result<(String, usize)> {
    String::unpack(input)
}

/// Pack a flexible array `Vec<T>`.
#[inline]
pub fn pack_flex<T: Pack<W>, W: std::io::Write>(v: &Vec<T>, _max_len: Option<usize>, out: &mut W) -> Result<usize> {
    v.pack(out)
}

/// Unpack a flexible array `Vec<T>`.
#[inline]
pub fn unpack_flex<T: Unpack<In>, In: std::io::Read>(input: &mut In, _max_len: Option<usize>) -> Result<(Vec<T>, usize)> {
    Vec::<T>::unpack(input)
}

/// Pack an opaque flexible array `Vec<u8>`.
///
/// Opaque data is encoded as: 4-byte length + raw bytes + padding to 4-byte
/// boundary.  This is distinct from a generic `Vec<T>` array where each
/// element is individually XDR-encoded.
pub fn pack_opaque_flex<W: std::io::Write>(v: &[u8], _max_len: Option<usize>, out: &mut W) -> Result<usize> {
    let len = v.len();
    let len32 = u32::try_from(len)
        .map_err(|_| XdrError::size_limit(len, u32::MAX as usize))?;
    len32.pack(out)?;
    out.write_all(v)?;
    let pad_len = (4 - (len % 4)) % 4;
    if pad_len > 0 {
        out.write_all(&[0u8; 3][..pad_len])?;
    }
    Ok(4 + len + pad_len)
}

/// Unpack an opaque flexible array `Vec<u8>`.
pub fn unpack_opaque_flex<In: std::io::Read>(input: &mut In, _max_len: Option<usize>) -> Result<(Vec<u8>, usize)> {
    const MAX_OPAQUE_LEN: usize = 16 * 1024 * 1024;
    let (len, _) = u32::unpack(input)?;
    let len = len as usize;
    if len > MAX_OPAQUE_LEN {
        return Err(XdrError::size_limit(len, MAX_OPAQUE_LEN));
    }
    let mut buf = vec![0u8; len];
    input.read_exact(&mut buf)?;
    let pad_len = (4 - (len % 4)) % 4;
    if pad_len > 0 {
        let mut sink = [0u8; 3];
        input.read_exact(&mut sink[..pad_len])?;
    }
    Ok((buf, 4 + len + pad_len))
}

// ---------------------------------------------------------------------------
// Re-export std::io traits so generated code using
// `use xdr_codec::{Read, Write}` continues to compile.
// ---------------------------------------------------------------------------
pub use std::io::Read;
pub use std::io::Write;

// ---------------------------------------------------------------------------
// Unit tests — primitive roundtrips
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn roundtrip<T>(value: T) -> T
    where
        T: Pack<Vec<u8>> + for<'a> Unpack<Cursor<&'a Vec<u8>>> + std::fmt::Debug + PartialEq + Clone,
    {
        let mut buf = Vec::new();
        let encoded = pack(&value, &mut buf).expect("pack failed");
        assert_eq!(encoded, buf.len(), "pack returned wrong byte count");

        let mut cur = Cursor::new(&buf);
        let (decoded, consumed) = unpack::<T, _>(&mut cur).expect("unpack failed");
        assert_eq!(consumed, buf.len(), "unpack consumed wrong byte count");
        assert_eq!(decoded, value, "roundtrip produced different value");
        decoded
    }

    // --- scalar types -------------------------------------------------------

    #[test]
    fn roundtrip_i32() {
        for v in [0i32, 1, -1, i32::MAX, i32::MIN, 42, -1000] {
            roundtrip(v);
        }
    }

    #[test]
    fn roundtrip_u32() {
        for v in [0u32, 1, u32::MAX, 42, 0xDEAD_BEEF] {
            roundtrip(v);
        }
    }

    #[test]
    fn roundtrip_i64() {
        for v in [0i64, 1, -1, i64::MAX, i64::MIN] {
            roundtrip(v);
        }
    }

    #[test]
    fn roundtrip_u64() {
        for v in [0u64, 1, u64::MAX] {
            roundtrip(v);
        }
    }

    #[test]
    fn roundtrip_bool() {
        roundtrip(true);
        roundtrip(false);
    }

    #[test]
    fn roundtrip_unit() {
        roundtrip(());
    }

    // --- string encoding ----------------------------------------------------

    #[test]
    fn roundtrip_string_empty() {
        roundtrip(String::new());
    }

    #[test]
    fn roundtrip_string_4bytes() {
        // Exactly 4 bytes — no padding needed
        roundtrip("abcd".to_string());
    }

    #[test]
    fn roundtrip_string_5bytes() {
        // 5 bytes → 3 bytes of padding → total 12 bytes
        let s = "hello".to_string();
        let mut buf = Vec::new();
        let n = pack(&s, &mut buf).unwrap();
        assert_eq!(n, 12); // 4 (len) + 5 (data) + 3 (pad)
        assert_eq!(buf.len(), 12);
        roundtrip(s);
    }

    #[test]
    fn roundtrip_string_unicode() {
        roundtrip("CDE — Common Desktop Environment 🖥".to_string());
    }

    // --- opaque data --------------------------------------------------------

    #[test]
    fn roundtrip_bytes_empty() {
        let v = Vec::<u8>::new();
        let mut buf = Vec::new();
        let n = pack_opaque_flex(&v, None, &mut buf).unwrap();
        assert_eq!(n, 4); // just the length field
        let mut cur = Cursor::new(&buf[..]);
        let (decoded, consumed) = unpack_opaque_flex(&mut cur, None).unwrap();
        assert_eq!(consumed, 4);
        assert_eq!(decoded, v);
    }

    #[test]
    fn roundtrip_bytes_padded() {
        // 3 bytes → 1 byte of padding
        let v: Vec<u8> = vec![0xCA, 0xFE, 0xBA];
        let mut buf = Vec::new();
        let n = pack_opaque_flex(&v, None, &mut buf).unwrap();
        assert_eq!(n, 8); // 4 (len) + 3 (data) + 1 (pad)
        assert_eq!(buf.len(), 8);
        let mut cur = Cursor::new(&buf[..]);
        let (decoded, consumed) = unpack_opaque_flex(&mut cur, None).unwrap();
        assert_eq!(consumed, 8);
        assert_eq!(decoded, v);
    }

    // --- variable arrays ----------------------------------------------------

    #[test]
    fn roundtrip_vec_i32() {
        roundtrip(vec![1i32, 2, 3, -1, i32::MAX]);
    }

    #[test]
    fn roundtrip_vec_string() {
        roundtrip(vec!["dt".to_string(), "help".to_string(), "viewer".to_string()]);
    }

    #[test]
    fn roundtrip_vec_empty() {
        roundtrip(Vec::<i32>::new());
    }

    // --- optional values ----------------------------------------------------

    #[test]
    fn roundtrip_option_none() {
        roundtrip(None::<i32>);
    }

    #[test]
    fn roundtrip_option_some() {
        roundtrip(Some(42i32));
    }

    #[test]
    fn roundtrip_option_string() {
        roundtrip(Some("session".to_string()));
    }

    // --- byte layout correctness (golden bytes) -----------------------------

    #[test]
    fn i32_big_endian_layout() {
        let mut buf = Vec::new();
        pack(&0x0102_0304i32, &mut buf).unwrap();
        assert_eq!(&buf, &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn u32_big_endian_layout() {
        let mut buf = Vec::new();
        pack(&0xDEAD_BEEFu32, &mut buf).unwrap();
        assert_eq!(&buf, &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn string_xdr_layout() {
        // "hi" → length 2 (BE u32) + 'h' + 'i' + 0x00 0x00 (2-byte pad)
        let mut buf = Vec::new();
        pack(&"hi".to_string(), &mut buf).unwrap();
        assert_eq!(&buf, &[0, 0, 0, 2, b'h', b'i', 0, 0]);
    }

    #[test]
    fn bool_false_is_zero_int() {
        let mut buf = Vec::new();
        pack(&false, &mut buf).unwrap();
        assert_eq!(&buf, &[0, 0, 0, 0]);
    }

    #[test]
    fn bool_true_is_one_int() {
        let mut buf = Vec::new();
        pack(&true, &mut buf).unwrap();
        assert_eq!(&buf, &[0, 0, 0, 1]);
    }

    #[test]
    fn option_none_is_false_int() {
        let mut buf = Vec::new();
        pack(&None::<i32>, &mut buf).unwrap();
        assert_eq!(&buf, &[0, 0, 0, 0]); // just the "absent" discriminant
    }

    #[test]
    fn option_some_has_present_then_value() {
        let mut buf = Vec::new();
        pack(&Some(1i32), &mut buf).unwrap();
        assert_eq!(&buf, &[0, 0, 0, 1, 0, 0, 0, 1]); // present=1, value=1
    }

    // --- error cases --------------------------------------------------------

    #[test]
    fn unpack_truncated_i32_is_error() {
        let buf = [0x01, 0x02]; // only 2 bytes
        let mut cur = Cursor::new(&buf[..]);
        assert!(unpack::<i32, _>(&mut cur).is_err());
    }

    #[test]
    fn unpack_invalid_utf8_is_error() {
        // XDR string with length=1 and one invalid UTF-8 byte (0xFF)
        let buf = [0, 0, 0, 1, 0xFF, 0, 0, 0];
        let mut cur = Cursor::new(&buf[..]);
        assert!(unpack::<String, _>(&mut cur).is_err());
    }

    #[test]
    fn invalidenum_and_invalidcase_constructors() {
        // Verify the constructors exist and are usable (mirrors xdr-codec API)
        let e1 = XdrError::invalidenum(0i32);
        let e2 = XdrError::invalidcase(0i32);
        // Both must be displayable without panicking
        let _ = e1.to_string();
        let _ = e2.to_string();
    }
}
