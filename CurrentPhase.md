# Current Phase: Phase 11 - The "Great Switch" (Bootstrapping)

**Status:** In Progress
**Goal:** Achieve full self-hosting by using the Korlang-based compiler to compile itself. This confirms the compiler's logic is sound and independent of the Rust bootstrap toolchain.

---

## 🔄 11.1 Stage 1: Native Compiler (Completed)
**Objective:** Compile the Korlang compiler source using the Rust-based toolchain.
- [x] **11.1.1 Build Stage 1:** Run `scripts/bootstrap.sh` to produce the first native binary from Korlang source.
- [x] **11.1.2 Verification:** Verify that the Stage 1 binary can compile basic examples.
- **Status:** Done. `dist/bootstrap-stage1/bin/korlang` and `build/korlang-selfhosted` are available.

## 🔄 11.2 Stage 2: Self-Compilation
**Objective:** Use the Stage 1 (self-hosted) binary to compile the Korlang compiler source.
- [x] **11.2.1 Self-Build:** Configure `scripts/build_selfhosted.sh` to use the Stage 1 binary as the compiler.
- [x] **11.2.2 Binary Generation:** Produce a Stage 2 binary.
- **Effort:** 4 Days | **Priority:** High

## 🔄 11.3 Stage 3: Fixpoint Verification
**Objective:** Verify that the compiler has reached a stable "fixpoint".
- [ ] **11.3.1 Reproducible Build:** Use the Stage 2 binary to compile the source again, producing a Stage 3 binary.
- [ ] **11.3.2 Bit-for-Bit Check:** Ensure Stage 2 and Stage 3 binaries are identical.
- **Effort:** 3 Days | **Priority:** Critical

**Status Note:** Stage 2 binary is currently a stub selfhosted binary (compiler sources + minimal `main`), so it cannot compile the compiler to produce Stage 3 yet. This blocks 11.3 until a selfhosted CLI/driver exists.

---

## 📈 Verification Status
- **Phase 10 (Backend):** **Completed.** KIR lowering and textual LLVM emission are implemented.
- **Phase 11 (Switch):** **Active.** Stage 1 is complete; proceeding to Stage 2 self-build.

---

## 📊 Phase 11 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Stage 2 Build | 4 Days | Stage 1 Binary | Medium |
| Fixpoint Check | 3 Days | Stage 2 Binary | Low |
| **Total** | **7 Days** | | |

**Next Step:** Modify `build_selfhosted.sh` to support using the self-hosted binary for the next stage of compilation.
