#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

EXECUTE=0
SKIP_VERIFY=0

usage() {
  cat <<USAGE
Usage:
  scripts/purge_bootstrap.sh [--execute] [--skip-verify]

Behavior:
  - Default mode is dry-run (prints what would be removed).
  - --execute performs deletion after verification checks pass.
  - --skip-verify bypasses stage checksum verification (not recommended).

Verification checks:
  - dist/stage2.bin exists
  - dist/stage3.bin exists
  - stage2/stage3 SHA256 hashes match
  - Native Korlang trees exist:
      src/compiler/korlang
      src/runtime/korlang
USAGE
}

log() {
  printf '[purge-bootstrap] %s\n' "$*"
}

die() {
  printf '[purge-bootstrap] ERROR: %s\n' "$*" >&2
  exit 1
}

hash_file() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
    return
  fi
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
    return
  fi
  die "No SHA256 tool found (need sha256sum or shasum)"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --execute)
      EXECUTE=1
      shift
      ;;
    --skip-verify)
      SKIP_VERIFY=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "Unknown argument: $1"
      ;;
  esac
done

if [[ "$SKIP_VERIFY" -ne 1 ]]; then
  STAGE2="$ROOT/dist/stage2.bin"
  STAGE3="$ROOT/dist/stage3.bin"

  [[ -f "$STAGE2" ]] || die "Missing $STAGE2"
  [[ -f "$STAGE3" ]] || die "Missing $STAGE3"
  [[ -d "$ROOT/src/compiler/korlang" ]] || die "Missing native compiler tree src/compiler/korlang"
  [[ -d "$ROOT/src/runtime/korlang" ]] || die "Missing native runtime tree src/runtime/korlang"

  H2="$(hash_file "$STAGE2")"
  H3="$(hash_file "$STAGE3")"
  if [[ "$H2" != "$H3" ]]; then
    die "Stage2/Stage3 hash mismatch: $H2 vs $H3"
  fi

  log "Verification passed: stage2/stage3 hashes match ($H2)"
else
  log "Verification checks skipped by --skip-verify"
fi

TARGETS=(
  "$ROOT/src/compiler/src"
  "$ROOT/src/compiler/Cargo.toml"
  "$ROOT/src/runtime/src"
  "$ROOT/src/runtime/include"
  "$ROOT/src/runtime/Cargo.toml"
  "$ROOT/src/runtime/Cargo.lock"
  "$ROOT/src/runtime/target"
)

log "Bootstrap Rust targets selected for purge:"
for t in "${TARGETS[@]}"; do
  if [[ -e "$t" ]]; then
    printf '  - %s\n' "$t"
  fi
done

if [[ "$EXECUTE" -ne 1 ]]; then
  log "Dry-run complete. Re-run with --execute to apply deletions."
  exit 0
fi

if [[ "${KORLANG_PURGE_CONFIRM:-}" != "YES" ]]; then
  die "Set KORLANG_PURGE_CONFIRM=YES to execute destructive purge"
fi

for t in "${TARGETS[@]}"; do
  if [[ -e "$t" ]]; then
    rm -rf "$t"
    log "Removed $t"
  fi
done

log "Bootstrap purge complete."
