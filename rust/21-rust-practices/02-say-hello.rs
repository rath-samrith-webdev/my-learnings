use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // Print the greeting lines
 		println!("Hello, {}!", name);
    println!("Welcome to programming!");
}
