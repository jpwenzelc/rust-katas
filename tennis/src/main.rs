use crate::tennis::scorer::calculate_score;

pub mod tennis;

fn main() {
    println!("Most intense tennis set!");
    println!("{}", calculate_score(1,0));
    println!("{}", calculate_score(2,0));
    println!("{}", calculate_score(2,1));
    println!("{}", calculate_score(2,2));
    println!("{}", calculate_score(3,2));
    println!("{}", calculate_score(3,3));
    println!("{}", calculate_score(3,4));
    println!("{}", calculate_score(4,4));
    println!("{}", calculate_score(4,5));
    println!("{}", calculate_score(5,5));
    println!("{}", calculate_score(5,6));
    println!("{}", calculate_score(5,7));
}
