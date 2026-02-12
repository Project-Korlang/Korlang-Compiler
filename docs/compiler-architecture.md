# Korlang Compiler Architecture (Phase 1.4)

This document defines the compiler architecture, IR strategy, bootstrapping plan, and runtime layout for Korlang.

## 1. LLVM Backend Strategy

### 1.1 Pipeline
1. Parse source to AST.
2. Lower AST to KIR (Korlang Intermediate Representation).
3. Lower KIR to LLVM IR.
4. Run LLVM optimization passes.
5. Emit object files and link to executable/library.

### 1.2 LLVM Mapping (Initial)
- `Int` -> `i64`
- `UInt` -> `i64` with unsigned ops
- `Float` -> `double`
- `Bool` -> `i1` (widened to `i8` in memory)
- `Char` -> `i32`
- `String` -> struct `{ i8*, i64, i64 }`
- `Array<T>` -> struct `{ T*, i64, i64 }`
- `Tensor<T, Shape>` -> struct `{ T*, i64* (shape), i64 (rank), i64 (len) }`

### 1.3 Linking Strategy
- Static runtime library (`libkorlang_rt.a`) linked into user binaries.
- Optional dynamic `libkorlang_rt.so` for faster iteration.
- FFI uses standard C ABI with thin wrappers.

## 2. KIR (Korlang IR)

### 2.1 Goals
- Preserve high-level semantics needed for ownership and GC.
- Be simple to lower to LLVM.
- Allow mid-level optimizations (inlining, escape analysis).

### 2.2 Node Categories
- `Module`, `Function`, `Block`
- `Let`, `Assign`, `Return`, `Branch`, `Jump`
- `Call`, `Invoke`, `Phi`
- `StructInit`, `EnumInit`, `TupleInit`
- `Load`, `Store`, `Borrow`, `Move`, `Drop`
- `TensorOp`, `PipelineOp`

### 2.3 Type Info in KIR
- All values are annotated with resolved types.
- Borrow and move operations are explicit.
- GC roots are annotated during lowering.

## 3. Bootstrapping Plan

### 3.1 Phase 1: Rust Bootstrap
- Implement lexer, parser, type checker in Rust.
- Use `inkwell` or LLVM C API for codegen.
- Output LLVM IR or object files.

### 3.2 Phase 2: Self-Hosting
- Once Korlang is stable enough, implement compiler in Korlang.
- Use the Rust compiler as the reference implementation.

## 4. Runtime Layout

### 4.1 Components
- **Allocator**: wrapper over system allocator, plus GC hooks.
- **GC**: generational collector with precise roots.
- **Task Scheduler**: lightweight task system for `spawn`.
- **FFI Layer**: C ABI boundary, pinning, and safe handles.

### 4.2 Root Scanning Strategy
- Stack map metadata emitted by LLVM.
- Global static root registry.
- Handle tables for external references (FFI).

### 4.3 Python Embedding (AI Bridge)
- Optional runtime feature flag.
- Embedding API exposes:
  - `py.init()`, `py.eval(String) -> PyObject`, `py.call(PyObject, [Any])`.
- Objects crossing boundary are pinned or copied.

