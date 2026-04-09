# CDE Rust Migration — Code Review & Prioritised Port Plan

> Branch: `claude/review-rust-migration-Fe9XF`  
> Reviewed: 2026-04-06  
> Scope: all 30 Rust source files across 11 crates

---

## 1. Executive Summary

The migration replaces legacy C (2 476 source files) with Rust across six subsystems:
**DtHelp**, **DtSearch**, **ToolTalk/libtt\_shim**, **CSA/Calendar**, **dtBrowser**, and
**dtterm\_shim**.  The overall architecture is sound — one crate per subsystem, clear FFI
boundaries, modern async/D-Bus IPC — but several modules contain undefined behaviour,
memory leaks, and incomplete implementations that would cause silent data corruption,
crashes, or privilege-escalation paths in production.  This document catalogues every
finding and proposes a phased remediation roadmap.

---

## 2. Rust Code Review Findings

### 2.1 CSA Calendar RPC Layer (`cde/lib/csa/rust/`)

**Severity: CRITICAL**

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| C-1 | `client.rs` | 86 | `static mut RES: CSA_return_code = 0` — mutable static shared across threads | Data race → UB |
| C-2 | `client.rs` | 97–99 | `std::mem::transmute` to coerce `fn` to variadic `xdrproc_t` — ABI mismatch | Stack corruption |
| C-3 | `client.rs` | 38 | `panic!("cl_ops->cl_call is NULL")` inside `unsafe extern "C"` — unwind crosses FFI | Process abort or worse |
| C-4 | `xdr_stubs.rs` | 31 | `std::ptr::drop_in_place(obj)` on `XDR_FREE` without knowing allocation ownership | Double-free / UB |
| C-5 | `conversion.rs` | 28–37 | Recursive `convert_access_list` with no depth bound — unbounded stack on malicious input | Stack overflow |
| C-6 | `client.rs` | 19–39 | `pub unsafe fn clnt_call` exposes raw `CLIENT*` without lifetime annotation | Use-after-free |

**Fixes required:**

```rust
// C-1: replace static mut with thread_local or Mutex-wrapped Option
thread_local! {
    static RES: std::cell::Cell<CSA_return_code> = std::cell::Cell::new(0);
}

// C-3: never panic across FFI; return an error code instead
if ops.cl_call.is_none() {
    return clnt_stat::RPC_SYSTEMERROR;
}

// C-4: XDR_FREE must only free data the Rust side owns; add an ownership flag
// C-5: add depth counter; return error at depth > 512
```

---

### 2.2 ToolTalk D-Bus Bridge (`cde/lib/tt/libtt_shim/`)

**Severity: HIGH**

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| T-1 | `lib.rs` | 150–153 | `CString::new(...).unwrap().into_raw()` — pointer returned to C, never reclaimed | Memory leak (unbounded) |
| T-2 | `lib.rs` | 176–185 | `tt_free` is a no-op (reclaim line commented out) — leaks every pointer given by `tt_open` / `tt_default_session` | Memory leak |
| T-3 | `lib.rs` | 131–134 | `libc::write()` return value ignored — silent failure if pipe is full | Lost wake-up, event starvation |
| T-4 | `lib.rs` | 138–141 | Error path in `listen_loop` sleeps 500 ms then retries — no circuit-breaker | D-Bus spam, CPU burn |
| T-5 | `lib.rs` | 14–15 | `TT_WRN_NOTFOUND = 1` is a placeholder comment — real values must match `tt_c.h` | Wrong status returned to callers |
| T-6 | `lib.rs` | 281–288 | Signal body serialised as `&()` — all ToolTalk arguments silently dropped | Broken IPC |

**Fixes required:**

```rust
// T-1/T-2: track allocations in a registry
static ALLOC_REGISTRY: Lazy<Mutex<HashSet<usize>>> = ...;
// in tt_free: check registry, then call CString::from_raw only for Rust-owned ptrs

// T-3: handle EINTR/EAGAIN properly
loop {
    match unsafe { libc::write(fd, buf.as_ptr() as *const c_void, 1) } {
        1 => break,
        -1 => { let e = std::io::Error::last_os_error(); if e.kind() != ErrorKind::Interrupted { break; } }
        _ => break,
    }
}

// T-6: implement zbus Body serialisation for TtMessage args
```

