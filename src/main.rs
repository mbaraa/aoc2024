use day1::{Day1Pt1, Day1Pt2};
use solve::Solution;

mod day1;
mod solve;

fn main() {
    let solution: &dyn Solution;
    solution = &Day1Pt2 {};

    solution.solve();
}
