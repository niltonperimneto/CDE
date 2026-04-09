# XDR Codec Replacement Plan

> Branch: `claude/review-rust-migration-Fe9XF`  
> Date: 2026-04-07  
> Scope: Replace `xdr-codec = "0.2"` in `cde/lib/csa/rust/`

---

## 1. Problem Statement

`xdr-codec 0.2` is the only unmaintained dependency remaining in the CDE Rust
migration.  Last published: **2018**.  Problems it creates:

| Problem | Impact |
|---|---|
| No security patches | CVEs in XDR parsing go unaddressed |
| Rust edition drift | Generates `edition = "2015"` artefacts; breaks future edition upgrades |
| No `cargo audit` green-light | Blocks CI gating on RUSTSEC advisories |
| `xdrgen` coupling | The code generator targets this crate's trait names; migration path is opaque |
| No fuzz coverage | Unknown decode behaviour under adversarial byte sequences |

---

## 2. Current Architecture

```
   .x IDL files  (cm.x, rtable4.x, rtable3.x, rtable2.x, agent.x)
         │
         ▼
   build.rs preprocessor
         │  • strips % / # lines
         │  • patches pointer+count → XDR array syntax
         │  • injects missing typedef stubs
         ▼
      xdrgen  (external binary, reads XDR IDL)
         │  generates Rust structs that derive
         │  xdr_codec::Pack + xdr_codec::Unpack
         ▼
   OUT_DIR/{cm,rtable4,rtable3,rtable2,agent}.rs
         │  included into lib.rs via include!()
         │
   xdr_stubs.rs  ────────────────────────────────────┐
         │  22× impl_xdr_func! macro                 │
         │  calls xdr_codec::pack()                  │
         │  calls xdr_codec::unpack()                │
         │                                           │
   client.rs  ─────────────────────────────────────── │
         │  44× .pack(&mut stream)                   │
         │   9× xdr_codec::Unpack import             │
         │                                           │
         ▼                                           │
   XdrStream (xdr_adapter.rs)  ◄───────────────────── ┘
         │  implements std::io::Read + Write
         │  delegates to C XDR x_getbytes/x_putbytes
         ▼
   ONC RPC / libtirpc (C runtime)
         │
         ▼
   Network (UDP / TCP)
```

### Types that carry Pack + Unpack implementations

**26 distinct types** across 3 protocol-version families:

| Module | Types | Notes |
|---|---|---|
| `cm` | ~18 types | Generated from cm.x; calendar wire types |
| `rtable4` | 16 types | Appointment table protocol v4 |
| `rtable3` | 5 types | Protocol v3 (backward compat) |
| `rtable2` | 5 types | Protocol v2 (legacy) |
| `agent` | 2–4 types | Agent registration |

### xdr-codec API surface actually used

```rust
// Traits
pub trait Pack        { fn pack<W: Write>(&self, out: &mut W) -> Result<usize>; }
pub trait Unpack: Sized { fn unpack<R: Read>(r: &mut R) -> Result<(Self, usize)>; }

// Free functions (thin wrappers)
pub fn pack<T: Pack, W: Write>(v: &T, w: &mut W) -> Result<usize>;
pub fn unpack<T: Unpack, R: Read>(r: &mut R) -> Result<(T, usize)>;

// Error type
pub struct Error { ... }
impl Error {
    pub fn invalidenum() -> Self;   // used by generated union match arms
    pub fn invalidcase() -> Self;   // used by generated discriminated unions
}
pub type Result<T> = std::result::Result<T, Error>;
```

---

## 3. Wire Format Contract (RFC 4506 / RFC 1832)

All types must produce byte-for-byte identical output to the C `libtirpc` XDR
routines.  The relevant encoding rules:

| XDR type | Encoding |
|---|---|
| `int` / `unsigned int` | 4 bytes, big-endian |
| `hyper` / `unsigned hyper` | 8 bytes, big-endian |
| `bool` | 4 bytes (0 or 1) |
| `string<>` | 4-byte length + UTF-8 bytes + NUL padding to 4-byte boundary |
| `opaque<N>` (fixed) | N bytes + padding to 4-byte boundary |
| `opaque<>` (variable) | 4-byte length + bytes + padding |
| `T array<N>` (fixed) | N × T |
| `T array<>` (variable) | 4-byte count + count × T |
| `T *` (optional pointer) | 4-byte bool (0=absent,1=present) + optional T |
| `union switch(int D) { ... }` | Encode D as int32 + matching case |

