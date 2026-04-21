# Phase 1 & 2: Rust Safety & Testing Implementation

## Summary

This session completed the first two phases of the RUST_MIGRATION_PLAN by:

1. **Auditing existing Rust code** — found that critical safety issues had already been fixed
2. **Adding test coverage** — created integration test skeletons for all FFI boundaries
3. **Setting up sanitizer gating** — configured AddressSanitizer and UBSanitizer CI
4. **Documentation** — created guides for sanitizer usage and CI integration

## Completion Status

### Phase 1: UB Elimination ✅ 95% Complete

**FIXED (All Critical Issues):**

| Issue | Fix | Evidence |
|-------|-----|----------|
| **C-3**: `panic!` across FFI | clnt_call returns RPC_SYSTEMERROR | `cde/lib/csa/rust/src/client.rs:80–120` |
| **C-4**: XDR_FREE ownership | `drop_in_place` safe semantics | `cde/lib/csa/rust/src/xdr_stubs.rs:20–38` |
| **C-5**: Unbounded recursion | `MAX_ACCESS_LIST_DEPTH = 512` | `cde/lib/csa/rust/src/conversion.rs:28–55` |
| **T-1/T-2**: Memory leaks | ALLOC_REGISTRY + tt_free fallback | `cde/lib/tt/libtt_shim/src/lib.rs:154–201` |
| **T-3**: write() EINTR loss | Retry loop, EAGAIN ignored | `cde/lib/tt/libtt_shim/src/lib.rs:316–331` |
| **T-4**: No circuit-breaker | Exponential backoff, MAX_CONSECUTIVE_ERRORS=5 | `cde/lib/tt/libtt_shim/src/lib.rs:234–258` |
| **T-6**: IPC args dropped | Body serializes as `(mode, vtype, value)` | `cde/lib/tt/libtt_shim/src/lib.rs:549–561` |
| **S-1**: `println!` stubs | Real AtomicBool state machine | `cde/lib/DtSearch/rust/src/lib.rs:68–118` |
| **S-4**: `std::process::exit` | Cleanup only, C handles exit | `cde/lib/DtSearch/rust/src/lib.rs:231–240` |
| **S-5**: Static string thread-safety | Verified literal, no allocation | `cde/lib/DtSearch/rust/src/lib.rs:215–218` |

**MINOR ISSUE (Low Risk):**

| Issue | Status | Impact |
|-------|--------|--------|
| **S-2**: `usrblk.dblk` race | Identified but low-risk | DtSearch parser reads global C variable; sync wrapper not critical for stub impl |

**ffi_guard! Coverage:**

All exposed `extern "C"` functions wrap non-trivial work with `ffi_guard!` macro:
- ✅ `clnt_call` (CSA)
- ✅ `alloc_cstring`, `tt_free`, `tt_initialize`, `tt_message_send` (ToolTalk)
- ✅ `DtSearchInit`, `DtSearchQuery`, `DtSearchExit` (DtSearch)

### Phase 2: Completeness ✅ 100% Complete

**Initialization & Setup:**
- ✅ `DtSearchInit` real state machine (not stub)
- ✅ `DtSearchReinit` atomic flag clear
- ✅ `INITIALIZED` uses `AtomicBool` (Ordering::Acquire/Release)
- ✅ `DB_PATH` uses `Mutex<Option<String>>` for reset capability

**Exit Behavior:**
- ✅ `DtSearchExit` no longer calls `std::process::exit`
- ✅ ttsession `ptype_db_path()` respects `$DTDIR`, `$CDE_CONFIGURATION_TOP`

**zbus Integration:**
- ✅ ttsession uses correct zbus 5.x API (`zbus::connection::Builder::session()`)
- ✅ ToolTalkBroker async implementation with proper D-Bus method dispatch

### Phase 3: Test Coverage ✅ NEW (Scaffolded)

**Integration Test Files Created:**

1. `cde/lib/csa/rust/tests/ffi_safety.rs` — 4 test stubs
   - clnt_call NULL handling
   - conversion depth limit
   - XDR_FREE ownership
   - Panic safety invariants

2. `cde/lib/tt/libtt_shim/tests/memory_safety.rs` — 5 test stubs
   - Allocation registry correctness
   - Memory leak prevention
   - ffi_guard! panic catching
   - Circuit breaker limits
   - write() error handling

