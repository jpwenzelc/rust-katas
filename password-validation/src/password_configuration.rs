pub struct PasswordConfiguration {
    pub min_length: u16,
    pub needs_uppercase: bool,
    pub needs_lowercase: bool,
    pub needs_number: bool,
    pub needs_underscore: bool,
}

impl Default for PasswordConfiguration {
    fn default() -> Self {
        Self {
            min_length: 8,
            needs_uppercase: true,
            needs_lowercase: true,
            needs_number: true,
            needs_underscore: true,
        }
    }
}