---

### 2.3 DtSearch Full-Text Search (`cde/lib/DtSearch/rust/`)

**Severity: HIGH**

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| S-1 | `lib.rs` | 43–56 | `DtSearchInit`, `DtSearchReinit` are `println!` stubs — callers believe the engine is initialised | Silent no-op |
| S-2 | `parser.rs` | 100 | `let dblk = usrblk.dblk` — `usrblk` is a C global; read without synchronisation | Data race with C callers |
| S-3 | `parser.rs` | 119 | `fread` result trusted; no check that `read_count == count` before indexing | Buffer over-read |
| S-4 | `lib.rs` | 134–137 | `DtSearchExit` calls `std::process::exit` directly — skips Rust destructors and C `atexit` handlers | Resource leaks, corrupt state |
| S-5 | `lib.rs` | 123–126 | `DtSearchGetMessages` returns a static string — concurrent callers sharing the same pointer | Thread-safety issue |

**Fixes required:**

```rust
// S-1: implement a real init state machine with AtomicBool
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// S-4: propagate exit via return code, let C caller invoke exit()
pub extern "C" fn DtSearchExit(exit_code: c_int) {
    // clean up Rust resources
    INITIALIZED.store(false, Ordering::SeqCst);
    // return to C; C side calls exit()
}
```

---

### 2.4 ToolTalk Session Daemon (`cde/lib/tt/rust/ttsession/`)

**Severity: MEDIUM**

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| TS-1 | `main.rs` | 25 | `.serve_at(…, manager.clone())?` — `ToolTalkManager` not yet `Clone`; compile error | Build failure |
| TS-2 | `main.rs` | 23 | `ConnectionBuilder::session()?` — uses zbus 3.x API but crate depends on zbus 5.x | API mismatch / compile error |
| TS-3 | `main.rs` | 21 | Hard-coded path `/etc/dt/tt/types` — breaks on user installs | Portability |
| TS-4 | `manager.rs` | — | `load_ptypes` ignores file-not-found errors silently | Silent failure |

---

### 2.5 DtHelp Text Rendering Engine (`cde/lib/DtHelp/rust/engine/`)

**Severity: MEDIUM**

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| H-1 | `lib.rs` | 86 | `slice::from_raw_parts_mut(pixels, width * height * 4)` — no validation that `width`/`height` are nonzero or that the product doesn't overflow | Buffer overread |
| H-2 | `lib.rs` | 57–120 | `dthelp_engine_render` is not thread-safe; `DtHelpEngine` mutably aliased through raw pointer | Data race |
| H-3 | `lib.rs` | 77 | `set_scroll` API changed in cosmic-text > 0.11 — likely compile error on upgrade | API drift |

**Fix for H-1:**
```rust
let byte_count = (width as usize)
    .checked_mul(height as usize)
    .and_then(|n| n.checked_mul(4))
    .filter(|&n| n > 0)
    .expect("invalid dimensions");
let data = unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, byte_count) };
```

---

### 2.6 dtterm\_shim Terminal Wrapper (`cde/programs/dtterm_shim/`)

**Severity: LOW** — cleanest module in the project; only minor issues.

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| DT-1 | `main.rs` | 103–104 | Geometry string (`-geometry WxH+X+Y`) is silently ignored | CDE apps that pass geometry get wrong window placement |
| DT-2 | `main.rs` | 64–74 | Theme load failure silently uses white-on-black defaults without warning the session manager | Poor UX |
| DT-3 | `cde_theme.rs` | — | TOML config written to `$XDG_CONFIG_HOME/dtterm_shim/` — not cleaned up on exit | Config drift |

---

### 2.7 dtBrowser IPC & Server (`cde/programs/dtbrowser/`)

