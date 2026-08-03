/**
In rust data structures are used to strore and organize values.
Rust provides many built-in data structures, such as arrays, vectors, hash maps, and tuples.
*/

//1. Arrays
// Arrays are fixed-size collections of elements of the same type. They are useful when you know the number of elements in advance. Arrays are defined using square brackets [] and can be initialized with values.

//Example: Arrays
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let fruits: [&str; 3] = ["apple", "banana", "cherry"];
println!("The first number is: {}", numbers[0]);

//2. Vectors
// Vectors are dynamic arrays that can grow or shrink in size. They are defined using the Vec<T> type and can be initialized with the vec! macro.

//Example : Vectors
let mut numbers = vec![1, 2, 3, 4, 5];
numbers.push(6); // Add an element to the end of the vector
println!("The last number is: {}", numbers[numbers.len() - 1]);

//3. Hash Maps
// Hash maps are collections of key-value pairs. They are defined using the HashMap<K, V> type and can be initialized with the HashMap::new() function. Hash maps are useful for storing data that can be looked up by a unique key.

//Example: Hash Maps
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
println!("Alice's score is: {}", scores["Alice"]);

//4. Tuples
// Tuples are fixed-size collections of elements of different types. They are defined using parentheses () and can be initialized with values. Tuples are useful for returning multiple values from a function.

//Example: Tuples
let person = ("Alice", 30, true);
println!("Name: {}, Age: {}, Is Student: {}", person.0, person.1, person.2);

//5. Structs
// Structs are custom data types that allow you to group related values together. They are defined using the struct keyword and can have named fields. Structs are useful for modeling real-world entities and their properties.

//Example: Structs
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
    is_student: true,
};

println!("Name: {}, Age: {}, Is Student: {}", person.name, person.age, person.is_student);

