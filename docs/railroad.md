# Railroad Diagram Generation

This is the Phase 1.1 documentation generation checklist item. It defines how to generate diagrams from `docs/grammar.ebnf` when the documentation toolchain is available.

## Recommended Tooling
Option A (node): `railroad-diagrams`
1. Convert EBNF to a JSON AST with a small script.
2. Render diagrams to SVG with `railroad-diagrams`.

Option B (python): `ebnf2railroad`
1. Install a compatible EBNF to railroad generator.
2. Run it against `docs/grammar.ebnf`.

## Inputs and Outputs
- Input: `docs/grammar.ebnf`
- Output: `docs/railroad/` (SVG files per production)

## Status
Diagrams are not generated in-repo yet; this file documents the process and expected output location.

