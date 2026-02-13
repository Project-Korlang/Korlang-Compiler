#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pass() { printf '[group2] PASS %s\n' "$1"; }
fail() { printf '[group2] FAIL %s\n' "$1" >&2; exit 1; }
req_file() { [[ -f "$1" ]] || fail "$2 (missing: $1)"; }
req_rg() { rg -n "$1" "$2" >/dev/null || fail "$3"; }

# O.2.1 String UTF-8 handling
req_file "$ROOT/src/stdlib/core/string.kor" "O.2.1"
req_file "$ROOT/src/runtime/korlang/stdlib/string_utf8.kor" "O.2.1"
req_rg 'fun utf8Validate' "$ROOT/src/stdlib/core/string.kor" "O.2.1 UTF-8 API missing"
req_rg 'fun utf8_validate' "$ROOT/src/runtime/korlang/stdlib/string_utf8.kor" "O.2.1 UTF-8 validator missing"
req_rg 'fun utf8_count_codepoints' "$ROOT/src/runtime/korlang/stdlib/string_utf8.kor" "O.2.1 UTF-8 codepoint counter missing"
pass "O.2.1"

# O.2.2 List / Array dynamic resizing
req_file "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.2"
req_rg 'fun dyn_reserve' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.2 dyn_reserve missing"
req_rg 'while \(new_cap < need\)' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.2 growth loop missing"
req_rg 'new_cap = new_cap \* 2' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.2 doubling growth missing"
pass "O.2.2"

# O.2.3 HashMap hashing + collision resolution
req_rg 'fun hash_u64' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.3 hash function missing"
req_rg 'collisions' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.3 collision counter missing"
req_rg 'let slot = \(idx \+ step\) % cap' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.3 probe sequence missing"
req_rg 'fun map_rehash' "$ROOT/src/runtime/korlang/stdlib/collections.kor" "O.2.3 rehash missing"
pass "O.2.3"

# O.2.4 Linux file syscalls
req_file "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.4"
req_rg 'LINUX_SYS_OPENAT' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.4 open syscall missing"
req_rg 'LINUX_SYS_READ' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.4 read syscall missing"
req_rg 'LINUX_SYS_WRITE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.4 write syscall missing"
req_rg 'LINUX_SYS_CLOSE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.4 close syscall missing"
pass "O.2.4"

# O.2.5 macOS file syscalls
req_rg 'DARWIN_SYS_OPEN' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.5 open syscall missing"
req_rg 'DARWIN_SYS_READ' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.5 read syscall missing"
req_rg 'DARWIN_SYS_WRITE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.5 write syscall missing"
req_rg 'DARWIN_SYS_CLOSE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.5 close syscall missing"
pass "O.2.5"

# O.2.6 Windows file syscalls
req_rg 'NT_CREATE_FILE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.6 create syscall missing"
req_rg 'NT_READ_FILE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.6 read syscall missing"
req_rg 'NT_WRITE_FILE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.6 write syscall missing"
req_rg 'NT_CLOSE' "$ROOT/src/runtime/korlang/stdlib/fs.kor" "O.2.6 close syscall missing"
pass "O.2.6"

# O.2.7 Native allocator stress
cargo test --manifest-path "$ROOT/src/runtime/Cargo.toml" --test allocator_stress -- --nocapture
pass "O.2.7"

# O.2.8 SHA-256 test vectors
req_file "$ROOT/src/stdlib/core/crypto/sha2.kor" "O.2.8"
req_rg 'fun sha256_test_vectors_ok' "$ROOT/src/stdlib/core/crypto/sha2.kor" "O.2.8 test vector function missing"
req_rg 'let abc = sha256_hash_blocks' "$ROOT/src/stdlib/core/crypto/sha2.kor" "O.2.8 abc vector missing"
pass "O.2.8"

# O.2.9 AES encryption/decryption correctness
req_file "$ROOT/src/stdlib/core/crypto/aes.kor" "O.2.9"
req_rg 'fun aes_encrypt_block' "$ROOT/src/stdlib/core/crypto/aes.kor" "O.2.9 encrypt missing"
req_rg 'fun aes_decrypt_block' "$ROOT/src/stdlib/core/crypto/aes.kor" "O.2.9 decrypt missing"
req_rg 'fun aes_test_vectors_ok' "$ROOT/src/stdlib/core/crypto/aes.kor" "O.2.9 vector validation missing"
pass "O.2.9"

# O.2.10 TLS 1.3 handshake state machine
req_file "$ROOT/src/stdlib/core/net/tls13.kor" "O.2.10"
req_rg 'enum TlsState' "$ROOT/src/stdlib/core/net/tls13.kor" "O.2.10 state enum missing"
req_rg 'fun tls_client_hello' "$ROOT/src/stdlib/core/net/tls13.kor" "O.2.10 client hello missing"
req_rg 'fun tls_on_server_hello' "$ROOT/src/stdlib/core/net/tls13.kor" "O.2.10 server hello missing"
req_rg 'fun tls_finish' "$ROOT/src/stdlib/core/net/tls13.kor" "O.2.10 finish missing"
pass "O.2.10"

# O.2.11 HTTP/3 framing + multiplexing
req_file "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.11"
req_rg 'struct Http3Frame' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.11 frame struct missing"
req_rg 'stream_id' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.11 stream id missing"
req_rg 'fun http3_mux_pack' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.11 mux pack missing"
req_rg 'fun http3_mux_unpack' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.11 mux unpack missing"
pass "O.2.11"

# O.2.12 WebSocket upgrade + masking
req_rg 'fun ws_upgrade_request' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.12 upgrade helper missing"
req_rg 'fun ws_apply_mask' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.12 masking function missing"
req_rg 'fun ws_mask_bytes' "$ROOT/src/stdlib/core/net/http3_ws.kor" "O.2.12 mask backend hook missing"
pass "O.2.12"

printf '\n[group2] all Group 2 checks passed\n'