**Severity: HIGH** — both crates are near-empty stubs with no real logic.

| # | File | Line | Issue | Impact |
|---|------|------|-------|--------|
| B-1 | `ipc/src/lib.rs` | 31, 40 | `CStr::from_ptr(op)`, `CStr::from_ptr(arg)` — no null-check before dereference | Null deref crash |
| B-2 | `server/src/lib.rs` | 13 | `CStr::from_ptr(root)` — same issue | Null deref crash |
| B-3 | Both | — | All public API functions return `0` unconditionally — browser will never report errors | Silent failure |

---

## 3. Cross-Cutting Issues

### 3.1 Dependency Health

| Crate | Version | Status | Risk |
|-------|---------|--------|------|
| `xdr-codec` | 0.2 | **Unmaintained** (last release 2018) | No security patches; XDR decode bugs unfixed |
| `zbus` | 3.14 (tt) vs 5.13 (shim) | **Version split** | Incompatible APIs; only one will compile |
| `libc` | 0.2.180 | Outdated | Missing modern Linux syscall bindings |
| `once_cell` | any | Superseded | `std::sync::OnceLock` available since Rust 1.70 |

### 3.2 Missing Testing Infrastructure

Zero test modules exist across all 11 crates.  For a safety-critical migration of this
scale this is the highest-priority process gap.  Every FFI function is untested.

### 3.3 Panic Propagation Across FFI

At least 3 locations call `panic!` or `unwrap()` inside `extern "C"` functions.
Rust's default panic handler unwinds the stack; when that unwind crosses an FFI
boundary the behaviour is **undefined** (usually a SIGABRT or silent stack
corruption).  All `extern "C"` functions must catch panics:

```rust
use std::panic;

pub extern "C" fn some_ffi_fn() -> c_int {
    match panic::catch_unwind(|| { /* real impl */ }) {
        Ok(v) => v,
        Err(_) => -1,
    }
}
```

---

## 4. Prioritised Migration Roadmap

### Phase 1 — Eliminate Undefined Behaviour (1–2 sprints)

These items cause UB or memory safety violations today and must be fixed before any
further work lands.

| ID | Component | Action | Owner |
|----|-----------|--------|-------|
| P1-1 | CSA `client.rs` | Replace `static mut RES` with `thread_local!` | Backend |
| P1-2 | CSA `client.rs` | Remove all `std::mem::transmute` of fn pointers; use proper `unsafe` fn-ptr casts | Backend |
| P1-3 | All `extern "C"` fn | Wrap every exported function body in `catch_unwind`; return error code on panic | All |
| P1-4 | `libtt_shim` | Implement `tt_free` allocation registry; fix the no-op | IPC |
| P1-5 | `libtt_shim` | Handle `libc::write` return value with retry on `EINTR` | IPC |
| P1-6 | `conversion.rs` | Add depth-bounded iteration (max 512) to `convert_access_list` | Calendar |
| P1-7 | `xdr_stubs.rs` | Remove `drop_in_place` on XDR_FREE; only free data Rust owns | Calendar |
| P1-8 | `dtbrowser` | Add null-pointer guards before all `CStr::from_ptr` calls | Browser |

### Phase 2 — Completeness & Reliability (2–4 sprints)

Turn stubs into real implementations; add error propagation.

| ID | Component | Action |
|----|-----------|--------|
| P2-1 | **DtSearch** | `INITIALIZED` AtomicBool, `DB_PATH` OnceLock, `DtSearchInit` wired | ✅ Done |
| P2-2 | **DtSearch** | `DtSearchQuery` calls real `QueryParser` + `Searcher`; `DtSearchExit` no longer calls `process::exit` | ✅ Done |
| P2-3 | **ttsession** | `ToolTalkManager` already derives `Clone`; updated to zbus 5 API | ✅ Done |
| P2-4 | **ttsession** | `ptype_db_path()` reads `$DTDIR` / `$CDE_CONFIGURATION_TOP` with `/etc/dt/tt/types` fallback | ✅ Done |
| P2-5 | **libtt_shim** | Serialise `TtMessage.args` into D-Bus body using `serde`; stop dropping arguments | pending |
| P2-6 | **libtt_shim** | Implement circuit-breaker in `listen_loop` (max 5 consecutive errors → exponential backoff) | pending |
| P2-7 | **DtHelp engine** | `checked_mul` overflow guard; zero-dimension early return in `dthelp_engine_render` | ✅ Done |
| P2-8 | **dtterm_shim** | `parse_geometry()` translates `WxH[+X+Y]` → `--dimensions C R`; position warn on Wayland | ✅ Done |
| P2-9 | **libtt_shim** | `once_cell` removed; `LazyLock`/`OnceLock` from `std::sync` used throughout | ✅ Done |

