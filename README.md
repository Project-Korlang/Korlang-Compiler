# Korlang

Korlang is a modern, high-performance, compiled programming language designed for systems programming with a focus on developer experience, safety, and performance.

## 🚀 Features

- **Blazing Fast Performance**: LLVM-backed backend for native machine code generation.
- **Modern Syntax**: Concise, expressive, and easy to learn.
- **Safety First**: Optional garbage collection with `@nogc` support for performance-critical code.
- **Pattern Matching**: Robust `match` expressions for idiomatic control flow.
- **Generics**: Type-safe generic functions and data structures.
- **LSP Support**: Rich editor integration with go-to-definition, real-time diagnostics, and more.
- **Self-Hosting**: A self-hosted compiler is currently in development (see `fresh_bootstrap/`).

## 🛠️ Quick Start

### Installation

```bash
# Build the compiler from source
cargo build --release
# Add the binary to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

### Hello World

Create a file named `hello.kor`:

```korlang
fun main() -> Int {
    println("Hello, Korlang!")
    return 0
}
```

Compile and run:

```bash
korlang run hello.kor
```

## 📂 Project Structure

- `src/compiler` — Rust implementation of the compiler.
- `stdlib/` — Standard library modules (io, collections, math, etc.).
- `tools/lsp` — Language Server Protocol implementation.
- `fresh_bootstrap/` — Self-hosted Korlang compiler source code.
- `tests/` — Comprehensive integration and unit test suite.

## 🛣️ Roadmap

- **v1.1**: Enhanced pattern matching, string interpolation, and multi-return values.
- **v1.2**: Traits with interface bounds, first-class functions, and module system improvements.
- **v1.3**: Async/await primitives, compile-time evaluation (comptime), and WASM target refinement.

## 🤝 Contributing

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) to get started.

---

Built with ❤️ by the Korlang community.
