use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// Loads passwords from a file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// * A `Result` containing a vector of strings if the file was successfully read.
pub fn load_passwords(file_path: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut passwords = Vec::new();
    for line in reader.lines() {
        let password = line?;
        passwords.push(password);
    }

    Ok(passwords)
}

/// Checks the strength of a password.
///
/// # Arguments
///
/// * `password` - A string slice that holds the password to check.
///
/// # Returns
///
/// * A string indicating the strength of the password ("weak", "medium", "strong", "very strong").
pub fn check_strength(password: &str) -> String {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 16 / 2 {
        score += 1;
    }

    let mut uppercase_count = 0;
    let mut lowercase_count = 0;
    let mut digit_count = 0;
    let mut special_count = 0;

    for c in password.chars() {
        if c.is_uppercase() {
            uppercase_count += 1;
        } else if c.is_lowercase() {
            lowercase_count += 1;
        } else if c.is_digit(10) {
            digit_count += 1;
        } else if !c.is_alphanumeric() {
            special_count += 1;
        }
    }

    if uppercase_count > 0 {
        score += 1;
    }
    if lowercase_count > 0 {
        score += 1;
    }
    if digit_count > 0 {
        score += 1;
    }
    if special_count > 0 {
        score += 1;
    }

    // Additional points for multiple occurrences
    if uppercase_count > 1 {
        score += 1;
    }
    if lowercase_count > 1 {
        score += 1;
    }
    if digit_count > 1 {
        score += 1;
    }
    if special_count > 1 {
        score += 1;
    }

    match score {
        0 | 1 | 2 | 3 => "weak".to_string(),
        4 | 5 | 6 => "medium".to_string(),
        7 | 8 | 9 => "strong".to_string(),
        _ => "very strong".to_string(),
    }
}