### Phase 3 — Dependency Modernisation (1–2 sprints)

| ID | Dependency | Action |
|----|-----------|--------|
| P3-1 | `xdr-codec 0.2` | Fork and vendor, or replace with a maintained alternative (`xdr` crate, or custom `impl`) |
| P3-2 | `zbus` | Pin **all** crates to `zbus = "5"` in a workspace `Cargo.toml`; remove 3.x remnants |
| P3-3 | `libc` | Upgrade to latest `0.2.x`; review any syscall wrappers for Linux 6.x changes |
| P3-4 | Build system | Add a shared `[workspace]` `Cargo.toml` at `cde/` root to unify dependency versions and enable `cargo audit` / `cargo deny` |

### Phase 4 — Testing & Security Hardening (ongoing)

| ID | Area | Action |
|----|------|--------|
| P4-1 | Unit tests | Add `#[cfg(test)]` modules to every non-FFI module; target ≥ 70% line coverage |
| P4-2 | Fuzzing | Add `cargo-fuzz` targets for: XDR decoder, DtSearch query parser, ToolTalk message parser |
| P4-3 | `cargo audit` | Add CI step; treat RUSTSEC advisories as blocking |
| P4-4 | `cargo clippy` | Enable `#![deny(clippy::pedantic)]` progressively per crate |
| P4-5 | ASAN/MSAN | Run integration tests under AddressSanitizer to catch remaining UB at the FFI boundary |
| P4-6 | `#![forbid(unsafe_code)]` | Add to crates with no FFI obligation (tt\_type\_comp, sgml2md, dtterm\_shim) |

---

## 5. Highest-ROI Rewrites

Based on attack surface, code complexity, and current stub-coverage the following
rewrites will deliver the greatest security + reliability gains per engineering hour:

### 5.1 CSA XDR/RPC Layer — Full Rust Native Rewrite  
**Why:** The C RPC layer (Sun RPC / ONC RPC) is ~30 years old, has no ASLR-friendly
build, and the current Rust shim has four UB issues.  A clean-room replacement using
`tarpc` (Rust-native async RPC) or `tonic` (gRPC) would eliminate the entire XDR
parsing attack surface and replace the obsolete `xdr-codec` crate.

**Scope:** `csa/rust/` — ~800 lines Rust, ~1 500 lines C stub.  
**Gain:** Eliminates C-1 through C-6; removes `xdr-codec` entirely.

### 5.2 ToolTalk → Pure D-Bus Protocol Replacement  
**Why:** ToolTalk was designed for a pre-network-security era.  The partial bridge
approach means all ToolTalk IPC messages pass through two serialisation layers, doubling
attack surface.  A full port to zbus with typed D-Bus interfaces would provide:
session isolation, policy-kit authentication, type-safe signals, and audit logging.

**Scope:** `libtt_shim/` + `ttsession/` — ~1 200 lines.  
**Gain:** Fixes T-1 through T-6; enables PK/systemd service hardening.

### 5.3 DtSearch — Replace Raima Database with SQLite/FTS5  
**Why:** The Raima db_VISTA III library dates to the 1980s.  The Rust layer currently
calls unsafe C function pointers into this library.  Replacing with SQLite FTS5 (via
`rusqlite`) gives a maintained, sandboxable, well-fuzzed backend.

