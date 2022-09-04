use super::year::AdventOfCodeYear;

mod days;

#[must_use]
pub fn get_2019_solutions<'a>() -> AdventOfCodeYear<'a> {
    AdventOfCodeYear {
        year: 2019,
        days: days::get_days(),
    }
}
