#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STAGE1="$ROOT/dist/bootstrap-stage1"

rm -rf "$STAGE1"
mkdir -p "$STAGE1/bin" "$STAGE1/lib"

# Build runtime + CLI using Rust bootstrap
cd "$ROOT/src/runtime"
cargo build --release

cd "$ROOT/src/tools/cli"
cargo build --release

# Install stage1 layout
cp "$ROOT/src/tools/cli/target/release/korlang" "$STAGE1/bin/"
cp "$ROOT/src/runtime/target/release/libkorlang_rt.a" "$STAGE1/lib/"

# Smoke test: compile example using stage1
cd "$ROOT"
KORLANG_HOME="$STAGE1" "$STAGE1/bin/korlang" build examples/hello.kor -o "$STAGE1/hello_stage1"

if [ ! -f "$STAGE1/hello_stage1" ]; then
  echo "Stage1 build failed" >&2
  exit 1
fi

echo "Stage1 bootstrap complete: $STAGE1"
