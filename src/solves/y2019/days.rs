use crate::solves::year::AdventOfCodeDay;

mod day1;
mod day2;
mod day3;

mod intcode;

pub fn get_days() -> Vec<AdventOfCodeDay> {
    vec![day1::SOLUTION, day2::SOLUTION, day3::SOLUTION]
}
