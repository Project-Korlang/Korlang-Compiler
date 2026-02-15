# Current Phase: Phase Reality - Functional Singularity

**Status:** Implementation & Hardening
**Goal:** Transform Korlang into a fully functional, self-sufficient system language. This phase ensures that the compiler correctly handles all expressions, the runtime correctly executes all logic, and the standard library provides all basic functions—all written in 100% pure Korlang.

---

## 🏛️ Group 1: The "Brains" - Compiler Logic Implementation [150 Tasks]
**Objective:** Finalize the backend lowering logic to ensure every operator and control flow structure generates valid machine code.

### 1.1 Arithmetic & Logic Pipeline (Reconfirmed)
- [x] **R.1.1** Implement `Add` lowering in `kir.kor`: Emit `KirInstr.Add`.
- [x] **R.1.2** Implement `Sub` lowering: Emit `KirInstr.Sub`.
- [x] **R.1.3** Implement `Mul` lowering: Emit `KirInstr.Mul`.
- [x] **R.1.4** Implement `Div` lowering: Emit `KirInstr.Div`.
- [x] **R.1.5** Implement `Mod` lowering: Emit `KirInstr.Mod`.
- [x] **R.1.6** Implement Float `Add` (FAdd).
- [x] **R.1.7** Implement Float `Sub` (FSub).
- [x] **R.1.8** Implement Float `Mul` (FMul).
- [x] **R.1.9** Implement Float `Div` (FDiv).
- [x] **R.1.10** Implement Comparison: `Eq` (Equal).
- [x] **R.1.11** Implement Comparison: `NotEq` (Not Equal).
- [x] **R.1.12** Implement Comparison: `Lt` (Less Than).
- [x] **R.1.13** Implement Comparison: `Gt` (Greater Than).
- [x] **R.1.14** Implement Comparison: `LtEq`.
- [x] **R.1.15** Implement Comparison: `GtEq`.
- [x] **R.1.16** Implement Logical `And`.
- [x] **R.1.17** Implement Logical `Or`.
- [x] **R.1.18** Implement Logical `Not`.
- [x] **R.1.19** Implement Bitwise `And`.
- [x] **R.1.20** Implement Bitwise `Or`.
- [x] **R.1.21** Implement Bitwise `Xor`.
- [x] **R.1.22** Implement Bitwise `Shl`.
- [x] **R.1.23** Implement Bitwise `Shr`.
- [x] **R.1.24** Implement Precedence handling for `(a + b) * c`.
- [x] **R.1.25** Implement Unary `-` (Negation).
- [x] **R.1.26** Implement Unary `+` (Identity).
- [x] **R.1.27** Implement Boolean to Integer casting.
- [x] **R.1.28** Implement Integer to Float casting.
- [x] **R.1.29** Implement Float to Integer casting.
- [x] **R.1.30** Implement String to Boolean truthiness.

### 1.2 Control Flow Stability (Reconfirmed)
- [x] **R.1.31** Implement `If` statement branching logic.
- [x] **R.1.32** Implement `Else` branch jump targets.
- [x] **R.1.33** Implement `While` loop condition checking.
- [x] **R.1.34** Implement `While` loop back-edges.
- [x] **R.1.35** Implement `For` loop iterator initialization.
- [x] **R.1.36** Implement `For` loop increment logic.
- [x] **R.1.37** Implement `Match` statement jump table generation.
- [x] **R.1.38** Implement `Match` default case (else).
- [x] **R.1.39** Implement `Break` statement scope searching.
- [x] **R.1.40** Implement `Continue` statement jump to header.
- [x] **R.1.41** Implement `Return` from nested blocks.
- [x] **R.1.42** Implement `Return` with values.
- [x] **R.1.43** Implement `Return` without values (Void).
- [x] **R.1.44** Implement Short-circuiting for `&&`.
- [x] **R.1.45** Implement Short-circuiting for `||`.
- [x] **R.1.46** Implement Ternary/Elvis expression lowering.
- [x] **R.1.47** Implement Nested `If` statements.
- [x] **R.1.48** Implement Nested `While` loops.
- [x] **R.1.49** Implement Infinite `Loop` construct.
- [x] **R.1.50** Implement Tail-call optimization (Basic).

### 1.3 Variable & Memory Management (Reconfirmed)
- [x] **R.1.51** Implement `Let` immutable binding enforcement.
- [x] **R.1.52** Implement `Var` mutable update logic.
- [x] **R.1.53** Implement Pointer-to-Variable resolution.
- [x] **R.1.54** Implement Global variable initialization.
- [x] **R.1.55** Implement Local variable stack allocation.
- [x] **R.1.56** Implement Struct field offset calculation.
- [x] **R.1.57** Implement Struct field loading.
- [x] **R.1.58** Implement Struct field storing.
- [x] **R.1.59** Implement Array index bound checking (Runtime).
- [x] **R.1.60** Implement Array indexing lowering.
- [x] **R.1.61** Implement Dynamic array allocation.
- [x] **R.1.62** Implement String concatenation lowering.
- [x] **R.1.63** Implement Character indexing into Strings.
- [x] **R.1.64** Implement Reference tracking for `&` (Borrow).
- [x] **R.1.65** Implement Dereference `*`.
- [x] **R.1.66** Implement Copy semantics for primitives.
- [x] **R.1.67** Implement Move semantics for unique objects.
- [x] **R.1.68** Implement Closure capture (Environment packing).
- [x] **R.1.69** Implement Lambda lifting.
- [x] **R.1.70** Implement Static member access.

*Verification:* `scripts/verify_group1.sh` now covers Region/Linear/Smartptr checks, so it validates the Variable & Memory Management block (R.1.51–R.1.70) whenever Group 1 runs.

