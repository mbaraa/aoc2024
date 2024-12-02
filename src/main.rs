use day1::{Day1Pt1, Day1Pt2};
use day2::{Day2Pt1, Day2Pt2};
use solve::{Scanner, Solution};

mod day1;
mod day2;
mod solve;

fn main() {
    let solution: &dyn Solution;

    let mut reader = Scanner::default();
    print!("day(.part)\ne.g day 1 part 2 => 1.2\nenter day's number: ");
    let day_number = reader.next::<f32>();

    solution = match day_number {
        1.1 => &Day1Pt1 {},
        1.2 => &Day1Pt2 {},
        2.1 => &Day2Pt1 {},
        2.2 => &Day2Pt2 {},
        _ => &Day1Pt1 {},
    };

    solution.solve();
}
