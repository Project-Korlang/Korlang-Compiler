#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -n "${KORLANG_BIN:-}" ]]; then
  BIN="$KORLANG_BIN"
elif [[ -x "$ROOT/src/tools/cli/target/debug/korlang" ]]; then
  BIN="$ROOT/src/tools/cli/target/debug/korlang"
elif command -v korlang >/dev/null 2>&1; then
  BIN="$(command -v korlang)"
else
  echo "missing korlang binary; set KORLANG_BIN or build src/tools/cli" >&2
  exit 1
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

expect_contains() {
  local out="$1"
  local needle="$2"
  if [[ "$out" != *"$needle"* ]]; then
    echo "expected output to contain: $needle" >&2
    echo "got output:" >&2
    echo "$out" >&2
    exit 1
  fi
}

# V.1.1 + V.1.2: run should execute app and propagate exit status.
mkdir -p "$tmp/v1_io/src"
cat > "$tmp/v1_io/Korlang.config" <<'EOF'
[package]
name = "v1_io"
version = "0.1.0"

[dependencies]
EOF
cat > "$tmp/v1_io/src/main.kor" <<'EOF'
fun main() -> Int {
  println("v1-stream")
  7
}
EOF

set +e
out="$("$BIN" run "$tmp/v1_io/src/main.kor" 2>&1)"
status=$?
set -e
expect_contains "$out" "Compiling:"
expect_contains "$out" "v1-stream"
if [[ "$status" -ne 7 ]]; then
  echo "expected exit code 7 from korlang run, got $status" >&2
  exit 1
fi

# V.1.3: import-based multi-file resolution and default src/main.kor run.
mkdir -p "$tmp/v1_import/src"
cat > "$tmp/v1_import/Korlang.config" <<'EOF'
[package]
name = "v1_import"
version = "0.1.0"

[dependencies]
EOF
cat > "$tmp/v1_import/src/util.kor" <<'EOF'
fun utilValue() -> Int {
  42
}
EOF
cat > "$tmp/v1_import/src/main.kor" <<'EOF'
import util
fun main() -> Int {
  println("v1-import")
  utilValue()
}
EOF

set +e
out="$(cd "$tmp/v1_import" && "$BIN" run 2>&1)"
status=$?
set -e
expect_contains "$out" "Compiling: src/main.kor"
expect_contains "$out" "v1-import"
if [[ "$status" -ne 0 ]]; then
  echo "expected exit code 0 from default korlang run, got $status" >&2
  exit 1
fi

rg -n 'cmd\.stdout\(Stdio::piped\(\)\)\.stderr\(Stdio::piped\(\)\)' "$ROOT/src/tools/cli/src/main.rs" >/dev/null
rg -n 'fn stream_pipe<' "$ROOT/src/tools/cli/src/main.rs" >/dev/null
rg -n 'resolve_source_with_imports' "$ROOT/src/tools/cli/src/main.rs" >/dev/null

echo "V.1 verification: ok"