### 1.4 Native Backend opcodes (R.1.71 - R.1.150)
- [x] **R.1.71** x86_64 GPR encoding (RAX-R15).
- [x] **R.1.72** x86_64 XMM encoding (XMM0-XMM15).
- [x] **R.1.73** AArch64 GPR encoding (X0-X30, SP).
- [x] **R.1.74** AArch64 Float encoding (V0-V31).
- [x] **R.1.75** REX prefix generation for x86_64.
- [x] **R.1.76** ModR/M byte encoding for x86_64.
- [x] **R.1.77** SIB byte encoding for x86_64.
- [x] **R.1.78** Immediate value encoding (8/16/32/64 bit).
- [x] **R.1.79** x86_64 `MOV` reg, reg instruction.
- [x] **R.1.80** x86_64 `MOV` reg, imm instruction.
- [x] **R.1.81** x86_64 `ADD` reg, reg instruction.
- [x] **R.1.82** x86_64 `SUB` reg, reg instruction.
- [x] **R.1.83** x86_64 `IMUL` reg, reg instruction.
- [x] **R.1.84** x86_64 `IDIV` (setup RDX:RAX).
- [x] **R.1.85** x86_64 `CMP` and `TEST` instructions.
- [x] **R.1.86** x86_64 `JMP` (relative 8/32).
- [x] **R.1.87** x86_64 `Jcc` (conditional branches).
- [x] **R.1.88** x86_64 `CALL` and `RET` instructions.
- [x] **R.1.89** x86_64 `PUSH` and `POP` instructions.
- [x] **R.1.90** x86_64 `LEA` for address calculation.
- [x] **R.1.91** x86_64 SSE `ADDSD` / `ADDSS`.
- [x] **R.1.92** x86_64 SSE `SUBSD` / `SUBSS`.
- [x] **R.1.93** x86_64 SSE `MULSD` / `MULSS`.
- [x] **R.1.94** x86_64 SSE `DIVSD` / `DIVSS`.
- [x] **R.1.95** x86_64 SSE `COMISD` (Compare).
- [x] **R.1.96** x86_64 bitwise `AND`, `OR`, `XOR`, `NOT`.
- [x] **R.1.97** x86_64 shift `SHL`, `SHR`, `SAR`.
- [x] **R.1.98** x86_64 conversion `CVTSI2SD` (Int to Float).
- [x] **R.1.99** x86_64 conversion `CVTTSD2SI` (Float to Int).
- [x] **R.1.100** x86_64 `SETcc` for boolean results.
- [x] **R.1.101** AArch64 `MOV` (register) instruction.
- [x] **R.1.102** AArch64 `MOV` (wide immediate) instruction.
- [x] **R.1.103** AArch64 `ADD` (shifted register) instruction.
- [x] **R.1.104** AArch64 `SUB` (shifted register) instruction.
- [x] **R.1.105** AArch64 `MUL` instruction.
- [x] **R.1.106** AArch64 `SDIV` / `UDIV` instructions.
- [x] **R.1.107** AArch64 `CMP` (SUBS alias).
- [x] **R.1.108** AArch64 `B` (unconditional branch).
- [x] **R.1.109** AArch64 `B.cond` (conditional branch).
- [x] **R.1.110** AArch64 `BL` (branch with link) and `RET`.
- [x] **R.1.111** AArch64 `LDR` / `STR` (base + offset).
- [x] **R.1.112** AArch64 `STP` / `LDP` (pair for stack).
- [x] **R.1.113** AArch64 `FADD` (double/single).
- [x] **R.1.114** AArch64 `FSUB`.
- [x] **R.1.115** AArch64 `FMUL`.
- [x] **R.1.116** AArch64 `FDIV`.
- [x] **R.1.117** AArch64 `FCMP`.
- [x] **R.1.118** AArch64 bitwise `AND`, `ORR`, `EOR`, `MVN`.
- [x] **R.1.119** AArch64 `LSL`, `LSR`, `ASR`.
- [x] **R.1.120** AArch64 `SCVTF` (Int to Float).
- [x] **R.1.121** AArch64 `FCVTZS` (Float to Int).
- [x] **R.1.122** AArch64 `CSET` for boolean results.
- [x] **R.1.123** AArch64 `ADRP` and `ADD` for global addressing.
- [x] **R.1.124** SysV ABI: x86_64 arg passing (RDI, RSI, RDX, RCX, R8, R9).
- [x] **R.1.125** SysV ABI: x86_64 return value (RAX).
- [x] **R.1.126** Win64 ABI: x86_64 arg passing (RCX, RDX, R8, R9).
- [x] **R.1.127** AArch64 ABI: arg passing (X0-X7).
- [x] **R.1.128** AArch64 ABI: return value (X0).
- [x] **R.1.129** Stack frame management: x86_64 Prologue.
- [x] **R.1.130** Stack frame management: x86_64 Epilogue.
- [x] **R.1.131** Stack frame management: AArch64 Prologue.
- [x] **R.1.132** Stack frame management: AArch64 Epilogue.
- [x] **R.1.133** Red zone utilization on x86_64 SysV.
- [x] **R.1.134** Shadow space management on Win64.
- [x] **R.1.135** Register allocator: Spill logic implementation.
- [x] **R.1.136** Register allocator: Coalescing logic.
- [x] **R.1.137** Peephole: `MOV RAX, 0` -> `XOR EAX, EAX`.
- [x] **R.1.138** Peephole: `ADD RAX, 1` -> `INC RAX`.
- [x] **R.1.139** Peephole: Constant folding at emit time.
- [x] **R.1.140** Peephole: Redundant load elimination.
- [x] **R.1.141** Emission of DWARF FDE/CIE for stack unwinding.
- [x] **R.1.142** x86_64 Atomic instructions (LOCK prefix).
- [x] **R.1.143** AArch64 Atomic instructions (LDADD, etc.).
- [x] **R.1.144** x86_64 `SYSCALL` instruction emission.
- [x] **R.1.145** AArch64 `SVC` instruction emission.
- [x] **R.1.146** Support for `REP STOSB` for zeroing memory.
- [x] **R.1.147** Vector instruction emission (Basic SSE/NEON).
- [x] **R.1.148** Backend sanity check: No-op binary generation.
- [x] **R.1.149** Backend sanity check: Minimal "exit(42)" binary.
- [x] **R.1.150** Finalize Backend independence from external assemblers.

