use std::{cmp::max, collections::HashMap, fmt};

static INPUT_FILE: &'static str = include_str!("input.txt");

type Point = (u32, u32);

fn get_lines(input: &str) -> Vec<(Point, Point)> {
    return input
        .split("\n")
        .map(|line| {
            let point_strs = line.split(" -> ").collect::<Vec<&str>>();
            let point0 = point_strs[0]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let point1 = point_strs[1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();

            ((point0[0], point0[1]), (point1[0], point1[1]))
        })
        .collect();
}

#[derive(Debug, Default)]
struct SeaFloor {
    vents: HashMap<Point, u32>,
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (row, column) = self.vents.keys().fold((0, 0), |accum, point| {
            (max(accum.0, point.0), max(accum.1, point.1))
        });

        // usually I print row first, but this matches the output from the example
        for c in 0..=column {
            for r in 0..=row {
                let count = self.vents.get(&(r, c));

                match count {
                    Some(value) => write!(f, "{:1}", *value).unwrap(),
                    None => write!(f, ".").unwrap(),
                }
            }
            writeln!(f).unwrap();
        }

        write!(f, "") // this is gross, not sure how to properly combine fmt::Results
    }
}

fn compute_vent_intersections(input: &str, with_diagonals: bool) -> u64 {
    let lines = get_lines(input);
    let mut sea_floor = SeaFloor {
        ..Default::default()
    };

    lines.iter().for_each(|&((x1, y1), (x2, y2))| {
        if x1 == x2 {
            let small = if y1 > y2 { y2 } else { y1 };
            let large = if y1 > y2 { y1 } else { y2 };
            for y in small..=large {
                let point: Point = (x1, y);
                let val = sea_floor.vents.get(&point).unwrap_or(&0).clone();
                sea_floor.vents.insert(point, val + 1);
            }
        } else if y1 == y2 {
            let small = if x1 > x2 { x2 } else { x1 };
            let large = if x1 > x2 { x1 } else { x2 };

            for x in small..=large {
                let point: Point = (x, y1);
                let val = sea_floor.vents.get(&point).unwrap_or(&0).clone();
                sea_floor.vents.insert(point, val + 1);
            }
        } else if with_diagonals {
            let descending = (x1 >= x2 && y1 >= y2) || (x1 <= x2 && y1 <= y2);
            // doesn't matter if we use x or y since we know diagonals are the same
            let distance = (x1 as i64 - x2 as i64).abs() as u32;
            let leftmost_point = if x1 < x2 { (x1, y1) } else { (x2, y2) };

            for i in 0..=distance {
                let point = if descending {
                    (leftmost_point.0 + i, leftmost_point.1 + i)
                } else {
                    (leftmost_point.0 + i, leftmost_point.1 - i)
                };

                let val = sea_floor.vents.get(&point).unwrap_or(&0).clone();
                sea_floor.vents.insert(point, val + 1);
            }
        }
    });

    let intersections = sea_floor.vents.values().fold(0, |accum, val| {
        if *val >= 2 {
            return accum + 1;
        }

        accum
    });

    // println!("{}", sea_floor);

    return intersections;
}

fn part1(input: &str) -> u64 {
    let intersections = compute_vent_intersections(input, false);
    println!("[Part 1] Number of intersections: {}", intersections);
    return intersections;
}

fn part2(input: &str) -> u64 {
    let intersections = compute_vent_intersections(input, true);
    println!("[Part 2] Number of intersections: {}", intersections);
    return intersections;
}

fn main() {
    part1(INPUT_FILE); // 6710
    part2(INPUT_FILE); // 20121
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part_1_test_input() {
        let test_input = include_str!("test.txt");
        assert_eq!(part1(test_input), 5);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = include_str!("test.txt");
        assert_eq!(part2(test_input), 12);
    }
}
