# Compiler Performance (Phase 5.3)

## Targets
- Reduce parse+typecheck time per 10k LOC to under 1s (baseline target).
- Ensure linear scaling with file count.

## Planned Optimizations
- Incremental parsing cache keyed by file hash.
- Arena allocation for AST nodes to reduce allocator churn.
- Interning for identifiers and string literals.
- Parallel file parsing for multi-core CPUs.

