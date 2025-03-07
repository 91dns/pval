/// `Pval` is a struct that holds the password validation criteria.
pub struct Pval {
    min_length: usize,
    max_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_digit: bool,
    require_special: bool,
}

impl Pval {
    /// Creates a new `Pval` instance with default values.
    pub fn new() -> Self {
        Self {
            min_length: 0,
            max_length: usize::MAX,
            require_uppercase: false,
            require_lowercase: false,
            require_digit: false,
            require_special: false,
        }
    }

    /// Sets the minimum length requirement for the password.
    ///
    /// # Arguments
    ///
    /// * `min_length` - A usize that sets the minimum length.
    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    /// Sets the maximum length requirement for the password.
    ///
    /// # Arguments
    ///
    /// * `max_length` - A usize that sets the maximum length.
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    /// Sets whether the password must contain at least one uppercase letter.
    ///
    /// # Arguments
    ///
    /// * `require` - A bool that sets whether the password must contain at least one uppercase letter.
    pub fn require_uppercase(mut self, require: bool) -> Self {
        self.require_uppercase = require;
        self
    }

    /// Sets whether the password must contain at least one lowercase letter.
    ///
    /// # Arguments
    ///
    /// * `require` - A bool that sets whether the password must contain at least one lowercase letter.
    pub fn require_lowercase(mut self, require: bool) -> Self {
        self.require_lowercase = require;
        self
    }

    /// Sets whether the password must contain at least one digit.
    ///
    /// # Arguments
    ///
    /// * `require` - A bool that sets whether the password must contain at least one digit.
    pub fn require_digit(mut self, require: bool) -> Self {
        self.require_digit = require;
        self
    }

    /// Sets whether the password must contain at least one special character.
    ///
    /// Special characters are defined as any character that is not a letter or digit.
    ///
    /// # Arguments
    ///
    /// * `require` - A bool that sets whether the password must contain at least one special character.
    pub fn require_special(mut self, require: bool) -> Self {
        self.require_special = require;
        self
    }

    /// Builds the `Pval` instance with the specified criteria.
    pub fn build(self) -> Self {
        self
    }

    /// Validates a password against the criteria.
    ///
    /// # Arguments
    ///
    /// * `password` - A string slice that holds the password to validate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the password is valid.
    pub fn validate(&self, password: &str) -> Result<(), String> {
        if password.len() < self.min_length {
            return Err(format!(
                "Password is too short. Minimum length is {}.",
                self.min_length
            ));
        }
        if password.len() > self.max_length {
            return Err(format!(
                "Password is too long. Maximum length is {}.",
                self.max_length
            ));
        }

        let mut has_uppercase: bool = false;
        let mut has_lowercase: bool = false;
        let mut has_digit: bool = false;
        let mut has_special: bool = false;

        for c in password.chars() {
            if c.is_uppercase() {
                has_uppercase = true;
            } else if c.is_lowercase() {
                has_lowercase = true;
            } else if c.is_digit(10) {
                has_digit = true;
            } else if !c.is_alphanumeric() {
                has_special = true;
            }
        }

        if self.require_uppercase && !has_uppercase {
            return Err("Password must contain at least one uppercase letter.".to_string());
        }
        if self.require_lowercase && !has_lowercase {
            return Err("Password must contain at least one lowercase letter.".to_string());
        }
        if self.require_digit && !has_digit {
            return Err("Password must contain at least one digit.".to_string());
        }
        if self.require_special && !has_special {
            return Err("Password must contain at least one special character.".to_string());
        }

        Ok(())
    }
}
