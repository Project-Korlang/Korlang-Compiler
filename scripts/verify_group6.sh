#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI_BIN="$ROOT/src/tools/cli/target/debug/korlang"

if [[ ! -f "$CLI_BIN" ]]; then
  echo "[group6] cli binary missing at $CLI_BIN"
  exit 1
fi

echo "[group6] Building runtime"
RUNTIME_MANIFEST="$ROOT/src/runtime/Cargo.toml"
cargo build --manifest-path "$RUNTIME_MANIFEST" >/tmp/group6-runtime.log

echo "[group6] Running cloud state tests"
cargo test --manifest-path "$RUNTIME_MANIFEST" state >/tmp/group6-state-test.log

TMP=$(mktemp -d)
cleanup() {
  rm -rf "$TMP"
}
trap cleanup EXIT

GPU_SRC="$ROOT/examples/verification/gpu_media_smoke.kor"
NET_SRC="$ROOT/examples/verification/network_crypto_smoke.kor"

echo "[group6] Compiling GPU verification smoke test"
"$CLI_BIN" build "$GPU_SRC" -o "$TMP/gpu_media"

echo "[group6] Compiling network + crypto verification smoke test"
"$CLI_BIN" build "$NET_SRC" -o "$TMP/network_crypto"

echo "[group6] Group 6 verification complete"
