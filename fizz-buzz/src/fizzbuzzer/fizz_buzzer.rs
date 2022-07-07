pub fn fizz_buzzer(input: i32) -> String {
    let is_divisible_by_3 = input % 3 == 0;
    let is_divisible_by_5 = input % 5 == 0;

    match (is_divisible_by_3, is_divisible_by_5) {
        (true, false) => return String::from("fizz"),
        (false, true) => return String::from("buzz"),
        (true, true) => return String::from("fizzbuzz"),
        (_, _) => return input.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::fizz_buzzer;
    use rstest::rstest;

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(4)]
    #[case(7)]
    fn that_returns_a_number_if_not_divisible_by_3_or_5(#[case] input: i32) {
        let result = fizz_buzzer(input);
        assert_eq!(result, input.to_string());
    }

    #[rstest]
    #[case(3)]
    #[case(6)]
    #[case(9)]
    #[case(12)]
    fn that_returns_fizz_if_is_divisible_by_3(#[case] input: i32) {
        let result = fizz_buzzer(input);
        assert_eq!(result, String::from("fizz"));
    }

    #[rstest]
    #[case(5)]
    #[case(10)]
    #[case(20)]
    #[case(25)]
    fn that_returns_buzz_if_is_divisible_by_5(#[case] input: i32) {
        let result = fizz_buzzer(input);
        assert_eq!(result, String::from("buzz"));
    }

    #[rstest]
    #[case(15)]
    #[case(30)]
    #[case(45)]
    fn that_returns_fizzbuzz_if_is_divisible_by_3_and_5(#[case] input: i32) {
        let result = fizz_buzzer(input);
        assert_eq!(result, String::from("fizzbuzz"));
    }
}
