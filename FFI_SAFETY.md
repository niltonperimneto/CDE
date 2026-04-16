# FFI Safety — Rust/C Boundary Contract

This document is the maintainer's guide to the concurrency, memory, and
panic-safety invariants that hold across every Rust crate in this tree whose
output is linked into C.  Read this before editing any `lib.rs` that lives
under a `rust/` subtree and exports `#[unsafe(no_mangle)] extern "C"` symbols.

The primary callers are legacy CDE code written in C89/C99; they do not
understand Rust panics, `Result`, or ownership.  Every invariant below exists
because a naive translation of idiomatic Rust would miscompile, leak, deadlock,
or invoke undefined behaviour when driven by those callers.

---

## 1. Panic safety: `ffi_guard!`

**Invariant.**  A Rust panic must never unwind across an `extern "C"` frame.
On every platform CDE targets (Linux glibc, musl, BSDs, macOS) this is
undefined behaviour — the C frame above us has no unwind metadata.  Rust 1.81+
aborts in this situation, which is at least defined, but still unacceptable:
CDE components run inside long-lived processes (dtlogin, dtsession, the CMS
daemon) where abort on a recoverable error is a regression from the original
C behaviour.

**Contract.**  Every non-trivial entry point must wrap its body in the
crate-local `ffi_guard!` macro, which uses `std::panic::catch_unwind`
+ `AssertUnwindSafe` and returns a typed fallback on unwind.

```rust
#[unsafe(no_mangle)]
pub extern "C" fn DtSearchQuery(/* ... */) -> c_int {
    ffi_guard!(DTSRFATAL, {
        // ...body that may panic (Mutex poisoning, unwrap, arithmetic, ...)
        DTSROK
    })
}
```

The macro is defined per-crate (search for `macro_rules! ffi_guard`) so that
each crate's logging prefix (`[DtSearch]`, `[libtt_shim]`, …) is baked into
the panic message.  Do **not** share a single definition across crates; the
file/line captured by `file!()`/`line!()` would then always point at the
shared definition.

**Fallback values.**
- Integer return: the crate's "fatal" sentinel (`DTSRFATAL`, `TT_ERR_INTERNAL`, …).
- `()` return: `()`.
- Pointer return: `std::ptr::null_mut()` / `std::ptr::null()`.

**What not to do.**
- Do not `panic!()` deliberately from within an FFI body — the guard converts
  it to a fatal return, but the caller has no way to distinguish that from a
  legitimate error.  Return the error code directly.
- Do not wrap the macro's body with another `std::panic::catch_unwind`; nested
  guards mask the innermost panic site.

---

## 2. Thread safety

**Invariant.**  C callers reach Rust entry points from many threads.  Several
callers (tt_dispatch, DtSearch worker pools, the CMS dispatcher) are
legitimately concurrent.  Any shared state owned by a Rust crate must be
`Send + Sync`.

**Primitives in use.**

| Primitive | When to use |
|---|---|
| `AtomicBool` | One-bit init / shutdown flags (`INITIALIZED`, `SHUTTING_DOWN`). Use `Acquire` on read, `Release` on write. |
| `OnceLock<T>` | Set-once values that never need to be reset (e.g. bindgen-generated symbol tables). |
| `LazyLock<T>` | Set-once values computed on first use. |
| `Mutex<T>` | Mutable state that may need to be reset (`DB_PATH`, the allocation registry). |
| `RwLock<T>` | Read-mostly shared state (only where profiling justified it). |
| `thread_local!` + `UnsafeCell` | ONC RPC result buffers: the C contract is "returns a pointer to a static buffer"; per-thread buffers uphold that contract without the data-race. |

**Poisoned mutex recovery.**  A mutex becomes poisoned when a holder panics
(our `ffi_guard` does not release the lock before unwinding).  Never
`.unwrap()` a lock acquisition in an FFI body — instead:

```rust
let guard = MY_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
```

This matches the original C behaviour of "the data is in an unknown state but
we keep going".  Panicking would propagate into the guard and surface as a
fatal return every subsequent call.

**Callback re-entrancy.**  If a Rust entry point calls back into C, the C code
may call back into Rust.  Drop every mutex guard *before* the callback fires;
see `tttk_Xt_input_handler` in `lib/tt/libtt_shim/src/lib.rs` for the canonical
pattern.

---

## 3. Memory model

### 3.1 The allocation registry

Historically the tt_shim crate returned C strings produced by
`CString::into_raw()`.  `tt_free()` was implemented as `libc::free()`, which
on systems where Rust's allocator is not `malloc` is **undefined behaviour**.

The fix is an allocation registry: every pointer handed to C is recorded in a
`Mutex<HashSet<usize>>`.  `tt_free()` checks membership:

