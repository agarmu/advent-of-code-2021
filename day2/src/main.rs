static INPUT_FILE: &'static str = include_str!("input.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(format!("Could not convert '{}' to a Direction", s)),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: i32,
}

fn get_commands() -> impl Iterator<Item = Command> {
    INPUT_FILE.split('\n').map(|line| {
        let values: Vec<&str> = line.split_ascii_whitespace().collect();
        let direction: Direction = values[0].parse().unwrap();
        let amount: i32 = values[1].parse().unwrap();

        Command { direction, amount }
    })
}

fn part1() {
    let mut x = 0;
    let mut y = 0;

    get_commands().for_each(|command| match command.direction {
        Direction::Up => y -= command.amount,
        Direction::Down => y += command.amount,
        Direction::Forward => x += command.amount,
    });

    println!(
        "[Part 1] Position is ({}, {}). Multiplied together: {}",
        x,
        y,
        x * y
    );
}

fn part2() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    get_commands().for_each(|command| match command.direction {
        Direction::Up => aim -= command.amount,
        Direction::Down => aim += command.amount,
        Direction::Forward => {
            x += command.amount;
            y += aim * command.amount;
        }
    });

    println!(
        "[Part 2] Position is ({}, {}). Multiplied together: {}",
        x,
        y,
        x * y
    );
}

fn main() {
    part1();
    part2();
}