---

## 📦 Group 2: The "Hands" - Simplified Standard Library [150 Tasks]
**Objective:** Implement core functionality using simple, intuitive keywords.

### 2.1 Basic I/O (Group 2.1)
- [x] **R.2.1** `print(val)` -> Write to stdout without newline.
- [x] **R.2.2** `echo(val)` -> Write to stdout with automatic newline.
- [x] **R.2.3** `input()` -> Read string from stdin.
- [x] **R.2.4** `input(msg)` -> Print prompt and read string.
- [x] **R.2.5** `write(val)` -> Alias for `print`.
- [x] **R.2.6** `error(val)` -> Write to stderr.
- [x] **R.2.7** `clear()` -> Clear terminal screen.
- [x] **R.2.8** `file.open(path)`
- [x] **R.2.9** `file.new(path)` -> Create/Overwrite file.
- [x] **R.2.10** `file.read()` -> Read whole file into string.
- [x] **R.2.11** `file.write(str)`
- [x] **R.2.12** `file.append(str)`
- [x] **R.2.13** `file.exists(path)`
- [x] **R.2.14** `file.delete(path)`
- [x] **R.2.15** `dir.list(path)`
- [x] **R.2.16** `dir.new(path)`
- [x] **R.2.17** `path.join(a, b)`
- [x] **R.2.18** `path.base(p)`
- [x] **R.2.19** `path.ext(p)`
- [x] **R.2.20** `path.abs(p)`

### 2.2 Simple Math (Group 2.2)
- [x] **R.2.21** `abs(n)`
- [x] **R.2.22** `sqrt(n)`
- [x] **R.2.23** `sin(n)`, `cos(n)`, `tan(n)`
- [x] **R.2.24** `pow(base, exp)`
- [x] **R.2.25** `log(n)`
- [x] **R.2.26** `ceil(n)`, `floor(n)`, `round(n)`
- [x] **R.2.27** `min(a, b)`, `max(a, b)`
- [x] **R.2.28** `clamp(v, min, max)`
- [x] **R.2.29** `random()`
- [x] **R.2.30** `PI`, `E` (Built-in constants).

### 2.3 Data & String (Group 2.3)
- [x] **R.2.31** `str.len(s)`
- [x] **R.2.32** `str.sub(s, start, len)`
- [x] **R.2.33** `str.find(s, sub)`
- [x] **R.2.34** `str.upper(s)`, `str.lower(s)`
- [x] **R.2.35** `str.trim(s)`
- [x] **R.2.36** `str.split(s, sep)`
- [x] **R.2.37** `json.parse(s)`
- [x] **R.2.38** `json.str(obj)` -> Stringify.
- [x] **R.2.39** `b64.enc(s)`, `b64.dec(s)`
- [x] **R.2.40** `hex.enc(s)`, `hex.dec(s)`
- [x] **R.2.41 - R.2.80** [40 Tasks] Implement all remaining `List` and `Map` methods from Phase Omega using `list.` and `map.` prefixes.

### 2.4 Time & OS (Group 2.4)
- [x] **R.2.81** `now()` -> Timestamp.
- [x] **R.2.82** `sleep(ms)`
- [x] **R.2.83** `args()` -> List of CLI arguments.
- [x] **R.2.84** `env(key)` -> Get environment variable.
- [x] **R.2.85** `exit(code)`
- [x] **R.2.86 - R.2.150** [65 Tasks] Finalize remaining syscall-backed OS interactions (chmod, fork, exec, wait, pipe, etc.).

---

## 🧵 Group 3: The "Life" - Runtime & Reliability [100 Tasks]
**Objective:** Hardening the M:N Scheduler and Smart-GC using only Korlang.

### 3.1 M:N Scheduler (R.3.1 - R.3.25)
- [x] **R.3.1** Define `Fiber` struct and status enum.
- [x] **R.3.2** Implement `Fiber_new` with stack allocation.
- [x] **R.3.3** Implement native `yield()` mechanism.
- [x] **R.3.4** Implement `Worker` thread local storage.
- [x] **R.3.5** Implement Global Run Queue (GRQ) with lock-free spinlink.
- [x] **R.3.6** Implement Local Run Queue (LRQ) for each worker.
- [x] **R.3.7** Implement Work-Stealing logic (Steal half from LRQ).
- [x] **R.3.8** Implement Fiber parking on blocking syscalls.
- [x] **R.3.9** Implement Scheduler `tick()` for preemption.
- [x] **R.3.10** Implement `spawn` keyword lowering to `scheduler_spawn`.
- [x] **R.3.11** Implement Worker thread parking (idle state).
- [x] **R.3.12** Implement Worker thread wakeup (on new work).
- [x] **R.3.13** Implement Fiber `exit` and stack cleanup.
- [x] **R.3.14** Implement Fiber `join` logic.
- [x] **R.3.15** Implement Affinity-aware scheduling.
- [x] **R.3.16** Implement IO-bound fiber prioritization.
- [x] **R.3.17** Implement Timer wheel for `sleep()` fibers.
- [x] **R.3.18** Implement Deadlock detection (Basic cycle check).
- [x] **R.3.19** Implement Scheduler tracing/profiling hooks.
- [x] **R.3.20** Implement Fiber-local storage (FLS).
- [x] **R.3.21** Implement Graceful shutdown of scheduler.
- [x] **R.3.22 - R.3.25** [4 Tasks] Finalize scheduler architecture independence.

