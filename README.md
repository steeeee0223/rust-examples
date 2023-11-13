# Rust Examples

### Project Structure

```text
<project-name>
├── Cargo.lock
├── Cargo.toml
├── examples
│   ├── <example>.rs
│   └── ...
├── <lib>
│   ├── Cargo.toml
│   ├── examples
│   │   └── <example>.rs
│   └── src
│       └── lib.rs
└── src
    └── main.rs
```

### Important Commands

##### `Run`

-   To run binary
    -   e.g. `cargo run` (This runs `./src/main.rs`)
-   To run examples under `./examples`, use `cargo run --example <example>`
    -   e.g. `cargo run --example foo` (This runs `./examples/foo.rs`)
    -   e.g. `cargo run --example bar` (This runs `./examples/bar/main.rs`)
-   To run examples inside a lib, use `cargo run -p <lib> --example <example>`
    -   e.g. `cargo run -p variables --example numbers` (This runs `./variables/examples/numbers.rs`)

##### `Test`

-   To run tests, use `cargo test`
-   To run tests in package, use `cargo test -p <lib>`
    -   e.g. `cargo test -p variables` (This runs the tests in `./variables/src/lib.rs`)

### Learning Path

💡 Learning from [Rust Lang-Book](https://doc.rust-lang.org/book/)

-   [x] Prompt Input (§2)
-   [x] Variables (§3.1~3.2)
-   [x] Control Flows (§3.5, §6.2~6.3)
-   [x] Structs (§5)
-   [x] Enums (§6)
-   [x] Packages, Crates, Modules (§7)
    -   [x] Backyard - binary crate
    -   [x] Restaurant - library crate
    -   [x] Restaurant (New) - library crate (Recommended package structure)
-   [x] Exceptions (§8)
