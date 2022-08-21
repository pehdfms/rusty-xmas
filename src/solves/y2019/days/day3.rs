use crate::solves::year::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Path {
    min: Point,
    max: Point,
}

struct Wire {
    paths: Vec<Path>,
}

impl Wire {
    fn new(path: Vec<Point>) -> Wire {
        let mut paths: Vec<Path> = Vec::new();

        let mut current = &path[0];
        for point in &path[1..] {
            let (xmin, xmax) = (current.x.min(point.x), current.x.max(point.x));
            let (ymin, ymax) = (current.y.min(point.y), current.y.max(point.y));

            paths.push(Path {
                min: Point { x: xmin, y: ymin },
                max: Point { x: xmax, y: ymax },
            });

            current = point;
        }

        Wire { paths }
    }

    fn intersects(a: &Path, b: &Path) -> bool {
        if a.min.x == 0 && b.min.x == 0 && a.min.y == 0 && b.min.y == 0 {
            return false;
        }

        let x_intersects = a.min.x <= b.max.x && a.max.x >= b.min.x;
        let y_intersects = a.min.y <= b.max.y && a.max.y >= b.min.y;

        x_intersects && y_intersects
    }

    fn fixed_point(a: &Path, b: &Path) -> Point {
        let x = if a.min.x == a.max.x { a.min.x } else { b.min.x };
        let y = if a.min.y == a.max.y { a.min.y } else { b.min.y };

        Point { x, y }
    }

    fn intersections(&self, wire: &Wire) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        for i in &self.paths {
            for j in &wire.paths {
                if Self::intersects(i, j) {
                    points.push(Self::fixed_point(i, j));
                }
            }
        }

        points
    }

    pub fn closest_intersection(&self, wire: &Wire) -> i32 {
        self.intersections(wire)
            .iter()
            .map(|point| point.x.abs() + point.y.abs())
            .min()
            .expect("Closest intersection should exist.")
    }

    pub fn first_intersection(&self, wire: &Wire) -> i32 {
        let mut first = 0;
        let mut set = false;

        let mut a_dist = 0;
        for a in &self.paths {
            a_dist += a.max.x - a.min.x + a.max.y - a.min.y;
            let mut b_dist = 0;

            for b in &wire.paths {
                b_dist += a.max.x - a.min.x + a.max.y - a.min.y;

                let dist = a_dist + b_dist;

                println!("{dist}");
                println!("{}", Self::intersects(a, b));
                println!("{a:?}");
                println!("{b:?}");
                println!("{:?}", Self::fixed_point(a, b));
                println!();

                if Self::intersects(a, b) {
                    first = dist;
                    set = true;
                }
            }
        }

        first
    }
}

fn parse_wire(path: &str) -> Wire {
    let mut wire_path = vec![Point { x: 0, y: 0 }];

    path.split(',').for_each(|vector| {
        let direction = vector
            .chars()
            .next()
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
    let wires: Vec<Wire> = data.split_whitespace().map(parse_wire).collect();

    let a = &wires[0];
    let b = &wires[1];

    a.closest_intersection(b).to_string()
}

fn part2(data: String) -> String {
    let wires: Vec<Wire> = data.split_whitespace().map(parse_wire).collect();

    let a = &wires[0];
    let b = &wires[1];

    a.first_intersection(b).to_string()
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
fn part2_test() {
    assert_eq!(part2(String::from("R8,U5,L5,D3\nU7,R6,D4,L4")), "30");

    assert_eq!(
        part1(String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
        )),
        "610"
    );

    assert_eq!(
        part1(String::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        )),
        "410"
    );
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Crossed Wires",
    part1: Some(part1),
    part2: Some(part2),
};
