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