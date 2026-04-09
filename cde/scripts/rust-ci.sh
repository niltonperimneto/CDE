#!/usr/bin/env bash
# CDE Rust workspace CI script
#
# Runs the full Rust quality gate:
#   1. cargo test  --workspace  — all unit + integration tests
#   2. cargo clippy --workspace — lint check (warnings-as-errors)
#   3. cargo audit              — CVE + unmaintained dependency check
#
# Usage:
#   cd cde/
#   bash scripts/rust-ci.sh
#
# Required tools:
#   cargo (rustup default stable)
#   cargo-audit  — install with: cargo install cargo-audit
#   (cargo clippy is included with rustup)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"

cd "$WORKSPACE_DIR"

echo "=== CDE Rust workspace CI ==="
echo "Workspace: $WORKSPACE_DIR"
echo

# ── 1. Tests ────────────────────────────────────────────────────────────────
echo "[1/3] Running tests..."
# libcsa_xdr requires system headers (libtirpc, csa.h) to build its bindgen-
# generated C bindings; skip it on minimal CI hosts without full CDE headers —
# same reason it is excluded from the clippy step below.
cargo test --workspace --exclude libcsa_xdr
echo "  Tests: PASS"
echo

# ── 2. Clippy ────────────────────────────────────────────────────────────────
echo "[2/3] Running clippy..."
# libcsa_xdr requires system headers (libtirpc, csa.h) to build; skip it on
# environments without full CDE C headers.
cargo clippy \
    --workspace \
    --exclude libcsa_xdr \
    -- \
    -D warnings \
    -D clippy::unwrap_used \
    -W clippy::pedantic
echo "  Clippy: PASS"
echo

# ── 3. cargo-audit ──────────────────────────────────────────────────────────
echo "[3/3] Running cargo audit..."
if ! command -v cargo-audit &>/dev/null; then
    echo "  SKIP: cargo-audit not installed."
    echo "        Install with: cargo install cargo-audit"
    echo "        Then re-run this script."
    exit 0
fi
cargo audit --deny warnings --deny unmaintained
echo "  Audit: PASS"
echo

echo "=== All checks passed ==="
