# Current Phase: Phase Reality - Functional Singularity

**Status:** Implementation & Hardening
**Goal:** Transform Korlang into a fully functional, self-sufficient system language. This phase ensures that the compiler correctly handles all expressions, the runtime correctly executes all logic, and the standard library provides all basic functions—all written in 100% pure Korlang.

---

## 🏛️ Group 1: The "Brains" - Compiler Logic Implementation [150 Tasks]
**Objective:** Finalize the backend lowering logic to ensure every operator and control flow structure generates valid machine code.

### 1.1 Arithmetic & Logic Pipeline (Reconfirmed)
- [ ] **R.1.1** Implement `Add` lowering in `kir.kor`: Emit `KirInstr.Add`.
- [ ] **R.1.2** Implement `Sub` lowering: Emit `KirInstr.Sub`.
- [ ] **R.1.3** Implement `Mul` lowering: Emit `KirInstr.Mul`.
- [ ] **R.1.4** Implement `Div` lowering: Emit `KirInstr.Div`.
- [ ] **R.1.5** Implement `Mod` lowering: Emit `KirInstr.Mod`.
- [ ] **R.1.6** Implement Float `Add` (FAdd).
- [ ] **R.1.7** Implement Float `Sub` (FSub).
- [ ] **R.1.8** Implement Float `Mul` (FMul).
- [ ] **R.1.9** Implement Float `Div` (FDiv).
- [ ] **R.1.10** Implement Comparison: `Eq` (Equal).
- [ ] **R.1.11** Implement Comparison: `NotEq` (Not Equal).
- [ ] **R.1.12** Implement Comparison: `Lt` (Less Than).
- [ ] **R.1.13** Implement Comparison: `Gt` (Greater Than).
- [ ] **R.1.14** Implement Comparison: `LtEq`.
- [ ] **R.1.15** Implement Comparison: `GtEq`.
- [ ] **R.1.16** Implement Logical `And`.
- [ ] **R.1.17** Implement Logical `Or`.
- [ ] **R.1.18** Implement Logical `Not`.
- [ ] **R.1.19** Implement Bitwise `And`.
- [ ] **R.1.20** Implement Bitwise `Or`.
- [ ] **R.1.21** Implement Bitwise `Xor`.
- [ ] **R.1.22** Implement Bitwise `Shl`.
- [ ] **R.1.23** Implement Bitwise `Shr`.
- [ ] **R.1.24** Implement Precedence handling for `(a + b) * c`.
- [ ] **R.1.25** Implement Unary `-` (Negation).
- [ ] **R.1.26** Implement Unary `+` (Identity).
- [ ] **R.1.27** Implement Boolean to Integer casting.
- [ ] **R.1.28** Implement Integer to Float casting.
- [ ] **R.1.29** Implement Float to Integer casting.
- [ ] **R.1.30** Implement String to Boolean truthiness.

### 1.2 Control Flow Stability (Reconfirmed)
- [ ] **R.1.31** Implement `If` statement branching logic.
- [ ] **R.1.32** Implement `Else` branch jump targets.
- [ ] **R.1.33** Implement `While` loop condition checking.
- [ ] **R.1.34** Implement `While` loop back-edges.
- [ ] **R.1.35** Implement `For` loop iterator initialization.
- [ ] **R.1.36** Implement `For` loop increment logic.
- [ ] **R.1.37** Implement `Match` statement jump table generation.
- [ ] **R.1.38** Implement `Match` default case (else).
- [ ] **R.1.39** Implement `Break` statement scope searching.
- [ ] **R.1.40** Implement `Continue` statement jump to header.
- [ ] **R.1.41** Implement `Return` from nested blocks.
- [ ] **R.1.42** Implement `Return` with values.
- [ ] **R.1.43** Implement `Return` without values (Void).
- [ ] **R.1.44** Implement Short-circuiting for `&&`.
- [ ] **R.1.45** Implement Short-circuiting for `||`.
- [ ] **R.1.46** Implement Ternary/Elvis expression lowering.
- [ ] **R.1.47** Implement Nested `If` statements.
- [ ] **R.1.48** Implement Nested `While` loops.
- [ ] **R.1.49** Implement Infinite `Loop` construct.
- [ ] **R.1.50** Implement Tail-call optimization (Basic).

### 1.3 Variable & Memory Management (Reconfirmed)
- [ ] **R.1.51** Implement `Let` immutable binding enforcement.
- [ ] **R.1.52** Implement `Var` mutable update logic.
- [ ] **R.1.53** Implement Pointer-to-Variable resolution.
- [ ] **R.1.54** Implement Global variable initialization.
- [ ] **R.1.55** Implement Local variable stack allocation.
- [ ] **R.1.56** Implement Struct field offset calculation.
- [ ] **R.1.57** Implement Struct field loading.
- [ ] **R.1.58** Implement Struct field storing.
- [ ] **R.1.59** Implement Array index bound checking (Runtime).
- [ ] **R.1.60** Implement Array indexing lowering.
- [ ] **R.1.61** Implement Dynamic array allocation.
- [ ] **R.1.62** Implement String concatenation lowering.
- [ ] **R.1.63** Implement Character indexing into Strings.
- [ ] **R.1.64** Implement Reference tracking for `&` (Borrow).
- [ ] **R.1.65** Implement Dereference `*`.
- [ ] **R.1.66** Implement Copy semantics for primitives.
- [ ] **R.1.67** Implement Move semantics for unique objects.
- [ ] **R.1.68** Implement Closure capture (Environment packing).
- [ ] **R.1.69** Implement Lambda lifting.
- [ ] **R.1.70** Implement Static member access.

