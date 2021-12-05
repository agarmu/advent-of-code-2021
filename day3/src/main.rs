static INPUT_FILE: &'static str = include_str!("input.txt");
const BIT_LENGTH: usize = 12;

fn get_numbers() -> Vec<u16> {
    return INPUT_FILE
        .split_ascii_whitespace()
        .map(|str| u16::from_str_radix(str, 2).unwrap())
        .collect::<Vec<u16>>();
}

fn part1() -> u64 {
    let numbers = get_numbers();
    let mask: u16 = 0x0FFF;
    let half_total = numbers.len() / 2;
    let mut counts: [usize; BIT_LENGTH] = [0; BIT_LENGTH];

    // tally up counts in each position
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
        "[Part 1] Gamma: {}, Epsilon: {}, Multipled: {}",
        gamma, epsilon, multiplied
    );

    return multiplied;
}

fn bit_matches(number: u16, position: usize, expected_bit: u16) -> bool {
    let mask = 1 << position;
    return ((number & mask) >> position) == (expected_bit as u16);
}

#[cfg(test)]
mod tests {
    use crate::bit_matches;

    #[test]
    fn bit_matches_binary() {
        assert_eq!(bit_matches(0b10010, 4, 1), true);
        assert_eq!(bit_matches(0b10010, 3, 0), true);
        assert_eq!(bit_matches(0b10010, 2, 0), true);
        assert_eq!(bit_matches(0b10010, 1, 1), true);
        assert_eq!(bit_matches(0b10010, 0, 0), true);
    }

    #[test]
    fn bit_does_not_match_binary() {
        assert_eq!(bit_matches(0b10010, 4, 0), false);
        assert_eq!(bit_matches(0b10010, 3, 1), false);
        assert_eq!(bit_matches(0b10010, 2, 1), false);
        assert_eq!(bit_matches(0b10010, 1, 0), false);
        assert_eq!(bit_matches(0b10010, 0, 1), false);
    }
}

fn part2() {
    enum BitSearch {
        LeastCommon,
        MostCommon,
    }

    fn compute_rating(search: BitSearch) -> u16 {
        let mut numbers = get_numbers();
        let mut counts: [usize; BIT_LENGTH];

        let mut pos = BIT_LENGTH;

        while numbers.len() > 1 {
            let mut current_total = numbers.len();
            let half_total = current_total as f32 / 2.0;

            // tally up counts in each position
            counts = [0; BIT_LENGTH];

            for number in &numbers {
                for position in 0..BIT_LENGTH {
                    let mask = 1 << position;

                    if ((mask & number) >> position) == 1 {
                        counts[position] += 1;
                    }
                }
            }

            let count = counts[pos - 1];
            let common_bit = match search {
                BitSearch::MostCommon => {
                    if (count as f32) >= half_total {
                        1
                    } else {
                        0
                    }
                }
                BitSearch::LeastCommon => {
                    if (count as f32) < half_total {
                        1
                    } else {
                        0
                    }
                }
            };

            numbers.retain(|number| {
                let matched = bit_matches(*number, pos - 1, common_bit);

                if !matched {
                    current_total -= 1;
                }

                return matched || current_total == 0;
            });

            pos -= 1;
        }

        assert_eq!(numbers.len(), 1);
        return numbers[0];
    }

    let oxygen_generator_rating = compute_rating(BitSearch::MostCommon);
    let co2_scrubber_rating = compute_rating(BitSearch::LeastCommon);

    println!("[Part 2]");
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    println!(
        "Multiplied together: {}",
        oxygen_generator_rating as u64 * co2_scrubber_rating as u64
    );
}

fn main() {
    part1();
    part2();
}
