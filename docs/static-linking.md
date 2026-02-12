# Static Linking (Phase 5.3)

## Goal
Produce a single, zero-dependency binary by statically linking the Korlang runtime.

## Plan
- Build `korlang_rt` as a static library (`libkorlang_rt.a`).
- Link all object files and runtime into a single executable.
- Provide a `-static` mode in the CLI where supported.

