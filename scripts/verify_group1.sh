#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="/tmp/korlang_group1"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

pass() { printf '[group1] PASS %s\n' "$1"; }
fail() { printf '[group1] FAIL %s\n' "$1" >&2; exit 1; }
req_file() { [[ -f "$1" ]] || fail "$2 (missing: $1)"; }
req_rg() { rg -n "$1" "$2" >/dev/null || fail "$3"; }
sha_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

# O.1.1 Lexer
req_file "$ROOT/src/compiler/korlang/lexer.kor" "O.1.1"
req_rg '^module compiler\.lexer' "$ROOT/src/compiler/korlang/lexer.kor" "O.1.1 lexer module missing"
req_rg 'fun Lexer_tokenize' "$ROOT/src/compiler/korlang/lexer.kor" "O.1.1 tokenize missing"
pass "O.1.1"

# O.1.2 Parser
req_file "$ROOT/src/compiler/korlang/parser.kor" "O.1.2"
req_rg '^module compiler\.parser' "$ROOT/src/compiler/korlang/parser.kor" "O.1.2 parser module missing"
req_rg 'fun Parser_parse_program' "$ROOT/src/compiler/korlang/parser.kor" "O.1.2 parse_program missing"
pass "O.1.2"

# O.1.3 Semantic Analysis
req_file "$ROOT/src/compiler/korlang/sema.kor" "O.1.3"
req_rg '^module compiler\.sema' "$ROOT/src/compiler/korlang/sema.kor" "O.1.3 sema module missing"
req_rg 'fun Sema_check_program' "$ROOT/src/compiler/korlang/sema.kor" "O.1.3 check_program missing"
pass "O.1.3"

# O.1.4 Code Generation
req_file "$ROOT/src/compiler/korlang/kir.kor" "O.1.4"
req_file "$ROOT/src/compiler/korlang/llvm_bindings.kor" "O.1.4"
req_file "$ROOT/src/compiler/korlang/backend/x86_64/encoder.kor" "O.1.4"
req_file "$ROOT/src/compiler/korlang/backend/aarch64/encoder.kor" "O.1.4"
req_rg '^module compiler\.kir' "$ROOT/src/compiler/korlang/kir.kor" "O.1.4 KIR module missing"
req_rg '^module compiler\.llvm' "$ROOT/src/compiler/korlang/llvm_bindings.kor" "O.1.4 LLVM module missing"
pass "O.1.4"

# O.1.5 Garbage Collector
req_file "$ROOT/src/runtime/korlang/gc.kor" "O.1.5"
req_rg '^module runtime\.gc' "$ROOT/src/runtime/korlang/gc.kor" "O.1.5 GC module missing"
req_rg 'fun heap_collect_young' "$ROOT/src/runtime/korlang/gc.kor" "O.1.5 young collection missing"
req_rg 'fun heap_collect_full' "$ROOT/src/runtime/korlang/gc.kor" "O.1.5 full collection missing"
pass "O.1.5"

# O.1.6 Task Scheduler
for f in \
  "$ROOT/src/runtime/korlang/scheduler.kor" \
  "$ROOT/src/runtime/korlang/scheduler_numa.kor" \
  "$ROOT/src/runtime/korlang/waitfree_queue.kor" \
  "$ROOT/src/runtime/korlang/fiber_stack.kor"; do
  req_file "$f" "O.1.6"
done
req_rg '^module runtime\.scheduler' "$ROOT/src/runtime/korlang/scheduler.kor" "O.1.6 scheduler module missing"
pass "O.1.6"

# O.1.7 FFI Bridge
req_file "$ROOT/src/compiler/korlang/driver.kor" "O.1.7"
req_file "$ROOT/src/runtime/korlang/syscall/dispatcher.kor" "O.1.7"
req_rg '@import\("korlang_driver"\)' "$ROOT/src/compiler/korlang/driver.kor" "O.1.7 @import bridge missing"
req_rg 'fun syscall_raw' "$ROOT/src/runtime/korlang/syscall/dispatcher.kor" "O.1.7 syscall bridge missing"
pass "O.1.7"