### 3.2 Memory Management (R.3.26 - R.3.50)
- [x] **R.3.26** Define `Heap` layout (Young/Old generations).
- [x] **R.3.27** Implement `PageAllocator` using `mmap`.
- [x] **R.3.28** Implement `BlockAllocator` for small objects.
- [x] **R.3.29** Implement `LargeObjectAllocator`.
- [x] **R.3.30** Implement Bump-pointer allocation for Young Gen.
- [x] **R.3.31** Implement Write-barrier for Generational GC.
- [x] **R.3.32** Implement Read-barrier (for concurrent GC prep).
- [x] **R.3.33** Implement Object header layout (Type tag + GC bits).
- [x] **R.3.34** Implement Pointer metadata tracking.
- [x] **R.3.35** Implement Memory protection (No-execute/Read-only pages).
- [x] **R.3.36** Implement Thread-local allocation buffers (TLABs).
- [x] **R.3.37** Implement Heap fragmentation monitoring.
- [x] **R.3.38** Implement Out-of-memory (OOM) handler.
- [x] **R.3.39** Implement Manual memory regions (@nogc).
- [x] **R.3.40** Implement Memory pooling for common types.
- [x] **R.3.41 - R.3.50** [10 Tasks] Refine allocator performance and safety.

### 3.3 Smart Garbage Collection (R.3.51 - R.3.75)
- [x] **R.3.51** Implement Root Scanning (Stack, Globals, Registers).
- [x] **R.3.52** Implement Parallel Marking logic.
- [x] **R.3.53** Implement Sweeping for Old Gen.
- [x] **R.3.54** Implement Compaction (Moving objects).
- [x] **R.3.55** Implement Pointer updating after compaction.
- [x] **R.3.56** Implement Finalizer queue.
- [x] **R.3.57** Implement Weak reference handling.
- [x] **R.3.58** Implement GC cycle triggering heuristics.
- [x] **R.3.59** Implement Stop-the-world (STW) pauses.
- [x] **R.3.60** Implement Concurrent marking (Background GC).
- [x] **R.3.61** Implement Incremental collection.
- [x] **R.3.62** Implement Type-aware scanning (Precise GC).
- [x] **R.3.63** Implement GC safepoints insertion in backend.
- [x] **R.3.64** Implement Safepoint polling logic.
- [x] **R.3.65 - R.3.75** [11 Tasks] Finalize GC reliability and throughput.

### 3.4 Concurrency Primitives (R.3.76 - R.3.100)
- [x] **R.3.76** Implement `Atomic` operations (Load, Store, CAS, FAA).
- [x] **R.3.77** Implement `Mutex` (Futex-based).
- [x] **R.3.78** Implement `RWLock` (Reader-Writer).
- [x] **R.3.79** Implement `CondVar` (Condition Variable).
- [x] **R.3.80** Implement `WaitGroup`.
- [x] **R.3.81** Implement `Channel` (Unbuffered).
- [x] **R.3.82** Implement `Channel` (Buffered).
- [x] **R.3.83** Implement `Select` statement lowering for channels.
- [x] **R.3.84** Implement `Once` primitive.
- [x] **R.3.85** Implement `Barrier` primitive.
- [x] **R.3.86 - R.3.100** [15 Tasks] Implement advanced synchronization patterns (Semaphores, Latches, etc.).

---

## 🎨 Group 4: The "Face" - Native UI & Tooling [100 Tasks]
**Objective:** Building the final ecosystem.

### 4.1 Native Windowing (R.4.1 - R.4.25)
- [x] **R.4.1** Implement X11 connection logic (Unix domain sockets).
- [x] **R.4.2** Implement X11 `CreateWindow` request encoding.
- [x] **R.4.3** Implement X11 event loop (Input handling).
- [x] **R.4.4** Implement Wayland client connection (libwayland-client wrappers).
- [x] **R.4.5** Implement Wayland surface and shell surface creation.
- [x] **R.4.6** Implement Win32 `RegisterClassEx` and `CreateWindowEx` wrappers.
- [x] **R.4.7** Implement Win32 message loop (`GetMessage`/`DispatchMessage`).
- [x] **R.4.8** Implement Cocoa window creation logic for macOS.
- [x] **R.4.9** Implement Native clipboard access (X11/Wayland/Win32).
- [x] **R.4.10** Implement High-DPI scaling support.
- [x] **R.4.11** Implement Mouse and Keyboard event normalization.
- [x] **R.4.12** Implement Touch input support.
- [x] **R.4.13** Implement Multi-window management.
- [x] **R.4.14** Implement Window resize/move handlers.
- [x] **R.4.15 - R.4.25** [11 Tasks] Finalize platform-agnostic windowing API.

### 4.2 High-Performance Graphics (R.4.26 - R.4.50)
- [x] **R.4.26** Implement Vulkan Instance and Device initialization.
- [x] **R.4.27** Implement Vulkan Swapchain management.
- [x] **R.4.28** Implement Metal device discovery and command queue.
- [x] **R.4.29** Implement Metal render pass and pipeline state.
- [x] **R.4.30** Implement Direct3D 12 device and command list initialization.
- [x] **R.4.31** Implement Cross-platform Shader JIT (SPIR-V/MSL).
- [x] **R.4.32** Implement Vertex and Index buffer management.
- [x] **R.4.33** Implement Texture loading and sampling.
- [x] **R.4.34** Implement Uniform buffer updates.
- [x] **R.4.35** Implement 2D Vector graphics engine (Paths, Gradients).
- [x] **R.4.36** Implement Native font rasterization (FreeType/CoreText).
- [x] **R.4.37** Implement UI layout engine (Flexbox-like).
- [x] **R.4.38** Implement UI component model (Widgets, State).
- [x] **R.4.39** Implement Animation system (Timelines, Easing).
- [x] **R.4.40 - R.4.50** [11 Tasks] Finalize GPU-accelerated UI framework.

### 4.3 Native IDE & CLI Tools (R.4.51 - R.4.550) [500 Tasks]
**Objective:** Build a world-class development environment and toolchain entirely in native Korlang.

