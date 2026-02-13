# Current Phase: Phase Omega - Total Self-Sufficiency & Ecosystem Singularity

**Status:** The Final Hardening & Massive Verification
**Goal:** Achieve absolute independence. This phase serves as the ultimate quality gate, verifying that phases 1-19 are not just "implemented" but are **self-sufficient**, **interdependent**, and **production-ready**. We validate that Korlang can build, run, debug, render, and deploy itself without relying on Rust, C++, or Python runtimes.

This phase contains **500+ granular tasks** to ensure no stone is left unturned.

---

## 🏛️ Group 1: Foundation & Bootstrap Removal (The Core) [Tasks 1-100]
**Objective:** Verify that the compiler and runtime are completely native and self-hosting.

- ### 1.1 Lexer Independence
- [x] **O.1.1** Verify native `Lexer` struct layout matches Rust equivalent.
- [x] **O.1.2** Verify `next_char()` reads from native UTF-8 string buffer.
- [x] **O.1.3** Verify `peek()` works without bounds check panics.
- [x] **O.1.4** Verify `skip_whitespace()` handles spaces, tabs, newlines correctly.
- [x] **O.1.5** Verify `skip_comment()` handles `//` single-line comments.
- [x] **O.1.6** Verify `skip_comment()` handles `/* */` multi-line comments.
- [x] **O.1.7** Verify nested multi-line comments `/* /* */ */`.
- [x] **O.1.8** Verify identifier parsing `[a-zA-Z_][a-zA-Z0-9_]*`.
- [x] **O.1.9** Verify integer literal parsing (decimal).
- [x] **O.1.10** Verify integer literal parsing (hex `0x`).
- [x] **O.1.11** Verify integer literal parsing (binary `0b`).
- [x] **O.1.12** Verify float literal parsing (standard `0.0`).
- [x] **O.1.13** Verify float literal parsing (scientific `1e10`).
- [x] **O.1.14** Verify string literal parsing (basic `""`).
- [x] **O.1.15** Verify string escape sequences `\n`, `\t`, `\r`, `\\`, `\"`.
- [x] **O.1.16** Verify unicode escape sequences `\u{XXXX}`.
- [x] **O.1.17** Verify string interpolation `"{var}"` lexing logic.
- [x] **O.1.18** Verify raw string literals (if supported).
- [x] **O.1.19** Verify char literal parsing `'a'`.
- [x] **O.1.20** Verify char escape parsing `'\n'`.
- [x] **O.1.21** Verify keyword recognition: `fun`.
- [x] **O.1.22** Verify keyword recognition: `let`, `var`.
- [x] **O.1.23** Verify keyword recognition: `if`, `else`.
- [x] **O.1.24** Verify keyword recognition: `while`, `for`.
- [x] **O.1.25** Verify keyword recognition: `match`, `return`.
- [x] **O.1.26** Verify keyword recognition: `break`, `continue`.
- [x] **O.1.27** Verify keyword recognition: `struct`, `enum`.
- [x] **O.1.28** Verify keyword recognition: `class`, `interface`.
- [x] **O.1.29** Verify keyword recognition: `import`, `module`.
- [x] **O.1.30** Verify keyword recognition: `view`, `resource`, `gpu`.
- [x] **O.1.31** Verify operator parsing: `+`, `-`, `*`, `/`, `%`.
- [x] **O.1.32** Verify operator parsing: `==`, `!=`, `<`, `<=`, `>`, `>=`.
- [x] **O.1.33** Verify operator parsing: `&&`, `||`, `!`.
- [x] **O.1.34** Verify operator parsing: `&`, `|`, `^`, `<<`, `>>`.
- [x] **O.1.35** Verify operator parsing: `->` (arrow).
- [x] **O.1.36** Verify operator parsing: `|>` (pipe).
- [x] **O.1.37** Verify operator parsing: `?:` (elvis).
- [x] **O.1.38** Verify delimiter parsing: `(`, `)`, `{`, `}`, `[`, `]`.
- [x] **O.1.39** Verify delimiter parsing: `,`, `;`, `:`, `.`.
- [x] **O.1.40** Verify EOF handling.

