//** Rust use ownership to manage memory in a safe way.
//** All values inside of rust has its very owner and it's usually a variable.

//**WHY IT MATTERS.
//**1. Rust uses ownership to automatically free memory when it's no longer needed
//**2. It prevents bug lik using memory that's been deleted
//**3. It is one of the reason Rust is so safe and fast

/*
@Rules
1. Each value has one owner
2. When the owner goes out of scope the value is deleted.
3. You can only have one owner at a time, unless you **borrow** it.
*/

//Basic example

let a = String::from("Hi, bro");
let b = a; // the value of a got deleted here.

// println!("{}", a) resulting an error no longer owns the value

println!("{}", b) // passed b now is the owner of the value.

//when you assigned the value a to b the owner moves. Meaning now only b has value while a doen't
//but simple types like **number**, **characters**, and **boolean** are copied, not moved.
//Meaning you can still has access the original variables after reassigned to another.

//Example: copied values.
let na = 3;
let nb = na;

println!("a= {}", na);
println!("b= {}", nb);

//Example: Cloning

let a = String::from("Hi, bro");
let b = a.clone(); // the value of a got deleted here.

println!("{}", a) //passed: a now still has its value since b has copied from it.
println!("{}", b) // passed: b now is the owner of the value.
