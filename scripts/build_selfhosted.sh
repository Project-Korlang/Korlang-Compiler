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
    i = 0
    while i < len(lines):
        line = lines[i]
        s = line.strip()
        if s.startswith('module ') or s.startswith('import '):
            i += 1
            continue
        # Drop forward declarations (fun signatures without body)
        if s.startswith('fun ') and '{' not in s and not s.endswith('{'):
            j = i + 1
            while j < len(lines) and lines[j].strip() == '':
                j += 1
            if j < len(lines) and lines[j].lstrip().startswith('{'):
                i += 1
                continue
        parts.append(line)
        i += 1
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