**Critical invariant:** the `xdr_void` case (no data) must encode to zero bytes.

---

## 4. Replacement Options

### 4.1 Vendor xdr-codec (interim)

Copy the xdr-codec 0.2 source into `cde/lib/csa/rust/vendor/xdr_codec/` and
reference it as a path dependency.

```toml
[dependencies]
xdr-codec = { path = "vendor/xdr_codec" }
```

**Pros:** Zero code changes, immediate `cargo audit` relief, we own the source.  
**Cons:** We inherit 2015-edition code, still no tests, still no fuzz targets.  
**Verdict:** Appropriate as a short-term bridge (≤ 1 sprint) while the proper
replacement is built.

---

### 4.2 `serde_xdr` crate

Replace `xdr-codec` with the `serde_xdr` crate, which implements a serde
`Serializer`/`Deserializer` for XDR.

**Pros:** Serde ecosystem; types get `#[derive(Serialize, Deserialize)]`.  
**Cons:**
- `serde_xdr` maps XDR discriminated unions to serde's enum model, which does
  **not** match the ONC RPC convention for all cases.  Specifically,
  `cms_attribute_value`'s 15-case union encodes the discriminant as an `int32`
  — serde enums typically encode indices, not explicit discriminants.
- The `xdrgen`-generated code derives `xdr_codec::Pack/Unpack`, not
  `serde::Serialize/Deserialize`.  Switching would require either rewriting
  xdrgen or hand-writing serde impls for all 26 types.
- `serde_xdr` has its own known bugs with string padding (trailing NUL
  handling differs from libtirpc on some paths).

**Verdict:** Not recommended.  The impedance mismatch between serde's type
model and XDR discriminated unions is too high for the calendar service types.

---

### 4.3 Custom `cde_xdr` micro-crate (recommended)

Write a small, focused crate (`cde/lib/csa/cde_xdr/`) that exposes **exactly**
the same trait surface as xdr-codec 0.2 and implements RFC 4506 encoding using
`byteorder`.

**Pros:**
- Zero changes to `xdr_stubs.rs`, `client.rs`, or any generated code — the
  trait names are identical.
- `byteorder` is battle-tested, well-maintained, and a transitive dependency of
  many Rust crates already.
- Full fuzz coverage is straightforward: one target per wire type.
- The crate is small enough (~400 LoC) to audit completely.
- Enables future `#[derive(XdrEncode, XdrDecode)]` proc-macro if needed.

**Cons:** ~2 days of engineering to write and test all primitive impls.  

**Verdict:** Best path forward.  Detailed specification follows.

---

### 4.4 Full protocol replacement (tarpc / D-Bus)

Replace the ONC RPC transport for `dtcm` with a modern RPC framework, making
XDR encoding unnecessary.

**Pros:** Eliminates the entire XDR attack surface; consistent with the
ToolTalk → D-Bus migration already underway.  
**Cons:** Requires changes to the C `dtcm` daemon, breaks wire compatibility
with any existing `dtcm` clients (networked calendar access), and is at least a
2-sprint effort.  
**Verdict:** Long-term goal (Phase 4 of the migration roadmap), not an
immediate replacement for `xdr-codec`.

---

## 5. Recommended Approach: Phased Plan

### Phase A — Vendor (< 1 day)

Create `cde/lib/csa/cde_xdr/` with the crate skeleton and update
`libcsa_xdr/Cargo.toml` to use it as a path dependency.  The vendor copy
ensures CI is unblocked while Phase B is completed.

**Done in this branch.**

---

### Phase B — Core `cde_xdr` implementation (~2 days)

Implement all RFC 4506 primitives and wire the `XdrStream` adapter to them.

**Crate layout:**

