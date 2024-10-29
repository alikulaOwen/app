use std::io::{self, Write};
use app2::process_text;

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let result = process_text(input.trim());
    println!("Processed Text: {}", result);
}
