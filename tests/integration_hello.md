# Integration Test: hello.kor

This test verifies that the compiler can parse, typecheck, and emit LLVM IR for `examples/hello.kor`.

Steps:
1. `korlang build examples/hello.kor -o hello`
2. Ensure `hello.ll` is emitted and linker command is printed.

