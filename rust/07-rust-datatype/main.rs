// data type
//In Rust, the type of a variable is decided by the value you give it. Rust looks at the value and automatically chooses the right type:
let my_num = 5;         // integer
let my_double = 5.99;   // float
let my_letter = 'D';    // character
let my_bool = true;     // boolean
let my_text = "Hello";  // string

// However you can explicitly declare the type of a variable:
let my_num: i32 = 5;
let my_double: f64 = 5.99;
let my_letter: char = 'D';
let my_bool: bool = true;
let my_text: &str = "Hello";
// data type conversion
//In Rust, you can convert between data types using the as keyword:
let my_num = 5;
let my_double = my_num as f64;
println!("{}", my_double);
// output: 5.0

// type inference
//In Rust, you can let the compiler infer the type of a variable:
let my_num = 5;
let my_double = 5.99;
let my_letter = 'D';
let my_bool = true;
let my_text = "Hello";
//Add explanation and examples for data type conversion and type inference in Rust
// example
// println!("Hello, {}", name);
// output: Hello, John Doe
// println!("Hello, {}", name);
// output: Hello, John Doe
// println!("Hello, {}", name);
// output: Hello, John Doe
// println!("Hello, {}", name);
// output: Hello, John Doe


//Basic data types in Rust
// Number (i32, f64)
// Character (char)
// Boolean (bool)
// String (&str)
// Tuple ((), Vec<T>)
// Enum (enum)
// Struct (struct)
// Union (union)