*Verification:* `scripts/verify_group1.sh` now covers Region/Linear/Smartptr checks, so it validates the Variable & Memory Management block (R.1.51–R.1.70) whenever Group 1 runs.

### 1.4 Native Backend opcodes (R.1.71 - R.1.150)
- [ ] **R.1.71 - R.1.150** [80 Tasks] Implement mapping for all x86_64 and AArch64 registers and instruction patterns required for Group 1.1-1.3.

---

## 📦 Group 2: The "Hands" - Simplified Standard Library [150 Tasks]
**Objective:** Implement core functionality using simple, intuitive keywords.

### 2.1 Basic I/O (Group 2.1)
- [ ] **R.2.1** `print(val)` -> Write to stdout without newline.
- [ ] **R.2.2** `echo(val)` -> Write to stdout with automatic newline.
- [ ] **R.2.3** `input()` -> Read string from stdin.
- [ ] **R.2.4** `input(msg)` -> Print prompt and read string.
- [ ] **R.2.5** `write(val)` -> Alias for `print`.
- [ ] **R.2.6** `error(val)` -> Write to stderr.
- [ ] **R.2.7** `clear()` -> Clear terminal screen.
- [ ] **R.2.8** `file.open(path)`
- [ ] **R.2.9** `file.new(path)` -> Create/Overwrite file.
- [ ] **R.2.10** `file.read()` -> Read whole file into string.
- [ ] **R.2.11** `file.write(str)`
- [ ] **R.2.12** `file.append(str)`
- [ ] **R.2.13** `file.exists(path)`
- [ ] **R.2.14** `file.delete(path)`
- [ ] **R.2.15** `dir.list(path)`
- [ ] **R.2.16** `dir.new(path)`
- [ ] **R.2.17** `path.join(a, b)`
- [ ] **R.2.18** `path.base(p)`
- [ ] **R.2.19** `path.ext(p)`
- [ ] **R.2.20** `path.abs(p)`

### 2.2 Simple Math (Group 2.2)
- [ ] **R.2.21** `abs(n)`
- [ ] **R.2.22** `sqrt(n)`
- [ ] **R.2.23** `sin(n)`, `cos(n)`, `tan(n)`
- [ ] **R.2.24** `pow(base, exp)`
- [ ] **R.2.25** `log(n)`
- [ ] **R.2.26** `ceil(n)`, `floor(n)`, `round(n)`
- [ ] **R.2.27** `min(a, b)`, `max(a, b)`
- [ ] **R.2.28** `clamp(v, min, max)`
- [ ] **R.2.29** `random()`
- [ ] **R.2.30** `PI`, `E` (Built-in constants).

### 2.3 Data & String (Group 2.3)
- [ ] **R.2.31** `str.len(s)`
- [ ] **R.2.32** `str.sub(s, start, len)`
- [ ] **R.2.33** `str.find(s, sub)`
- [ ] **R.2.34** `str.upper(s)`, `str.lower(s)`
- [ ] **R.2.35** `str.trim(s)`
- [ ] **R.2.36** `str.split(s, sep)`
- [ ] **R.2.37** `json.parse(s)`
- [ ] **R.2.38** `json.str(obj)` -> Stringify.
- [ ] **R.2.39** `b64.enc(s)`, `b64.dec(s)`
- [ ] **R.2.40** `hex.enc(s)`, `hex.dec(s)`
- [ ] **R.2.41 - R.2.80** [40 Tasks] Implement all remaining `List` and `Map` methods from Phase Omega using `list.` and `map.` prefixes.

### 2.4 Time & OS (Group 2.4)
- [ ] **R.2.81** `now()` -> Timestamp.
- [ ] **R.2.82** `sleep(ms)`
- [ ] **R.2.83** `args()` -> List of CLI arguments.
- [ ] **R.2.84** `env(key)` -> Get environment variable.
- [ ] **R.2.85** `exit(code)`
- [ ] **R.2.86 - R.2.150** [65 Tasks] Finalize remaining syscall-backed OS interactions.

---

## 🧵 Group 3: The "Life" - Runtime & Reliability [100 Tasks]
**Objective:** Hardening the M:N Scheduler and Smart-GC using only Korlang.

- [ ] **R.3.1 - R.3.100** [100 Tasks] Implement the entire Scheduler and GC roadmap from Phase Omega using pure `@nogc` Korlang and Assembly.

---

## 🎨 Group 4: The "Face" - Native UI & Tooling [100 Tasks]
**Objective:** Building the final ecosystem.

- [ ] **R.4.1 - R.4.50** [50 Tasks] Implement direct Windowing and Graphics (Vulkan/Metal) using direct kernel calls and Assembly.
- [ ] **R.4.51 - R.4.100** [50 Tasks] Implement the IDE and CLI tools using the newly functional standard library.

---

## 📊 Summary of Effort (Phase Reality)
| Section | Tasks | Dependency | Risk |
| :--- | :--- | :--- | :--- |
| Brains (Compiler) | 150 | KIR Base | Critical |
| Hands (Stdlib) | 150 | Syscalls | High |
| Life (Runtime) | 100 | Assembly | High |
| Face (UI/Tools) | 100 | Graphics | Medium |
| **Total** | **500 Tasks** | | |

**Next Step:** Implement `R.1.1` (Add) and `R.2.1` (print) to fix the complex demo.
