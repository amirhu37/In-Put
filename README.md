
# Rust Macros Library

This library provides two macros for enhanced input and output functionalities in Rust.

## Macros

1. `print_without_newline`
2. `input`

## 1. `print_without_newline`

### Overview

The `print_without_newline` macro is used to print text to the standard output without appending a newline character at the end.

### Usage

This macro can be used similarly to the `print!` macro but explicitly indicates that no newline will be appended.

### Example

```rust
print_without_newline!("Hello, ");
print_without_newline!("world!");
```

Output:
```
Hello, world!
```

## 2. `input`

### Overview

The `input` macro provides a convenient way to read user input from the standard input, optionally with a prompt message, and attempts to parse it into the specified type.

### Usage

This macro can be used in various ways to read and parse user input.

### Examples

#### Basic Usage

Read a line of input as a `String` without a prompt:

```rust
let user_input: String = input!();
```

#### With Prompt

Read a line of input as a `String` with a prompt:

```rust
let user_input: String = input!("Please enter something: ");
```

#### Specified Type

Read a line of input and parse it into a specified type (e.g., `i32`):

```rust
let user_input: i32 = input!(i32);
```

#### Specified Type with Prompt

Read a line of input with a prompt and parse it into a specified type (e.g., `f64`):

```rust
let user_input: f64 = input!(f64, "Please enter a number: ");
```

### Error Handling

If the input cannot be parsed into the specified type, the macro will panic with an error message indicating the parsing failure.

## Getting Started

To use these macros in your Rust project, include the following lines in your `lib.rs` or `main.rs`:

```rust
#[macro_use]
extern crate your_crate_name;
```

Replace `your_crate_name` with the name of your crate.

## License

This project is licensed under the MIT License.
```

Save this content into a `README.md` file. This file provides an overview of the library, explains the purpose and usage of each macro, and includes example code snippets.