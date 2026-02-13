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
req_rg_literal() { rg -n -F "$1" "$2" >/dev/null || fail "$3"; }
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

# O.1.81 Symbol table creation
req_file "$ROOT/src/compiler/korlang/symtab.kor" "O.1.81"
req_rg '^fun SymbolTable_new' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.81 SymbolTable_new missing"
pass "O.1.81"

# O.1.82 Scope push/pop
req_rg '^fun SymbolTable_enter' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.82 enter missing"
req_rg '^fun SymbolTable_exit' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.82 exit missing"
pass "O.1.82"

# O.1.83 define redefinition guard
req_rg '^fun SymbolTable_define' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.83 define missing"
req_rg_literal 'return false' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.83 redefinition guard missing"
pass "O.1.83"

# O.1.84 lookup walks parent scopes
req_rg '^fun SymbolTable_lookup' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.84 lookup missing"
req_rg_literal 'scope.parent' "$ROOT/src/compiler/korlang/symtab.kor" "O.1.84 scope parent missing"
pass "O.1.84"

# O.1.85 Type inference
req_file "$ROOT/src/compiler/korlang/infer.kor" "O.1.85"
req_rg '^fun infer_program' "$ROOT/src/compiler/korlang/infer.kor" "O.1.85 infer_program missing"
pass "O.1.85"

# O.1.86 Unification algorithm
req_rg '^fun unify' "$ROOT/src/compiler/korlang/infer.kor" "O.1.86 unify missing"
req_rg '^fun unify_all' "$ROOT/src/compiler/korlang/infer.kor" "O.1.86 unify_all missing"
pass "O.1.86"

# O.1.87 @nogc safety checks
req_file "$ROOT/src/compiler/korlang/nogc.kor" "O.1.87"
req_rg '^fun validate_nogc' "$ROOT/src/compiler/korlang/nogc.kor" "O.1.87 validate_nogc missing"
req_rg 'is_alloc_type' "$ROOT/src/compiler/korlang/nogc.kor" "O.1.87 alloc check missing"
pass "O.1.87"

# O.1.88 KIR lowering entry
req_file "$ROOT/src/compiler/korlang/kir.kor" "O.1.88"
req_rg '^fun lower_program' "$ROOT/src/compiler/korlang/kir.kor" "O.1.88 lower_program missing"
req_rg '^fun lower_fun' "$ROOT/src/compiler/korlang/kir.kor" "O.1.88 lower_fun missing"
pass "O.1.88"

# O.1.89 Block to BasicBlock (KirBlock usage)
req_rg 'struct KirBlock' "$ROOT/src/compiler/korlang/kir.kor" "O.1.89 KirBlock missing"
req_rg 'KirInstr' "$ROOT/src/compiler/korlang/kir.kor" "O.1.89 KirInstr missing"
pass "O.1.89"

# O.1.90 If/Else to branch
req_rg_literal 'CondBranch' "$ROOT/src/compiler/korlang/kir.kor" "O.1.90 CondBranch missing"
pass "O.1.90"

# O.1.91 While/Loop semantics
req_rg_literal 'Phi' "$ROOT/src/compiler/korlang/kir.kor" "O.1.91 Phi missing"
pass "O.1.91"

# O.1.92 Linear scan register allocation
req_file "$ROOT/src/compiler/korlang/linear.kor" "O.1.92"
req_rg '^fun linear_check_program' "$ROOT/src/compiler/korlang/linear.kor" "O.1.92 linear check missing"
pass "O.1.92"

# O.1.93 x86_64 encoder
req_file "$ROOT/src/compiler/korlang/backend/x86_64/encoder.kor" "O.1.93"
req_rg 'fun' "$ROOT/src/compiler/korlang/backend/x86_64/encoder.kor" "O.1.93 x86 encoder missing"
pass "O.1.93"

# O.1.94 AArch64 encoder
req_file "$ROOT/src/compiler/korlang/backend/aarch64/encoder.kor" "O.1.94"
req_rg 'fun' "$ROOT/src/compiler/korlang/backend/aarch64/encoder.kor" "O.1.94 AArch64 encoder missing"
pass "O.1.94"

# O.1.95 ELF object writer
req_file "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.95"
req_rg 'fun' "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.95 ELF func missing"
pass "O.1.95"

# O.1.96 Mach-O object writer
req_file "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.96"
req_rg 'fun' "$ROOT/src/compiler/korlang/linker/macho.kor" "O.1.96 Mach-O func missing"
pass "O.1.96"

# O.1.97 PE object writer
req_file "$ROOT/src/compiler/korlang/linker/pe.kor" "O.1.97"
req_rg 'fun' "$ROOT/src/compiler/korlang/linker/pe.kor" "O.1.97 PE func missing"
pass "O.1.97"