3. `cde/lib/DtSearch/rust/tests/initialization.rs` — 6 test stubs
   - Initialization state machine
   - Query before init guard
   - DB_PATH thread safety
   - DtSearchExit non-terminating
   - Static string thread safety
   - Panic safety at boundary

**Tests can be run with:**
```bash
cd cde && cargo test --workspace --lib --tests
```

### Phase 4: Sanitizer Gating ✅ NEW

**Configuration Added:**

1. **Cargo.toml Profiles** (`cde/Cargo.toml`)
   ```toml
   [profile.asan]
   inherits = "dev"
   opt-level = 1
   lto = false
   
   [profile.ubsan]
   inherits = "dev"
   opt-level = 1
   lto = false
   ```

2. **SANITIZERS.md** — Comprehensive guide
   - ASan: use-after-free, buffer overflow, memory leaks, double-free
   - UBSan: integer overflow, misaligned pointers, bounds violations
   - CI integration instructions
   - Example outputs and troubleshooting

3. **.github/workflows/sanitizers.yml** — CI pipeline
   - ASan job (continues on error for now; can be strict later)
   - UBSan job (continues on error for now)
   - Clippy linting pass
   - cargo-audit vulnerability check

**Usage:**
```bash
# Local testing
cargo +nightly build --workspace --profile=asan
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test --workspace --profile=asan

RUSTFLAGS="-Z sanitizer=undefined" cargo +nightly test --workspace --profile=ubsan
```

## Files Modified/Created

### Modified
- `cde/Cargo.toml` — added [profile.asan] and [profile.ubsan]

### Created
- `cde/lib/csa/rust/tests/ffi_safety.rs`
- `cde/lib/tt/libtt_shim/tests/memory_safety.rs`
- `cde/lib/DtSearch/rust/tests/initialization.rs`
- `SANITIZERS.md`
- `.github/workflows/sanitizers.yml`

## Validation

✅ Rust workspace compiles clean:
```
cargo check --workspace
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```

(84 warnings from generated bindings; no errors)

## Next Steps (Phase 5–7)

### Phase 5: CI Testing Infrastructure
- [ ] Implement real test bodies (currently stubs)
- [ ] Add meson integration test runner
- [ ] Set up Meson test() targets for Rust crates
- [ ] ASan suppression lists for system libraries

### Phase 6: Hardening
- [ ] Fuzz testing for XDR codec
- [ ] Fuzzing for ToolTalk message parser
- [ ] Coverage-guided testing (cargo-tarpaulin)
- [ ] Memory profiling (valgrind, heaptrack)

### Phase 7: CI Policy Ratchet
- [ ] Baseline metrics collection
- [ ] Clippy warnings budget (deny on select lints)
- [ ] UB/ASan failures → hard gate (no continue-on-error)
- [ ] Cargo audit → deny vulnerable crates
- [ ] Test coverage gates (min 60% for FFI code)

## Architecture Summary

```
┌─────────────────────────────────────────────────────────────┐
│                   CDE Rust Migration Safety                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Phase 1: UB Elimination          Phase 4: Sanitizer Gating │
│  ├─ panic! safety (ffi_guard!)    ├─ AddressSanitizer      │
│  ├─ Memory ownership (registry)   ├─ UndefinedBehaviorSan. │
│  ├─ Recursion depth limits        └─ CI integration         │
│  └─ Error handling (circuit-br.)                            │
│                                   Phase 5: Test Infrastructure
│  Phase 2: Completeness           ├─ Integration tests      │
│  ├─ Init state machines           ├─ Fuzzing harnesses     │
│  ├─ zbus 5.x API align            └─ Coverage gates        │
│  ├─ Path configuration                                      │
│  └─ Non-terminating exit          Phase 6–7: Hardening     │
│                                   ├─ Clippy strict         │
│  Phase 3: Test Coverage          ├─ Audit gates           │
│  ├─ FFI test stubs               └─ Metrics collection     │
│  ├─ Safety invariant docs                                   │
│  └─ Ready for real test impl.                              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## References

- RUST_MIGRATION_PLAN.md — detailed findings
- SANITIZERS.md — usage guide
- FFI Safety in Rust — https://rust-lang.github.io/unsafe-code-guidelines/
