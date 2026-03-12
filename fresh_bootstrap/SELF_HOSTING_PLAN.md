# Korlang Self-Hosting Plan

This document outlines the roadmap to transition the Korlang compiler from its current Rust implementation to a fully self-hosted compiler written in Korlang.

## Current State
- **Rust Frontend:** Lexer, Parser, Sema, Codegen (LLVM).
- **Korlang Frontend (`fresh_bootstrap/`):** Lexer and Parser are largely complete and mirror the Rust implementation.
- **Runtime:** Native C/Rust runtime for memory management and FFI.

## 5-Step Roadmap

### Step 1: Frontend Parity [CRITICAL]
Ensure the Korlang-written lexer and parser in `fresh_bootstrap/` can handle the entire Korlang grammar, including generics, attributes, and complex pattern matching.
- **Task:** Run the Rust-based compiler on the self-hosted lexer/parser and fix all reported type-checker errors.

### Step 2: Minimal Codegen Seed [HIGH]
The self-hosted compiler needs a way to emit machine code. Initially, this will be done by calling the existing LLVM bindings from Korlang.
- **Task:** Complete `fresh_bootstrap/llvm_bindings.kor` to provide access to basic LLVM IR generation.

### Step 3: Stage 1 - The First Self-Compile [MEDIUM]
Use the Rust compiler to compile the Korlang-written compiler.
- **Task:** `korlang build fresh_bootstrap/main.kor -o korlang-stage1`

### Step 4: Stage 2 - Full Self-Hosting [MEDIUM]
Use `korlang-stage1` (the Korlang-written compiler) to compile its own source code.
- **Task:** `./korlang-stage1 fresh_bootstrap/main.kor -o korlang-stage2`

### Step 5: Verification & Cleanup [LOW]
Verify that `korlang-stage1` and `korlang-stage2` are binary-identical (fixed-point reached).
- **Task:** Bit-for-bit comparison of the emitted binaries. Once stable, the Rust implementation can be moved to a `legacy/` or `deprecated/` folder.

## Dependencies
- Robust `std::collections` (Vector, HashMap).
- Working FFI bridge for LLVM calls.
- Recursive-descent parser stability.
