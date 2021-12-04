static INPUT_FILE: &'static str = include_str!("input.txt");
const BIT_LENGTH: usize = 12;

fn part1() {
    let numbers = INPUT_FILE
        .split_ascii_whitespace()
        .map(|str| return u16::from_str_radix(str, 2).unwrap())
        .collect::<Vec<u16>>();

    let mask: u16 = 0x0FFF;
    let half_total = numbers.len() / 2;
    let mut counts: [usize; BIT_LENGTH] = [0; BIT_LENGTH];

    for number in numbers {
        for position in 0..BIT_LENGTH {
            let mask = 1 << position;

            if ((mask & number) >> position) == 1 {
                counts[position] += 1;
            }
        }
    }

    // assemble back to an number
    let gamma: u16 = counts
        .iter()
        .enumerate()
        .fold(0, |accum, (position, count)| {
            let most_common_bit: u16 = if count > &half_total { 1 } else { 0 };
            return accum | (most_common_bit << position);
        });

    let epsilon = !gamma & mask;
    let multiplied = gamma as u64 * epsilon as u64;

    println!(
        "Gamma: {}, Epsilon: {}, Multipled: {}",
        gamma, epsilon, multiplied
    );
}

fn main() {
    part1();
}
