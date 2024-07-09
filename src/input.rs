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
/// use in_put::input;
/// let user_input : String = input!();
/// let user_input : String = input!("Please enter something: ");
/// let user_input : i32 = input!(i32);
/// let user_input : f64 = input!(f64, "Please enter a number: ");
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
        let trimmed_input =  user_input.trim().to_string();
        println!("--> {}", trimmed_input);
        
        match trimmed_input.parse::<$input_type>() {
            Ok(r) => r,
            Err(error) =>  panic!("Cannot parse: {:?}", error),
        }
    }};
}






/// Simple Function to return datatype in String 
/// # variable
/// - value : Type T
/// 
/// # Examples
/// ```rust
/// use in_put::input::datatype;
/// let value_1 : u8 = 2;
/// let value_2 : f64 = 2.1;
/// let value_3 : f32 = 3.14;
/// let value_4 : i32 = 12;
/// let value_5 : String = "val".to_string();
/// let value_6 : &str = "val";
/// 
/// assert_eq!("u8"     , datatype(&value_1));
/// assert_eq!("f64" , datatype(&value_2));
/// assert_eq!("f32"     , datatype(&value_3));
/// assert_eq!("i32" , datatype(&value_4));
/// assert_eq!("String" , datatype(&value_5));
/// assert_eq!("&str" , datatype(&value_6));
/// ```
pub fn datatype<T>(_: &T) -> &str {
    // println!("{}", std::any::type_name::<T>());
    let rtn: &str = std::any::type_name::<T>();
    let mut e: Vec<&str> = rtn.split("::").collect::<Vec<&str>>();
    e.reverse();
    e[0]
    }
