#!/usr/bin/env bash
set -euo pipefail

ROOT=$(pwd)
CLI_DIR="$ROOT/src/tools/cli"
CLI_TARGET_DIR="${CARGO_TARGET_DIR:-$CLI_DIR/target}"
CLI_BIN="$CLI_TARGET_DIR/debug/korlang"

echo "[group5] Building CLI"
cd "$CLI_DIR"
cargo build --verbose

if [[ ! -f "$CLI_BIN" ]]; then
  echo "[group5] binary missing: $CLI_BIN"
  exit 1
fi

cd "$ROOT"

TMP=$(mktemp -d)
cleanup() {
  rm -rf "$TMP"
}
trap cleanup EXIT

DEMO="$TMP/demo-app"
echo "[group5] Generating project"
"$CLI_BIN" new "$DEMO"

if [[ ! -f "$DEMO/Korlang.config" ]]; then
  echo "[group5] project missing Konfig"
  exit 1
fi

echo "[group5] Building demo"
"$CLI_BIN" build "$DEMO/src/main.kor" -o "$DEMO/bin/demo"

if [[ ! -f "$DEMO/bin/demo" ]]; then
  echo "[group5] expected binary not created"
  exit 1
fi

echo "[group5] Running demo"
OUTPUT=$("$CLI_BIN" run "$DEMO/src/main.kor")

echo "[group5] Verifying incremental cache"
REPEAT=$("$CLI_BIN" run "$DEMO/src/main.kor" 2>&1)
if ! grep -q "Inputs unchanged; using incremental cache" <<< "$REPEAT"; then
  echo "[group5] incremental cache message missing"
  echo "$REPEAT"
  exit 1
fi

echo "[group5] Running korlang test"
cd "$ROOT"
"$CLI_BIN" test

echo "[group5] Running korlang doc"
rm -rf "$ROOT/dist/docs"
"$CLI_BIN" doc
if [[ ! -f "$ROOT/dist/docs/index.html" ]]; then
  echo "[group5] docs missing"
  exit 1
fi

echo "[group5] Group 5 verification complete"
