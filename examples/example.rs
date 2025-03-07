use pval::Pval;
use pval::utils::{check_strength, load_passwords};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = Pval::new()
        .min_length(7)
        .max_length(16)
        .require_uppercase(false)
        .require_lowercase(true)
        .require_digit(true)
        .require_special(true)
        .build();

    let passwords = load_passwords("examples/passwords.txt")?;

    for pwd in passwords {
        match validator.validate(&pwd) {
            Ok(_) => println!("Password '{}' is valid", pwd),
            Err(e) => println!("Password '{}' is invalid: {}", pwd, e),
        }

        let strength = check_strength(&pwd);
        println!("Password {} strength: {}", pwd, strength);
    }

    Ok(())
}
