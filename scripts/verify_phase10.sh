#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT/scripts/bootstrap.sh"

# Verify the selfhosted binary was produced
if [ ! -f "$ROOT/build/korlang-selfhosted" ]; then
  echo "Selfhosted build missing" >&2
  exit 1
fi

echo "Phase 10 verification complete."
