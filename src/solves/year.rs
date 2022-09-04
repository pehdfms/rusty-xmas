pub type SolveFunction = Option<fn(data: &str) -> String>;

pub enum DayProgress {
    Unsolved,
    PartlySolved,
    FullySolved,
}

pub struct AdventOfCodeDay<'a> {
    pub name: &'a str,
    pub part1: SolveFunction,
    pub part2: SolveFunction,
}

impl AdventOfCodeDay<'_> {
    #[must_use]
    /// # Panics
    /// Advent of Code days are separated into two parts, with the second
    /// one being unlocked only after you finish the first. It shouldn't
    /// be possible for only Part 2 to be solved as it indicates that
    /// part 1 has just not been added. This requirement can be relaxed
    /// but it implies an error most of the time.
    pub fn progress(&self) -> DayProgress {
        match (self.part1, self.part2) {
            (Some(_), Some(_)) => DayProgress::FullySolved,
            (Some(_), None) => DayProgress::PartlySolved,
            (None, None) => DayProgress::Unsolved,
            _ => panic!("Didn't expect only Part 2 to be solved!"),
        }
    }
}

pub struct AdventOfCodeYear<'a> {
    pub year: u64,
    pub days: Vec<AdventOfCodeDay<'a>>,
}
