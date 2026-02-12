# Korlang FFI Interfaces (Phase 1.3)

This document defines the initial FFI surface for C++, Rust, and Python.

## 1. C / C++ FFI

### 1.1 Importing Symbols
```
@import("c")
fun c_strlen(s: *u8) -> Int
```

### 1.2 Calling Convention
- Default ABI: C ABI
- Pointers are raw (`*T`) and unsafe by default
- Caller is responsible for lifetime and ownership

## 2. Rust FFI

### 2.1 Importing Rust Symbols
```
@import("rust")
fun rs_hash(data: *u8, len: Int) -> UInt
```

### 2.2 Safety Notes
- Korlang treats Rust functions as C ABI wrappers
- Rust libraries should expose `extern "C"` entry points

## 3. Python FFI (Bridge)

### 3.1 Embedding API
```
@bridge("python")
fun py.eval(code: String) -> PyObject

@bridge("python")
fun py.call(fn: PyObject, args: [Any]) -> PyObject
```

### 3.2 Types
- `PyObject` is an opaque handle
- Values are marshaled via runtime bridge

## 4. Validation Rules (Compiler)
- `@import` / `@bridge` must be used with a string literal argument
- `@bridge("python")` functions must return `PyObject` or primitive types
- Imported functions are not allowed in `@nogc` unless explicitly marked safe

