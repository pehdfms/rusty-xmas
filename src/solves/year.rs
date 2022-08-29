pub type SolveFunction = Option<fn(data: String) -> String>;

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
    pub year: u32,
    pub days: Vec<AdventOfCodeDay<'a>>,
}