*Verification:* `scripts/verify_group1.sh` now covers O.1.1–O.1.40 (including the keywords/operators/delimiters/EOF checks) and passed after running the Group 1 bundle.

### 1.2 Parser Independence
- [ ] **O.1.41** Verify `Parser` struct initialization.
- [ ] **O.1.42** Verify Pratt parser precedence table logic.
- [ ] **O.1.43** Parse `Program` root node.
- [ ] **O.1.44** Parse `Item.Fun` declaration.
- [ ] **O.1.45** Parse function parameters.
- [ ] **O.1.46** Parse return type annotations.
- [ ] **O.1.47** Parse `Item.Struct` declaration.
- [ ] **O.1.48** Parse struct fields.
- [ ] **O.1.49** Parse `Item.Enum` declaration.
- [ ] **O.1.50** Parse enum variants.
- [ ] **O.1.51** Parse `Item.Impl` block (methods).
- [ ] **O.1.52** Parse `Item.View` definition.
- [ ] **O.1.53** Parse `Item.Resource` definition.
- [ ] **O.1.54** Parse `Stmt.Let` declaration.
- [ ] **O.1.55** Parse `Stmt.Var` declaration.
- [ ] **O.1.56** Parse `Stmt.Expr` statement.
- [ ] **O.1.57** Parse `Stmt.Return` statement.
- [ ] **O.1.58** Parse `Stmt.If` (with else).
- [ ] **O.1.59** Parse `Stmt.While` loop.
- [ ] **O.1.60** Parse `Stmt.For` loop.
- [ ] **O.1.61** Parse `Stmt.Match` expression.
- [ ] **O.1.62** Parse match arms and patterns.
- [ ] **O.1.63** Parse `Expr.Binary` (math).
- [ ] **O.1.64** Parse `Expr.Binary` (logic).
- [ ] **O.1.65** Parse `Expr.Unary` (negation/not).
- [ ] **O.1.66** Parse `Expr.Call` (function invocation).
- [ ] **O.1.67** Parse `Expr.MethodCall` (dot notation).
- [ ] **O.1.68** Parse `Expr.FieldAccess`.
- [ ] **O.1.69** Parse `Expr.Index` (array access).
- [ ] **O.1.70** Parse `Expr.Literal`.
- [ ] **O.1.71** Parse `Expr.Array` literal `[1, 2]`.
- [ ] **O.1.72** Parse `Expr.Map` literal `{k: v}`.
- [ ] **O.1.73** Parse `Expr.Lambda` / Closure syntax.
- [ ] **O.1.74** Parse `Expr.Block` scopes.
- [ ] **O.1.75** Parse `Type.Path` (User types).
- [ ] **O.1.76** Parse `Type.Array` `[T]`.
- [ ] **O.1.77** Parse `Type.Map` `{K: V}`.
- [ ] **O.1.78** Parse `Type.Func` `(A) -> B`.
- [ ] **O.1.79** Parse `Type.Tuple` `(A, B)`.
- [ ] **O.1.80** Verify error recovery strategy (panic mode).

