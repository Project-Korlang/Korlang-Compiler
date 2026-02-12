# Korlang Type System (Phase 1.2)

This document defines the Korlang type system for Phase 1.2, covering primitives, composite types, Smart-GC ownership, `@nogc` rules, and Tensor shape calculus.

## 1. Core Type Hierarchy

### 1.1 Primitive Types
- `Int`: signed 64-bit integer (i64)
- `UInt`: unsigned 64-bit integer (u64)
- `Float`: 64-bit float (f64)
- `Bool`: 1-bit logical value (stored as u8 in memory)
- `Char`: 32-bit Unicode scalar value (u32)
- `String`: UTF-8 string slice backed by heap buffer (see 2.2)

### 1.2 Special Types
- `Any`: top type for dynamic erasure and FFI boundaries; all types are subtypes of `Any`.
- `Nothing`: bottom type representing no value; used for non-terminating expressions or impossible branches.

### 1.3 Composite Types
- Structs: `struct Name { field: Type; ... }`
- Enums/Variants: `enum Name { Variant(Type, ...); ... }`
- Tuples: `(T1, T2, ...)`
- Arrays: `[T]` (homogeneous, growable)
- Slices: `Slice<T>` (borrowed view into array or buffer)
- Option: `Option<T>` (desugars to enum `Some(T) | None`)
- Result: `Result<T, E>` (desugars to enum `Ok(T) | Err(E)`)

## 2. Memory Model and Layout

### 2.1 Value Categories
- **Copy types**: small, fixed-size values with implicit bitwise copy.
  - Default: `Int`, `UInt`, `Float`, `Bool`, `Char`, and tuples/structs of Copy types.
- **Move types**: types that own heap resources or require drop.
  - Default: `String`, `[T]`, `Tensor<...>`, and any struct/enum containing a Move type.

### 2.2 Standard Layout Rules
- Struct layout: fields are laid out in declared order with natural alignment.
- Enum layout: tagged union with a discriminant (u32) and the largest variant payload.
- String layout: `{ ptr: *u8, len: Int, cap: Int }`.
- Array layout: `{ ptr: *T, len: Int, cap: Int }`.

## 3. Smart-GC Ownership Model

Korlang uses a hybrid memory system with three tiers:

### 3.1 Tier 1: Ephemeral (Stack)
- Non-escaping values are stack-allocated.
- Escape analysis decides if a value outlives its defining scope.
- Copy types are always stack-friendly.

### 3.2 Tier 2: Generational GC (Heap)
- Escaping values are allocated in the managed heap.
- The GC is precise for Korlang-managed allocations.
- GC roots: stack slots, global statics, and handle tables.

### 3.3 Tier 3: Managed (ARC / Off-Heap)
- Large buffers (e.g., Tensor data) may use reference counting or mmap.
- GC holds a lightweight handle to off-heap storage.
- Off-heap memory is reclaimed when the last handle is dropped.

### 3.4 Ownership Rules (Default)
- Move types are single-owner by default; assignment moves the value.
- Copy types are implicitly copied on assignment and pass-by-value.
- Borrowing is implicit in safe code: `&T` and `&mut T` are inferred where possible.

## 4. `@nogc` and Borrow Checker Rules

`@nogc` blocks enforce deterministic memory behavior and prohibit GC allocation.

### 4.1 Constraints
- No heap allocations inside `@nogc` unless explicitly pinned or stack-only.
- No implicit cloning of Move types.
- Functions annotated `@nogc` may only call other `@nogc` functions.

### 4.2 Borrowing
- `&T`: immutable borrow, multiple allowed.
- `&mut T`: exclusive borrow, only one at a time.
- Borrow lifetimes are inferred; explicit lifetimes are allowed if inference fails.

### 4.3 Moves in `@nogc`
- Move types must have a clear owner in the scope.
- Moving out of a borrowed location is forbidden.
- Returning a move type transfers ownership to the caller.

### 4.4 Pinning for FFI
- `pin(x)` produces a pinned handle that prevents relocation.
- Pinned objects are safe to pass to FFI; unpin releases the pin.

## 5. Tensor and Shape Calculus

### 5.1 Type Form
`Tensor<T, Shape>` where `T` is an element type and `Shape` is a compile-time shape descriptor.

### 5.2 Shape Syntax
- Shape is a list: `[d1, d2, ...]`
- Each `d` is either a constant integer, a symbolic dimension, or `_` for unknown.

### 5.3 Shape Rules
- Elementwise ops require compatible shapes under broadcasting.
- Broadcasting aligns from the trailing dimensions.
- Two dimensions are compatible if:
  - They are equal constants, or
  - One of them is `1`, or
  - One of them is `_` (unknown, unified at compile time if possible).

### 5.4 Inference and Validation
- When both operands have known shapes, the result shape is computed statically.
- When shapes are partially unknown, the compiler introduces constraints.
- Incompatible shapes are compile-time errors unless explicitly cast.

## 6. Type Inference (Brief)

- Local type inference within expressions and function bodies.
- Function signatures are required for public APIs.
- Polymorphism is explicit via generics: `fun map<T, U>(...) -> ...`.

