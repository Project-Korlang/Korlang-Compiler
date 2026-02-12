# Current Phase: Phase 13 - Native Backend (Removing LLVM Dependency)

**Status:** Initializing Native Code Generation
**Goal:** Achieve total toolchain independence by implementing a native backend that emits machine code (x86_64 and AArch64) directly, removing the dependency on LLVM.

---

## 🚀 13.1 x86_64 Machine Code Generator
**Objective:** Emit binary instructions for Intel/AMD processors.
- [x] **13.1.1 Encoder Implementation:** Added a native x86_64 encoder subset with REX/ModR/M in `src/compiler/korlang/backend/x86_64/encoder.kor`.
- [x] **13.1.2 Register Allocation:** Added linear-scan allocator skeleton in `src/compiler/korlang/backend/x86_64/regalloc.kor`.
- [x] **13.1.3 ABI Implementation:** Added System V + Win64 ABI tables in `src/compiler/korlang/backend/x86_64/abi.kor`.
- **Effort:** 20 Days | **Priority:** High

## 🚀 13.2 AArch64 Machine Code Generator
**Objective:** Emit binary instructions for ARM processors (Apple Silicon, Android).
- [x] **13.2.1 Instruction Encoding:** Added an AArch64 encoder subset in `src/compiler/korlang/backend/aarch64/encoder.kor`.
- [x] **13.2.2 ARM-Specific Optimization:** Added fixed-length encoding helpers in `src/compiler/korlang/backend/aarch64/opt.kor`.
- **Effort:** 15 Days | **Priority:** Medium

## 🚀 13.3 Native Linker Implementation
**Objective:** Produce executable files (ELF, Mach-O, PE) from generated object data.
- [x] **13.3.1 Format Parsers:** Added ELF/Mach-O/PE writer stubs in `src/compiler/korlang/linker/*`.
- [x] **13.3.2 Symbol Resolution:** Added relocation resolution stubs in `src/compiler/korlang/linker/resolve.kor`.
- **Effort:** 12 Days | **Priority:** Critical

---

## 📈 Verification Status
- **Phase 12 (Independence):** **Completed.** The runtime (GC, Scheduler, Stdlib) is now implemented in pure Korlang.
- **Phase 13 (Native Backend):** **Active.** Beginning the transition from LLVM IR emission to direct machine code generation.

---

## 📊 Phase 13 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| x86_64 Gen | 20 Days | KIR Structure | High |
| AArch64 Gen | 15 Days | KIR Structure | Medium |
| Native Linker| 12 Days | OS Formats | Medium |
| **Total** | **47 Days** | | |

**Next Step:** Wire native backend into KIR lowering and emit object formats.
