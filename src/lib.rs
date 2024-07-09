/// Macro to print text to the standard output without appending a newline character.
#[macro_export]
macro_rules! print_without_newline {
    ( $($arg:tt)* ) => {{
        print!($($arg)*);
    }};
}

/// Macro to read user input from the standard input and parse it into the specified type.
/// 
/// # Variants
///
/// - `input!()`: Reads a line of input as a `String`.
/// - `input!($input_type:ty)`: Reads a line of input and parses it into the specified type.
/// - `input!($msg:expr)`: Reads a line of input as a `String` with a prompt message.
/// - `input!($input_type:ty, $msg:expr)`: Reads a line of input with a prompt message and parses it into the specified type.
///
/// # Examples
///
/// ```rust
/// let user_input: String = input!();
/// let user_input: String = input!("Please enter something: ");
/// let user_input: i32 = input!(i32);
/// let user_input: f64 = input!(f64, "Please enter a number: ");
/// ```
#[macro_export]
macro_rules! input {
    ( $input_type:ty ) => {
        input!($input_type, "");
    };

    ( ) => {
        input!(String, "");
    };

    (  $msg:expr ) => {
        input!(String, $msg);
    };

    ( $input_type:ty, $msg:expr ) => {{
        use std::io::{self, Write};
        print!("{}", $msg);
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let trimmed_input = user_input.trim().to_string();
        match trimmed_input.parse::<$input_type>() {
            Ok(r) => r,
            Err(error) => panic!("Cannot parse: {:?}", error),
        }
    }};
}
