# Cross-Compilation (Phase 6.4)

## Targets
- linux-x86_64
- windows-x86_64
- macos-x86_64

## Plan
- Ship prebuilt runtimes per target.
- Add `korlang build --target <triple>` support.
- Cross-link using target toolchains (clang/llvm).

