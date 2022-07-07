use year_utils::leap_year::is_leap;

pub mod year_utils;

fn main() {
    for year in 1000..2000 {
        println!("is {} that {year} is leap year", is_leap(year));
    }
}
