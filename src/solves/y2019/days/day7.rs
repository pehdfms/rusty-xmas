use crate::solves::{y2019::days::intcode::IntcodeComputer, year::AdventOfCodeDay};

fn non_repeating_permutations(array: &[i32]) -> Vec<Vec<i32>> {
    if array.len() == 2 {
        return vec![vec![array[0], array[1]], vec![array[1], array[0]]];
    }

    let first = array[0];
    non_repeating_permutations(&array[1..])
        .iter()
        .flat_map(|perm| {
            (0..perm.len() + 1)
                .map(|pos| {
                    let mut new_array = perm.clone();
                    new_array.insert(pos, first);
                    new_array
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect()
}

fn part1(data: String) -> String {
    let memory = IntcodeComputer::parse(&data);

    non_repeating_permutations(&vec![0, 1, 2, 3, 4])
        .iter()
        .map(|perm| {
            perm.iter().fold(0, |prev, phase_setting| {
                let mut computer = IntcodeComputer::new(memory.clone());

                computer.add_input(*phase_setting);
                computer.add_input(prev);
                computer.run();

                *computer
                    .read_outputs()
                    .first()
                    .expect("Output should exist")
            })
        })
        .max()
        .expect("Max thruster signal should exist")
        .to_string()
}

fn part2(data: String) -> String {
    data
}

#[test]
fn part1_test() {
    assert_eq!(
        part1(String::from(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        )),
        "43210"
    );

    assert_eq!(
        part1(String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
        )),
        "54321"
    );

    assert_eq!(
        part1(String::from(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        )),
        "65210"
    );
}

#[test]
fn part2_test() {
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Amplification Circuit",
    part1: Some(part1),
    part2: None,
};
