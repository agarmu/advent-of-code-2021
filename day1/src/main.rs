static INPUT_FILE: &'static str = include_str!("input.txt");

fn get_numbers() -> Vec<i32> {
    let numbers = INPUT_FILE
        .split_ascii_whitespace()
        .map(|f| {
            f.parse::<i32>()
                .expect("Found a non-integer value in the input file")
        })
        .collect::<Vec<i32>>();

    return numbers;
}

fn part1() {
    let numbers = get_numbers();
    let mut increased = 0;

    numbers.iter().enumerate().for_each(|(index, _num)| {
        if index > 0 && numbers[index] > numbers[index - 1] {
            increased += 1;
        }
    });

    println!(
        "[Part 1] The number of increasing values are: {}",
        increased
    );
}

const WINDOW_SIZE: usize = 3;

// This was my original implementation. I didn't catch the simplification
// that you can just compare A and D, where:
//
// A + B + C < B + C + D
// simplifies to
// A < D
fn part2() {
    let numbers = get_numbers();
    let mut increased = 0;

    numbers.iter().enumerate().for_each(|(index, _num)| {
        let iteration_within_window = index >= WINDOW_SIZE && index < numbers.len();

        if iteration_within_window {
            let mut prev_sum = 0;
            let mut current_sum = 0;

            for n in 0..WINDOW_SIZE {
                prev_sum += numbers[index - n - 1];
                current_sum += numbers[index - n];
            }

            if current_sum > prev_sum {
                increased += 1;
            }
        }
    });

    println!(
        "[Part 2] The number of increasing values are: {}",
        increased
    );
}

fn main() {
    part1();
    part2();
}
