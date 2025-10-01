# ScanParse
**Rust CLI for Tokenizing & Parsing Expressions**

# PROJECT OVERVIEW
ScanParse is a Rust-based command-line utility that scans (tokenizes) and parses simple arithmetic expressions into readable grammar steps. It supports identifiers, multi-digit numbers, +, *, and parentheses, and prints the derivation using the nonterminals EXPR, TERM, FACTOR, EXPRDASH, and TERMDASH. Whitespace is ignored, and basic syntax errors (like a missing )`)` are reported with clear messages.

# KEY FEATURES 🔑
- Reads a text file of expressions (one per line) and prints the grammar expansion for each line.

- 🧩 Deterministic recursive-descent parser implementing:
```
        EXPR → TERM EXPRDASH

        EXPRDASH → + TERM EXPRDASH | ε

        TERM → FACTOR TERMDASH

        TERMDASH → * FACTOR TERMDASH | ε

        FACTOR → IDENTIFIER | NUMBER | ( EXPR ) 
```

- Scanner recognizes identifiers ([A-Za-z]+), numbers ([0-9]+), +, *, (, ), and skips whitespace.


# TECHNICAL STACK 🧱
- Programming Languages/Technologies: Rust, Cargo

- Project Layout: src/main.rs, test input/output files in project root

- Build Tooling: Cargo (cargo build, cargo run <file>)

= Dev Utilities: Bash test runner (run_tests.sh), Git/GitHub

# WHAT'S NEXT?
- Add pretty-printed parse trees and/or AST output.

- Optional evaluator (compute expression results) behind a flag.

- Better diagnostics: token position/line/column in error messages.

# Contributors

- Kelvin Ihezue
