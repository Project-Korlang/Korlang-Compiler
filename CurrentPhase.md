# Current Phase: Phase 8 - Self-Hosting Part 1: Korlang-in-Korlang Frontend

**Status:** Initializing Independence
**Goal:** Begin the journey toward language independence by rewriting the Korlang compiler in Korlang itself. This phase focuses on the Lexer and Parser—the "Frontend"—using pure Korlang syntax.

---

## 🏗️ 8.1 Lexer Implementation in Korlang
**Objective:** Replace the Rust lexer with a native Korlang implementation.
- [x] **8.1.1 String Scanner:** Build a high-performance character stream processor using Korlang's `String` and `Char` types.
- [x] **8.1.2 Token Definitions:** Port all `TokenKind` variants to Korlang `enum` or `class` structures.
- [x] **8.1.3 Interpolation Logic:** Implement the complex `"{variable}"` and `@{...}` nesting logic in Korlang.
- **Effort:** 5 Days | **Priority:** High

## 🏗️ 8.2 Parser Implementation in Korlang
**Objective:** Build a recursive descent and Pratt parser in pure Korlang.
- [x] **8.2.1 Abstract Syntax Tree (AST):** Define the AST node hierarchy using Korlang classes and interfaces.
- [x] **8.2.2 Expression Parser:** Implement Pratt Parsing for operator precedence, specifically the `->` pipeline and math operators.
- [x] **8.2.3 Statement Parser:** Implement function declarations, `view` blocks, and `resource` orchestration syntax.
- **Effort:** 10 Days | **Priority:** High

## 🏗️ 8.3 Self-Hosting Verification
**Objective:** Ensure the new compiler frontend is bit-for-bit compatible in its logic.
- [x] **8.3.1 AST Parity:** Create a tool to compare AST outputs from the Rust-compiler and the new Korlang-compiler.
- [x] **8.3.2 Regression Testing:** Use the new parser to parse the existing `stdlib` and `examples`.
- **Effort:** 4 Days | **Priority:** Critical

---

## 📈 The Road to Independence
By completing this phase, we move one step closer to removing the Rust bootstrap. 
1. **Phase 8 (Current):** Frontend in Korlang.
2. **Phase 9:** Sema in Korlang.
3. **Phase 10:** Codegen in Korlang.
4. **Phase 11:** The Great Switch (Compiler compiles itself).

---

## 📊 Phase 8 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Native Lexer | 5 Days | None | Low |
| Native Parser| 10 Days | Lexer | Medium |
| Parity Tests | 4 Days | Rust-Compiler | Low |
| **Total** | **19 Days** | | |

**Next Step:** Wire the Korlang frontend into a runnable CLI and activate parity checks.
