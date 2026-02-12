#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT/src/runtime"
cargo build

cd "$ROOT/src/tools/cli"
cargo build

cd "$ROOT"
./src/tools/cli/target/debug/korlang build examples/hello.kor -o hello --lto

