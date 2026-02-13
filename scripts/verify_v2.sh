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

mkdir -p "$tmp/v2_hello/src"
cat > "$tmp/v2_hello/Korlang.config" <<'EOF'
[package]
name = "v2_hello"
version = "0.1.0"

[dependencies]
EOF
cat > "$tmp/v2_hello/src/main.kor" <<'EOF'
fun main() -> Int {
  println("hello-v2")
  0
}
EOF

(cd "$tmp/v2_hello" && "$BIN" build src/main.kor -o hello_v2)
if [[ ! -x "$tmp/v2_hello/hello_v2" ]]; then
  echo "missing Linux verification binary: hello_v2" >&2
  exit 1
fi
if command -v file >/dev/null 2>&1; then
  file "$tmp/v2_hello/hello_v2" | rg -n 'ELF' >/dev/null
fi

out="$("$tmp/v2_hello/hello_v2" 2>&1)"
[[ "$out" == *"hello-v2"* ]] || { echo "hello_v2 did not run correctly" >&2; exit 1; }

# GC + ARC stress tests on the native runtime implementation.
cargo test --manifest-path "$ROOT/src/runtime/Cargo.toml" --test gc_stress -- --nocapture

# Syscall wrapper surface checks for Linux/Darwin/Windows modules.
for f in "$ROOT/src/runtime/korlang/syscall/linux.kor" \
         "$ROOT/src/runtime/korlang/syscall/darwin.kor" \
         "$ROOT/src/runtime/korlang/syscall/windows.kor"; do
  [[ -f "$f" ]] || { echo "missing syscall wrapper: $f" >&2; exit 1; }
done
rg -n 'import runtime\.syscall\.dispatcher' "$ROOT/src/runtime/korlang/syscall/linux.kor" >/dev/null
rg -n 'import runtime\.syscall\.dispatcher' "$ROOT/src/runtime/korlang/syscall/darwin.kor" >/dev/null
rg -n 'import runtime\.syscall\.dispatcher' "$ROOT/src/runtime/korlang/syscall/windows.kor" >/dev/null
rg -n 'fun read\(' "$ROOT/src/runtime/korlang/syscall/linux.kor" >/dev/null
rg -n 'fun write\(' "$ROOT/src/runtime/korlang/syscall/linux.kor" >/dev/null
rg -n 'fun exit\(' "$ROOT/src/runtime/korlang/syscall/linux.kor" >/dev/null
rg -n 'fun read\(' "$ROOT/src/runtime/korlang/syscall/darwin.kor" >/dev/null
rg -n 'fun write\(' "$ROOT/src/runtime/korlang/syscall/darwin.kor" >/dev/null
rg -n 'fun exit\(' "$ROOT/src/runtime/korlang/syscall/darwin.kor" >/dev/null
rg -n 'fun exit_process\(' "$ROOT/src/runtime/korlang/syscall/windows.kor" >/dev/null
rg -n 'fun syscall_raw\(' "$ROOT/src/runtime/korlang/syscall/dispatcher.kor" >/dev/null
rg -n 'fun map_errno\(' "$ROOT/src/runtime/korlang/syscall/errors.kor" >/dev/null
rg -n 'fun map_ntstatus\(' "$ROOT/src/runtime/korlang/syscall/errors.kor" >/dev/null

echo "V.2 verification: ok"