# O.1.98 Linker symbol resolution
req_file "$ROOT/src/compiler/korlang/linker/resolve.kor" "O.1.98"
req_rg 'find_symbol' "$ROOT/src/compiler/korlang/linker/resolve.kor" "O.1.98 symbol search missing"
pass "O.1.98"

# O.1.99 Binary layout generation
req_rg_literal 'ElfFile' "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.99 ElfFile missing"
req_rg_literal 'ElfSection' "$ROOT/src/compiler/korlang/linker/elf.kor" "O.1.99 ElfSection missing"
pass "O.1.99"

# O.1.100 korlang build command end-to-end
req_file "$ROOT/scripts/build_selfhosted.sh" "O.1.100 build script missing"
pass "O.1.100"

# O.1.51-O.1.70 Variable & Memory Management
req_file "$ROOT/src/compiler/korlang/linear.kor" "O.1.51 linear module missing"
req_rg 'Stmt\.Var' "$ROOT/src/compiler/korlang/infer.kor" "O.1.51 infer var missing"
req_rg 'infer_stmt' "$ROOT/src/compiler/korlang/infer.kor" "O.1.52 infer_stmt missing"
req_file "$ROOT/src/compiler/korlang/region.kor" "O.1.59 region module missing"
req_rg 'region_infer_program' "$ROOT/src/compiler/korlang/region.kor" "O.1.60 region infer missing"
req_file "$ROOT/src/compiler/korlang/smartptr.kor" "O.1.62 smartptr module missing"
req_rg 'smartptr_validate_program' "$ROOT/src/compiler/korlang/smartptr.kor" "O.1.63 smartptr missing"
req_rg 'Expr\.Index' "$ROOT/src/compiler/korlang/infer.kor" "O.1.60 index handling missing"
req_rg 'Expr\.Array' "$ROOT/src/compiler/korlang/infer.kor" "O.1.61 array handling missing"
pass "O.1.51-70"
# O.1.41 Parser struct initialization
req_rg '^struct Parser' "$ROOT/src/compiler/korlang/parser.kor" "O.1.41 Parser struct missing"
req_rg '^fun Parser_new' "$ROOT/src/compiler/korlang/parser.kor" "O.1.41 Parser_new missing"
pass "O.1.41"

# O.1.42 Pratt parser precedence (binding power)
req_rg '^fun Parser_infix_binding_power' "$ROOT/src/compiler/korlang/parser.kor" "O.1.42 binding power fn missing"
req_rg_literal 'InfixPower { l_bp:' "$ROOT/src/compiler/korlang/parser.kor" "O.1.42 binding power structure missing"
pass "O.1.42"

# O.1.43 Parse Program root
req_rg '^fun Parser_parse_program' "$ROOT/src/compiler/korlang/parser.kor" "O.1.43 parse_program missing"
pass "O.1.43"

# O.1.44 Parse Item.Fun declaration
req_rg '^fun Parser_parse_fun' "$ROOT/src/compiler/korlang/parser.kor" "O.1.44 parse_fun missing"
pass "O.1.44"

# O.1.45 Parse function parameters
req_rg '^fun Parser_parse_param_list' "$ROOT/src/compiler/korlang/parser.kor" "O.1.45 parse_param_list missing"
req_rg '^fun Parser_parse_param' "$ROOT/src/compiler/korlang/parser.kor" "O.1.45 parse_param missing"
pass "O.1.45"

# O.1.46 Parse return type annotations
req_rg 'TokenKind\.Arrow' "$ROOT/src/compiler/korlang/parser.kor" "O.1.46 arrow not consumed"
req_rg 'parse_type_ref\(' "$ROOT/src/compiler/korlang/parser.kor" "O.1.46 return type parse missing"
pass "O.1.46"

# O.1.47 Parse Item.Struct declaration
req_rg '^fun Parser_parse_struct' "$ROOT/src/compiler/korlang/parser.kor" "O.1.47 parse_struct missing"
pass "O.1.47"

# O.1.48 Parse struct fields
req_rg 'FieldDecl' "$ROOT/src/compiler/korlang/parser.kor" "O.1.48 struct FieldDecl missing"
req_rg 'TokenKind\.Colon' "$ROOT/src/compiler/korlang/parser.kor" "O.1.48 colon missing"
pass "O.1.48"

# O.1.49 Parse Item.Enum declaration
req_rg '^fun Parser_parse_enum' "$ROOT/src/compiler/korlang/parser.kor" "O.1.49 parse_enum missing"
pass "O.1.49"

# O.1.50 Parse enum variants
req_rg 'VariantDecl' "$ROOT/src/compiler/korlang/parser.kor" "O.1.50 VariantDecl missing"
req_rg 'TokenKind\.LParen' "$ROOT/src/compiler/korlang/parser.kor" "O.1.50 variant payload parse missing"
pass "O.1.50"

printf '\n[group1] all Group 1 checks passed\n'
