# Korlang Grammar (Phase 1.1)

This document defines the Phase 1.1 lexical and syntactic specification for Korlang. The authoritative machine-readable grammar is `docs/grammar.ebnf`.

## 1. Tokenizer Specification

### 1.1 Keywords
`fun`, `let`, `var`, `if`, `else`, `match`, `for`, `while`, `break`, `continue`, `return`, `view`, `resource`, `state`, `spawn`, `@nogc`, `import`, `as`

### 1.2 Operators and Punctuation
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Tensor ops: `.+`, `.-`, `.*`, `./`, `@`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- Pipeline: `->`, `|>`
- Null-coalescing: `?:`
- Member/index: `.`, `[ ]`
- Grouping: `( )`, `{ }`
- Separators: `,`, `;`, `:`

### 1.3 Literals
- Integer: `123`
- Hex Integer: `0xFF`
- Float: `3.14`, `2e10`, `1.0e-3`
- Bool: `true`, `false`
- Char: `'a'`, `'\n'`
- String: `"hello"`, supports escapes (`\n`, `\t`, `\\`, `\"`)

### 1.4 Comments and Whitespace
- Line comments: `// like this`
- Block comments: `/* like this */` (non-nesting)
- Whitespace separates tokens and is otherwise ignored.

## 2. Expression Precedence and Associativity

Highest to lowest:
1. Primary: literals, identifiers, parenthesized, array/tensor literals, blocks
2. Postfix: calls, member access, indexing
3. Unary: `!`, `-`, `+`, `~`
4. Multiplicative: `*`, `/`, `%`, `.*`, `./`, `@`
5. Additive: `+`, `-`, `.+`, `.-`
6. Relational: `<`, `<=`, `>`, `>=`
7. Equality: `==`, `!=`
8. Logical AND: `&&`
9. Logical OR: `||`
10. Null-coalescing: `?:` (right-associative)
11. Pipeline: `->`, `|>` (left-associative)
12. Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=` (right-associative)

Notes:
- The pipeline operators are intentionally lower than logical/coalescing, so `a ?: b -> f` parses as `(a ?: b) -> f`.
- Assignment is lowest, so `x = a -> f` assigns the pipeline result.

## 3. Functions and Control Flow

Function declaration:
```
fun add(a: Int, b: Int) -> Int {
  a + b
}
```

Variables:
```
let x: Int = 10;
var y = x * 2;
```

Expression-based control flow:
```
let z = if x > 0 {
  x
} else {
  -x
};
```

## 4. UI: `view` Blocks

A `view` declares a UI component using nested view nodes:
```
view Counter(stateCount: Int) {
  VStack(spacing: 8) {
    Text("Count:", value: @{stateCount});
    Button("Increment", onClick: @{stateCount = stateCount + 1});
  };
}
```

Rules:
- `view` nodes are component calls with arguments.
- A reactive binding uses `@{ expr }`.
- View node bodies are nested blocks of child nodes.

## 5. Cloud: `resource` Blocks

A `resource` defines infrastructure with a typed block:
```
resource AppBucket(aws.s3.Bucket) {
  name: "my-app-bucket";
  versioning: true;
  tags: ["prod", "public"];
}
```

Rules:
- `resource` type is a qualified identifier in parentheses.
- Entries are `key: value;` pairs; values may be nested blocks or arrays.

