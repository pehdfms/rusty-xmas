use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::solves::{y2019::days::intcode::Computer, year::AdventOfCodeDay};

fn non_repeating_permutations(array: &[i64]) -> Vec<Vec<i64>> {
    if array.len() == 2 {
        return vec![vec![array[0], array[1]], vec![array[1], array[0]]];
    }

    let first = array[0];
    non_repeating_permutations(&array[1..])
        .par_iter()
        .flat_map(|perm| {
            (0..=perm.len())
                .map(|pos| {
                    let mut new_array = perm.clone();
                    new_array.insert(pos, first);
                    new_array
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect()
}

fn part1(data: &str) -> String {
    let memory = Computer::parse(data);

    non_repeating_permutations(&[0, 1, 2, 3, 4])
        .par_iter()
        .map(|perm| {
            perm.iter().fold(0, |prev, phase_setting| {
                let mut computer = Computer::from_vec(memory.clone());

                computer.add_input(*phase_setting);
                computer.add_input(prev);
                computer.run();

                *computer
                    .read_outputs()
                    .first()
                    .expect("Output should exist!")
            })
        })
        .max()
        .expect("Max thruster signal should exist!")
        .to_string()
}

fn part2(data: &str) -> String {
    let memory = Computer::parse(data);

    non_repeating_permutations(&[5, 6, 7, 8, 9])
        .par_iter()
        .map(|perm| {
            let mut amplifiers = vec![];
            (0..5).for_each(|_| amplifiers.push(Computer::from_vec(memory.clone())));

            perm.iter().enumerate().for_each(|(idx, phase_setting)| {
                amplifiers[idx].add_input(*phase_setting);
            });

            let mut last_output = 0;
            // I would really like to use cycle with fold here, but
            // a) fold doesn't support early exit afaik
            // b) iter_mut would require cloning each interpreter, not ideal
            loop {
                for amplifier in &mut amplifiers {
                    if amplifier.finished() {
                        return last_output;
                    }

                    amplifier.add_input(last_output);
                    amplifier.run();

                    last_output = *amplifier
                        .read_outputs()
                        .last()
                        .expect("Output should exist!");
                }
            }
        })
        .max()
        .expect("Max thruster signal should exist!")
        .to_string()
}

#[test]
fn part1_test() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        "43210"
    );

    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        "54321"
    );

    assert_eq!(
        part1(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        ),
        "65210"
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        part2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\n27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        "139629729"
    );

    assert_eq!(
        part2(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\n-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\n53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        ),
        "18216"
    );
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Amplification Circuit",
    part1: Some(part1),
    part2: Some(part2),
};
