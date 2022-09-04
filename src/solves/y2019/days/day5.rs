use crate::solves::year::AdventOfCodeDay;

use super::intcode::Computer;

fn part1(data: &str) -> String {
    let mut computer = Computer::from_string(data);

    computer.add_input(1);

    computer.run();

    computer.read_outputs().last().unwrap().to_string()
}

fn part2(data: &str) -> String {
    let mut computer = Computer::from_string(data);

    computer.add_input(5);

    computer.run();

    computer.read_outputs().last().unwrap().to_string()
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Sunny with a Chance of Asteroids",
    part1: Some(part1),
    part2: Some(part2),
};
