#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

KORLANG_BIN="${KORLANG_BIN:-$ROOT/dist/bootstrap-stage1/bin/korlang}"
KORLANG_BOOTSTRAP="${KORLANG_BOOTSTRAP:-$KORLANG_BIN}"

if [[ ! -x "$KORLANG_BIN" ]]; then
  echo "Missing KORLANG_BIN: $KORLANG_BIN" >&2
  exit 1
fi

export KORLANG_BOOTSTRAP
export KORLANG_NATIVE_RUNTIME=1

# Stage2 build through Korlang's native orchestration path.
"$KORLANG_BIN" build --native-selfhost
