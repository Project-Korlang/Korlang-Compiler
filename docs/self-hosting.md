# Self-Hosting Plan (Phase 5.3)

## Strategy
1. Stabilize the Korlang AST and parser.
2. Implement a minimal compiler subset in Korlang (lexer + parser).
3. Use the Rust compiler to compile the Korlang compiler.
4. Add codegen in Korlang and validate equivalence.

## Milestones
- M1: Korlang lexer + parser passes tests.
- M2: Korlang type checker compiles with Rust compiler.
- M3: Self-hosted compiler emits LLVM IR for simple programs.

