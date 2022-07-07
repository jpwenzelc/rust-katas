pub trait DivisionUtility {
    fn is_divisible_by(&self, denominator: u32) -> bool;
}

impl DivisionUtility for u32 {
    fn is_divisible_by(&self, denominator: u32) -> bool {
        self % denominator == 0
    }
}

pub fn is_leap(year: u32) -> bool {
    year.is_divisible_by(4) && !(year.is_divisible_by(100) && !year.is_divisible_by(400))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::is_leap;

    #[rstest]
    #[case(1993)]
    #[case(1997)]
    #[case(1999)]
    fn that_a_year_is_not_leap_if_not_divisible_by_4(#[case] year: u32) {
        assert!(!is_leap(year));
    }

    #[rstest]
    #[case(1996)]
    #[case(2000)]
    #[case(2004)]
    fn that_a_year_is_leap_if_divisible_by_4(#[case] year: u32) {
        assert!(is_leap(year));
    }

    #[rstest]
    #[case(1800)]
    fn that_a_year_is_not_leap_if_divisible_by_4_and_100_but_not_400(#[case] year: u32) {
        assert!(!is_leap(year));
    }
}