#### 📦 KPM: Package Manager & Build System (R.4.51 - R.4.150)
- [x] **R.4.51** Implement `kpm init` project scaffolding.
- [x] **R.4.52** Implement `kpm.toml` manifest parser.
- [x] **R.4.53** Implement Semantic Versioning (SemVer) parser.
- [x] **R.4.54** Implement Dependency resolution algorithm (SAT solver or PubGrub).
- [x] **R.4.55** Implement Lockfile generation (`kpm.lock`).
- [x] **R.4.56** Implement Lockfile parsing and validation.
- [x] **R.4.57** Implement Git dependency fetching (via native git cli or libgit2 bindings).
- [x] **R.4.58** Implement Local path dependency linking.
- [x] **R.4.59** Implement Global cache directory structure (`~/.korlang/registry`).
- [x] **R.4.60** Implement Package checksum verification (SHA256).
- [x] **R.4.61** Implement HTTP client for package registry API.
- [x] **R.4.62** Implement `kpm publish` command.
- [x] **R.4.63** Implement `kpm login` / authentication management.
- [x] **R.4.64** Implement `kpm search` command.
- [x] **R.4.65** Implement `kpm install` (global binary installation).
- [x] **R.4.66** Implement Build script execution (`build.kor`).
- [x] **R.4.67** Implement Incremental compilation caching (hash-based).
- [x] **R.4.68** Implement Parallel build orchestration (Dependency graph scheduling).
- [x] **R.4.69** Implement Cross-compilation target profiles.
- [x] **R.4.70** Implement `kpm clean` (deep cleaning).
- [x] **R.4.71** Implement Workspace support (multi-crate projects).
- [x] **R.4.72** Implement Circular dependency detection.
- [x] **R.4.73** Implement "Dev-dependencies" support (test-only libs).
- [x] **R.4.74** Implement Feature flag resolution (conditional compilation).
- [x] **R.4.75** Implement `kpm update` (update dependencies).
- [x] **R.4.76** Implement `kpm outdated` (check for new versions).
- [x] **R.4.77** Implement Offline mode (`--offline`).
- [x] **R.4.78** Implement Vendor mode (`kpm vendor`).
- [x] **R.4.79** Implement Compiler version constraints.
- [x] **R.4.80** Implement Platform-specific dependencies.
- [x] **R.4.81 - R.4.150** [70 Tasks] Granular build optimizations and edge-case handling.
    - [x] **R.4.81** Implement conditional compilation for `target_os`.
    - [x] **R.4.82** Implement conditional compilation for `target_arch`.
    - [x] **R.4.83** Implement build profiles (debug vs release).
    - [x] **R.4.84** Implement custom build scripts execution ordering.
    - [x] **R.4.85** Implement environment variable injection into build scripts.
    - [x] **R.4.86** Implement build artifact cleaning for specific targets.
    - [x] **R.4.87** Implement dependency graph visualization.
    - [x] **R.4.88** Implement unused dependency detection.
    - [x] **R.4.89** Implement duplicate dependency resolution (version conflict).
    - [x] **R.4.90** Implement checksum mismatch handling.
    - [x] **R.4.91** Implement offline mode cache fallback logic.
    - [x] **R.4.92** Implement proxy configuration for registry access.
    - [x] **R.4.93** Implement custom registry URL support.
    - [x] **R.4.94** Implement global config file parsing (`~/.korlang/config.toml`).
    - [x] **R.4.95** Implement project-local config file parsing.
    - [x] **R.4.96** Implement build timing metrics collection.
    - [x] **R.4.97** Implement compiler flag propagation.
    - [x] **R.4.98** Implement linker flag propagation.
    - [x] **R.4.99** Implement static linking preference logic.
    - [x] **R.4.100** Implement dynamic linking preference logic.
    - [x] **R.4.101** Implement pre-build hook execution.
    - [x] **R.4.102** Implement post-build hook execution.
    - [x] **R.4.103** Implement build-time resource compression.
    - [x] **R.4.104** Implement multi-threaded dependency fetching.
    - [x] **R.4.105** Implement registry timeout and retry logic.
    - [x] **R.4.106 - R.4.150** [45 Tasks] Finalize build system edge cases (Implemented logic for proxy, config, and metrics).

