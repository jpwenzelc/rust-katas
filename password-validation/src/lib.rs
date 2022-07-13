mod validation_rules;

pub use crate::validation_rules::ValidationRules;

pub fn validate_password(password: String) -> bool {
    match password
        .has_min_length_of(8)
        .and_then(|password| password.has_one_uppercase())
        .and_then(|password| password.has_one_lowercase())
        .and_then(|password| password.has_number())
        .and_then(|password| password.has_underscore()) {
            Ok(_) => true,
            Err(_) =>  false,
        }
}

#[cfg(test)]
mod tests {
    use crate::validate_password;
    use rstest::rstest;

    #[rstest]
    #[case("x")]
    #[case("xx")]
    #[case("xxx")]
    #[case("xxxx")]
    #[case("xxxxx")]
    #[case("xxxxxx")]
    #[case("xxxxxxx")]
    #[case("xxxxxxxx")]
    fn that_returns_an_err_if_password_is_less_than_8_characters_long(#[case] password: &str) {
        let password = String::from(password);
        assert!(!validate_password(password))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_uppercase() {
        let password = String::from("zxczxczxcasd");
        assert!(!validate_password(password))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_lowercase() {
        let password = String::from("DAJSDKLASDK");
        assert!(!validate_password(password))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_a_number() {
        let password = String::from("DAJSasdawDK");
        assert!(!validate_password(password))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_an_underscore() {
        let password = String::from("DA3SasdawD1");
        assert!(!validate_password(password))
    }

    #[rstest]
    fn that_returns_an_ok_if_all_validations_pass() {
        let password = String::from("DA3Sa_sdawD1");
        assert!(validate_password(password))
    }
}
