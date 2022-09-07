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

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::from_vec(memory.clone());

            computer.replace(1, noun);
            computer.replace(2, verb);

            computer.run();

            if computer.read(0) == 19_690_720 {
                return (100 * noun + verb).to_string();
            }
        }
    }

    panic!("Couldn't find result in sample!");
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "1202 Program Alarm",
    part1: Some(part1),
    part2: Some(part2),
};
