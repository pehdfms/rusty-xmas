use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::solves::year::AdventOfCodeDay;

use super::intcode::Computer;

fn part1(data: &str) -> String {
    let mut computer = Computer::from_string(data);

    computer.replace(1, 12);
    computer.replace(2, 2);

    computer.run();

    computer.read(0).to_string()
}

fn part2(data: &str) -> String {
    let memory = Computer::parse(data);

    (0..100)
        .into_par_iter()
        .map(|noun| {
            (0..100)
                .into_par_iter()
                .map(|verb| {
                    let mut computer = Computer::from_vec(memory.clone());

                    computer.replace(1, noun);
                    computer.replace(2, verb);

                    computer.run();

                    match computer.read(0) {
                        19_690_720 => Some(100 * noun + verb),
                        _ => None,
                    }
                })
                .reduce(|| None, Option::or)
        })
        .reduce(|| None, Option::or)
        .expect("Valid noun / verb combination should exist!")
        .to_string()
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "1202 Program Alarm",
    part1: Some(part1),
    part2: Some(part2),
};
