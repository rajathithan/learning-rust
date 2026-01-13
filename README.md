# learning-rust

Rust learning workspace with a few standalone crates. Each folder is a separate Cargo project you can build and run independently.

**Exercises**
- **`01_firstrun/`**: Minimal first Rust program to verify toolchain and project setup. Useful as a sanity check that `cargo run` works.

- **`02_variables/`**: Exercises around variable declarations & interpolations.

- **`03_mutables/`**: Experiments with mutable variables and reassignment. Demonstrates `let` vs `mut`, basic types, and simple output.

- **`04_shadow/`**: Focused on variable shadowing examples. Shows how re-declaring with `let` can transform values while keeping immutability per binding.

- **`05_scopes/`**: Demonstrates scope rules in Rust: block scopes, variable visibility, and lifetimes of bindings within nested `{} ` blocks.

- **`06_constants/`**: Shows how to define and use `const` and `static` values, naming conventions (SCREAMING_SNAKE_CASE), and compile-time evaluation.

- **`07_typealiases/`**: Introduces `type` aliases to simplify complex types (e.g., `Result<T, E>` forms), improve readability, and clarify intent.

 - **`08_directives/`**: Demonstrates compiler attributes and directives (e.g., `#[allow(unused_variables)]`) alongside `type` aliases. Shows how to use attributes to control warnings and annotate code.

- **`09_strings/`**: Basics of Rust strings. Demonstrates regular string literals (with escape sequences), raw string literals (`r"..."`, `r#"..."#`), and explains the distinction between borrowed `&str` slices and owned `String` values.

- **`10_floats/`**: Floating-point numbers in Rust. Covers `f32` vs `f64` (IEEE-754), printing with format specifiers (e.g., `{:.2}` for precision, `{:e}` for scientific notation), and common methods like `.ceil()`, `.floor()`, and `.round()`.

- **`11_casting/`**: Type casting in Rust using the `as` keyword. Demonstrates converting between integer and floating-point types, including potential data loss.

- **`12_mathops/`**: Basic arithmetic operations in Rust. Covers addition, subtraction, multiplication, division, and modulo operations on integers.

- **`13_augops/`**: Augmented assignment operators. Shows compound assignments like `+=`, `-=`, `*=`, `/=` for modifying mutable variables.

- **`14_boolean/`**: Boolean logic and conditional statements. Demonstrates `bool` type, logical operators (`&&`, `||`, `!`), and `if-else` chains for decision-making.

- **`15_characters/`**: Introduction to the `char` type in Rust. Demonstrates single Unicode characters, including letters, symbols, and emojis, and how to print them.

- **`16_arrays/`**: Working with arrays in Rust. Covers fixed-size arrays, indexing, iteration, mutability, slicing, and multidimensional arrays.

- **`17_tuples/`**: Working with tuples in Rust. Covers tuple creation, destructuring, accessing elements by index, and tuple methods.

- **`18_ranges/`**: Introduction to ranges in Rust. Demonstrates exclusive (`..`) and inclusive (`..=`) ranges for integers and characters, iteration over ranges, and range types like `Range` and `RangeInclusive`.

- **`19_functions/`**: Introduction to functions in Rust. Demonstrates defining functions with parameters, return types, and different return values including strings, booleans, and tuples.

 - **`20_ifelse/`**: `if`, `else if`, and `else` conditionals. Shows branching logic, nested conditionals, and simple comparisons in practice.

- **`21_match/`**: Introduction to the `match` expression in Rust. Demonstrates pattern matching on values, multiple patterns in a single arm, match guards with `if` conditions, and exhaustive matching.

**Usage**
- **Prereq**: Install Rust and Cargo via `rustup`.
- **Run a crate**:
 	- `cd folder && cargo run`
- **Build only**:
	- `cargo build` (debug)
	- `cargo build --release` (optimized)
- **Clean build artifacts**:
	- `cargo clean`

Tip: Each crate has its own `Cargo.toml` and `src/main.rs`. Run commands from within the respective folder.