### 1.3 Compiler Self-Sufficiency
- [ ] **O.1.81** Verify `SymbolTable` creation in Korlang.
- [ ] **O.1.82** Verify `Scope` pushing/popping logic.
- [ ] **O.1.83** Verify `define_symbol` checks for redefinition.
- [ ] **O.1.84** Verify `resolve_symbol` walks up scope tree.
- [ ] **O.1.85** Verify Type Inference (Hindley-Milner) implementation.
- [ ] **O.1.86** Verify Unification algorithm for generic types.
- [ ] **O.1.87** Verify `@nogc` safety checks logic.
- [ ] **O.1.88** Verify `KIR` lowering: Function to IR.
- [ ] **O.1.89** Verify `KIR` lowering: Block to BasicBlock.
- [ ] **O.1.90** Verify `KIR` lowering: If/Else to Branch.
- [ ] **O.1.91** Verify `KIR` lowering: While to Loop/Phi.
- [ ] **O.1.92** Verify `KIR` register allocation (Linear Scan).
- [ ] **O.1.93** Verify x86_64 Instruction Encoding implementation.
- [ ] **O.1.94** Verify AArch64 Instruction Encoding implementation.
- [ ] **O.1.95** Verify ELF Object Writer logic.
- [ ] **O.1.96** Verify Mach-O Object Writer logic.
- [ ] **O.1.97** Verify PE Object Writer logic.
- [ ] **O.1.98** Verify Linker symbol resolution logic.
- [ ] **O.1.99** Verify final binary layout generation.
- [ ] **O.1.100** Verify `korlang build` command end-to-end self-build.

---

## 📦 Group 2: The Core Standard Library (Data & I/O) [Tasks 101-200]
**Objective:** Ensure all standard library features work via direct syscalls.

### 2.1 Core Types
- [ ] **O.2.1** `String.len()`
- [ ] **O.2.2** `String.get(index)`
- [ ] **O.2.3** `String.slice(start, end)`
- [ ] **O.2.4** `String.concat(other)`
- [ ] **O.2.5** `String.split(delimiter)`
- [ ] **O.2.6** `String.trim()`
- [ ] **O.2.7** `String.replace()`
- [ ] **O.2.8** `String.to_int()`
- [ ] **O.2.9** `String.to_float()`
- [ ] **O.2.10** `List.push()`
- [ ] **O.2.11** `List.pop()`
- [ ] **O.2.12** `List.len()`
- [ ] **O.2.13** `List.get()`
- [ ] **O.2.14** `List.set()`
- [ ] **O.2.15** `List.insert()`
- [ ] **O.2.16** `List.remove()`
- [ ] **O.2.17** `List.sort()` (QuickSort/MergeSort implementation).
- [ ] **O.2.18** `List.map()`
- [ ] **O.2.19** `List.filter()`
- [ ] **O.2.20** `List.reduce()`
- [ ] **O.2.21** `Map.put()` (Hashing logic).
- [ ] **O.2.22** `Map.get()`
- [ ] **O.2.23** `Map.remove()`
- [ ] **O.2.24** `Map.keys()`
- [ ] **O.2.25** `Map.values()`
- [ ] **O.2.26** `Result.ok()`
- [ ] **O.2.27** `Result.err()`
- [ ] **O.2.28** `Result.unwrap()`
- [ ] **O.2.29** `Option.some()`
- [ ] **O.2.30** `Option.none()`

### 2.2 System I/O
- [ ] **O.2.31** `File.open(path, mode)` (Linux Syscall).
- [ ] **O.2.32** `File.open(path, mode)` (macOS Syscall).
- [ ] **O.2.33** `File.open(path, mode)` (Windows Syscall).
- [ ] **O.2.34** `File.read(buffer)`
- [ ] **O.2.35** `File.write(buffer)`
- [ ] **O.2.36** `File.seek(offset)`
- [ ] **O.2.37** `File.close()`
- [ ] **O.2.38** `File.metadata()` (Size, permissions).
- [ ] **O.2.39** `Dir.read()` (List directory).
- [ ] **O.2.40** `Dir.create()`
- [ ] **O.2.41** `Dir.remove()`
- [ ] **O.2.42** `Path.join()`
- [ ] **O.2.43** `Path.exists()`
- [ ] **O.2.44** `Env.args()` (Command line args parsing).
- [ ] **O.2.45** `Env.vars()` (Environment variables).
- [ ] **O.2.46** `Process.spawn()` (Fork/Exec).
- [ ] **O.2.47** `Process.wait()`
- [ ] **O.2.48** `Process.kill()`
- [ ] **O.2.49** `Time.now()` (Realtime clock).
- [ ] **O.2.50** `Time.sleep()` (Nano-sleep).

