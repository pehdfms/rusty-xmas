use super::year::AdventOfCodeYear;

mod days;

pub fn get_2019_solutions() -> AdventOfCodeYear {
    AdventOfCodeYear {
        year: 2019,
        days: days::get_days(),
    }
}
