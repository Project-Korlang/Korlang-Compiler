#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

required=(
  "$ROOT/src/runtime/korlang/scheduler_numa.kor"
  "$ROOT/src/runtime/korlang/waitfree_queue.kor"
  "$ROOT/src/runtime/korlang/fiber_stack.kor"
  "$ROOT/src/runtime/korlang/gpu/kernels.kor"
  "$ROOT/src/runtime/korlang/gpu/marshalling.kor"
  "$ROOT/src/runtime/korlang/gpu/jit.kor"
)

for f in "${required[@]}"; do
  [[ -f "$f" ]] || { echo "missing: $f" >&2; exit 1; }
done

# Parser/lexer keyword support checks
rg -n '"gpu"' "$ROOT/src/compiler/src/lexer.rs" >/dev/null
rg -n 'match_keyword\("gpu"\)' "$ROOT/src/compiler/src/parser.rs" >/dev/null
rg -n 's == "gpu"' "$ROOT/src/compiler/korlang/lexer.kor" >/dev/null
rg -n 'match_keyword\("gpu"\)' "$ROOT/src/compiler/korlang/parser.kor" >/dev/null

echo "phase16 verification: ok"