### 2.3 Networking (Native)
- [ ] **O.2.51** `Socket.create(tcp)`
- [ ] **O.2.52** `Socket.bind(addr)`
- [ ] **O.2.53** `Socket.listen()`
- [ ] **O.2.54** `Socket.accept()`
- [ ] **O.2.55** `Socket.connect()`
- [ ] **O.2.56** `Socket.send()`
- [ ] **O.2.57** `Socket.recv()`
- [ ] **O.2.58** `Socket.close()`
- [ ] **O.2.59** `Socket.setsockopt()`
- [ ] **O.2.60** `Dns.resolve()` (UDP query construction).
- [ ] **O.2.61** `Http.parse_request()`
- [ ] **O.2.62** `Http.serialize_response()`
- [ ] **O.2.63** `Tls.handshake()` (Client).
- [ ] **O.2.64** `Tls.handshake()` (Server).
- [ ] **O.2.65** `Tls.encrypt_record()`
- [ ] **O.2.66** `Tls.decrypt_record()`
- [ ] **O.2.67** `Ws.handshake()`
- [ ] **O.2.68** `Ws.frame_encode()`
- [ ] **O.2.69** `Ws.frame_decode()`
- [ ] **O.2.70** `Ssl.certificate_verify()`

### 2.4 Crypto (Native)
- [ ] **O.2.71** `Sha256.init()`
- [ ] **O.2.72** `Sha256.update()`
- [ ] **O.2.73** `Sha256.final()`
- [ ] **O.2.74** `Aes.encrypt_block()`
- [ ] **O.2.75** `Aes.decrypt_block()`
- [ ] **O.2.76** `ChaCha20.encrypt()`
- [ ] **O.2.77** `Ed25519.sign()`
- [ ] **O.2.78** `Ed25519.verify()`
- [ ] **O.2.79** `Rsa.encrypt()` (BigInt math).
- [ ] **O.2.80** `Rsa.decrypt()`
- [ ] **O.2.81** `Random.bytes()` (OS entropy source).
- [ ] **O.2.82** `Base64.encode()`
- [ ] **O.2.83** `Base64.decode()`
- [ ] **O.2.84** `Hex.encode()`
- [ ] **O.2.85** `Hex.decode()`

### 2.5 Allocator & Runtime
- [ ] **O.2.86** `Alloc.malloc()` (mmap based).
- [ ] **O.2.87** `Alloc.free()`
- [ ] **O.2.88** `Alloc.realloc()`
- [ ] **O.2.89** `Gc.collect()` (Trigger GC).
- [ ] **O.2.90** `Gc.stat()` (Memory usage).
- [ ] **O.2.91** `Thread.spawn()` (Native thread).
- [ ] **O.2.92** `Thread.join()`
- [ ] **O.2.93** `Mutex.lock()` (Futex based).
- [ ] **O.2.94** `Mutex.unlock()`
- [ ] **O.2.95** `CondVar.wait()`
- [ ] **O.2.96** `CondVar.notify()`
- [ ] **O.2.97** `Panic.handler()` (Stack trace generation).
- [ ] **O.2.98** `Signal.handle()` (SIGINT/SIGTERM).
- [ ] **O.2.99** `DynLib.load()` (dlopen).
- [ ] **O.2.100** `DynLib.symbol()` (dlsym).

---

## 🧵 Group 3: Concurrency & Runtime Reliability [Tasks 201-300]
**Objective:** Prove the scheduler and memory model under extreme load.

