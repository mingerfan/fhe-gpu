#!/usr/bin/env bash
# run_difftest.sh — Initialize Python oracle environment and run differential tests.
#
# Usage:
#   ./scripts/run_difftest.sh
#   ./scripts/run_difftest.sh --test-filter difftest_add_ct_ct

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ORACLE_DIR="$WORKSPACE_ROOT/oracle"

echo "=== FHE Differential Test Runner ==="
echo "Workspace: $WORKSPACE_ROOT"
echo "Oracle:    $ORACLE_DIR"
echo ""

# Check prerequisites
if ! command -v uv &>/dev/null; then
    echo "ERROR: 'uv' not found. Install from https://github.com/astral-sh/uv"
    exit 1
fi

if ! command -v cargo &>/dev/null; then
    echo "ERROR: 'cargo' not found. Install Rust from https://rustup.rs"
    exit 1
fi

# Initialize Python environment
echo "[1/3] Setting up Python oracle environment..."
cd "$ORACLE_DIR"
uv sync
echo "      OK: dependencies installed"

# Verify oracle works
echo "[2/3] Verifying oracle..."
ORACLE_TEST_INPUT='{"operation":"add_ct_ct","inputs":{"x":[[1.0,0.0]],"y":[[2.0,0.0]]},"params":{"poly_degree":4096,"scale_bits":40,"depth":3}}'
ORACLE_OUTPUT=$(echo "$ORACLE_TEST_INPUT" | uv run python -m fhe_oracle 2>/dev/null || true)
if [ -z "$ORACLE_OUTPUT" ]; then
    echo "      WARNING: Oracle returned no output — openfhe may not be installed"
    echo "      Install: pip install openfhe  (or see https://openfhe.org)"
    echo "      Continuing anyway (difftest will be skipped by the harness)"
else
    echo "      OK: Oracle response: $ORACLE_OUTPUT"
fi

# Run differential tests
echo "[3/3] Running differential tests..."
cd "$WORKSPACE_ROOT"
RUN_DIFFTESTS=1 cargo test \
    --features difftest \
    ${1:+--test "$1"} \
    -- --nocapture \
    2>&1 | tee /tmp/difftest-output.log

echo ""
echo "=== Done. Full output: /tmp/difftest-output.log ==="
