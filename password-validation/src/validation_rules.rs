pub trait ValidationRules {
    fn has_min_length_of(&self, min_length: u16) -> Result<String, String>;
    fn has_one_uppercase(&self) -> Result<String, String>;
    fn has_one_lowercase(&self) -> Result<String, String>;
    fn has_number(&self) -> Result<String, String>;
    fn has_underscore(&self) -> Result<String, String>;
}

impl ValidationRules for String {
    fn has_min_length_of(&self, min_length: u16) -> Result<String, String> {
        if self.len() > min_length.into() {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: too short"));
    }

    fn has_one_uppercase(&self) -> Result<String, String> {
        if self.chars().any(char::is_uppercase) {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing capital letter"));
    }

    fn has_one_lowercase(&self) -> Result<String, String> {
        if self.chars().any(char::is_lowercase) {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing lowercase letter"));
    }

    fn has_number(&self) -> Result<String, String> {
        if self.chars().any(char::is_numeric) {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing a number"));
    }

    fn has_underscore(&self) -> Result<String, String> {
        if self.chars().any(|symbol| symbol == '_') {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing an underscore"));
    }
}
