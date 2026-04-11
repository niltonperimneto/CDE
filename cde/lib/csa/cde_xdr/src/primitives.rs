/// RFC 4506 primitive XDR encodings.
///
/// Every `Pack` and `Unpack` implementation in this module is byte-for-byte
/// compatible with the C libtirpc XDR routines.  The encoding rules follow
/// RFC 4506 §4 (XDR: External Data Representation Standard).
use crate::{Pack, Result, Unpack, XdrError};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

// ---------------------------------------------------------------------------
// Padding helper (RFC 4506 §4.2)
// All variable-length fields are padded to a 4-byte boundary.
// ---------------------------------------------------------------------------

/// Number of pad bytes needed to reach the next 4-byte boundary.
#[inline]
fn pad_len(data_len: usize) -> usize {
    (4 - (data_len % 4)) % 4
}

/// Write zero-bytes up to the next 4-byte boundary.
fn write_padding<W: Write>(w: &mut W, data_len: usize) -> Result<usize> {
    let pad = pad_len(data_len);
    if pad > 0 {
        w.write_all(&[0u8; 3][..pad])?;
    }
    Ok(pad)
}

/// Consume padding bytes up to the next 4-byte boundary.
fn read_padding<R: Read>(r: &mut R, data_len: usize) -> Result<usize> {
    let pad = pad_len(data_len);
    if pad > 0 {
        let mut sink = [0u8; 3];
        r.read_exact(&mut sink[..pad])?;
    }
    Ok(pad)
}

// ---------------------------------------------------------------------------
// i32 — XDR `int` (§4.1)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for i32 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_i32::<BigEndian>(*self)?;
        Ok(4)
    }
}

impl<In: Read> Unpack<In> for i32 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_i32::<BigEndian>()?, 4))
    }
}

// ---------------------------------------------------------------------------
// u32 — XDR `unsigned int` (§4.2)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for u32 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_u32::<BigEndian>(*self)?;
        Ok(4)
    }
}

impl<In: Read> Unpack<In> for u32 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_u32::<BigEndian>()?, 4))
    }
}

// ---------------------------------------------------------------------------
// i64 — XDR `hyper` (§4.5)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for i64 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_i64::<BigEndian>(*self)?;
        Ok(8)
    }
}

impl<In: Read> Unpack<In> for i64 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_i64::<BigEndian>()?, 8))
    }
}

// ---------------------------------------------------------------------------
// u64 — XDR `unsigned hyper` (§4.5)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for u64 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_u64::<BigEndian>(*self)?;
        Ok(8)
    }
}

impl<In: Read> Unpack<In> for u64 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_u64::<BigEndian>()?, 8))
    }
}

// ---------------------------------------------------------------------------
// bool — XDR `bool` (§4.4): encoded as int32 (0 = FALSE, 1 = TRUE)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for bool {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        (*self as i32).pack(out)
    }
}

impl<In: Read> Unpack<In> for bool {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        let (v, sz) = i32::unpack(input)?;
        Ok((v != 0, sz))
    }
}

// ---------------------------------------------------------------------------
// f32 — XDR `float` (§4.6)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for f32 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_f32::<BigEndian>(*self)?;
        Ok(4)
    }
}

impl<In: Read> Unpack<In> for f32 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_f32::<BigEndian>()?, 4))
    }
}

// ---------------------------------------------------------------------------
// f64 — XDR `double` (§4.7)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for f64 {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        out.write_f64::<BigEndian>(*self)?;
        Ok(8)
    }
}

impl<In: Read> Unpack<In> for f64 {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        Ok((input.read_f64::<BigEndian>()?, 8))
    }
}

// ---------------------------------------------------------------------------
// () — XDR `void`: encodes to zero bytes (§4.16)
// ---------------------------------------------------------------------------

impl<Out: Write> Pack<Out> for () {
    fn pack(&self, _out: &mut Out) -> Result<usize> {
        Ok(0)
    }
}

impl<In: Read> Unpack<In> for () {
    fn unpack(_input: &mut In) -> Result<(Self, usize)> {
        Ok(((), 0))
    }
}

// ---------------------------------------------------------------------------
// String — XDR `string<>` (§4.11)
//
// Encoding: 4-byte length (bytes, not chars) + UTF-8 data + NUL-pad to 4 bytes.
// The maximum length limit of 4 GiB is not enforced; callers should validate
// application-level constraints before calling pack().
// ---------------------------------------------------------------------------

