#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

BIN="${1:-}"
if [[ -z "$BIN" ]]; then
  if [[ -x "$ROOT/build/korlang-selfhosted" ]]; then
    BIN="$ROOT/build/korlang-selfhosted"
  elif command -v korlang >/dev/null 2>&1; then
    BIN="$(command -v korlang)"
  elif [[ -x "$ROOT/dist/stage3.bin" ]]; then
    BIN="$ROOT/dist/stage3.bin"
  else
    echo "[verify-zero-deps] ERROR: no binary found; pass path as first arg" >&2
    exit 1
  fi
fi

if [[ ! -x "$BIN" ]]; then
  echo "[verify-zero-deps] ERROR: binary not executable: $BIN" >&2
  exit 1
fi

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

echo "[verify-zero-deps] checking: $BIN"

verify_linux() {
  command -v ldd >/dev/null 2>&1 || { echo "[verify-zero-deps] ERROR: ldd not found" >&2; exit 1; }

  local bad=0
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue

    # Allow virtual/system loader lines
    if [[ "$line" =~ linux-vdso ]] || [[ "$line" =~ ld-linux ]] || [[ "$line" =~ /lib64/ld-linux ]] || [[ "$line" =~ /lib/ld-linux ]]; then
      continue
    fi

    # Normalize resolved path if present
    local path=""
    if [[ "$line" == *"=>"* ]]; then
      path="$(printf '%s' "$line" | awk -F'=> ' '{print $2}' | awk '{print $1}')"
    else
      path="$(printf '%s' "$line" | awk '{print $1}')"
    fi

    # Allow only core glibc/libgcc loader set
    case "$path" in
      */libc.so.*|*/libm.so.*|*/libpthread.so.*|*/libdl.so.*|*/librt.so.*|*/libgcc_s.so.*)
        ;;
      "not")
        bad=1
        echo "[verify-zero-deps] disallowed/unresolved: $line"
        ;;
      "")
        ;;
      *)
        bad=1
        echo "[verify-zero-deps] disallowed dependency: $line"
        ;;
    esac
  done < <(ldd "$BIN")

  if [[ "$bad" -ne 0 ]]; then
    echo "[verify-zero-deps] FAILED (linux)"
    exit 1
  fi
  echo "[verify-zero-deps] PASSED (linux)"
}

verify_macos() {
  command -v otool >/dev/null 2>&1 || { echo "[verify-zero-deps] ERROR: otool not found" >&2; exit 1; }

  local bad=0
  local first=1
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    if [[ "$first" -eq 1 ]]; then
      first=0
      continue
    fi
    local dep
    dep="$(printf '%s' "$line" | awk '{print $1}')"

    case "$dep" in
      /usr/lib/*|/System/Library/*|@executable_path/*)
        ;;
      *)
        bad=1
        echo "[verify-zero-deps] disallowed dependency: $dep"
        ;;
    esac
  done < <(otool -L "$BIN")

  if [[ "$bad" -ne 0 ]]; then
    echo "[verify-zero-deps] FAILED (macos)"
    exit 1
  fi
  echo "[verify-zero-deps] PASSED (macos)"
}

verify_windows() {
  command -v dumpbin >/dev/null 2>&1 || {
    echo "[verify-zero-deps] ERROR: dumpbin not found (run in VS dev shell)" >&2
    exit 1
  }

  local bad=0
  while IFS= read -r dep; do
    [[ -z "$dep" ]] && continue
    case "${dep^^}" in
      KERNEL32.DLL|NTDLL.DLL|USER32.DLL|ADVAPI32.DLL|WS2_32.DLL|SHELL32.DLL)
        ;;
      *)
        bad=1
        echo "[verify-zero-deps] disallowed dependency: $dep"
        ;;
    esac
  done < <(dumpbin /DEPENDENTS "$BIN" | awk '/\.DLL/{print $1}')

  if [[ "$bad" -ne 0 ]]; then
    echo "[verify-zero-deps] FAILED (windows)"
    exit 1
  fi
  echo "[verify-zero-deps] PASSED (windows)"
}

case "$OS" in
  linux*) verify_linux ;;
  darwin*) verify_macos ;;
  mingw*|msys*|cygwin*) verify_windows ;;
  *)
    echo "[verify-zero-deps] ERROR: unsupported OS: $OS" >&2
    exit 1
    ;;
esac
