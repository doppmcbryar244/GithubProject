use std::io;

fn main() {
    let input = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Example usage of Rust's built-in functions and variables

    // This is just an example, you can use any function or variable you want
    let age = 18;
    println!("Your current age is: {}", age);

    let is_student = true;
    if is_student {
        println!("You are a student.");
    } else {
        println!("You are not a student.");
    }

    // Example of using strings and arrays

    let message = "Hello, world!";
    println!("{}", message);
}
