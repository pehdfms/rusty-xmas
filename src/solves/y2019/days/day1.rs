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
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2
    // to get 2.
    assert_eq!(part1(String::from("12")), "2");

    // For a mass of 14, dividing by 3 and rounding down still yields 4, so
    // the fuel required is also 2.
    assert_eq!(part1(String::from("14")), "2");

    // For a mass of 1969, the fuel required is 654.
    assert_eq!(part1(String::from("1969")), "654");

    //For a mass of 100756, the fuel required is 33583.
    assert_eq!(part1(String::from("100756")), "33583");
}

#[test]
fn part2_test() {
    // A module of mass 14 requires 2 fuel. This fuel requires no further
    // fuel (2 divided by 3 and rounded down is 0, which would call for a
    // negative fuel), so the total fuel required is still just 2.
    assert_eq!(part2(String::from("14")), "2");

    // At first, a module of mass 1969 requires 654 fuel. Then, this fuel
    // requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel,
    // which requires 21 fuel, which requires 5 fuel, which requires no
    // further fuel. So, the total fuel required for a module of mass 1969 is
    // 654 + 216 + 70 + 21 + 5 = 966.
    assert_eq!(part2(String::from("1969")), "966");

    // The fuel required by a module of mass 100756 and its fuel is:
    // 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    assert_eq!(part2(String::from("100756")), "50346");
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "The Tyranny of the Rocket Equation",
    part1: Some(part1),
    part2: Some(part2),
};
