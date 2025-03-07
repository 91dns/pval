## pval

`pval` is a Rust library for validating passwords with customizable rules.

## Installation

Add `pval` to your `Cargo.toml`:

```toml
[dependencies]
pval = "0.1.0"
```

Or you can add it via the command line:

```sh
cargo add pval
```

## Usage

Here's an example of how to use `pval`:

```rust
use pval::Pval;

fn main() -> Result<(), String> {
    let validator = Pval::new()
        .min_length(8)
        .max_length(16)
        .require_uppercase(true)
        .require_lowercase(true)
        .require_digit(true)
        .require_special(true)
        .build();

    let password = "P@ssw0rd";

    validator.validate(password)?;
    println!("Password is valid.");

    Ok(())
}
```

## Utility Functions

In addition to the main password validator, `pval` provides utility functions for common tasks related to password management.

### Check Password Strength

You can check the strength of a password using the `check_strength` function:

```rust
use pval::utils::check_strength;

fn main() {
    let password = "P@ssw0rd";

    let strength = check_strength(password);
    println!("Password strength: {}", strength);
}
```

The `check_strength` function evaluates the strength of a password based on its length and the presence of uppercase letters, lowercase letters, digits, and special characters. It returns a string indicating the strength of the password ("weak", "medium", "strong", "very strong").

### Load Passwords from a File

You can load passwords from a file using the `load_passwords` function:

```rust
use pval::utils::load_passwords;

fn main() -> Result<(), std::io::Error> {
    let passwords = load_passwords("passwords.txt")?;

    for password in passwords {
        println!("Loaded password: {}", password);
    }

    Ok(())
}
```

The `load_passwords` function reads passwords from a specified file and returns them as a vector of strings.

You can also validate and check the loaded passwords:

```rust
use pval::Pval;
use pval::utils::load_passwords;

fn main() -> Result<(), std::io::Error> {
    let validator = Pval::new()
        .min_length(8)
        .max_length(16)
        .require_uppercase(true)
        .require_lowercase(true)
        .require_digit(true)
        .require_special(true)
        .build();

    let passwords = load_passwords("passwords.txt")?;

    for password in passwords {
        match validator.validate(&password) {
            Ok(_) => println!("Password '{}' is valid.", password),
            Err(e) => println!("Password '{}' is invalid: {}", password, e),
        }

        let strength = check_strength(&password);
        println!("Password '{}' strength: {}", password, strength);
    }

    Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
