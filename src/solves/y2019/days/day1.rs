use crate::solves::year::AdventOfCodeDay;

fn get_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part1(data: String) -> String {
    let result: i32 = data
        .split_whitespace()
        .map(|n| {
            let mass: i32 = n.parse().expect("All splits should parse as number.");
            get_fuel(mass)
        })
        .sum();

    result.to_string()
}

fn part2(data: String) -> String {
    let result: i32 = data
        .split_whitespace()
        .map(|n| {
            let mut mass: i32 = n.parse().expect("All splits should parse as number.");
            let mut total_cost = 0;

            while mass > 0 {
                let result = get_fuel(mass);
                if result < 0 {
                    break;
                }

                total_cost += result;
                mass = result;
            }

            total_cost
        })
        .sum();

    result.to_string()
}

#[test]
fn part1_test() {
    assert_eq!(part1(String::from("12")), "2");

    assert_eq!(part1(String::from("14")), "2");

    assert_eq!(part1(String::from("1969")), "654");

    assert_eq!(part1(String::from("100756")), "33583");
}

#[test]
fn part2_test() {
    assert_eq!(part2(String::from("14")), "2");

    assert_eq!(part2(String::from("1969")), "966");

    assert_eq!(part2(String::from("100756")), "50346");
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "The Tyranny of the Rocket Equation",
    part1: Some(part1),
    part2: Some(part2),
};
