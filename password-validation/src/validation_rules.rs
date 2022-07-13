pub trait ValidationRules {
    fn has_min_length_of(&self, min_length: u16) -> Result<String, String>;
    fn has_one_uppercase(&self, is_active_rule: bool) -> Result<String, String>;
    fn has_one_lowercase(&self, is_active_rule: bool) -> Result<String, String>;
    fn has_number(&self, is_active_rule: bool) -> Result<String, String>;
    fn has_underscore(&self, is_active_rule: bool) -> Result<String, String>;
}

impl ValidationRules for String {
    fn has_min_length_of(&self, min_length: u16) -> Result<String, String> {
        if self.len() >= min_length.into() {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: too short"));
    }

    fn has_one_uppercase(&self, is_active_rule: bool) -> Result<String, String> {
        if self.chars().any(char::is_uppercase) || !is_active_rule {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing capital letter"));
    }

    fn has_one_lowercase(&self, is_active_rules: bool) -> Result<String, String> {
        if self.chars().any(char::is_lowercase) || !is_active_rules {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing lowercase letter"));
    }

    fn has_number(&self, is_active_rule: bool) -> Result<String, String> {
        if self.chars().any(char::is_numeric) || !is_active_rule {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing a number"));
    }

    fn has_underscore(&self, is_active_rule: bool) -> Result<String, String> {
        if self.chars().any(|symbol| symbol == '_') || !is_active_rule {
            return Ok(self.to_string());
        }
        return Err(String::from("Invalid password: missing an underscore"));
    }
}
