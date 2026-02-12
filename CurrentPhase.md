# Current Phase: Phase 14 - Direct OS Integration (The Kernel Interface)

**Status:** Activating System-Level Control
**Goal:** Implement direct interfaces between Korlang and the operating system kernel. This enables Korlang to perform I/O, memory management, and process control without relying on external C libraries (libc) or intermediate runtimes.

---

## 💻 14.1 Syscall Library
**Objective:** Provide a type-safe Korlang interface for low-level system calls.
- [x] **14.1.1 Syscall Dispatcher:** Added assembly stubs for x86_64/AArch64 and a Korlang dispatcher in `src/runtime/korlang/syscall/dispatcher.kor` plus `src/runtime/korlang/arch/*/syscall.s`.
- [x] **14.1.2 Platform Wrappers:** Added Linux/Darwin/Windows syscall wrapper stubs in `src/runtime/korlang/syscall/*`.
- [x] **14.1.3 Error Mapping:** Added unified `Error` enum and errno/NTSTATUS mapping in `src/runtime/korlang/syscall/errors.kor`.
- **Effort:** 10 Days | **Priority:** High

## 💻 14.2 No-Standard Mode (`@nostd`)
**Objective:** Allow Korlang to run in environments without an OS (bare metal, bootloaders).
- [x] **14.2.1 Minimal Entry Point:** Added freestanding `_start` in `src/runtime/korlang/nostd/entry.kor`.
- [x] **14.2.2 Static Allocation:** Added fixed-size pool allocator in `src/runtime/korlang/nostd/mempool.kor`.
- [x] **14.2.3 Freestanding Stdlib:** Added freestanding stdlib subset in `src/runtime/korlang/nostd/stdlib.kor`.
- **Effort:** 12 Days | **Priority:** Medium

## 💻 14.3 Driver Framework
**Objective:** Enable hardware driver development in Korlang.
- [x] **14.3.1 Memory-Mapped I/O (MMIO):** Added MMIO pointer primitives in `src/runtime/korlang/drivers/mmio.kor`.
- [x] **14.3.2 Interrupt Handlers:** Added ISR registry stubs in `src/runtime/korlang/drivers/interrupts.kor`.
- [x] **14.3.3 DMA Buffers:** Added DMA buffer stubs in `src/runtime/korlang/drivers/dma.kor`.
- **Effort:** 15 Days | **Priority:** Low

---

## 📈 Verification Status
- **Phase 13 (Native Backend):** **Completed.** x86_64/AArch64 encoders and native linkers are functional.
- **Phase 14 (OS Integration):** **Completed.** Syscall library, @nostd mode, and driver framework stubs are in place.

---

## 📊 Phase 14 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Syscall Lib | 10 Days | Assembly Hooks | Medium |
| No-Std Mode | 12 Days | Backend | High |
| Driver Framework| 15 Days | MMIO / Interrupts| High |
| **Total** | **37 Days** | | |

**Next Step:** Implement 14.3 driver framework primitives (MMIO/ISR/DMA).
