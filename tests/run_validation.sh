#!/usr/bin/env bash
set -e

# Korlang Runtime Validation Suite Runner
# This script builds and runs the validation Korlang program.

echo "--- Korlang Runtime Validation ---"

# Use the current build of the compiler
KORLANG_BIN="./target/debug/korlang"
if [ ! -f "$KORLANG_BIN" ]; then
    # Try release if debug doesn't exist
    KORLANG_BIN="./target/release/korlang"
fi

# If not found in target/, assume it's in PATH
if ! command -v "$KORLANG_BIN" &> /dev/null; then
    KORLANG_BIN="korlang"
fi

echo "Using compiler: $KORLANG_BIN"

# 1. Build validation suite
echo "Building validation suite..."
if $KORLANG_BIN build tests/runtime_validation.kor -o tests/validation_bin --verbose; then
    echo "✓ Build runtime_validation successful"
else
    echo "FAILED: Build of validation suite failed"
    exit 1
fi

echo "Building type system test..."
if $KORLANG_BIN build tests/type_system_test.kor -o tests/type_test_bin --verbose; then
    echo "✓ Build type_system_test successful"
else
    echo "FAILED: Build of type system test failed"
    exit 1
fi

echo "Building nogc test..."
if $KORLANG_BIN build tests/nogc_test.kor -o tests/nogc_test_bin --verbose; then
    echo "✓ Build nogc_test successful"
else
    echo "FAILED: Build of nogc test failed"
    exit 1
fi

echo "Building GC test..."
if $KORLANG_BIN build tests/gc_test.kor -o tests/gc_test_bin --verbose; then
    echo "✓ Build gc_test successful"
else
    echo "FAILED: Build of gc test failed"
    exit 1
fi

# 2. Run validation suites
echo "Running validation suite..."
if ./tests/validation_bin; then
    echo "✓ Runtime validation PASSED"
else
    EXIT_CODE=$?
    echo "FAILED: Runtime validation reported $EXIT_CODE failures"
    exit 1
fi

echo "Running type system test..."
if ./tests/type_test_bin; then
    echo "✓ Type system test PASSED"
else
    EXIT_CODE=$?
    echo "FAILED: Type system test reported $EXIT_CODE failures"
    exit 1
fi

echo "Running nogc test..."
if ./tests/nogc_test_bin; then
    echo "✓ nogc test PASSED"
else
    EXIT_CODE=$?
    echo "FAILED: nogc test reported $EXIT_CODE failures"
    exit 1
fi

echo "Running GC test..."
if ./tests/gc_test_bin; then
    echo "✓ GC test PASSED"
else
    EXIT_CODE=$?
    echo "FAILED: GC test reported $EXIT_CODE failures"
    exit 1
fi

echo "--- Validation Complete: SUCCESS ---"
