# Current Phase: Phase 17 - Standard Library 2.0 (The Global Standard)

**Status:** Activating Native Ecosystem
**Goal:** Implement high-performance, security-first standard library modules for Crypto and Networking. All code must be written in pure Korlang or inline Assembly to ensure zero reliance on external language runtimes.

---

## 📦 17.1 Native Crypto
**Objective:** Provide industry-standard cryptographic primitives.
- [x] **17.1.1 Hashing Primitives:** Added SHA-256, SHA-3, and BLAKE3 reference implementations in `src/stdlib/core/crypto/{sha2,sha3,blake3}.kor`.
- [x] **17.1.2 Symmetric Encryption:** Added AES (with hardware hook) and ChaCha20 in `src/stdlib/core/crypto/{aes,chacha20}.kor`.
- [x] **17.1.3 Asymmetric Encryption:** Added Ed25519 and RSA verification logic in `src/stdlib/core/crypto/{ed25519,rsa}.kor`.
- **Effort:** 12 Days | **Priority:** High

## 📦 17.2 Native Networking
**Objective:** High-throughput, modern networking stack.
- [x] **17.2.1 TCP/UDP Core:** Added syscall-dispatcher-backed socket wrappers in `src/stdlib/core/net/tcp_udp.kor`.
- [x] **17.2.2 TLS 1.3 Implementation:** Added TLS 1.3 handshake/record reference stack in `src/stdlib/core/net/tls13.kor`.
- [x] **17.2.3 HTTP/3 & WebSockets:** Added HTTP/3 + WebSocket framing with zero-copy buffers in `src/stdlib/core/net/http3_ws.kor`.
- **Effort:** 15 Days | **Priority:** High

---

## 🛡️ Total Decoupling Mandate
**Strategy:** "Korlang-Only"
- **Rule:** No new C, Rust, or Python code is allowed.
- **Task:** As we implement 17.1 and 17.2, any existing runtime shims (e.g., in `src/runtime/korlang/os.kor` or `io.kor`) that still link to Rust-era symbols must be rewritten to use the assembly `syscall` dispatcher.
- **Execution:** Completed by routing `src/runtime/korlang/os.kor`, `src/runtime/korlang/stdlib/io.kor`, and `src/runtime/korlang/syscall.kor` through `runtime.syscall.dispatcher`.

---

## 📈 Verification Status
- **Phase 16 (Parallelism):** **Completed.** Work-stealing 2.0 and GPU JIT are operational.
- **Phase 17 (Stdlib 2.0):** **Completed.** Native crypto/network reference stack and decoupling mandate are implemented.

---

## 📊 Phase 17 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Crypto | 12 Days | BigInt Support | High |
| Networking | 15 Days | Syscall Lib | Medium |
| **Total** | **27 Days** | | |

**Next Step:** Begin Phase 18 Korlang-native IDE foundations.
