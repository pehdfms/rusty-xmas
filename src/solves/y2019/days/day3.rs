use std::collections::HashSet;

use crate::solves::year::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Wire {
    points: HashSet<Point>,
}

impl Wire {
    fn new(path: Vec<Point>) -> Wire {
        let mut points: HashSet<Point> = HashSet::new();

        let mut current = &path[0];
        for point in &path[1..] {
            let xpair = (current.x, point.x);
            let ypair = (current.y, point.y);

            let (xmin, xmax) = (current.x.min(point.x), current.x.max(point.x));
            let (ymin, ymax) = (current.y.min(point.y), current.y.max(point.y));

            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    points.insert(Point { x, y });
                }
            }

            current = point;
        }

        points.remove(&Point { x: 0, y: 0 });

        Wire { points }
    }

    fn intersections<'a>(&'a self, wire: &'a Wire) -> Vec<&Point> {
        self.points.intersection(&wire.points).collect()
    }

    fn closest(points: Vec<&Point>) -> i32 {
        points
            .iter()
            .map(|point| point.x.abs() + point.y.abs())
            .min()
            .expect("Closest point should exist.")
    }
}

fn parse_wire(path: &str) -> Wire {
    let mut wire_path = vec![Point { x: 0, y: 0 }];

    path.split(',').for_each(|vector| {
        let direction = vector
            .chars()
            .nth(0)
            .expect("All input vectors should have a direction and distance");

        let distance: i32 = vector[1..]
            .parse()
            .expect("2nd character onwards in input vector should parse as number.");

        let current = wire_path
            .last()
            .expect("Vector starts with one element and we never remove from it.");

        let new_point = match direction {
            'U' => Point {
                x: current.x,
                y: current.y + distance,
            },
            'D' => Point {
                x: current.x,
                y: current.y - distance,
            },
            'L' => Point {
                x: current.x - distance,
                y: current.y,
            },
            'R' => Point {
                x: current.x + distance,
                y: current.y,
            },
            direction => panic!("Unexpected direction! Expected (UDLR) found {direction}"),
        };

        wire_path.push(new_point);
    });

    Wire::new(wire_path)
}

fn part1(data: String) -> String {
    let wires: Vec<Wire> = data
        .split_whitespace()
        .map(|path| parse_wire(path))
        .collect();

    let a = &wires[0];
    let b = &wires[1];

    let intersections = a.intersections(b);

    Wire::closest(intersections).to_string()
}

fn part2(data: String) -> String {
    data
}

#[test]
fn part1_test() {
    assert_eq!(part1(String::from("R8,U5,L5,D3\nU7,R6,D4,L4")), "6");

    assert_eq!(
        part1(String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
        )),
        "159"
    );
    assert_eq!(
        part1(String::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        )),
        "135"
    );
}

#[test]
fn part2_test() {}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Crossed Wires",
    part1: Some(part1),
    part2: None,
};
