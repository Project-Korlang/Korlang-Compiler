# Korlang Language Development Roadmap & Tasks (Post-Bootstrap Edition)

This document tracks the evolution of the **Korlang** programming language. Phases 1-6 are fully operational.

---

## ✅ Phase 1: Core Design & Specification (Completed)
- [x] 1.1 Formal Grammar Definition
- [x] 1.2 Type System Specification

## ✅ Phase 2: The Bootstrap Compiler (Completed)
- [x] 2.1 Lexer, Parser, Sema, Codegen

## ✅ Phase 3: The "Smart-GC" & Runtime (Completed)
- [x] 3.1 Tiered GC, Scheduler, FFI

## ✅ Phase 4: Native UI Engine (`view`) (Completed)
- [x] 4.1 Declarative UI & WGPU Backend

## ✅ Phase 5: Cloud & AI Native Features (Completed)
- [x] 5.1 `resource` blocks & Tensor SIMD

## ✅ Phase 6: Tooling & Ecosystem (Completed)
- [x] 6.1 KPM & Basic CLI Tooling

---

## 🚀 Phase 7: The Professional CLI & Global Distribution (Current)
**Goal:** Make `korlang` a world-class CLI tool like `cargo`. Ensure it can be installed globally, manage projects, and handle toolchain updates.

- [ ] **7.1 Project Scaffolding (`korlang new`):** Implement robust template generation for Apps, Libraries, and Cloud-services.
- [ ] **7.2 Global Installation & Pathing:** Implement logic for `korlang` to find its own standard library and runtime regardless of where it's called from.
- [ ] **7.3 `korup`: The Toolchain Manager:** (See `tasks_site.md` for installer/site details).
- [ ] **7.4 Build Caching:** Implement a global cache directory (`~/.korlang/cache`) to speed up subsequent builds by reusing compiled standard libraries.
- [ ] **7.5 OS-Specific Packaging:** Prepare `.deb`, `.rpm`, Homebrew formula, and MSI installers for native system integration.

---

## ⚡ Phase 8: Optimization & Peak Performance
- [ ] 8.1 LLVM LTO & PGO
- [ ] 8.2 GC Tuning for Sub-ms Latency
- [ ] 8.3 Tensor Kernel JIT

## 📱 Phase 9: Mobile, Embedded & WASM
... (continuing the previous roadmap)