```
cde/lib/csa/cde_xdr/
├── Cargo.toml
└── src/
    ├── lib.rs          — public API re-exports + free functions
    ├── error.rs        — XdrError type
    ├── primitives.rs   — Pack/Unpack for all scalar and container types
    └── tests/
        ├── roundtrip.rs — property-based roundtrip tests
        └── compat.rs    — golden-byte tests vs C libtirpc output
```

**Trait definitions (drop-in compatible with xdr-codec 0.2):**

```rust
pub type Result<T> = std::result::Result<T, XdrError>;

/// Encodes a value into an XDR byte stream.
/// Returns the number of bytes written.
pub trait Pack {
    fn pack<W: std::io::Write>(&self, w: &mut W) -> Result<usize>;
}

/// Decodes a value from an XDR byte stream.
/// Returns the value and the number of bytes consumed.
pub trait Unpack: Sized {
    fn unpack<R: std::io::Read>(r: &mut R) -> Result<(Self, usize)>;
}

pub fn pack<T: Pack, W: std::io::Write>(v: &T, w: &mut W) -> Result<usize> {
    v.pack(w)
}

pub fn unpack<T: Unpack, R: std::io::Read>(r: &mut R) -> Result<(T, usize)> {
    T::unpack(r)
}
```

**Primitive implementations (all RFC 4506 §4):**

| Rust type | XDR encoding | Bytes written |
|---|---|---|
| `i32` | 4-byte big-endian signed | 4 |
| `u32` | 4-byte big-endian unsigned | 4 |
| `i64` | 8-byte big-endian signed (hyper) | 8 |
| `u64` | 8-byte big-endian unsigned | 8 |
| `bool` | 4-byte int (0=false, 1=true) | 4 |
| `f32` | 4-byte IEEE 754 single | 4 |
| `f64` | 8-byte IEEE 754 double | 8 |
| `String` | u32 len + bytes + NUL-pad to 4 | 4 + len + pad |
| `Vec<u8>` | u32 len + bytes + NUL-pad to 4 | 4 + len + pad |
| `Vec<T: Pack>` | u32 count + T… | 4 + Σ(T) |
| `Option<T>` | u32 present (0/1) + optional T | 4 [+ T] |
| `()` | nothing | 0 |

**XdrError:**

```rust
#[derive(Debug)]
pub struct XdrError {
    kind: XdrErrorKind,
}

#[derive(Debug)]
enum XdrErrorKind {
    Io(std::io::Error),
    /// Discriminant value did not match any known enum variant.
    InvalidEnum,
    /// Discriminant matched an enum arm but the case data was invalid.
    InvalidCase,
    /// String data is not valid UTF-8.
    InvalidUtf8(std::str::Utf8Error),
    /// Variable-length data exceeded the declared maximum.
    SizeLimit,
}

impl XdrError {
    pub fn invalidenum() -> Self { Self { kind: XdrErrorKind::InvalidEnum } }
    pub fn invalidcase() -> Self { Self { kind: XdrErrorKind::InvalidCase } }
}
impl From<std::io::Error> for XdrError { ... }
impl std::error::Error for XdrError { ... }
impl std::fmt::Display for XdrError { ... }
```

---

### Phase C — build.rs patch (~half day)

Update the `build.rs` to replace the xdrgen error-shim patches with `cde_xdr`
references:

```rust
// Old
.replace("xdr_codec :: Error :: invalidenum", "crate :: my_shim :: invalidenum")
.replace("xdr_codec :: Error :: invalidcase", "crate :: my_shim :: invalidcase")

// New (cde_xdr uses the same error method names — no patching needed)
// These patches can be removed entirely; the methods exist directly on cde_xdr::XdrError.
```

Also replace the xdrgen-generated `use xdr_codec` imports:

```rust
.replace("use xdr_codec", "use cde_xdr as xdr_codec")
// This single alias means zero changes to the generated impl bodies.
```

---

### Phase D — Remove xdr-codec dependency (~10 min)

```toml
# libcsa_xdr/Cargo.toml — remove:
xdr-codec = "0.2"

# Add:
cde_xdr = { path = "../cde_xdr" }
```

Remove the `my_shim` module from `lib.rs` (no longer needed).

---

### Phase E — Fuzz targets (~1 day)

