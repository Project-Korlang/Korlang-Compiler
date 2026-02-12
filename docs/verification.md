# Phase 7 Verification

This checklist validates the "Working" toolchain status.

## Steps
1. Build runtime:
   - `cd src/runtime && cargo build`
2. Build CLI:
   - `cd src/tools/cli && cargo build`
3. Compile example:
   - `./src/tools/cli/target/debug/korlang build examples/hello.kor -o hello --lto`
4. (Optional) PGO build:
   - `./src/tools/cli/target/debug/korlang build examples/hello.kor -o hello --pgo-generate`
   - Run binary, then rebuild with `--pgo-use <profile>`

Expected:
- LLVM IR is emitted.
- Link command is printed with LTO/PGO flags.

