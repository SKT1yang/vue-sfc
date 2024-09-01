# Vue SFC Rust Library

## Overview

This Rust library is designed for parsing Vue Single-File Components (SFCs), generating Abstract Syntax Trees (ASTs), modifying those ASTs, and generating new Vue SFC files.

**Note:** This library is **not** a compiler and does not generate runtime objects for Vue SFCs. It is intended for batch modification of Vue SFC source code files.

## Features

- **Parse Vue SFCs:** Efficiently parse Vue SFC files to generate ASTs.
- **Modify ASTs:** Make modifications to the ASTs, allowing for programmatic changes to the Vue component structure.
- **Generate Vue SFCs:** Create new Vue SFC files from modified ASTs.

## Installation

Add this library to your `Cargo.toml` file:

```toml
[dependencies]
vue-sfc = "0.1.0"
```

Then, in your Rust code, you can import and use the library:

```rust
use vue_sfc_rs::{parse_sfc, generate_sfc, modify_ast};
```

## Usage

1. Parsing a Vue SFC File:

```rust
let sfc = parse_sfc("path/to/your/vue.sfc");
```

2. Modifying the AST:

```rust
let modified_ast = modify_ast(&sfc.ast, |ast| {
    // Your modification logic here
});
```

3. Generating a new Vue SFC File:

```rust
generate_sfc("path/to/output/vue.sfc", &modified_ast);
```

## Contributing

Contributions are welcome! Please feel free to submit issues, pull requests, or feature requests.

## License
