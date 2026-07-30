// You already know that Rust supports familiar comparison conditions from mathematics, such as:

// Less than: a < b
// Less than or equal to: a <= b
// Greater than: a > b
// Greater than or equal to: a >= b
// Equal to a == b
// Not Equal to: a != b
// You can use these conditions to perform different actions for different decisions.

// Rust has the following conditional statements:

// Use if to specify a block of code to be executed, if a specified condition is true
// Use else to specify a block of code to be executed, if the same condition is false
// Use else if to specify a new condition to test, if the first condition is false
// Use match to specify many alternative blocks of code to be executed
// Note: Unlike many other programming languages, if..else can be used as a statement or as an expression (to assign a value to a variable) in Rust. See an example at the bottom of the page to better understand it.


// Use if to specify a block of code to be executed if a condition is true.
let age = 25;
if age >= 18 {
    println!("I am an adult.");
} else {
    println!("I am a minor.");
}

// Don't Mix Types
// Note: The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).

// When you mix types, like a string and an integer, you'll get an error:

// Example

let my_age = 25;
let result = if my_age >= 19 {
    "I am an adult."
} else {
    100
};

println!("{}", result); //error[E0308]: `if` and `else` have incompatible types
