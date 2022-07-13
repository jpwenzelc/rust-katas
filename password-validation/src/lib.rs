mod validation_rules;
mod password_configuration;

use password_configuration::PasswordConfiguration;
use validation_rules::ValidationRules;

pub fn validate_password(password: String, rules: Option<PasswordConfiguration>) -> bool {
    let rules = rules.unwrap_or_default();

    match password
        .has_min_length_of(rules.min_length)
        .and_then(|password| password.has_one_uppercase(rules.needs_uppercase))
        .and_then(|password| password.has_one_lowercase(rules.needs_lowercase))
        .and_then(|password| password.has_number(rules.needs_number))
        .and_then(|password| password.has_underscore(rules.needs_underscore)) {
            Ok(_) => true,
            Err(_) =>  false,
        }
}

#[cfg(test)]
mod tests {
    use crate::{validate_password, password_configuration::PasswordConfiguration};

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
        assert!(!validate_password(password, None))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_uppercase() {
        let password = String::from("zxczxczxcasd");
        assert!(!validate_password(password, None))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_lowercase() {
        let password = String::from("DAJSDKLASDK");
        assert!(!validate_password(password, None))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_a_number() {
        let password = String::from("DAJSasdawDK");
        assert!(!validate_password(password, None))
    }

    #[rstest]
    fn that_returns_an_err_if_password_doesnt_contain_an_underscore() {
        let password = String::from("DA3SasdawD1");
        assert!(!validate_password(password, None))
    }

    #[rstest]
    fn that_returns_an_ok_if_all_validations_pass() {
        let password = String::from("DA3Sa_sdawD1");
        assert!(validate_password(password, None))
    }

    #[rstest]
    fn that_accepts_a_configuration_to_change_min_length() {
        let password = String::from("1aA_");
        let validation_rules = PasswordConfiguration {
            min_length: 4,
            ..Default::default()
        };
        assert!(validate_password(password, Some(validation_rules)))
    }
    
    #[rstest]
    fn that_accepts_a_configuration_to_allow_missing_capital_letters() {
        let password = String::from("ash1234_1d");
        let validation_rules = PasswordConfiguration {
            needs_uppercase: false,
            ..Default::default()
        };
        assert!(validate_password(password, Some(validation_rules)))
    }

    #[rstest]
    fn that_accepts_a_configuration_to_allow_missing_lowercase_letters() {
        let password = String::from("ASH1234_1D");
        let validation_rules = PasswordConfiguration {
            needs_lowercase: false,
            ..Default::default()
        };
        assert!(validate_password(password, Some(validation_rules)))
    }

    #[rstest]
    fn that_accepts_a_configuration_to_allow_missing_numbers() {
        let password = String::from("ASHlike_aD");
        let validation_rules = PasswordConfiguration {
            needs_number: false,
            ..Default::default()
        };
        assert!(validate_password(password, Some(validation_rules)))
    }

    #[rstest]
    fn that_accepts_a_configuration_to_allow_missing_underscore() {
        let password = String::from("ASHlike1aD");
        let validation_rules = PasswordConfiguration {
            needs_underscore: false,
            ..Default::default()
        };
        assert!(validate_password(password, Some(validation_rules)))
    }
}