### 3.1 Scheduler Core
- [ ] **O.3.1** M:N Scheduler initialization.
- [ ] **O.3.2** Worker thread creation (per core).
- [ ] **O.3.3** Global task queue implementation.
- [ ] **O.3.4** Local task queue implementation (Deque).
- [ ] **O.3.5** Work stealing algorithm (victim selection).
- [ ] **O.3.6** Task spawning (Fiber creation).
- [ ] **O.3.7** Context switch logic (Save registers).
- [ ] **O.3.8** Context switch logic (Restore registers).
- [ ] **O.3.9** Stack swapping logic.
- [ ] **O.3.10** Fiber state management (Ready, Running, Blocked).
- [ ] **O.3.11** Scheduler parking (Sleep when idle).
- [ ] **O.3.12** Scheduler waking (Wake on IO/Task).
- [ ] **O.3.13** Task priority handling.
- [ ] **O.3.14** Preemption timer implementation.
- [ ] **O.3.15** Yield implementation.

### 3.2 Synchronization
- [ ] **O.3.16** Atomic Load (relaxed/acquire/seq_cst).
- [ ] **O.3.17** Atomic Store.
- [ ] **O.3.18** Atomic CAS (Compare-And-Swap).
- [ ] **O.3.19** Atomic FAA (Fetch-And-Add).
- [ ] **O.3.20** Spinlock implementation.
- [ ] **O.3.21** Semaphore implementation.
- [ ] **O.3.22** Barrier implementation.
- [ ] **O.3.23** RWLock (Read-Write Lock).
- [ ] **O.3.24** Once (Initialization).
- [ ] **O.3.25** Channel (SPSC) creation.
- [ ] **O.3.26** Channel send (non-blocking).
- [ ] **O.3.27** Channel recv (blocking).
- [ ] **O.3.28** Channel (MPMC) implementation.
- [ ] **O.3.29** Channel close logic.
- [ ] **O.3.30** Select over multiple channels.

### 3.3 Memory Management
- [ ] **O.3.31** Heap Region creation (1MB blocks).
- [ ] **O.3.32** Bump allocation in nursery.
- [ ] **O.3.33** Write barrier implementation.
- [ ] **O.3.34** Root scanning (Stack walking).
- [ ] **O.3.35** Global root scanning.
- [ ] **O.3.36** Mark phase implementation.
- [ ] **O.3.37** Sweep phase implementation.
- [ ] **O.3.38** Object relocation (Compaction).
- [ ] **O.3.39** Pointer updating after relocation.
- [ ] **O.3.40** Finalizer registry.
- [ ] **O.3.41** Finalizer execution thread.
- [ ] **O.3.42** Large object allocator (OS direct).
- [ ] **O.3.43** ARC (Atomic Reference Counting) logic.
- [ ] **O.3.44** ARC increment.
- [ ] **O.3.45** ARC decrement & free.
- [ ] **O.3.46** Weak reference implementation.
- [ ] **O.3.47** Thread-Local Storage (TLS) allocator.
- [ ] **O.3.48** Out-of-memory handling.
- [ ] **O.3.49** Memory leak detection mode.
- [ ] **O.3.50** Heap dump generation.

### 3.4 Stress Testing
- [ ] **O.3.51** 10k Task spawn test.
- [ ] **O.3.52** 100k Task spawn test.
- [ ] **O.3.53** Deep recursion test (Stack growth).
- [ ] **O.3.54** Ping-pong message test (Channels).
- [ ] **O.3.55** Chain reaction test (Task spawning tasks).
- [ ] **O.3.56** Mutex contention test (High load).
- [ ] **O.3.57** RWLock reader starvation test.
- [ ] **O.3.58** GC churn test (Rapid alloc/free).
- [ ] **O.3.59** GC long-lived object test.
- [ ] **O.3.60** Mixed workload test (CPU + IO).

### 3.5 Runtime Hooks
- [ ] **O.3.61** Uncaught exception handler.
- [ ] **O.3.62** Signal handler integration.
- [ ] **O.3.63** Process exit hooks (`at_exit`).
- [ ] **O.3.64** Debugger attachment hook.
- [ ] **O.3.65** Profiler sampling hook.
- [ ] **O.3.66** Trace logging hook.
- [ ] **O.3.67** Hot-reload entry point (future).
- [ ] **O.3.68** Stack trace formatter.
- [ ] **O.3.69** Symbol demangler.
- [ ] **O.3.70** CPU Id feature detection.

