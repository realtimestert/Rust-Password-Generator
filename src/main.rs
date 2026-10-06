use rand::RngExt;
use std::io::{self};

const ALLOWED_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*?";

fn ask_length() -> Option<usize> {
    println!("Enter the desired length of the password (or type 'exit' to quit):");

    let mut num_str = String::new();

    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line");

    let input = num_str.trim();

    if input.eq_ignore_ascii_case("exit") {
        return None;
    }

    match input.parse::<usize>() {
        Ok(length) if length > 0 => Some(length),
        Ok(_) => {
            println!("Password length must be greater than 0.");
            None
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            None
        }
    }
}

fn generate_password(length: usize) -> String {
    let mut rng = rand::rng();
    let chars: Vec<char> = ALLOWED_CHARS.chars().collect();

    (0..length)
        .map(|_| {
            let index = rng.random_range(0..chars.len());
            chars[index]
        })
        .collect()
}

fn main() {
    let pwd_length = match ask_length() {
        Some(length) => length,
        None => return,
    };

    println!("Generating password from {} characters...", pwd_length);

    let generated_pw = generate_password(pwd_length);

    println!("Generated password: {}", generated_pw);
}