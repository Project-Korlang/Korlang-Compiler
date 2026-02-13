#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

required=(
  "$ROOT/src/stdlib/core/crypto/sha2.kor"
  "$ROOT/src/stdlib/core/crypto/sha3.kor"
  "$ROOT/src/stdlib/core/crypto/blake3.kor"
  "$ROOT/src/stdlib/core/crypto/aes.kor"
  "$ROOT/src/stdlib/core/crypto/chacha20.kor"
  "$ROOT/src/stdlib/core/crypto/ed25519.kor"
  "$ROOT/src/stdlib/core/crypto/rsa.kor"
  "$ROOT/src/stdlib/core/net/tcp_udp.kor"
  "$ROOT/src/stdlib/core/net/tls13.kor"
  "$ROOT/src/stdlib/core/net/http3_ws.kor"
)

for f in "${required[@]}"; do
  [[ -f "$f" ]] || { echo "missing: $f" >&2; exit 1; }
done

# Total Decoupling Mandate checks
rg -n "import runtime\.syscall\.dispatcher" "$ROOT/src/runtime/korlang/os.kor" >/dev/null
rg -n "import runtime\.syscall\.dispatcher" "$ROOT/src/runtime/korlang/stdlib/io.kor" >/dev/null
rg -n "import runtime\.syscall\.dispatcher" "$ROOT/src/runtime/korlang/syscall.kor" >/dev/null

# ensure legacy arithmetic syscall shim is gone
if rg -n "n \+ a1|n \+ a1 \+ a2" "$ROOT/src/runtime/korlang/syscall.kor" >/dev/null; then
  echo "legacy syscall shim arithmetic still present" >&2
  exit 1
fi

echo "phase17 verification: ok"
