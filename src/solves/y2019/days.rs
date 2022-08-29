use crate::solves::year::AdventOfCodeDay;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod intcode;

pub fn get_days<'a>() -> Vec<AdventOfCodeDay<'a>> {
    vec![
        day1::SOLUTION,
        day2::SOLUTION,
        day3::SOLUTION,
        day4::SOLUTION,
        day5::SOLUTION,
    ]
}