---

## 🎨 Group 4: Native UI & Graphics Engine [Tasks 301-400]
**Objective:** Confirm the UI stack renders pixels without external windowing libraries.

### 4.1 Native Windowing
- [ ] **O.4.1** X11 Connection (Linux).
- [ ] **O.4.2** X11 Window Creation.
- [ ] **O.4.3** X11 Event Loop.
- [ ] **O.4.4** Wayland Connection (Linux).
- [ ] **O.4.5** Wayland Surface Creation.
- [ ] **O.4.6** Wayland Event Loop.
- [ ] **O.4.7** Win32 Class Registration.
- [ ] **O.4.8** Win32 Window Creation.
- [ ] **O.4.9** Win32 Message Pump.
- [ ] **O.4.10** Cocoa App Init (macOS).
- [ ] **O.4.11** Cocoa Window Creation.
- [ ] **O.4.12** Cocoa Event Loop.
- [ ] **O.4.13** Cursor handling (Set cursor).
- [ ] **O.4.14** Window resizing logic.
- [ ] **O.4.15** Fullscreen toggle.
- [ ] **O.4.16** Clipboard Get/Set.
- [ ] **O.4.17** Drag and Drop file handling.
- [ ] **O.4.18** Window minimization/maximization.
- [ ] **O.4.19** Multi-monitor handling.
- [ ] **O.4.20** DPI scaling detection.

### 4.2 WGPU / Graphics
- [ ] **O.4.21** Vulkan Instance Creation.
- [ ] **O.4.22** Vulkan Device Selection.
- [ ] **O.4.23** Vulkan Swapchain Creation.
- [ ] **O.4.24** Metal Device Creation.
- [ ] **O.4.25** DX12 Device Creation.
- [ ] **O.4.26** Command Encoder creation.
- [ ] **O.4.27** Render Pass begin/end.
- [ ] **O.4.28** Pipeline Layout creation.
- [ ] **O.4.29** Shader Module loading.
- [ ] **O.4.30** Buffer creation (Vertex/Index).
- [ ] **O.4.31** Texture creation.
- [ ] **O.4.32** Sampler creation.
- [ ] **O.4.33** Bind Group creation.
- [ ] **O.4.34** Uniform Buffer updates.
- [ ] **O.4.35** Draw call dispatch.
- [ ] **O.4.36** Present / Swap buffers.
- [ ] **O.4.37** Compute Pass dispatch.
- [ ] **O.4.38** Shader JIT (Korlang -> SPIRV).
- [ ] **O.4.39** Shader JIT (Korlang -> MSL).
- [ ] **O.4.40** Shader JIT (Korlang -> HLSL).

### 4.3 UI Framework
- [ ] **O.4.41** `View` base class implementation.
- [ ] **O.4.42** `Element` tree structure.
- [ ] **O.4.43** `Text` element rendering.
- [ ] **O.4.44** `Button` element rendering.
- [ ] **O.4.45** `Image` element rendering.
- [ ] **O.4.46** `Input` (Text Field) rendering.
- [ ] **O.4.47** `Slider` element rendering.
- [ ] **O.4.48** `Checkbox` element rendering.
- [ ] **O.4.49** `ScrollView` container.
- [ ] **O.4.50** `VStack` layout logic.
- [ ] **O.4.51** `HStack` layout logic.
- [ ] **O.4.52** `ZStack` layout logic.
- [ ] **O.4.53** `Grid` layout logic.
- [ ] **O.4.54** Padding/Margin calculation.
- [ ] **O.4.55** Flexbox-style alignment.
- [ ] **O.4.56** Hit testing (Mouse -> Element).
- [ ] **O.4.57** Event propagation (Bubbling).
- [ ] **O.4.58** State binding (Reactive updates).
- [ ] **O.4.59** Diffing algorithm (Virtual tree).
- [ ] **O.4.60** Patch application to Render tree.

