use crate::solves::year::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Path {
    start: Point,
    end: Point,
}

impl Path {
    fn min(&self) -> Point {
        Point {
            x: self.start.x.min(self.end.x),
            y: self.start.y.min(self.end.y),
        }
    }

    fn max(&self) -> Point {
        Point {
            x: self.start.x.max(self.end.x),
            y: self.start.y.max(self.end.y),
        }
    }

    fn intersects(&self, b: &Self) -> bool {
        let min = (self.min(), b.min());
        let max = (self.max(), b.max());

        if min == (Point { x: 0, y: 0 }, Point { x: 0, y: 0 }) {
            return false;
        }

        let x_intersects = min.0.x <= max.1.x && max.0.x >= min.1.x;
        let y_intersects = min.0.y <= max.1.y && max.0.y >= min.1.y;

        x_intersects && y_intersects
    }

    const fn fixed_point(&self, b: &Self) -> Point {
        let x = if self.start.x == self.end.x {
            self.start.x
        } else {
            b.start.x
        };

        let y = if self.start.y == self.end.y {
            self.start.y
        } else {
            b.start.y
        };

        Point { x, y }
    }
}

#[derive(Debug)]
struct Wire {
    paths: Vec<Path>,
}

impl Wire {
    fn new(path: &[Point]) -> Self {
        let mut paths: Vec<Path> = Vec::new();

        let mut current = &path[0];
        for point in &path[1..] {
            paths.push(Path {
                start: Point {
                    x: current.x,
                    y: current.y,
                },
                end: Point {
                    x: point.x,
                    y: point.y,
                },
            });

            current = point;
        }

        Self { paths }
    }

    fn intersections(&self, wire: &Self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        for i in &self.paths {
            for j in &wire.paths {
                if i.intersects(j) {
                    let fixed_point = i.fixed_point(j);
                    if fixed_point.x == 0 && fixed_point.y == 0 {
                        continue;
                    }
                    points.push(fixed_point);
                }
            }
        }

        points
    }

    pub fn closest_intersection(&self, wire: &Self) -> i64 {
        self.intersections(wire)
            .iter()
            .map(|point| point.x.abs() + point.y.abs())
            .min()
            .expect("Closest intersection should exist.")
    }

    pub fn first_intersection(&self, wire: &Self) -> i64 {
        let mut adist = 0;
        let mut shortest = None;
        for a in &self.paths {
            adist += a.max().x - a.min().x + a.max().y - a.min().y;

            let mut bdist = 0;
            for b in &wire.paths {
                bdist += b.max().x - b.min().x + b.max().y - b.min().y;

                if a.intersects(b) {
                    let fixed_point = a.fixed_point(b);

                    if fixed_point.x == 0 && fixed_point.y == 0 {
                        continue;
                    }

                    let intersection_adist =
                        adist - (a.end.x - fixed_point.x).abs() - (a.end.y - fixed_point.y).abs();

                    let intersection_bdist =
                        bdist - (b.end.x - fixed_point.x).abs() - (b.end.y - fixed_point.y).abs();

                    let total_dist = intersection_adist + intersection_bdist;

                    shortest = match shortest {
                        Some(dist) if dist > total_dist => Some(total_dist),
                        Some(dist) => Some(dist),
                        None => Some(total_dist),
                    };
                }
            }
        }

        shortest.expect("Should have a shortest path")
    }
}

fn parse_wire(path: &str) -> Wire {
    let mut wire_path = vec![Point { x: 0, y: 0 }];

    path.split(',').for_each(|vector| {
        let direction = vector
            .chars()
            .next()
            .expect("All input vectors should have a direction and distance");

        let distance: i64 = vector[1..]
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

    Wire::new(&wire_path)
}

fn part1(data: &str) -> String {
    let wires: Vec<Wire> = data.split_whitespace().map(parse_wire).collect();

    let a = &wires[0];
    let b = &wires[1];

    a.closest_intersection(b).to_string()
}

fn part2(data: &str) -> String {
    let wires: Vec<Wire> = data.split_whitespace().map(parse_wire).collect();

    let a = &wires[0];
    let b = &wires[1];

    a.first_intersection(b).to_string()
}

#[test]
fn part1_test() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), "6");

    assert_eq!(
        part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "159"
    );

    assert_eq!(
        part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "135"
    );
}

#[test]
fn part2_test() {
    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), "30");

    assert_eq!(
        part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        "610"
    );

    assert_eq!(
        part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        "410"
    );
}

#[test]
#[should_panic(expected = "Unexpected direction!")]
fn should_panic_on_unexpected_direction() {
    part1("J2");
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Crossed Wires",
    part1: Some(part1),
    part2: Some(part2),
};
