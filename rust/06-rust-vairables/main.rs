let name = "John Doe";
println!("Hello, {}", name);
// output: Hello, John Doe

// what is {}?
// it is a placeholder for a variable
// you can use as many placeholders as you want
// example
// println!("Hello, {} {}", name, "Doe");
// output: Hello, John Doe
// println!("Hello, {}", name);
// output: Hello, John Doe

// example

let jname = "John Doe";
let age = 30;
println!("Hello, {} you are {} years old", jname, age);
// output: Hello, John Doe you are 30 years old

//Note: you can use as many placeholders as you want
// you can use as many variables as you want
// you can use as many types as you want
// you cannot change value of a variable
// example
// let age = 30;
// println!("I am {} years old", age);
// output: I am 30 years old

// if you want to change value of a variable, you can use let keyword
// example
let mut age = 30;
println!("I am {} years old", age);
// output: I am 30 years old

// you can change value of a variable
// example
age = 31;
println!("I am {} years old", age);
// output: I am 31 years old
