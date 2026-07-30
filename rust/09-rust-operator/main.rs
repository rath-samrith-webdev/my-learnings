// Operator in rust consist of the following:
// Arithmetic Operators
// Assignment Operators
// Comparison Operators
// Logical Operators

//Arithmetic Operators
// +, -, *, /, %, **, ++, --

//Assignment Operators
// =, +=, -=, *=, /=, %=, **=

//Comparison Operators
// ==, !=, <, >, <=, >=

//Logical Operators
// &&, ||, !

//example

// Arithmetic Operators
let a = 5;
let b = 3;
let c = a + b;
println!("{}", c); // output: 8

// Assignment Operators
let mut a = 5;
a += 3;
println!("{}", a); // output: 8

// Comparison Operators
let a = 5;
let b = 3;
let c = a > b;
println!("{}", c); // output: true

// Logical Operators
let a = true;
let b = false;
let c = a && b;
println!("{}", c); // output: false

// example
let a = true;
let b = false;
let c = a || b;
println!("{}", c); // output: true


fn main() {
   let add = 5 + 3;
   let sub = 5 - 3;
   let mul = 5 * 3;
   let div = 5 / 3;
   let mod_ = 5 % 3;
   println!("{}", add); // output: 8
   println!("{}", sub); // output: 2
   println!("{}", mul); // output: 15
   println!("{}", div); // output: 1
   println!("{}", mod_); // output: 2
}


// Comparison
fn comparison() {
   let a = 5;
   let b = 3;
   let c = a > b;
   println!("{}", c); // output: true
}

// Logical
fn logical() {
   let a = true;
   let b = false;
   let c = a && b;
   println!("{}", c); // output: false
}