### 4.4 Rendering Utils
- [ ] **O.4.61** Font loading (TrueType/OpenType).
- [ ] **O.4.62** Glyph rasterization (Atlas generation).
- [ ] **O.4.63** Text layout (Line wrapping).
- [ ] **O.4.64** Image decoding (PNG).
- [ ] **O.4.65** Image decoding (JPG).
- [ ] **O.4.66** Vector path tessellation (SVG/Canvas).
- [ ] **O.4.67** Color space conversion (sRGB/Linear).
- [ ] **O.4.68** Animation loop (requestAnimationFrame).
- [ ] **O.4.69** Easing functions (Linear, Bezier).
- [ ] **O.4.70** Transformation matrices (2D/3D).

---

## 🛠️ Group 5: The Integrated Developer Experience [Tasks 401-470]
**Objective:** Ensure the toolchain tools are self-hosting and fully functional.

### 5.1 CLI Tools
- [ ] **O.5.1** `korlang help` output.
- [ ] **O.5.2** `korlang version` output.
- [ ] **O.5.3** `korlang new` (App template).
- [ ] **O.5.4** `korlang new` (Lib template).
- [ ] **O.5.5** `korlang build` (Debug).
- [ ] **O.5.6** `korlang build` (Release).
- [ ] **O.5.7** `korlang build` (Output path).
- [ ] **O.5.8** `korlang run` (Execution).
- [ ] **O.5.9** `korlang run` (Argument passing).
- [ ] **O.5.10** `korlang run` (Exit code).
- [ ] **O.5.11** `korlang test` (Discovery).
- [ ] **O.5.12** `korlang test` (Execution).
- [ ] **O.5.13** `korlang test` (Reporting).
- [ ] **O.5.14** `korlang doc` (Parsing).
- [ ] **O.5.15** `korlang doc` (HTML Generation).
- [ ] **O.5.16** `korlang fmt` (Formatting).
- [ ] **O.5.17** `korlang check` (Syntax check only).
- [ ] **O.5.18** `korlang clean` (Artifact removal).
- [ ] **O.5.19** `korlang update` (Self-update via korup).
- [ ] **O.5.20** CLI color output support.

### 5.2 KPM (Package Manager)
- [ ] **O.5.21** `Korlang.config` parsing.
- [ ] **O.5.22** Dependency resolution (SemVer).
- [ ] **O.5.23** Circular dependency detection.
- [ ] **O.5.24** Lockfile generation.
- [ ] **O.5.25** Lockfile parsing.
- [ ] **O.5.26** Git dependency fetching.
- [ ] **O.5.27** Path dependency linking.
- [ ] **O.5.28** Registry API client (Search).
- [ ] **O.5.29** Registry API client (Download).
- [ ] **O.5.30** Registry API client (Publish).
- [ ] **O.5.31** Cache management (`~/.korlang/cache`).
- [ ] **O.5.32** Build script execution (`build.kor`).
- [ ] **O.5.33** Native library linking configuration.
- [ ] **O.5.34** Workspace support (Multi-package).
- [ ] **O.5.35** Offline mode support.

### 5.3 LSP & IDE
- [ ] **O.5.36** LSP: Initialize handshake.
- [ ] **O.5.37** LSP: textDocument/didOpen.
- [ ] **O.5.38** LSP: textDocument/didChange.
- [ ] **O.5.39** LSP: textDocument/didSave.
- [ ] **O.5.40** LSP: textDocument/publishDiagnostics.
- [ ] **O.5.41** LSP: textDocument/completion.
- [ ] **O.5.42** LSP: textDocument/hover.
- [ ] **O.5.43** LSP: textDocument/definition.
- [ ] **O.5.44** LSP: textDocument/formatting.
- [ ] **O.5.45** LSP: textDocument/rename.
- [ ] **O.5.46** IDE: Buffer (Piece Table) insert.
- [ ] **O.5.47** IDE: Buffer delete.
- [ ] **O.5.48** IDE: Buffer undo/redo.
- [ ] **O.5.49** IDE: Syntax Highlighting engine.
- [ ] **O.5.50** IDE: Theme loader.
- [ ] **O.5.51** IDE: Project file tree view.
- [ ] **O.5.52** IDE: File system watcher integration.
- [ ] **O.5.53** IDE: Terminal emulator widget.
- [ ] **O.5.54** IDE: Git status integration.
- [ ] **O.5.55** IDE: Search in files.

