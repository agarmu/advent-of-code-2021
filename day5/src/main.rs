use std::{collections::HashMap, fmt};

static INPUT_FILE: &'static str = include_str!("input.txt");

type Point = (u32, u32);
type Bounds = (u32, u32);

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
    bounds: Bounds,
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (row, column) = self.bounds;

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

fn part1(input: &str) -> u64 {
    let lines = get_lines(input);
    let mut sea_floor = SeaFloor {
        ..Default::default()
    };

    let mut maxX = 0;
    let mut maxY = 0;

    lines.iter().for_each(|&((x1, y1), (x2, y2))| {
        if x1 > maxX {
            maxX = x1;
        }

        if x2 > maxX {
            maxX = x2;
        }

        if y1 > maxY {
            maxY = y1;
        }

        if y2 > maxY {
            maxY = y2;
        }

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
        } else {
            println!("Skipping line ({}, {}) -> ({}, {})", x1, y1, x2, y2);
        }
    });

    sea_floor.bounds = (maxX, maxY);

    let intersections = sea_floor.vents.values().fold(0, |accum, val| {
        if *val >= 2 {
            return accum + 1;
        }

        accum
    });

    // dbg!(&sea_floor, lines);
    // println!("{}", lines);
    // println!("{}", &sea_floor);
    println!("Number of intersections: {}", intersections);
    // dbg!(intersections);
    return intersections;
}

fn main() {
    part1(INPUT_FILE);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part_1_test_input() {
        let test_input = include_str!("test.txt");
        assert_eq!(part1(test_input), 5);
    }

    #[test]
    fn ranges() {
        for x in 6..0 {
            assert_eq!(0, 1); // should fail if this runs
        }
    }

    // #[test]
    // fn part_2_test_input() {
    //     let test_input = include_str!("test.txt");
    //     assert_eq!(part2(test_input), 1924);
    // }
}
