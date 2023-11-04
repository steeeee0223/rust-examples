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
-   [ ] Control Flows (§3.5)
-   [ ] Structs (§5)