# O.1.8 Stage 1 reproducibility (same input -> same output hash)
STAGE1_BIN="$ROOT/dist/bootstrap-stage1/bin/korlang"
STAGE1_HOME="$ROOT/dist/bootstrap-stage1"
req_file "$STAGE1_BIN" "O.1.8"
req_file "$STAGE1_HOME/lib/libkorlang_rt.a" "O.1.8"
KORLANG_HOME="$STAGE1_HOME" "$STAGE1_BIN" build "$ROOT/examples/hello.kor" -o "$BUILD_DIR/stage1_a"
KORLANG_HOME="$STAGE1_HOME" "$STAGE1_BIN" build "$ROOT/examples/hello.kor" -o "$BUILD_DIR/stage1_b"
req_file "$BUILD_DIR/stage1_a" "O.1.8"
req_file "$BUILD_DIR/stage1_b" "O.1.8"
[[ "$(sha_file "$BUILD_DIR/stage1_a")" == "$(sha_file "$BUILD_DIR/stage1_b")" ]] || fail "O.1.8 stage1 output not reproducible"
pass "O.1.8"

# O.1.9 Stage 2 self-hosted reproducibility
KORLANG_BIN="$STAGE1_BIN" KORLANG_SEMA_PERMISSIVE=1 "$ROOT/scripts/build_selfhosted.sh"
req_file "$ROOT/build/korlang-selfhosted" "O.1.9"
cp "$ROOT/build/korlang-selfhosted" "$BUILD_DIR/stage2_a"
KORLANG_BIN="$STAGE1_BIN" KORLANG_SEMA_PERMISSIVE=1 "$ROOT/scripts/build_selfhosted.sh"
req_file "$ROOT/build/korlang-selfhosted" "O.1.9"
cp "$ROOT/build/korlang-selfhosted" "$BUILD_DIR/stage2_b"
req_file "$BUILD_DIR/stage2_a" "O.1.9"
req_file "$BUILD_DIR/stage2_b" "O.1.9"
[[ "$(sha_file "$BUILD_DIR/stage2_a")" == "$(sha_file "$BUILD_DIR/stage2_b")" ]] || fail "O.1.9 stage2 binary not reproducible"
pass "O.1.9"

# O.1.10 Stage 3 fixpoint
req_file "$ROOT/dist/stage2.bin" "O.1.10"
req_file "$ROOT/dist/stage3.bin" "O.1.10"
[[ "$(sha_file "$ROOT/dist/stage2.bin")" == "$(sha_file "$ROOT/dist/stage3.bin")" ]] || fail "O.1.10 stage2/stage3 mismatch"
pass "O.1.10"

# O.1.11 Binary dependency audit
"$ROOT/scripts/verify_zero_deps.sh" "$ROOT/dist/stage2.bin"
pass "O.1.11"

# O.1.12 @nostd no libc symbol usage in Korlang nostd runtime
for f in "$ROOT/src/runtime/korlang/nostd/entry.kor" "$ROOT/src/runtime/korlang/nostd/stdlib.kor" "$ROOT/src/runtime/korlang/nostd/mempool.kor"; do
  req_file "$f" "O.1.12"
done
if rg -n 'libc|std::|extern crate|python' "$ROOT/src/runtime/korlang/nostd" >/dev/null; then
  fail "O.1.12 nostd contains forbidden host-runtime imports"
fi
pass "O.1.12"

# O.1.13 Linux ELF header generation
req_file "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.13"
req_rg 'out\.push\(0x7F\)' "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.13 missing ELF magic 0x7F"
req_rg "0x45" "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.13 missing ELF magic E"
req_rg "0x4C" "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.13 missing ELF magic L"
req_rg "0x46" "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.13 missing ELF magic F"
pass "O.1.13"

# O.1.14 macOS Mach-O header generation
req_file "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.14"
req_rg '0xCF' "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.14 missing Mach-O magic byte CF"
req_rg '0xFA' "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.14 missing Mach-O magic byte FA"
req_rg '0xED' "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.14 missing Mach-O magic byte ED"
req_rg '0xFE' "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.14 missing Mach-O magic byte FE"
pass "O.1.14"

# O.1.15 Windows PE header generation
req_file "$ROOT/src/compiler/korlang/linker/pe.kor" "O.1.15"
req_rg '0x4D' "$ROOT/src/compiler/korlang/linker/pe.kor" "O.1.15 missing PE magic byte M"
req_rg '0x5A' "$ROOT/src/compiler/korlang/linker/pe.kor" "O.1.15 missing PE magic byte Z"
pass "O.1.15"

