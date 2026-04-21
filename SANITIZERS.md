# Sanitizer Testing for CDE Rust Migration

## Overview

This document explains how to run Address Sanitizer (ASan) and Undefined Behavior Sanitizer (UBSan) on the CDE Rust codebase to detect memory errors and undefined behavior.

## Prerequisites

- **Rust Nightly**: Sanitizers require unstable features
  ```bash
  rustup update nightly
  rustup target add x86_64-unknown-linux-gnu --toolchain nightly
  ```
- **LLVM Tools**: Used by the sanitizer runtime
  ```bash
  rustup component add llvm-tools-next --toolchain nightly
  ```

## Running Address Sanitizer (ASan)

Address Sanitizer detects:
- Use-after-free
- Buffer overflows
- Heap buffer overflows
- Memory leaks
- Double-free
- Invalid free

### Build with ASan

```bash
cd /path/to/CDE-rs
cargo +nightly build --workspace --profile=asan
```

### Run tests with ASan

```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test --workspace --profile=asan
```

### Example output on error

```
==12345==ERROR: AddressSanitizer: heap-use-after-free on unknown address 0x602000000090
```

## Running Undefined Behavior Sanitizer (UBSan)

Undefined Behavior Sanitizer detects:
- Integer overflow
- Misaligned pointers
- Out-of-bounds shifts
- Invalid enum values
- Null pointer dereferences

### Build with UBSan

```bash
cd /path/to/CDE-rs
cargo +nightly build --workspace --profile=ubsan
```

### Run tests with UBSan

```bash
RUSTFLAGS="-Z sanitizer=undefined" cargo +nightly test --workspace --profile=ubsan
```

### Example output on error

```
runtime error: index out of bounds: the len is 10 but the index is 15
```

## Combined sanitizer testing script

Create a `.github/workflows/sanitizers.yml` (or run locally):

```bash
#!/bin/bash
set -e

echo "=== Running ASan on all crates ==="
cargo +nightly test --workspace --profile=asan --lib --tests

echo "=== Running UBSan on all crates ==="
cargo +nightly test --workspace --profile=ubsan --lib --tests

echo "=== All sanitizer tests passed ==="
```

## CI Integration

For GitHub Actions, add this to `.github/workflows/rust.yml`:

```yaml
sanitizer-asan:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    - uses: taiki-e/install-action@cargo-hack
    - run: cargo +nightly test --workspace --profile=asan --lib --tests

sanitizer-ubsan:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
    - run: cargo +nightly test --workspace --profile=ubsan --lib --tests
```

## Interpreting Results

### ASan output

- `ERROR: AddressSanitizer: heap-use-after-free` → caller passed invalid pointer to tt_free()
- `ERROR: AddressSanitizer: SEGV on unknown address` → NULL/dangling pointer dereference
- `detected memory leaks` → allocation in ALLOC_REGISTRY that was never freed

### UBSan output

- `runtime error: index out of bounds` → buffer over-read in MAX_ACCESS_LIST_DEPTH depth counter
- `runtime error: shift exponent` → invalid bit shift in wire format conversion

## Known limitations

1. **ASan + libdbus**: The system D-Bus libraries may have pre-existing leaks that ASan reports. These are not CDE issues but can be suppressed via ASAN_OPTIONS environment variable:
   ```bash
   ASAN_OPTIONS=detect_leaks=0 cargo +nightly test --workspace --profile=asan
   ```

2. **Performance**: Sanitizers add ~2x overhead; tests run slower than normal

3. **Cross-compilation**: Sanitizers only work on platforms where LLVM has instrumentation support

## References

- Rust Sanitizers: https://github.com/rust-lang/rust/blob/master/src/doc/unstable-book/src/compiler-flags/sanitizer.md
- LLVM AddressSanitizer: https://clang.llvm.org/docs/AddressSanitizer.html
- LLVM UndefinedBehaviorSanitizer: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
