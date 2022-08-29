use crate::solves::year::AdventOfCodeDay;

use super::intcode::IntcodeComputer;

fn part1(data: String) -> String {
    let mut computer = IntcodeComputer::from_string(&data);

    computer.set_input(1);

    computer.run();

    computer.read_outputs().last().unwrap().to_string()
}

fn part2(data: String) -> String {
    let mut computer = IntcodeComputer::from_string(&data);

    computer.set_input(5);

    computer.run();

    computer.read_outputs().last().unwrap().to_string()
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Sunny with a Chance of Asteroids",
    part1: Some(part1),
    part2: Some(part2),
};
