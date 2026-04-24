# Fuzzing Guide for CDE Rust Migration

This document explains how to run the fuzzing harnesses for the Rust FFI boundaries in CDE. Fuzzing is essential for finding edge cases, buffer overflows, and undefined behavior in codec implementations that must handle untrusted input.

## Overview

Three fuzzing targets test critical FFI code paths:

1. **cde_xdr/fuzz/fuzz_targets/fuzz_decode_arbitrary.rs** — XDR codec
   - Tests all Unpack implementations against arbitrary byte sequences
   - Catches decode panics, infinite loops, buffer overflows

2. **lib/tt/libtt_shim/fuzz/fuzz_targets/fuzz_message_encoding.rs** — ToolTalk IPC
   - Tests message argument encoding with arbitrary data
   - Verifies mode/type string generation never panics
   - Confirms roundtrip consistency

3. **lib/DtSearch/rust/fuzz/fuzz_targets/fuzz_query_parser.rs** — DtSearch Parser
   - Tests boolean query parsing with arbitrary input
   - Detects parser panics and infinite loops
   - Verifies UTF-8 and non-UTF-8 handling

## Prerequisites

- Rust nightly toolchain (fuzzing requires unstable features)
- cargo-fuzz (installed via rustup or `cargo install cargo-fuzz`)

```bash
rustup toolchain install nightly
cargo +nightly install cargo-fuzz
```

## Running Fuzzing Locally

### Single Target (No Time Limit)

```bash
cd cde/lib/csa/cde_xdr
cargo +nightly fuzz run fuzz_decode_arbitrary
```

Press Ctrl+C to stop.  The fuzzer will output any crashes or panics found.

### Single Target (Time Limit)

```bash
cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=300
```

Runs for 300 seconds then exits. Useful for CI pipelines.

### Run All Fuzz Targets

```bash
#!/bin/bash
set -e

cd cde/lib/csa/cde_xdr
cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=60
cargo +nightly fuzz run fuzz_roundtrip -- -max_total_time=60

cd ../../tt/libtt_shim
cargo +nightly fuzz run fuzz_message_encoding -- -max_total_time=60

cd ../../DtSearch/rust
cargo +nightly fuzz run fuzz_query_parser -- -max_total_time=60
```

## Fuzzing with Sanitizers

Sanitizers detect memory errors that the fuzzer might trigger. Running with both ASan and UBSan catches additional bugs:

### AddressSanitizer (ASan)

Detects use-after-free, buffer overflow, memory leaks:

```bash
cd cde/lib/csa/cde_xdr
RUSTFLAGS="-Z sanitizer=address" cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=60
```

### UndefinedBehaviorSanitizer (UBSan)

Detects integer overflow, misaligned pointers, invalid shifts:

```bash
RUSTFLAGS="-Z sanitizer=undefined" cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=60
```

### Combined (ASan + UBSan)

```bash
RUSTFLAGS="-Z sanitizer=address -Z sanitizer=undefined" \
  cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=60
```

## Interpreting Results

### Success

```
#0  READ units: 1 exec/s: 5
#8192  NEW    cov: 512 ft: 1234 corp: 3/15b exec/s: 41
```

The fuzzer found new code paths (NEW) and increased coverage. No panic = success.

### Failure: Panic

```
thread '<unnamed>' panicked at 'assertion failed: len > 0', src/codec.rs:123
...
SEGV on unknown address 0x123456 (T=0 [0,0] in unknown-vm-region)
...
ERROR: libFuzzer encountered a fatal error. Exiting.
```

A panic was triggered. The fuzzer will save the input that caused it to:

```
artifacts/fuzz_decode_arbitrary/crash-<hash>
```

### Failure: ASan

```
==12345==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x607...
READ of size 4 at 0x607...
...
#0 0x123456 in unpack_vec src/codec.rs:45
#1 0x789abc in fuzz_target src/lib.rs:30
```

A buffer overflow was detected. The crash artifact can be replayed:

```bash
cargo +nightly fuzz run fuzz_decode_arbitrary artifacts/fuzz_decode_arbitrary/crash-<hash>
```

## Reproducing Failures

If a fuzz run finds a crash, the input is saved to `artifacts/`:

