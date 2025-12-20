# Forge — small CLI-oriented language (minimal implementation)

This repository contains a minimal yet working compiler pipeline (lexer → parser → type checker → compiler → VM) implemented in Rust.

Requirements

- Rust (stable)

Build

On Windows and Linux (assuming Rust toolchain installed):

```bash
cargo build --release
```

Run

Create a source file `example.forge`:

```text
function main() {
    let x: int = 10;
    print(x);
}
```

Then run:

```bash
# run (compile & execute)
forge run example.forge
```

Or build (bytecode kept in-memory in this minimal demo):

```bash
forge build example.forge
```

Notes

This implementation focuses on a small, clear pipeline and implements a subset of the requested language features suitable for CLI tools: functions, let, basic expressions, arrays (syntax parsed but not compiled), function calls and a built-in `print`.

The code is modular and can be extended to support enums, traits, structures, match, exhaustiveness checks, more bytecode instructions, and a real bytecode file format.
