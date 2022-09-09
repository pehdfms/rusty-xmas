use std::collections::HashMap;

use crate::solves::year::AdventOfCodeDay;

fn parse_data(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    data.split_whitespace().for_each(|obj| {
        let parts: Vec<&str> = obj.split(')').collect();

        nodes.entry(parts[1]).or_default();
        nodes.entry(parts[0]).or_default().push(parts[1]);
    });

    nodes
}

fn count_orbits(count: u64, node: &str, map: &HashMap<&str, Vec<&str>>) -> u64 {
    let children = map.get(node).expect("Node should exist");

    count
        + children
            .iter()
            .map(|node| count_orbits(count + 1, node, map))
            .sum::<u64>()
}

fn shortest_path(node: &str, map: &HashMap<&str, Vec<&str>>) -> (Option<i64>, Option<i64>) {
    let mut result = (None, None);
    map[node].iter().for_each(|node| {
        let child_result = shortest_path(node, map);

        match child_result {
            (Some(_), Some(_)) => result = child_result,
            (Some(a), _) => result.0 = Some(a + 1),
            (_, Some(b)) => result.1 = Some(b + 1),
            (None, None) => (),
        }
    });

    if node == "YOU" {
        result.0 = Some(-1);
    }

    if node == "SAN" {
        result.1 = Some(-1);
    }

    result
}

fn part1(data: &str) -> String {
    let graph = parse_data(data);

    count_orbits(0, "COM", &graph).to_string()
}

fn part2(data: &str) -> String {
    let graph = parse_data(data);

    let result = shortest_path("COM", &graph);
    (result.0.unwrap() + result.1.unwrap()).to_string()
}

#[test]
fn part1_test() {
    assert_eq!(
        part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
        "42"
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
        "4"
    );
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Universal Orbit Map",
    part1: Some(part1),
    part2: Some(part2),
};
