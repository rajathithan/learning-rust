# learning-rust

Rust learning workspace with a few standalone crates. Each folder is a separate Cargo project you can build and run independently.

**Exercises**
- **`firstrun/`**: Minimal first Rust program to verify toolchain and project setup. Useful as a sanity check that `cargo run` works.

- **`variables/`**: Exercises around variable declarations & interpolations.

- **`mutables/`**: Experiments with mutable variables and reassignment. Demonstrates `let` vs `mut`, basic types, and simple output.

- **`shadow/`**: Focused on variable shadowing examples. Shows how re-declaring with `let` can transform values while keeping immutability per binding.

- **`scopes/`**: Demonstrates scope rules in Rust: block scopes, variable visibility, and lifetimes of bindings within nested `{} ` blocks.

- **`constants/`**: Shows how to define and use `const` and `static` values, naming conventions (SCREAMING_SNAKE_CASE), and compile-time evaluation.

- **`typealiases/`**: Introduces `type` aliases to simplify complex types (e.g., `Result<T, E>` forms), improve readability, and clarify intent.

 - **`directives/`**: Demonstrates compiler attributes and directives (e.g., `#[allow(unused_variables)]`) alongside `type` aliases. Shows how to use attributes to control warnings and annotate code.

- **`strings/`**: Basics of Rust strings. Demonstrates regular string literals (with escape sequences), raw string literals (`r"..."`, `r#"..."#`), and explains the distinction between borrowed `&str` slices and owned `String` values.

- **`floats/`**: Floating-point numbers in Rust. Covers `f32` vs `f64` (IEEE-754), printing with format specifiers (e.g., `{:.2}` for precision, `{:e}` for scientific notation), and common methods like `.ceil()`, `.floor()`, and `.round()`.

- **`casting/`**: Type casting in Rust using the `as` keyword. Demonstrates converting between integer and floating-point types, including potential data loss.

- **`mathops/`**: Basic arithmetic operations in Rust. Covers addition, subtraction, multiplication, division, and modulo operations on integers.

- **`augops/`**: Augmented assignment operators. Shows compound assignments like `+=`, `-=`, `*=`, `/=` for modifying mutable variables.

- **`boolean/`**: Boolean logic and conditional statements. Demonstrates `bool` type, logical operators (`&&`, `||`, `!`), and `if-else` chains for decision-making.

- **`characters/`**: Introduction to the `char` type in Rust. Demonstrates single Unicode characters, including letters, symbols, and emojis, and how to print them.

- **`arrays/`**: Working with arrays in Rust. Covers fixed-size arrays, indexing, iteration, mutability, slicing, and multidimensional arrays.


**Usage**
- **Prereq**: Install Rust and Cargo via `rustup`.
- **Run a crate**:
	- `cd folder && cargo run src/main.rs`
- **Build only**:
	- `cargo build` (debug)
	- `cargo build --release` (optimized)
- **Clean build artifacts**:
	- `cargo clean`

Tip: Each crate has its own `Cargo.toml` and `src/main.rs`. Run commands from within the respective folder.