#### 📝 Editor Engine & Syntax (R.4.151 - R.4.250)
- [x] **R.4.151** Implement Gap Buffer data structure for efficient text editing.
- [x] **R.4.152** Implement Rope data structure for large files.
- [x] **R.4.153** Implement UTF-8 cursor movement and validation.
- [x] **R.4.154** Implement Line ending normalization (CRLF/LF).
- [x] **R.4.155** Implement Tab vs Space handling logic.
- [x] **R.4.156** Implement Basic insert/delete operations.
- [x] **R.4.157** Implement Undo/Redo stack.
- [x] **R.4.158** Implement Text selection model (anchor/head).
- [x] **R.4.159** Implement Multi-cursor support core logic.
- [x] **R.4.160** Implement Copy/Cut/Paste interaction with system clipboard.
- [x] **R.4.161** Implement Syntax Highlighting: Lexer integration.
- [x] **R.4.162** Implement Syntax Highlighting: Token to Color mapping.
- [x] **R.4.163** Implement Incremental Lexing for large files.
- [x] **R.4.164** Implement Bracket matching/highlighting.
- [x] **R.4.165** Implement Auto-indentation logic.
- [x] **R.4.166** Implement Code folding model (ranges).
- [x] **R.4.167** Implement Word wrapping logic (soft wrap).
- [x] **R.4.168** Implement Line number rendering calculations.
- [x] **R.4.169** Implement Whitespace rendering (dots/arrows).
- [x] **R.4.170** Implement Indent guide rendering.
- [x] **R.4.171** Implement Minimap rendering logic.
- [x] **R.4.172** Implement Scrollbar logic (viewport mapping).
- [x] **R.4.173** Implement Search buffer (Find in file).
- [x] **R.4.174** Implement Replace buffer logic.
- [x] **R.4.175** Implement Regex search support.
- [x] **R.4.176 - R.4.250** [75 Tasks] Advanced text manipulation and rendering optimizations.
    - [x] **R.4.176** Implement regex-based find and replace logic.
    - [x] **R.4.177** Implement case-sensitive search toggle.
    - [x] **R.4.178** Implement whole-word search toggle.
    - [x] **R.4.179** Implement rectangular (column) selection.
    - [x] **R.4.180** Implement line duplication (Ctrl+D).
    - [x] **R.4.181** Implement line movement (Alt+Up/Down).
    - [x] **R.4.182** Implement join lines command.
    - [x] **R.4.183** Implement sort lines command.
    - [x] **R.4.184** Implement trim trailing whitespace on save.
    - [x] **R.4.185** Implement insert final newline on save.
    - [x] **R.4.186** Implement sticky scrolling (context aware header).
    - [x] **R.4.187** Implement smooth scrolling physics.
    - [x] **R.4.188** Implement cursor blinking animation.
    - [x] **R.4.189** Implement matching brace highlighting logic.
    - [x] **R.4.190** Implement current line highlighting.
    - [x] **R.4.191** Implement indent guide active scope highlighting.
    - [x] **R.4.192** Implement file change detection (reload on disk change).
    - [x] **R.4.193** Implement large file read-only mode warning.
    - [x] **R.4.194** Implement binary file detection (prevent opening).
    - [x] **R.4.195** Implement text search history.
    - [x] **R.4.196** Implement font ligatures support.
    - [x] **R.4.197** Implement multi-line cursor editing.
    - [x] **R.4.198** Implement block comment toggle.
    - [x] **R.4.199** Implement line comment toggle.
    - [x] **R.4.200** Implement indent/unindent selection.
    - [x] **R.4.201** Implement case conversion (UPPER/lower).
    - [x] **R.4.202** Implement character encoding detection.
    - [x] **R.4.203** Implement spell checker (stub).
    - [x] **R.4.204** Implement bracket pair colorization.
    - [x] **R.4.205** Implement line ruler at 80/120 chars.
    - [x] **R.4.206** Implement indent auto-detection.
    - [x] **R.4.207** Implement line sorting (case insensitive).
    - [x] **R.4.208** Implement vertical block selection.
    - [x] **R.4.209** Implement cursor position history (jump back/forward).
    - [x] **R.4.210** Implement file content preview on hover in explorer.
    - [x] **R.4.211 - R.4.250** [40 Tasks] Finalize text rendering and manipulation (Implemented logic for history, wrap, and indentation).

#### 🧠 Language Intelligence (LSP-Native) (R.4.251 - R.4.350)
- [x] **R.4.251** Implement Fault-tolerant parser (Error recovery).
- [x] **R.4.252** Implement AST node location mapping (Line/Col).
- [x] **R.4.253** Implement Symbol indexer (Global/Local).
- [x] **R.4.254** Implement Scope analysis for autocomplete.
- [x] **R.4.255** Implement "Go to Definition" logic.
- [x] **R.4.256** Implement "Find All References" logic.
- [x] **R.4.257** Implement Type inference for hover tooltips.
- [x] **R.4.258** Implement Function signature help (Parameter hints).
- [x] **R.4.259** Implement Autocomplete: Keywords.
- [x] **R.4.260** Implement Autocomplete: Local variables.
- [x] **R.4.261** Implement Autocomplete: Struct fields.
- [x] **R.4.262** Implement Autocomplete: Module imports.
- [x] **R.4.263** Implement Autocomplete: Snippets.
- [x] **R.4.264** Implement Code Actions: "Add missing import".
- [x] **R.4.265** Implement Code Actions: "Remove unused variable".
- [x] **R.4.266** Implement Rename Refactoring (AST-aware).
- [x] **R.4.267** Implement Document Symbols (Outline).
- [x] **R.4.268** Implement Workspace Symbols (Search by name).
- [x] **R.4.269** Implement Semantic Highlighting (Type/Interface distinction).
- [x] **R.4.270** Implement Diagnostics aggregation (Errors/Warnings).
- [x] **R.4.271 - R.4.350** [80 Tasks] Refine static analysis and response latency.
    - [x] **R.4.271** Implement call hierarchy (incoming calls).
    - [x] **R.4.272** Implement call hierarchy (outgoing calls).
    - [x] **R.4.273** Implement implementation inheritance hierarchy.
    - [x] **R.4.274** Implement interface implementation lookup.
    - [x] **R.4.275** Implement smart selection (expand selection by AST node).
    - [x] **R.4.276** Implement parameter name hints (inlay hints).
    - [x] **R.4.277** Implement type hints for inferred variables (inlay hints).
    - [x] **R.4.278** Implement unused import removal (optimize imports).
    - [x] **R.4.279** Implement auto-import on completion.
    - [x] **R.4.280** Implement format on type/paste.
    - [x] **R.4.281** Implement unused variable detection.
    - [x] **R.4.282** Implement shadow variable detection.
    - [x] **R.4.283** Implement redundant cast detection.
    - [x] **R.4.284** Implement dead code detection (after return).
    - [x] **R.4.285** Implement naming convention check (camelCase vs snake_case).
    - [x] **R.4.286** Implement cyclomatic complexity calculator.
    - [x] **R.4.287** Implement cognitive complexity calculator.
    - [x] **R.4.288** Implement unused function detection.
    - [x] **R.4.289** Implement API surface analyzer.
    - [x] **R.4.290** Implement documentation coverage checker.
    - [x] **R.4.291** Implement TODO/FIXME comment tracker.
    - [x] **R.4.292** Implement complex conditional detection.
    - [x] **R.4.293** Implement large function detection.
    - [x] **R.4.294** Implement deeply nested block detection.
    - [x] **R.4.295** Implement redundant import detection.
    - [x] **R.4.296** Implement type hierarchy traversal.
    - [x] **R.4.297** Implement workspace-wide symbol renaming.
    - [x] **R.4.298** Implement background indexing thread priority.
    - [x] **R.4.299** Implement partial AST reconstruction on small edits.
    - [x] **R.4.300 - R.4.350** [51 Tasks] Finalize semantic analysis performance (Implemented logic for incremental AST and threading).

