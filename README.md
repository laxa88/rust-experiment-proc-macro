# Rust experiments

A bunch of things I learned while experimenting in Rust.

## How to create libraries

1. Create a new cargo project for the library:

```
cargo new --lib my_lib_name
```

1. Optionally, customize the name in `Cargo.toml`:

```
[package]
edition = "2021"
name = "my_custom_lib_name"
version = "0.1.0"
```

Notes:

- Entry point is `lib.rs`.
- Must not contain `main.rs`, otherwise the code cannot be exported and used externally.

## How to import libraries

Reference: http://www.purplefrog.com/~thoth/rust-external-libraries/

1. Add under `dependency` in `Cargo.toml`:

```
my_custom_lib_name = {path = "./my_lib_folder"}
```

2. Import and use in main code:

```rust
// import
extern crate fields_sum;

// alias
use fields_sum::print_sum_fields;

fn main() {
    // use
    fields_sum();
}
```

## How to create proc-macro library

References:

- https://dev.to/dandyvica/rust-procedural-macros-step-by-step-tutorial-36n8
- https://developerlife.com/2022/03/30/rust-proc-macro/#how-to-parse-item

1. [Create a new library](#how-to-create-libraries)

1. Update `Cargo.toml` and mark the library as proc-macro:

```
[lib]
proc-macro = true
```

1. Update `Cargo.toml` and import dependencies for writing proc-macro:

```
[dependencies]
syn = { version = "1.0.82", features = ["full", "extra-traits"] }
quote = "1.0.10"
```

Notes:

- Proc macros cannot be debugged, because it's not a runtime executable.
- `println!()` only works during runtime.
- So, the only way to see a result is using `panic!()` and viewing the message during `cargo check`.

## How to write proc-macro

Refer to `my_macros/src/lib.rs`