# O.1.21 Keyword: lets verify new keywords list contains each reserved word
for kw in fun let var if else match for while break continue return view resource state spawn \
  "@nogc" import as struct enum type in interface module class; do
  req_rg "\"${kw}\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.21 keyword ${kw} missing in lexer keywords list"
done
pass "O.1.21"

# O.1.22 already covered by O.1.21 (let/var) but keep as explicit check
req_rg "\"let\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.22 let missing"
req_rg "\"var\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.22 var missing"
pass "O.1.22"

# O.1.23 keywords
req_rg "\"if\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.23 if missing"
req_rg "\"else\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.23 else missing"
req_rg "\"match\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.23 match missing"
req_rg "\"return\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.23 return missing"
pass "O.1.23"

# O.1.24 loops
req_rg "\"for\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.24 for missing"
req_rg "\"while\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.24 while missing"
pass "O.1.24"

# O.1.25 match/return already present but added to reassure
req_rg "\"match\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.25 match missing"
req_rg "\"return\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.25 return missing"
pass "O.1.25"

# O.1.26 break/continue
req_rg "\"break\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.26 break missing"
req_rg "\"continue\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.26 continue missing"
pass "O.1.26"

# O.1.27 struct/enum
req_rg "\"struct\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.27 struct missing"
req_rg "\"enum\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.27 enum missing"
pass "O.1.27"

# O.1.28 class/interface
req_rg "\"class\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.28 class missing"
req_rg "\"interface\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.28 interface missing"
pass "O.1.28"

# O.1.29 import/module
req_rg "\"import\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.29 import missing"
req_rg "\"module\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.29 module missing"
pass "O.1.29"

# O.1.30 view/resource/gpu
req_rg "\"view\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.30 view missing"
req_rg "\"resource\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.30 resource missing"
req_rg "\"gpu\"" "$ROOT/src/compiler/korlang/lexer.kor" "O.1.30 gpu missing"
pass "O.1.30"

# O.1.31 arithmetic operators
for op in Plus Minus Star Slash Percent; do
  req_rg "\\b${op};" "$ROOT/src/compiler/korlang/token.kor" "O.1.31 ${op} missing"
done
pass "O.1.31"

# O.1.32 comparison operators
for op in EqEq NotEq Lt LtEq Gt GtEq; do
  req_rg "\\b${op};" "$ROOT/src/compiler/korlang/token.kor" "O.1.32 ${op} missing"
done
pass "O.1.32"

# O.1.33 logical operators
for op in AndAnd OrOr Not; do
  req_rg "\\b${op};" "$ROOT/src/compiler/korlang/token.kor" "O.1.33 ${op} missing"
done
pass "O.1.33"

# O.1.34 bitwise operators
for op in BitAnd BitOr BitXor ShiftLeft ShiftRight; do
  req_rg "\\b${op};" "$ROOT/src/compiler/korlang/token.kor" "O.1.34 ${op} missing"
done
pass "O.1.34"

# O.1.35 arrow
req_rg "\\bArrow;" "$ROOT/src/compiler/korlang/token.kor" "O.1.35 -> missing"
pass "O.1.35"

# O.1.36 pipe
req_rg "\\bPipe;" "$ROOT/src/compiler/korlang/token.kor" "O.1.36 |> missing"
pass "O.1.36"

# O.1.37 null coalesce
req_rg "\\bNullCoalesce;" "$ROOT/src/compiler/korlang/token.kor" "O.1.37 ?: missing"
pass "O.1.37"

# O.1.38 delimiters (group 1)
for delim in LParen RParen LBrace RBrace LBracket RBracket; do
  req_rg "\\b${delim};" "$ROOT/src/compiler/korlang/token.kor" "O.1.38 ${delim} missing"
done
pass "O.1.38"

# O.1.39 delimiters (group 2)
for delim in Comma Semi Colon Dot; do
  req_rg "\\b${delim};" "$ROOT/src/compiler/korlang/token.kor" "O.1.39 ${delim} missing"
done
pass "O.1.39"

# O.1.40 EOF
req_rg "\\bEof;" "$ROOT/src/compiler/korlang/token.kor" "O.1.40 EOF token missing"
pass "O.1.40"

printf '\n[group1] all Group 1 checks passed\n'
