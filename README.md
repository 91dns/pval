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

You can also load passwords from a file and validate them:

```rust
use pval::Pval;
use pval::utils::load_passwords;

fn main() -> Result<(), std::io::Error> {
    let validator = Pval::new()
        .min_length(8)
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
    }

    Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.