#!/usr/bin/env bash
set -e

# Installation Verification Script for Korlang
# This script ensures that the korlang compiler is correctly installed and functional.

echo "--- Korlang Installation Verification ---"

# 1. Check if korlang is in PATH
if ! command -v korlang &> /dev/null; then
    echo "FAILED: 'korlang' command not found in PATH."
    exit 1
fi
echo "✓ 'korlang' command is in PATH"

# 2. Check version output
VERSION=$(korlang --version)
if [[ $VERSION == *"Korlang Compiler"* ]]; then
    echo "✓ Version check passed: $VERSION"
else
    echo "FAILED: 'korlang --version' produced unexpected output: $VERSION"
    exit 1
fi

# 3. Test compilation and execution
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Testing compilation..."
cat << 'EOF' > "$TMP_DIR/verify_smoke.kor"
fun main() -> Int {
    let msg = "Korlang is alive!";
    // In a real verification, we'd check stdout
    0
}
EOF

# We use --verbose to see the phases
if korlang build "$TMP_DIR/verify_smoke.kor" -o "$TMP_DIR/verify_smoke" --verbose; then
    echo "✓ Compilation successful"
else
    echo "FAILED: Compilation of smoke test failed"
    exit 1
fi

if [ -f "$TMP_DIR/verify_smoke" ]; then
    echo "✓ Binary generated: $TMP_DIR/verify_smoke"
    # Execute it
    if "$TMP_DIR/verify_smoke"; then
        echo "✓ Execution successful"
    else
        echo "FAILED: Execution of smoke test binary failed"
        exit 1
    fi
else
    echo "FAILED: Binary not found after build"
    exit 1
fi

echo "--- Verification Complete: SUCCESS ---"