Add `cargo-fuzz` targets in `cde/lib/csa/cde_xdr/fuzz/`:

```
fuzz/
├── Cargo.toml
└── fuzz_targets/
    ├── fuzz_pack_unpack_cm.rs          — roundtrip all cm.x types
    ├── fuzz_pack_unpack_rtable4.rs     — roundtrip all rtable4 types
    └── fuzz_decode_arbitrary.rs        — feed random bytes to Unpack impls
```

The roundtrip fuzzer structure:

```rust
fuzz_target!(|data: (CmsAttribute, Vec<u8>)| {
    let mut buf = Vec::new();
    if cde_xdr::pack(&data.0, &mut buf).is_ok() {
        let mut cursor = std::io::Cursor::new(&buf);
        let _ = cde_xdr::unpack::<CmsAttribute, _>(&mut cursor);
    }
});
```

The arbitrary-decode fuzzer ensures no panic on malformed input:

```rust
fuzz_target!(|data: &[u8]| {
    let mut cursor = std::io::Cursor::new(data);
    // None of these must panic — errors must be returned, not unwrapped.
    let _ = cde_xdr::unpack::<cms_open_res, _>(&mut cursor);
    let _ = cde_xdr::unpack::<cms_attribute_value, _>(&mut cursor);
    let _ = cde_xdr::unpack::<Table_Res_4, _>(&mut cursor);
});
```

---

### Phase F — Golden-byte compatibility tests (~half day)

Write a C test program that encodes each type via `libtirpc` XDR routines and
writes the bytes to stdout.  Then write a Rust test that decodes the same bytes
using `cde_xdr` and asserts structural equality.

```
cde/lib/csa/
├── tests/
│   ├── golden_bytes/        — pre-generated C libtirpc output (committed binary)
│   └── compat_test.rs       — Rust tests that decode the golden bytes
```

This ensures byte-for-byte wire compatibility without a live C environment.

---

## 6. Migration Checklist

```
[x] Audit: identify all xdr-codec usages (this document)
[ ] Phase A: create cde_xdr crate skeleton + vendor copy for CI unblock
[ ] Phase B: implement all RFC 4506 primitives with byteorder
[ ] Phase B: implement Pack/Unpack for String, Vec<T>, Option<T>, ()
[ ] Phase B: implement XdrError with invalidenum()/invalidcase()
[ ] Phase B: add roundtrip unit tests for every primitive
[ ] Phase C: update build.rs alias patch
[ ] Phase D: remove xdr-codec = "0.2" from Cargo.toml
[ ] Phase D: remove my_shim module from lib.rs
[ ] Phase E: add cargo-fuzz targets
[ ] Phase F: golden-byte compat tests vs libtirpc
[ ] Run: cargo test --workspace
[ ] Run: cargo audit (should be clean)
[ ] Run: cargo fuzz run fuzz_decode_arbitrary -- -max_total_time=300
```

---

## 7. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| String padding differs from libtirpc | Medium | High — wire corruption | Golden-byte tests (Phase F) catch this before release |
| `cms_attribute_value` 15-case union has undocumented cases | Medium | Medium — decode error | Fuzz target covers arbitrary discriminants |
| `xdrgen` output format changes between installs | Low | High — build breaks | Commit pre-generated `.rs` files as fallback |
| `byteorder` API changes | Very low | Low | Pin major version; byteorder is extremely stable |
| 32-bit `time_t` in XDR wire format | Certain | Long-term | Tracked as P4-2; out of scope for this replacement |

---

## 8. Long-term Vision

Once `cde_xdr` is in place, Phase 4 of the migration roadmap can eliminate the
entire XDR layer for the calendar service:

```
Current:  dtcm ←──ONC RPC / XDR──→ libcsa (Rust shim) ←── CDE apps
Future:   dtcm ←──D-Bus (zbus)───→ libcsa (Rust pure) ←── CDE apps
```

This mirrors the completed ToolTalk → D-Bus migration and would make `dtcm`
sessionable via systemd user services, sandboxable via `RestrictAddressFamilies`,
and auditable via the D-Bus system log — security improvements that are
structurally impossible with ONC RPC.

---

*Generated by Claude Code as part of the CDE Rust migration review.*
