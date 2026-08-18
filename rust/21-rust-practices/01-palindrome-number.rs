use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    // Negative numbers cannot be palindromes
    if n < 0 {
        println!("Negative numbers cannot be palindromes.");
        return;
    }

    // Check if n is a palindrome using arithmetic
    let original = n;
    let mut reversed_num = 0;
    let mut temp = n;

    while temp > 0 {
        let digit = temp % 10;
        reversed_num = reversed_num * 10 + digit;
        temp /= 10;
    }

    if original == reversed_num {
        println!("Yes");
    } else {
        println!("No");
    }
}