```bash
# List crashes
ls -la cde/lib/csa/cde_xdr/artifacts/fuzz_decode_arbitrary/

# Replay a crash
cargo +nightly fuzz run fuzz_decode_arbitrary -- artifacts/fuzz_decode_arbitrary/crash-<hash>
```

This is useful for confirming a fix:

```bash
# 1. Run fuzzer, find crash
cargo +nightly fuzz run fuzz_decode_arbitrary

# 2. Fix the bug in source code
# ... edit src/codec.rs ...

# 3. Verify fix by replaying crash
cargo +nightly fuzz run fuzz_decode_arbitrary -- artifacts/fuzz_decode_arbitrary/crash-<hash>
```

If no crash is printed, the fix worked.

## Corpus Management

The fuzzer keeps a corpus of interesting inputs in the `corpus/` directory:

```bash
# View corpus size
du -sh cde/lib/csa/cde_xdr/corpus/fuzz_decode_arbitrary/

# Clear corpus (optional; forces fuzzer to restart from scratch)
rm -rf cde/lib/csa/cde_xdr/corpus/
```

A larger corpus = better coverage but slower startup. For CI, you may want to start with a small corpus and let it grow over time.

## CI Integration

### GitHub Actions

Add to `.github/workflows/fuzzing.yml`:

```yaml
name: Fuzzing

on:
  schedule:
    # Run nightly fuzzing at 2 AM UTC
    - cron: '0 2 * * *'
  pull_request:
    branches: [ master ]

jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
      
      - name: Install cargo-fuzz
        run: cargo +nightly install cargo-fuzz
      
      - name: Fuzz XDR codec
        run: |
          cd cde/lib/csa/cde_xdr
          cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=300
          cargo +nightly fuzz run fuzz_roundtrip -- -max_total_time=300
      
      - name: Fuzz ToolTalk message encoding
        run: |
          cd cde/lib/tt/libtt_shim
          cargo +nightly fuzz run fuzz_message_encoding -- -max_total_time=300
      
      - name: Fuzz DtSearch query parser
        run: |
          cd cde/lib/DtSearch/rust
          cargo +nightly fuzz run fuzz_query_parser -- -max_total_time=300

  fuzz-asan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
      
      - name: Install cargo-fuzz
        run: cargo +nightly install cargo-fuzz
      
      - name: Fuzz with AddressSanitizer
        env:
          RUSTFLAGS: -Z sanitizer=address
        run: |
          cd cde/lib/csa/cde_xdr
          cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=300 || true
```

This runs fuzzing on a schedule (nightly) and on every PR.

### Local Nightly Fuzzing

For local development, add a git hook to run fuzzing before commit:

```bash
#!/bin/bash
# .git/hooks/pre-push

set -e

echo "Running fuzzing checks..."

cd cde/lib/csa/cde_xdr
cargo +nightly fuzz run fuzz_decode_arbitrary -- -max_total_time=30

cd ../../tt/libtt_shim
cargo +nightly fuzz run fuzz_message_encoding -- -max_total_time=30

cd ../../DtSearch/rust
cargo +nightly fuzz run fuzz_query_parser -- -max_total_time=30

echo "Fuzzing checks passed!"
```

Make it executable:

```bash
chmod +x .git/hooks/pre-push
```

## Coverage Analysis

To measure code coverage achieved by fuzzing, use `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin

cd cde/lib/csa/cde_xdr
cargo tarpaulin --out Html --output-dir coverage/
```

This generates an HTML coverage report in `coverage/index.html`.

## Known Limitations

1. **libFuzzer seed timeout**: Very large corpus can slow startup. Clean occasionally if corpus grows > 100MB.

2. **Nondeterminism**: Fuzzing is non-deterministic; bugs found in one run may not reproduce immediately. Re-run fuzzer multiple times to confirm.

3. **Timeout vs. Infinite Loop**: A function that takes > 2 seconds (default) to process a single input will be marked as timeout. This is intentional — it catches inefficient parsing.

## References

- libFuzzer: https://llvm.org/docs/LibFuzzer/
- cargo-fuzz: https://docs.rs/cargo-fuzz/
- Arbitrary crate: https://docs.rs/arbitrary/
- ASan: https://clang.llvm.org/docs/AddressSanitizer.html
- UBSan: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
