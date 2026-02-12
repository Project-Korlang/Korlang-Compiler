#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTDIR="$ROOT/build"
OUTFILE="$OUTDIR/selfhosted.kor"

rm -rf "$OUTDIR"
mkdir -p "$OUTDIR"

# Concatenate Korlang compiler sources, stripping module/import lines.
ROOT="$ROOT" python3 - <<'PY'
from pathlib import Path
import os

root = Path(os.environ['ROOT'])
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
            i += 1
            continue
        parts.append(line)
        i += 1
    parts.append('')
parts.append('fun main() -> Int {')
parts.append('  0')
parts.append('}')
text = '\n'.join(parts)

def replace_generic(text, name, to_brackets):
    out = []
    i = 0
    pat = name + "<"
    while i < len(text):
        if text.startswith(pat, i):
            i += len(pat)
            depth = 1
            inner = []
            while i < len(text) and depth > 0:
                c = text[i]
                if c == '<':
                    depth += 1
                elif c == '>':
                    depth -= 1
                    if depth == 0:
                        i += 1
                        break
                if depth > 0:
                    inner.append(c)
                i += 1
            inner_text = ''.join(inner).strip()
            if to_brackets:
                out.append('[' + inner_text + ']')
            else:
                out.append('Any')
        else:
            out.append(text[i])
            i += 1
    return ''.join(out)

while 'List<' in text:
    text = replace_generic(text, 'List', True)
while 'Result<' in text:
    text = replace_generic(text, 'Result', False)

out.write_text(text)
print(out)
PY

# Build a stage1 selfhosted binary (compile-only, no real runtime yet).
cd "$ROOT"
KORLANG_BIN="${KORLANG_BIN:-korlang}"
if ! command -v "$KORLANG_BIN" >/dev/null 2>&1 && [ ! -x "$KORLANG_BIN" ]; then
  echo "korlang binary not found (set KORLANG_BIN or add to PATH)" >&2
  exit 1
fi

# Ensure a runtime lib exists for linking.
RUNTIME_HOME="$OUTDIR/runtime"
RUNTIME_LIB="$RUNTIME_HOME/lib/libkorlang_rt.a"
if [ ! -f "$RUNTIME_LIB" ]; then
  if [ -f "$ROOT/src/runtime/target/release/libkorlang_rt.a" ]; then
    mkdir -p "$RUNTIME_HOME/lib"
    cp "$ROOT/src/runtime/target/release/libkorlang_rt.a" "$RUNTIME_LIB"
  else
    (cd "$ROOT/src/runtime" && cargo build --release)
    mkdir -p "$RUNTIME_HOME/lib"
    cp "$ROOT/src/runtime/target/release/libkorlang_rt.a" "$RUNTIME_LIB"
  fi
fi

KORLANG_HOME="$RUNTIME_HOME" KORLANG_SEMA_PERMISSIVE=1 "$KORLANG_BIN" build "$OUTFILE" -o "$OUTDIR/korlang-selfhosted"
