#!/bin/bash
# Local fuzzing test harness for CDE Rust migration
#
# Usage:
#   ./fuzz_all.sh             # Run all fuzz targets for 60s each
#   ./fuzz_all.sh --asan      # Run with AddressSanitizer
#   ./fuzz_all.sh --ubsan     # Run with UndefinedBehaviorSanitizer
#   ./fuzz_all.sh --time 300  # Run for 300 seconds each
#
# Requires:
#   cargo +nightly install cargo-fuzz

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Parse arguments
MODE="standard"
TIME="60"

while [[ $# -gt 0 ]]; do
    case $1 in
        --asan)
            MODE="asan"
            shift
            ;;
        --ubsan)
            MODE="ubsan"
            shift
            ;;
        --time)
            TIME="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Set up environment
RUST_FLAGS=""
case $MODE in
    asan)
        echo "=== Fuzzing with AddressSanitizer ==="
        RUST_FLAGS="-Z sanitizer=address"
        ;;
    ubsan)
        echo "=== Fuzzing with UndefinedBehaviorSanitizer ==="
        RUST_FLAGS="-Z sanitizer=undefined"
        ;;
    *)
        echo "=== Fuzzing (standard) ==="
        ;;
esac

# Verify cargo-fuzz is installed
if ! cargo +nightly fuzz --version &>/dev/null; then
    echo "Installing cargo-fuzz..."
    cargo +nightly install cargo-fuzz --locked
fi

# Define fuzz targets
declare -a FUZZ_TARGETS=(
    "cde/lib/csa/cde_xdr:fuzz_decode_arbitrary"
    "cde/lib/csa/cde_xdr:fuzz_roundtrip"
    "cde/lib/tt/libtt_shim:fuzz_message_encoding"
    "cde/lib/DtSearch/rust:fuzz_query_parser"
)

# Run each target
for target in "${FUZZ_TARGETS[@]}"; do
    IFS=':' read -r crate fuzz_target <<< "$target"
    
    echo ""
    echo "Running: $crate / $fuzz_target (${TIME}s)"
    echo "=========================================="
    
    cd "$crate/fuzz"
    
    if [ -n "$RUST_FLAGS" ]; then
        RUSTFLAGS="$RUST_FLAGS" cargo +nightly fuzz run "$fuzz_target" -- \
            -max_total_time="$TIME" \
            -timeout=5 \
            -print_final_stats=1 || true
    else
        cargo +nightly fuzz run "$fuzz_target" -- \
            -max_total_time="$TIME" \
            -timeout=5 \
            -print_final_stats=1
    fi
    
    cd "$SCRIPT_DIR"
done

echo ""
echo "=== Fuzzing Complete ==="
echo "Check for crashes in: cde/*/fuzz/artifacts/"
