# Rust Password Generator

A simple command-line password generator written in Rust.

The program allows you to specify the desired password length, generates a random password using uppercase letters, lowercase letters, numbers, and special characters, and saves the generated password to `passwords.txt`.

## Features

* Choose the password length
* Generates random passwords
* Uses:

  * Lowercase letters
  * Uppercase letters
  * Numbers
  * Special characters
* Saves generated passwords to `passwords.txt`
* Type `exit` to quit without generating a password
* Written in Rust

## Requirements

You will need:

* Rust
* Cargo

You can check whether Rust is installed with:

```bash
rustc --version
cargo --version
```

If both commands return a version number, you're ready to go.

## Getting the Project

Clone the repository:

```bash
git clone <https://github.com/realtimestert/Rust-Password-Generator.git>
```

Enter the project directory:

```bash
cd Rust-Password-Generator
```

## Dependencies

This project uses the [`rand`](https://crates.io/crates/rand) crate for random password generation.

Cargo will automatically download the required dependencies when the project is built.

## Running the Program

The easiest way to run the program is:

```bash
cargo run
```

You will be prompted for the desired password length:

```text
Enter the desired length of the password (or type 'exit' to quit):
```

Enter a number, for example:

```text
20
```

The program will generate a password:

```text
Generating password from 20 characters...
Generated password: X7@kP2!mQ9#vL3$xR8?aB
```

## Exiting

You can type:

```text
exit
```

when prompted for the password length.

The program will then exit without generating a password.

## Building the Program

To compile the program without running it:

```bash
cargo build
```

The executable will be located at:

```text
target/debug/Rust-Password-Generator
```

You can run it directly with:

```bash
./target/debug/Rust-Password-Generator
```

## Release Build

For an optimized release build:

```bash
cargo build --release
```

The optimized executable will be located at:

```text
target/release/Rust-Password-Generator
```

You can run it directly:

```bash
./target/release/Rust-Password-Generator
```

## Important Security Note

This program is intended as a simple Rust learning project and password generator.

The generated passwords are saved in plain text to:


**Do not use this program to store important passwords unless you understand and accept the security implications of storing passwords in an unencrypted text file.**

## Recommended `.gitignore`

Create a `.gitignore` file in the project directory:

```gitignore
/target/
```

This prevents Cargo's build directory and your generated passwords from being committed to Git.

## License

Add your preferred license here.
