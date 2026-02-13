#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT/scripts/verify_v1.sh"
"$ROOT/scripts/verify_v2.sh"
"$ROOT/scripts/verify_v3.sh"

echo "verification phase: ok"
