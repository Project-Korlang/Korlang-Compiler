#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTDIR="$ROOT/build"
OUTFILE="$OUTDIR/selfhosted.kor"

rm -rf "$OUTDIR"
mkdir -p "$OUTDIR"

# Concatenate Korlang compiler sources, stripping module/import lines.
python3 - <<'PY'
from pathlib import Path
root = Path('/mnt/c/Users/nanda/Desktop/KUBUNTU/Korlang/Korlang-Compiler')
out = root / 'build' / 'selfhosted.kor'
parts = []
for p in sorted((root / 'src' / 'compiler' / 'korlang').glob('*.kor')):
    lines = p.read_text().splitlines()
    for line in lines:
        if line.strip().startswith('module '):
            continue
        if line.strip().startswith('import '):
            continue
        parts.append(line)
    parts.append('')
parts.append('fun main() -> Int {')
parts.append('  0')
parts.append('}')
out.write_text('\n'.join(parts))
print(out)
PY

# Build a stage1 selfhosted binary (compile-only, no real runtime yet).
cd "$ROOT"
KORLANG_BIN="${KORLANG_BIN:-$ROOT/src/tools/cli/target/release/korlang}"
if [ ! -x "$KORLANG_BIN" ]; then
  echo "korlang binary not found at $KORLANG_BIN" >&2
  exit 1
fi
"$KORLANG_BIN" build "$OUTFILE" -o "$OUTDIR/korlang-selfhosted"
