pub type SolveFunction = Option<fn(data: &str) -> String>;

#[derive(Debug)]
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

#[cfg(test)]
mod test {
    use super::{AdventOfCodeDay, DayProgress, SolveFunction};

    fn helper_solve(s: &str) -> String {
        s.to_string()
    }

    fn new_day<'a>(part1: SolveFunction, part2: SolveFunction) -> AdventOfCodeDay<'a> {
        AdventOfCodeDay {
            name: "irrelevant",
            part1,
            part2,
        }
    }

    #[test]
    #[should_panic(expected = "Part 2")]
    fn should_panic_on_only_part2_solved() {
        let _ = new_day(None, Some(helper_solve)).progress();
    }

    #[test]
    fn should_return_correct_progress() {
        assert!(matches!(
            new_day(None, None).progress(),
            DayProgress::Unsolved
        ));

        assert!(matches!(
            new_day(Some(helper_solve), None).progress(),
            DayProgress::PartlySolved
        ));

        assert!(matches!(
            new_day(Some(helper_solve), Some(helper_solve)).progress(),
            DayProgress::FullySolved
        ));
    }
}

pub struct AdventOfCodeYear<'a> {
    pub year: u64,
    pub days: Vec<AdventOfCodeDay<'a>>,
}