### 5.4 Debugger (K-Trace)
- [ ] **O.5.56** Snapshot serialization.
- [ ] **O.5.57** Snapshot deserialization.
- [ ] **O.5.58** Execution trace recording.
- [ ] **O.5.59** Execution trace replay.
- [ ] **O.5.60** Reverse step logic.
- [ ] **O.5.61** Variable inspection UI.
- [ ] **O.5.62** Breakpoint management.
- [ ] **O.5.63** Heap visualization.
- [ ] **O.5.64** Thread/Fiber inspector.
- [ ] **O.5.65** Panic catching and pausing.

---

## 🤖 Group 6: AI & Cloud Integration [Tasks 471-500]
**Objective:** Confirm high-level abstractions map to hardware correctly.

### 6.1 Tensors & AI
- [ ] **O.6.1** Tensor allocation (CPU).
- [ ] **O.6.2** Tensor shape/stride logic.
- [ ] **O.6.3** Tensor slicing/viewing.
- [ ] **O.6.4** Tensor add (SIMD).
- [ ] **O.6.5** Tensor mul (SIMD).
- [ ] **O.6.6** Tensor matmul (SIMD).
- [ ] **O.6.7** Tensor activation (ReLU/Sigmoid).
- [ ] **O.6.8** GPU Buffer creation (Tensor mapping).
- [ ] **O.6.9** GPU Compute dispatch (Matmul).
- [ ] **O.6.10** GPU Readback (Result).
- [ ] **O.6.11** Model weight loading (Format parsing).
- [ ] **O.6.12** Inference pipeline execution.
- [ ] **O.6.13** Autodiff graph construction (Basic).
- [ ] **O.6.14** Gradient calculation (Backprop).
- [ ] **O.6.15** Optimizer step (SGD).

### 6.2 Cloud Resources
- [ ] **O.6.16** `resource` syntax parsing.
- [ ] **O.6.17** State file (`tfstate` equivalent) creation.
- [ ] **O.6.18** State file read/diffing.
- [ ] **O.6.19** Provider plugin loading interface.
- [ ] **O.6.20** AWS Provider stub verification.
- [ ] **O.6.21** Azure Provider stub verification.
- [ ] **O.6.22** GCP Provider stub verification.
- [ ] **O.6.23** Resource `apply()` logic.
- [ ] **O.6.24** Resource `destroy()` logic.
- [ ] **O.6.25** Secret management logic.

### 6.3 Final Integration
- [ ] **O.6.26** Full "Hello World" build & run.
- [ ] **O.6.27** Full "HTTP Server" build & run.
- [ ] **O.6.28** Full "GUI App" build & run.
- [ ] **O.6.29** Full "AI Inference" build & run.
- [ ] **O.6.30** Self-Compilation benchmark test.

---

## 📊 Phase Omega Metrics
| Category | Task Count | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Foundation | 100 | None | High |
| Stdlib | 100 | Foundation | High |
| Runtime | 70 | Foundation | Critical |
| UI/Graphics | 70 | Runtime | Medium |
| Toolchain | 65 | Stdlib | Medium |
| AI/Cloud | 30 | Runtime | Low |
| **Total** | **500 Tasks** | | |

**Next Step:** Systematic execution of Task O.1.1 to O.1.100.
