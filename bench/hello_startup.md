# Benchmark: hello.kor Startup

## Goal
Measure startup time of `examples/hello.kor` with and without LTO.

## Procedure
1. Build runtime and CLI.
2. Build without LTO:
   - `korlang build examples/hello.kor -o hello_nolto`
3. Build with LTO:
   - `korlang build examples/hello.kor -o hello_lto --lto`
4. Time execution with your system tool (e.g., `time`).

