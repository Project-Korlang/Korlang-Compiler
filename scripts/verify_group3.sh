#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pass() { printf '[group3] PASS %s\n' "$1"; }
fail() { printf '[group3] FAIL %s\n' "$1" >&2; exit 1; }
req_file() { [[ -f "$1" ]] || fail "$2 (missing: $1)"; }
req_rg() { rg -n "$1" "$2" >/dev/null || fail "$3"; }

# O.3.1 / O.3.2 / O.3.5 / O.3.6 / O.3.7 runtime stress tests
cargo test --manifest-path "$ROOT/src/runtime/Cargo.toml" o3_ -- --nocapture
pass "O.3.1"
pass "O.3.2"
pass "O.3.5"
pass "O.3.6"
pass "O.3.7"

# O.3.3 x86_64 register preservation coverage
req_file "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3"
req_rg 'push %rbx' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing RBX save"
req_rg 'push %rbp' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing RBP save"
req_rg 'push %r12' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing R12 save"
req_rg 'push %r13' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing R13 save"
req_rg 'push %r14' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing R14 save"
req_rg 'push %r15' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing R15 save"
req_rg 'pop %r15' "$ROOT/src/runtime/korlang/arch/x86_64/context_switch.s" "O.3.3 missing restore sequence"
pass "O.3.3"

# O.3.4 AArch64 register preservation coverage
req_file "$ROOT/src/runtime/korlang/arch/aarch64/context_switch.s" "O.3.4"
req_rg 'stp x29, x30' "$ROOT/src/runtime/korlang/arch/aarch64/context_switch.s" "O.3.4 missing frame save"
req_rg 'stp x19, x20' "$ROOT/src/runtime/korlang/arch/aarch64/context_switch.s" "O.3.4 missing x19/x20 save"
req_rg 'ldp x19, x20' "$ROOT/src/runtime/korlang/arch/aarch64/context_switch.s" "O.3.4 missing x19/x20 restore"
req_rg 'ldp x29, x30' "$ROOT/src/runtime/korlang/arch/aarch64/context_switch.s" "O.3.4 missing frame restore"
pass "O.3.4"

# O.3.8 Mutex + CondVar deadlock-freedom model presence
req_file "$ROOT/src/runtime/korlang/sync.kor" "O.3.8"
req_rg 'fun mutex_lock' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing mutex_lock"
req_rg 'fun mutex_unlock' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing mutex_unlock"
req_rg 'fun cond_wait' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing cond_wait"
req_rg 'fun cond_signal' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing cond_signal"
req_rg 'fun cond_broadcast' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing cond_broadcast"
req_rg 'os_yield' "$ROOT/src/runtime/korlang/sync.kor" "O.3.8 missing scheduler yield path"
pass "O.3.8"

printf '\n[group3] all Group 3 checks passed\n'
