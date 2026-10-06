# Rust Password Generator

A simple command-line password generator written in Rust.

The program allows you to specify the desired password length and generates a random password using lowercase letters, uppercase letters, numbers, and special characters.

## Features

* Choose the password length
* Generate random passwords
* Uses:

  * Lowercase letters
  * Uppercase letters
  * Numbers
  * Special characters
* Type `exit` to quit
* Passwords are displayed directly in the terminal
* Passwords are **not saved to a file**
* Written in Rust

## Requirements

You will need:

* Rust
* Cargo

Check that Rust and Cargo are installed:

```bash
rustc --version
cargo --version
```

If both commands return a version number, you're ready to go.

## Getting the Project

Clone the repository:

```bash
git clone https://github.com/realtimestert/Rust-Password-Generator.git
```

Enter the project directory:

```bash
cd Rust-Password-Generator
```

## Dependencies

This project uses the [`rand`](https://crates.io/crates/rand) crate for random password generation.

Cargo automatically downloads the required dependencies when the project is built.

## Running with Cargo

The easiest way to run the program while developing is:

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
Exiting in 5 seconds...
```

The generated password is displayed directly in the terminal and is not saved to a file.

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
target/debug/password_generator
```

You can run it directly with:

```bash
./target/debug/password_generator
```

## Release Build

For an optimized release build:

```bash
cargo build --release
```

The optimized executable will be located at:

```text
target/release/password_generator
```

You can run it directly with:

```bash
./target/release/password_generator
```

## Installing as a Terminal Command

On Linux and macOS, you can install the release executable into your local binary directory:

```bash
mkdir -p ~/.local/bin
cp target/release/password_generator ~/.local/bin/password-generator
```

You can then run the program from anywhere in the terminal:

```bash
password-generator
```

To verify which executable is being used:

```bash
which password-generator
```

## Updating the Program

If you already cloned the repository, update it with:

```bash
cd ~/Rust-Password-Generator
git pull
```

Rebuild the release version:

```bash
cargo build --release
```

Update the installed terminal command:

```bash
cp target/release/password_generator ~/.local/bin/password-generator
```

You can then run the updated version:

```bash
password-generator
```

## Security Note

This program generates passwords locally on your computer.

Generated passwords are displayed directly in the terminal and are not saved to a file or transmitted over the network by this program.

Be aware that terminal history, terminal scrollback, screen recording, or other software on your computer could potentially expose a generated password.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
