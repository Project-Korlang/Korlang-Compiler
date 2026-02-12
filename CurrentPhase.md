# Current Phase: Phase 7 - The Professional CLI & Global Distribution

**Status:** Activating Global Toolchain
**Goal:** Transition `korlang` from a project-local compiler to a system-wide tool. After this phase, you will be able to install `korlang` once and use it in any directory to create, build, and run projects.

---

## 🛠️ 7.1 Advanced Scaffolding (`korlang new`)
**Objective:** Create standardized project structures with a single command.
- [x] **7.1.1 Template Engine:** Implement `korlang new <name> [--lib | --ui | --cloud]`.
- [x] **7.1.2 Auto-Config:** Automatically generate a valid `Korlang.config` and a `hello.kor` entry point.
- **Effort:** 3 Days | **Priority:** High

## 🌍 7.2 Global Environment & Pathing
**Objective:** Ensure the compiler knows where the standard library lives.
- [x] **7.2.1 KORLANG_HOME:** Implement environment variable detection to locate `core` and `runtime` binaries.
- [x] **7.2.2 Relative Discovery:** If no env-var is found, the CLI should check relative to the binary's location (standard for portable installs).
- **Effort:** 4 Days | **Priority:** Critical

## 📦 7.3 `korup`: The Toolchain Manager
**Objective:** A dedicated tool for installing and updating Korlang (See `tasks_site.md`).
- [x] **7.3.1 Installation Scripts:** Develop `install.sh` and `install.ps1` in the `Korlang-Site` project.
- [x] **7.3.2 CI/Release Pipeline:** Ensure the compiler builds are automatically uploaded to GitHub Releases so the script can find them.
- **Effort:** 6 Days | **Priority:** Medium

## ⚡ 7.4 Build Artifact Management
**Objective:** Keep the project folder clean and builds fast.
- [x] **7.4.1 Target Directory:** Move all build artifacts (LLVM IR, object files, binaries) into a `.korlang/target` folder.
- [x] **7.4.2 Incremental Rebuilds:** Detect if source files haven't changed to skip redundant compilation steps.
- **Effort:** 5 Days | **Priority:** High

---

## 🚀 How it will look (The Cargo Experience)

```bash
# 1. Install globally (hosted via GitHub Pages)
curl -fsSL https://project-korlang.github.io/korlang/install.sh | sh

# 2. Create a new app
korlang new my_app
cd my_app

# 3. Add a dependency
kpm add ui-kit

# 4. Run immediately
korlang run
```

---

## 📊 Phase 7 Metrics
| Module | Est. Effort | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Scaffolding | 3 Days | None | Low |
| Pathing | 4 Days | CLI Core | Medium |
| `korup` | 6 Days | Distribution | Medium |
| Artifacts | 5 Days | Linker | Low |
| **Total** | **18 Days** | | |

**Next Step:** Implement the `new` command in `src/tools/cli/src/main.rs` and the `KORLANG_HOME` resolution logic.