/// Default maximum string length (64 MiB) — sanity cap for decode.
///
/// CSA `buffer<>` payloads (e.g. `cms_archive_res.data`) can reach tens of
/// megabytes; the old 4 MiB cap caused spurious `SizeLimit` errors on large
/// archive responses.  64 MiB is still well below the 4 GiB XDR wire limit and
/// prevents run-away allocation from malformed data.
const MAX_STRING_LEN: usize = 64 * 1024 * 1024;

impl<Out: Write> Pack<Out> for String {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        let bytes = self.as_bytes();
        let len = bytes.len();
        // XDR length field is 4 bytes (u32). Strings longer than u32::MAX
        // cannot be represented; return an error rather than silently truncate.
        let len32 = u32::try_from(len)
            .map_err(|_| XdrError::size_limit(len, u32::MAX as usize))?;
        len32.pack(out)?;
        out.write_all(bytes)?;
        let pad = write_padding(out, len)?;
        Ok(4 + len + pad)
    }
}

impl<In: Read> Unpack<In> for String {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        let (len, _) = u32::unpack(input)?;
        let len = len as usize;
        if len > MAX_STRING_LEN {
            return Err(XdrError::size_limit(len, MAX_STRING_LEN));
        }
        let mut buf = vec![0u8; len];
        input.read_exact(&mut buf)?;
        let pad = read_padding(input, len)?;
        let s = String::from_utf8(buf)?;
        Ok((s, 4 + len + pad))
    }
}

// ---------------------------------------------------------------------------
// Vec<T: Pack+Unpack> — XDR variable-length array `T array<>` (§4.13)
// ---------------------------------------------------------------------------

/// Maximum element count for variable arrays — sanity cap for decode.
const MAX_ARRAY_ELEMS: usize = 1_000_000;

impl<T: Pack<Out>, Out: Write> Pack<Out> for Vec<T> {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        // XDR array count field is 4 bytes; reject oversized inputs.
        let count32 = u32::try_from(self.len())
            .map_err(|_| XdrError::size_limit(self.len(), u32::MAX as usize))?;
        let mut sz = count32.pack(out)?;
        for item in self {
            sz += item.pack(out)?;
        }
        Ok(sz)
    }
}

impl<T: Unpack<In>, In: Read> Unpack<In> for Vec<T> {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        let (count, mut sz) = u32::unpack(input)?;
        let count = count as usize;
        if count > MAX_ARRAY_ELEMS {
            return Err(XdrError::size_limit(count, MAX_ARRAY_ELEMS));
        }
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            let (item, item_sz) = T::unpack(input)?;
            sz += item_sz;
            vec.push(item);
        }
        Ok((vec, sz))
    }
}

// ---------------------------------------------------------------------------
// Option<T> — XDR optional data `T *` (§4.19)
//
// Encoding: 4-byte bool (0 = absent, 1 = present) + optional T.
// This matches how xdr_pointer() works in libtirpc.
// ---------------------------------------------------------------------------

impl<T: Pack<Out>, Out: Write> Pack<Out> for Option<T> {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        match self {
            None => false.pack(out),
            Some(v) => {
                let sz = true.pack(out)?;
                Ok(sz + v.pack(out)?)
            }
        }
    }
}

impl<T: Unpack<In>, In: Read> Unpack<In> for Option<T> {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        let (present, sz) = bool::unpack(input)?;
        if !present {
            return Ok((None, sz));
        }
        let (v, vsz) = T::unpack(input)?;
        Ok((Some(v), sz + vsz))
    }
}

// ---------------------------------------------------------------------------
// Box<T> — transparent wrapper (used by xdrgen for recursive struct pointers)
// ---------------------------------------------------------------------------

impl<T: Pack<Out>, Out: Write> Pack<Out> for Box<T> {
    fn pack(&self, out: &mut Out) -> Result<usize> {
        self.as_ref().pack(out)
    }
}

impl<T: Unpack<In>, In: Read> Unpack<In> for Box<T> {
    fn unpack(input: &mut In) -> Result<(Self, usize)> {
        let (v, sz) = T::unpack(input)?;
        Ok((Box::new(v), sz))
    }
}
