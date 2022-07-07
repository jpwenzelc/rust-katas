use fizzbuzzer::fizz_buzzer::fizz_buzzer;

pub mod fizzbuzzer;

fn main() {
    for number in 1..=100 {
        println!("{}", fizz_buzzer(number));
    }
}
