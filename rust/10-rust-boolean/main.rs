// Very often, in programming, you will need a data type that can only have one of two values, like:

// YES / NO
// ON / OFF
// TRUE / FALSE
// For this, Rust has a bool data type, which is known as booleans.

// Booleans represent values that are either true or false.

let is_programmer: bool = true;
let is_student: bool = false;
 println!("Hello, am I a programmer? {}", is_programmer); // output: Hello, I am a programmer? true
 println!("Hello, am I a student? {}", is_student); // output: Hello, I am a student? false


 //boolean comparison

 let age = 25;
 let is_adult = age >= 18;
 println!("{}", is_adult); // output: true


 // if statement with boolean comparison
 if is_adult {
    println!("I am an adult.");
 } else {
    println!("I am a minor.");
 }