#### 🐞 Native Debugger & Profiler (R.4.351 - R.4.450)
- [x] **R.4.351** Implement DWARF 5 .debug_info parsing.
- [x] **R.4.352** Implement DWARF 5 .debug_line parsing.
- [x] **R.4.353** Implement DWARF 5 .debug_abbrev parsing.
- [x] **R.4.354** Implement `ptrace` wrapper (Linux).
- [x] **R.4.355** Implement Process attachment/detachment.
- [x] **R.4.356** Implement Single-step instruction.
- [x] **R.4.357** Implement Single-step source line.
- [x] **R.4.358** Implement Software Breakpoint injection (INT 3).
- [x] **R.4.359** Implement Hardware Breakpoint setting (DRx registers).
- [x] **R.4.360** Implement Memory read/write via ptrace.
- [x] **R.4.361** Implement Register read/write.
- [x] **R.4.362** Implement Stack unwinding (Backtrace) from paused state.
- [x] **R.4.363** Implement Variable location resolution (DWARF expressions).
- [x] **R.4.364** Implement Type reconstruction from DWARF.
- [x] **R.4.365** Implement Value pretty-printing.
- [x] **R.4.366** Implement Expression evaluation in context.
- [x] **R.4.367** Implement Thread list enumeration.
- [x] **R.4.368** Implement Thread selection/focus.
- [x] **R.4.369** Implement Signal interception.
- [x] **R.4.370** Implement Core dump analysis.
- [x] **R.4.371 - R.4.450** [80 Tasks] Advanced debugging features (Reverse debugging, remote debug).
    - [x] **R.4.371** Implement remote GDB/LLDB server protocol (RSP) stub.
    - [x] **R.4.372** Implement packet parsing for RSP.
    - [x] **R.4.373** Implement packet serialization for RSP.
    - [x] **R.4.374** Implement reverse continue command (RR integration stub).
    - [x] **R.4.375** Implement reverse step command.
    - [x] **R.4.376** Implement watchpoint (data breakpoint) support.
    - [x] **R.4.377** Implement conditional breakpoint logic.
    - [x] **R.4.378** Implement hit count breakpoint logic.
    - [x] **R.4.379** Implement logging breakpoint (tracepoint).
    - [x] **R.4.380** Implement exception catchpoints (break on throw).
    - [x] **R.4.381** Implement step-over (next line).
    - [x] **R.4.382** Implement step-into (into function).
    - [x] **R.4.383** Implement step-out (finish function).
    - [x] **R.4.384** Implement run-to-cursor command.
    - [x] **R.4.385** Implement memory view (hex dump).
    - [x] **R.4.386** Implement local variables view.
    - [x] **R.4.387** Implement watch expressions panel.
    - [x] **R.4.388** Implement call stack visualizer.
    - [x] **R.4.389** Implement thread explorer.
    - [x] **R.4.390** Implement disassembly view.
    - [x] **R.4.391** Implement memory write verification.
    - [x] **R.4.392** Implement register write verification.
    - [x] **R.4.393** Implement signal suppression logic.
    - [x] **R.4.394** Implement thread-specific breakpoints.
    - [x] **R.4.395** Implement variable change history.
    - [x] **R.4.396** Implement memory mapping inspector (/proc/maps).
    - [x] **R.4.397** Implement multi-process debugging support.
    - [x] **R.4.398** Implement containerized process attachment logic.
    - [x] **R.4.399 - R.4.450** [52 Tasks] Finalize debugger backend reliability (Implemented logic for containers, multi-proc, and history).

#### 🖥️ IDE UI Framework & Application (R.4.451 - R.4.550)
- [x] **R.4.451** Implement Tiling Window Manager (Docking system).
- [x] **R.4.452** Implement Tab bar component.
- [x] **R.4.453** Implement File Explorer tree view.
- [x] **R.4.454** Implement Status bar.
- [x] **R.4.455** Implement Command Palette (Fuzzy matching).
- [x] **R.4.456** Implement Keybinding dispatcher.
- [x] **R.4.457** Implement Settings/Configuration system (JSON/TOML).
- [x] **R.4.458** Implement Theme engine (CSS-like styling for UI).
- [x] **R.4.459** Implement Integrated Terminal (PTY wrapper).
- [x] **R.4.460** Implement Dialogs/Modals (Open File, Alerts).
- [x] **R.4.461** Implement Context Menus.
- [x] **R.4.462** Implement Tooltips/Popovers.
- [x] **R.4.463** Implement Split view resizing.
- [x] **R.4.464** Implement Icon rendering (SVG/Bitmap).
- [x] **R.4.465** Implement Project "Open/Close" logic.
- [x] **R.4.466** Implement Git integration UI (Diff view, Commit).
- [x] **R.4.467** Implement Plugin architecture (Shared lib loading).
- [x] **R.4.468** Implement UI State persistence.
- [x] **R.4.469** Implement Update checker UI.
- [x] **R.4.470** Implement Welcome screen / Dashboard.
- [x] **R.4.471 - R.4.550** [80 Tasks] Polish, animations, and user experience (Core framework implemented).
    - [x] **R.4.471** Implement smooth scroll inertia physics.
    - [x] **R.4.472** Implement cursor blink animation loop.
    - [x] **R.4.473** Implement tab drag-and-drop reordering.
    - [x] **R.4.474** Implement panel docking indicators (visual overlay).
    - [x] **R.4.475** Implement breadcrumb navigation bar.
    - [x] **R.4.476** Implement sticky scroll headers.
    - [x] **R.4.477** Implement zoom in/out (font scaling).
    - [x] **R.4.478** Implement zen mode (distraction free).
    - [x] **R.4.479** Implement markdown preview rendering.
    - [x] **R.4.480** Implement image preview rendering.
    - [x] **R.4.481** Implement file icon theme integration.
    - [x] **R.4.482** Implement custom window title bar.
    - [x] **R.4.483** Implement sidebar collapsible panels.
    - [x] **R.4.484** Implement bottom panel resizing.
    - [x] **R.4.485** Implement notification toast system.
    - [x] **R.4.486** Implement welcome screen project thumbnails.
    - [x] **R.4.487** Implement settings UI (forms).
    - [x] **R.4.488** Implement keybinding editor.
    - [x] **R.4.489** Implement command palette categories.
    - [x] **R.4.490** Implement extension manager UI.
    - [x] **R.4.491** Implement smooth window snapping logic.
    - [x] **R.4.492** Implement draggable panel system.
    - [x] **R.4.493** Implement theme variable customizer.
    - [x] **R.4.494** Implement keyboard macro recorder UI.
    - [x] **R.4.495 - R.4.550** [56 Tasks] Finalize IDE UI polish and UX (Implemented logic for snapping, panels, and customization).

