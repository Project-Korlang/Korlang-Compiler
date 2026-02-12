# Current Phase: Phase 10 - Self-Hosting Part 3: KIR & LLVM Codegen

**Status:** Completed
**Goal:** Finalize the self-hosted compiler's backend. This includes lowering the type-checked AST to Korlang Intermediate Representation (KIR), implementing the LLVM FFI bindings, and producing the "Stage 1" native compiler binary.

---

## ⚙️ 10.1 KIR Lowering Logic
**Objective:** Translate the type-checked AST into Korlang Intermediate Representation (KIR).
- [x] **10.1.1 AST to KIR Translation:** Implement the logic in `kir.kor` to lower high-level constructs (Pipelines, Views, Resources) into linear IR.
- [x] **10.1.2 Variable Lowering:** Map semantic symbols to KIR registers and memory slots.
- **Effort:** 8 Days | **Priority:** High

## ⚙️ 10.2 LLVM FFI & Codegen
**Objective:** Bridge the native compiler to the LLVM infrastructure.
- [x] **10.2.1 FFI Integration:** Finalize the `@import("c")` bindings in `llvm_bindings.kor` for the LLVM C API.
- [x] **10.2.2 LLVM IR Emission:** Implement the visitor that converts KIR modules into LLVM IR modules.
- **Effort:** 10 Days | **Priority:** High

## ⚙️ 10.3 Stage 1 Bootstrap
**Objective:** Compile the Korlang compiler using the existing Rust-based toolchain.
- [x] **10.3.1 Compiler Build Script:** Automate the compilation of `src/compiler/korlang/*.kor` using the current `korlang build` command.
- [x] **10.3.2 Binary Verification:** Ensure the produced "Stage 1" binary can successfully lex and parse itself.
- **Effort:** 5 Days | **Priority:** Critical

---

## 📈 Verification Status
- **Phase 8 (Frontend):** **Completed.** Native Lexer and Parser are operational.
- **Phase 9 (Sema):** **Completed.** Type inference and symbol table logic are fully implemented in Korlang.
- **Phase 10 (Backend):** **Completed.** KIR lowering, LLVM IR emission, and Stage 1 bootstrap script are implemented.

---

## 📊 Phase 10 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| KIR Lowering | 8 Days | Semantic AST | Medium |
| LLVM Bindings | 10 Days | KIR / FFI | High |
| Stage 1 Build | 5 Days | All Above | Medium |
| **Total** | **23 Days** | | |

**Next Step:** Begin Phase 11 bootstrapping stages and bit-for-bit verification.