- Present → `CString::from_raw()` + drop (reclaims with Rust's allocator).
- Absent → `libc::free()` (compatibility path for `tt_malloc`-produced pointers
  that C may have forwarded through some other `tt_*` API).

Always allocate C strings via the crate-local helper
(`alloc_cstring` in libtt_shim) rather than calling `CString::into_raw()`
directly; the helper registers the pointer atomically.

### 3.2 Caller-provided buffers

When a C signature passes a buffer by pointer + length (e.g.
`DtSearchHighlight` receives `*mut c_char` + `c_long`), do **not** copy into an
owned `Vec` and then memcpy back.  Construct a `&mut [u8]` with
`std::slice::from_raw_parts_mut` and operate in place.  The extra allocation
shows up in profiles for long-lived daemons.

### 3.3 Static data

C APIs that return "a pointer to a string" must have `'static` lifetime.  Use
byte-string literals (`b"...\0"`) rather than `CString` for anything whose
content is known at compile time.  See `DtSearchGetMessages` for the pattern.

### 3.4 Output parameters

When the contract says "caller must set `*out = NULL` on error" (and nearly
every DtSearch entry point says exactly that), do it *unconditionally* at
every error-return site.  A stale pointer in an out-parameter is the single
most common cause of crashes in this codebase's history.  The
`DtSearchQuery` "database unavailable" branch is the canonical example.

---

## 4. Checklist for new FFI entry points

Before merging a new `#[unsafe(no_mangle)] extern "C"` function:

- [ ] Body wrapped in `ffi_guard!(fallback, { … })`.
- [ ] Every raw pointer checked for `is_null()` before deref.
- [ ] `CStr::from_ptr` guarded with `.to_str().ok()` — never `.unwrap()`.
- [ ] Every mutex lock uses `unwrap_or_else(|e| e.into_inner())`.
- [ ] Every out-pointer is zeroed/nulled on every error path.
- [ ] C strings returned to the caller come from the allocation registry
      (via the crate's `alloc_cstring` helper) — never `CString::into_raw()`.
- [ ] Doc comment explains the C contract in terms the caller understands
      (buffer ownership, lifetimes, thread safety, return-code meanings).
- [ ] No `std::process::exit` — return to C and let C decide.

---

## 5. Known caller assumptions that are *not* Rust-idiomatic

These are places where Rust's defaults would be wrong and you must fight the
compiler:

- **Re-entrant init.**  Many callers call `DtSearchInit` more than once.  It
  must be idempotent, not "the second call is an error".
- **Implicit success on missing resource.**  `DtSearchQuery` returns 0 hits
  (not an error) when the database file is absent.  The C UI handles that
  gracefully; a `DTSRFATAL` would crash dtfile.
- **Silent acceptance of NULL arguments.**  Several `tt_*` functions treat
  NULL arguments as "use the default".  Do not reject NULL at the boundary.
- **atexit ordering.**  C callers may have registered cleanup via `atexit()`
  that depends on our data structures still being alive.  `DtSearchExit` only
  resets our own state; it does not call `std::process::exit`.

---

## 6. Verifying safety

Run the full workspace build and test matrix before touching any FFI
surface:

```sh
cargo build --workspace --exclude libcsa_xdr
cargo test  --workspace --exclude libcsa_xdr
cargo clippy --workspace --exclude libcsa_xdr -- -D warnings
```

(`libcsa_xdr` requires `libtirpc-dev` on the host.  See `lib/csa/rust/build.rs`
for the diagnostic `fail()` helper.)

For FFI-specific regressions, the CDE suites that exercise the Rust shims the
hardest are:

- `cde/programs/dttypes` (tt message round-trip)
- `cde/programs/dtfile` (DtSearch invocation)
- `cde/programs/dtcm`   (calendar recurrence)

A passing Rust test suite is necessary but not sufficient — a C smoke-test
across these three programs catches ABI drift that `cargo test` does not.

---

## 7. History

The first pass of the Rust migration (see `RUST_MIGRATION_PLAN.md`) was
scaffolded by a mix of hand-written code and AI-assisted translation.  Known
categories of issue fixed during the safety-pass are recorded in the git
log under the `claude/rust-migration-c23-*` branches; use them as examples
of what to look for when auditing other crates:

- `CString::into_raw()` bypassing the allocation registry (memory leak).
- `parser(input)` call sites expecting nom ≤ 7 (silent breakage on nom 8).
- `Tag::Heading(_,_,_)` pattern on pulldown-cmark ≥ 0.13 (compile error).
- `NamedSource` not parameterised on miette ≥ 7.1 (compile error).
- `std::process::exit` called from an FFI entry point (skipped destructors).
- Lock held across C callback (deadlock under dtfile concurrency).