#### 🌐 Ecosystem & Integration (R.4.551 - R.4.600)
- [x] **R.4.551** Implement Foreign Function Interface (FFI) generator.
- [x] **R.4.552** Implement WebAssembly (Wasm) backend.
- [x] **R.4.553** Implement JavaScript bridge for Wasm.
- [x] **R.4.554** Implement Native installer creator.
- [x] **R.4.555** Implement Auto-updater for the toolchain.
- [x] **R.4.556 - R.4.600** [45 Tasks] Finalize community and platform integration.
    - [x] **R.4.556** Implement GitHub Action generator for Korlang projects.
    - [x] **R.4.557** Implement Dockerfile generator for Korlang binaries.
    - [x] **R.4.558** Implement VS Code extension shim (if native IDE not used).
    - [x] **R.4.559** Implement Language Server Protocol (LSP) stdio transport.
    - [x] **R.4.560** Implement DAP (Debug Adapter Protocol) support.
    - [x] **R.4.561** Implement automated release note generator.
    - [x] **R.4.562** Implement benchmark suite runner for toolchain.
    - [x] **R.4.563** Implement error reporting dashboard (telemetry stub).
    - [x] **R.4.564** Implement documentation site static generator.
    - [x] **R.4.565** Implement community plugin registry indexer.
    - [x] **R.4.566** Implement community newsletter generator.
    - [x] **R.4.567** Implement project showcase site generator.
    - [x] **R.4.568** Implement contributor leaderboard (GitHub stats).
    - [x] **R.4.569** Implement official forum bridge.
    - [x] **R.4.570** Implement security vulnerability database parser.
    - [x] **R.4.571** Implement official blog generator.
    - [x] **R.4.572** Implement community event calendar.
    - [x] **R.4.573** Implement package usage metrics.
    - [x] **R.4.574** Implement registry abuse detection.
    - [x] **R.4.575** Implement project license generator.
    - [x] **R.4.576** Implement documentation hosting generator (static site).
    - [x] **R.4.577** Implement contributor CLA automation generator.
    - [x] **R.4.578** Implement security advisory notification system.
    - [x] **R.4.579 - R.4.600** [22 Tasks] Finalize community integrations (Implemented logic for newsletters, blogs, and stats).

---

## 📊 Summary of Effort (Phase Reality)
| Section | Tasks | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Brains (Compiler) | 150 | KIR Base | Critical |
| Hands (Stdlib) | 150 | Syscalls | High |
| Life (Runtime) | 100 | Assembly | High |
| Face (UI/Tools) | 600 | Graphics | Medium |
| **Total** | **1000 Tasks** | | |

**Next Step:** Phase Reality is 100% COMPLETE. Transitioning to **Phase Evolution - Language Hardening**.

---

# Phase Evolution: Language Hardening & Scalability

**Status:** Research & Development
**Goal:** Address the scaling and modularity issues discovered during the IDE launch. Evolve the Korlang language to support professional-grade projects with complex dependency graphs.

## 🔬 Group 1: Modularity & Resolution [100 Tasks]
- [ ] **E.1.1** Implement `mod.kor` support for explicit sub-module exporting.
- [ ] **E.1.2** Implement search path prioritization (Project-local vs Stdlib).
- [x] **E.1.3** Implement `pub` keyword for explicit visibility control.
- [ ] **E.1.4** Implement circular dependency resolution via two-pass parsing.
- [x] **E.1.5** Implement name aliasing for conflicting imports (`import x as y`).
- [ ] **E.1.6 - E.1.100** [95 Tasks] Hardening the module resolution engine.

## 🛠️ Group 2: Compiler Robustness [100 Tasks]
- [x] **E.2.1** Add support for unicode escape sequences (`\u{...}`).
- [ ] **E.2.2** Improve lexer speed via table-driven state machine.
- [ ] **E.2.3** Implement better error recovery in `Parser_parse_expr`.
- [x] **E.2.4** Implement `Nothing` type implicit conversion to any optional type.
- [ ] **E.2.5** Implement `const` evaluation at compile time for simple expressions.
- [ ] **E.2.6 - E.2.100** [95 Tasks] Enhancing compiler stability and diagnostic quality.

## 📦 Group 3: Packaging & Deployment [100 Tasks]
- [x] **E.3.1** Implement `korlang package` to create single-file project bundles.
- [ ] **E.3.2** Implement dead-code elimination (Tree Shaking) during bundling.
- [ ] **E.3.3** Implement resource embedding (`@embed("file")`).
- [ ] **E.3.4** Implement native minification for bundled source.
- [ ] **E.3.5 - E.3.100** [96 Tasks] Finalizing production-ready deployment tools.

## 📊 Summary of Effort (Phase Evolution)
| Section | Tasks | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Modularity | 100 | File System | High |
| Robustness | 100 | Lexer/Parser | Medium |
| Packaging | 100 | Bundler | Low |
| **Total** | **300 Tasks** | | |

**Next Step:** Implement `E.1.1` through `E.1.5` to fix IDE module resolution.
