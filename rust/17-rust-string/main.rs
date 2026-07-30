let mut greeting: &str = "Hello";
println("{} greeting", greeting);

// Note that strings are surrounded by double quotes ("Hello").

// There are two main types of strings in Rust:

// &str - is called "string slices", and is used for fixed text like "Hello"
// String - used when you need a string that can change

//You can create a String from a string literal using the to_string() method or the String::from() function:

//It is up to you which one to choose - both to_string() and String::from() are very common in Rust.

let text1 = "Hello World".to_string();
let text2 = String::from("Hello World");

let mut greeting = String::from("Hello");
greeting.push_str(" World");
println!("{}", greeting); // Hello World



//Use push() to add one character:

let mut word = String::from("Hi");
word.push('!');
println!("{}", word); // Hi!

// Concatenate Strings
// You can combine strings using the format! macro:

let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);

// You can also use the + operator to combine strings, but it can get messy with many values.

let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = s1 + " " + &s2 + " " + &s3;
println!("{}", result);

// Good to know: format! is often the preferred choice than using + for combining strings.


//You can use the .len() method to get the length of a string

let name = String::from("John");
println!("Length: {}", name.len()); // 4