**Scope:** `DtSearch/rust/` — ~500 lines Rust.  
**Gain:** Fixes S-1 through S-5; eliminates `raima.rs` unsafe block entirely.

---

## 6. Compatibility & Modern-Platform Targets

| Feature | Current | Target |
|---------|---------|--------|
| IPC | ToolTalk (legacy TCP/socket, broken on modern kernels) | D-Bus (systemd-activated, session-isolated) |
| Calendar RPC | ONC RPC / XDR (Sun RPC, often firewalled) | D-Bus or gRPC/TLS |
| Search DB | Raima db_VISTA III (32-bit, no thread safety) | SQLite 3 FTS5 |
| Rendering | X11 direct pixmap | cosmic-text + tiny-skia (Wayland-compatible framebuffer) |
| Terminal | dtterm (Motif widget, X11 only) | Alacritty wrapper (X11 + Wayland) |
| Config | CDE-style resource files (`~/.dt/`) | XDG Base Directory Specification |
| Build | Meson + per-crate Cargo.toml | Meson + unified Cargo workspace |

---

## 7. Immediate Action Checklist

The following items should be resolved before the next release candidate:

- [x] **P1-1** `static mut RES` → `thread_local! UnsafeCell` in `csa/rust/src/client.rs` *(done)*
- [x] **P1-2** `as_xdrproc!` macro replaces double-blind transmute in `client.rs` *(done)*
- [x] **P1-3** `panic!` → `RPC_SYSTEMERROR` return in `clnt_call` *(done)*
- [x] **P1-4** `ALLOC_REGISTRY` + `alloc_cstring()` implement proper `tt_free` *(done)*
- [x] **P1-5** Fix ignored `write()` return value in `libtt_shim/src/lib.rs` *(done — prior commit)*
- [x] **P1-6** Depth-limit `convert_access_list` recursion *(done — prior commit)*
- [x] **P1-7** `drop_in_place` → `ptr::write(zeroed())` in `xdr_stubs.rs` *(done)*
- [x] **P1-8** Add null guards in `dtbrowser` FFI entry points *(done — prior commit)*
- [x] **P3-1** Replace `xdr-codec 0.2` with in-tree `cde_xdr` crate *(done — Phases A–F complete, 65 tests pass)*
- [x] **P3-2** Unify zbus to a single version across all crates *(done — libtt_shim and ttsession both on zbus 5)*
- [x] **P3-4** Workspace `Cargo.toml` at `cde/` root *(done — 12 crates unified, env_logger 0.11 everywhere)*
- [x] **P4-3** `cargo audit` CI integration *(done — `cde/.cargo/audit.toml` + `cde/scripts/rust-ci.sh`; install cargo-audit to activate)*
- [x] **P4-6** `#![forbid(unsafe_code)]` / `#![deny(unsafe_op_in_unsafe_fn)]` on all remaining crates *(done)*

---

## Appendix: File → Issue Matrix

| File | Issues |
|------|--------|
| `csa/rust/src/client.rs` | C-1, C-2, C-3, C-6 |
| `csa/rust/src/xdr_stubs.rs` | C-4 |
| `csa/rust/src/conversion.rs` | C-5 |
| `lib/tt/libtt_shim/src/lib.rs` | T-1, T-2, T-3, T-4, T-5, T-6 |
| `lib/tt/rust/ttsession/src/main.rs` | TS-1, TS-2, TS-3 |
| `lib/tt/rust/ttsession/src/manager.rs` | TS-4 |
| `lib/DtSearch/rust/src/lib.rs` | S-1, S-4, S-5 |
| `lib/DtSearch/rust/src/parser.rs` | S-2, S-3 |
| `lib/DtHelp/rust/engine/src/lib.rs` | H-1, H-2, H-3 |
| `programs/dtbrowser/ipc/src/lib.rs` | B-1, B-3 |
| `programs/dtbrowser/server/src/lib.rs` | B-2, B-3 |
| `programs/dtterm_shim/src/main.rs` | DT-1, DT-2 |
