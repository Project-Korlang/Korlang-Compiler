# Current Phase: Phase 19 - Multimedia, Real-Time Engine & CLI Hardening

**Status:** Activating Media & Hardening Toolchain
**Goal:** Transition Korlang from a "compiles but remains silent" state to a "fully functional system language". This involves finishing media primitives while simultaneously hardening the CLI and Runtime so that `korlang run` produces real output, handles errors professionally, and behaves like a world-class tool.

---

## 🎭 19.1 Audio/Video Pipeline
**Objective:** A professional-grade media processing engine.
- [x] **19.1.1 Media Graph Core**
    - [x] Implement the directed acyclic graph (DAG) node manager.
    - [x] Implement asynchronous sample-rate conversion.
    - [x] Add support for "Side-chain" inputs in the audio graph.
- [x] **19.1.2 Audio Backend Integration**
    - [x] Bind Linux ALSA/PulseAudio via direct syscalls (bypass LibC).
    - [x] Implement macOS CoreAudio native bindings.
    - [x] Implement Windows WASAPI (low-latency mode) bindings.
- [x] **19.1.3 Video Codec & Frame Buffers**
    - [x] Implement zero-copy YUV to RGB conversion in WGPU shaders.
    - [x] Integrate H.264/H.265 hardware decoding via native OS APIs (VA-API/VideoToolbox).

## 🎭 19.2 Zero-Latency Graphics
**Objective:** Direct hardware access for high-speed rendering.
- [x] **19.2.1 Native Driver Bindings:** Direct syscall-level access to Vulkan/Metal to remove any remaining Rust abstraction overhead.
- [x] **19.2.2 Shader JIT Compiler:** Build a native Korlang module that translates Korlang ASTs directly into SPIR-V or Metal Bytecode.
- [x] **19.2.3 Windowing & Surface Management:** Implement native window creation (Wayland/X11, AppKit, Win32) without external libraries.

## 💻 19.3 Professional CLI & Runtime Hardening (The "Realism" Pass)
**Objective:** Ensure that `korlang run` behaves like a mature development tool with full I/O fidelity.

- [x] **19.3.1 Standard I/O Fidelity (The `print` Fix)**
    - [x] **Direct Syscall Binding:** Ensure the Korlang `print()` function is directly wired to the `write(1, ...)` assembly syscall.
    - [x] **Buffering Strategy:** Implement a thread-safe `LineBuffer` for `stdout` to ensure interleaved output from `spawn` tasks doesn't corrupt lines.
    - [x] **Automatic Stringification:** Implement implicit `.toString()` calling for any object passed to `print`.
- [x] **19.3.2 CLI Execution Engine**
    - [x] **Real-time Streaming:** Update `korlang run` to stream `stdout`/`stderr` from the child process in real-time, rather than waiting for compilation to end.
    - [x] **Process Lifecycle Management:** Correctly capture child process exit codes and return them to the shell.
    - [x] **Argument Forwarding:** Implement the `--` syntax: `korlang run file.kor -- arg1 arg2` must correctly populate `fun main(args: List<String>)`.
- [x] **19.3.3 Advanced Error Reporting & Stack Traces**
    - [x] **Dwarf/PDB Symbol Support:** Implement a native parser for debug symbols so the runtime can show file names and line numbers during a `panic()`.
    - [x] **Colorized Diagnostics:** High-quality, accessible terminal output for compiler errors (using the "Aero" UI style).
- [x] **19.3.4 REPL & Interactive Mode**
    - [x] **Incremental Compilation:** Support for the JIT engine to compile and execute single expressions in a persistent session.
    - [x] **The `korlang repl` command:** A professional-grade shell with history, tab-completion, and live type-checking.

---

## 🛡️ Independence & Performance Mandate
- **No Silence:** A running Korlang app must be able to communicate with the user immediately via `print`.
- **Pure Native:** No dependency on `printf` or other LibC symbols.
- **Zero Jitter:** Real-time audio/video tasks must have priority in the M:N scheduler to prevent audio dropouts.

---

## 📈 Verification Status
- **Phase 18 (Native IDE):** **Completed.**
- **Phase 19 (Multimedia & CLI):** **Completed.** Media graph, native graphics bindings, and CLI hardening delivered.

---

## 📊 Phase 19 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Media Graph | 15 Days | Task Scheduler | Medium |
| CLI Hardening | 10 Days | Syscall Lib | Low |
| Native GPU | 25 Days | WGPU Backend | High |
| **Total** | **50 Days** | | |

**Next Step:** Begin Phase 20.1 by defining built-in LLM primitives and provider abstraction interfaces.
