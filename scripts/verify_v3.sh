#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IDE_DIR="${KORLANG_IDE_DIR:-$ROOT/../Korlang-IDE}"

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

[[ -d "$IDE_DIR" ]] || { echo "missing Korlang-IDE dir: $IDE_DIR" >&2; exit 1; }
[[ -f "$IDE_DIR/src/main.kor" ]] || { echo "missing Korlang-IDE entry: $IDE_DIR/src/main.kor" >&2; exit 1; }

# V.3.1: self-build IDE to a standalone binary and execute it.
(cd "$IDE_DIR" && "$BIN" build src/main.kor -o korlang-ide.bin)
[[ -x "$IDE_DIR/korlang-ide.bin" ]] || { echo "failed to build IDE binary" >&2; exit 1; }
set +e
"$IDE_DIR/korlang-ide.bin" >/dev/null 2>&1
status=$?
set -e
if [[ "$status" -ne 0 ]]; then
  echo "unexpected IDE binary exit code: $status (expected 0)" >&2
  exit 1
fi

# V.3.2: network/crypto smoke app build/run + stdlib symbol checks.
out="$("$BIN" run "$ROOT/examples/verification/network_crypto_smoke.kor" 2>&1)"
[[ "$out" == *"Server listening on :8080"* ]] || { echo "network smoke output missing" >&2; exit 1; }
rg -n 'fun tls_new\(' "$ROOT/src/stdlib/core/net/tls13.kor" >/dev/null
rg -n 'fun tcp_socket\(' "$ROOT/src/stdlib/core/net/tcp_udp.kor" >/dev/null
rg -n 'fun sha256_' "$ROOT/src/stdlib/core/crypto/sha2.kor" >/dev/null
rg -n 'fun aes_' "$ROOT/src/stdlib/core/crypto/aes.kor" >/dev/null

# V.3.3: GPU/media sanity app build/run + backend symbol checks.
out="$("$BIN" run "$ROOT/examples/verification/gpu_media_smoke.kor" 2>&1)"
[[ "$out" == *"Initializing GPU..."* ]] || { echo "gpu smoke output missing" >&2; exit 1; }
[[ "$out" == *"Media graph ready"* ]] || { echo "media smoke output missing" >&2; exit 1; }
rg -n 'fun gpu_jit_compile\(' "$ROOT/src/runtime/korlang/gpu/jit.kor" >/dev/null
rg -n 'fun window_create\(' "$ROOT/src/runtime/korlang/graphics/windowing.kor" >/dev/null
rg -n 'fun Graph_connect_sidechain\(' "$ROOT/src/runtime/korlang/media/graph.kor" >/dev/null
rg -n 'fun dispatch_yuv_rgb_shader\(' "$ROOT/src/runtime/korlang/media/video_pipeline.kor" >/dev/null

echo "V.3 verification: ok"
