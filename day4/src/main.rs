use std::{
    fmt,
    ops::{Index, IndexMut},
    str::FromStr,
};

static INPUT_FILE: &'static str = include_str!("input.txt");
const BOARD_SIZE: usize = 5;

#[derive(Debug, Default, Clone, Copy)]
struct BoardCell {
    number: u8,
    called: bool,
}

#[derive(Default)]
struct BingoBoard {
    board: [[BoardCell; BOARD_SIZE]; BOARD_SIZE],
    bingo: bool,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for cell in row {
                if cell.called {
                    write!(f, "[{:>2}] ", cell.number).unwrap();
                } else {
                    write!(f, " {:>2}  ", cell.number).unwrap();
                }
            }

            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", self)
    }
}

impl Index<Position> for BingoBoard {
    type Output = BoardCell;
    fn index(&self, index: Position) -> &Self::Output {
        &self.board[index.row][index.column]
    }
}

impl IndexMut<Position> for BingoBoard {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.board[index.row][index.column]
    }
}

impl FromStr for BingoBoard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board: [[BoardCell; BOARD_SIZE]; BOARD_SIZE] = [[BoardCell {
            ..Default::default()
        }; BOARD_SIZE]; BOARD_SIZE];

        for (row, row_str) in s.split("\n").enumerate() {
            for (column, cell_str) in row_str.split_ascii_whitespace().enumerate() {
                board[row][column].number = cell_str.parse().unwrap();
            }
        }

        Ok(BingoBoard {
            board,
            ..Default::default()
        })
    }
}

#[allow(dead_code)]
struct WinningConditions {
    number: u8,
    position: Position,
    vertical: bool,
    horizontal: bool,
    ascending: bool,
    descending: bool,
}

impl BingoBoard {
    fn check_win(&self, from: Position) -> Option<WinningConditions> {
        let vertical = (0..BOARD_SIZE).all(|r| self[Position { row: r, ..from }].called);
        let horizontal = (0..BOARD_SIZE).all(|c| self[Position { column: c, ..from }].called);

        // Diagonals don't count...
        let descending = false;
        let ascending = false;

        // let descending = from.column == from.row
        //     && (0..BOARD_SIZE).all(|i| self[Position { column: i, row: i }].called);

        // let ascending = (from.row + from.column) == (BOARD_SIZE - 1)
        //     && (0..BOARD_SIZE).all(|i| {
        //         self[Position {
        //             column: i,
        //             row: BOARD_SIZE - 1 - i,
        //         }]
        //         .called
        //     });

        if vertical || horizontal || descending || ascending {
            return Some(WinningConditions {
                number: self[from].number,
                position: from,
                vertical,
                horizontal,
                descending,
                ascending,
            });
        }

        None
    }

    fn draw(&mut self, number: u8) -> Option<WinningConditions> {
        let mut position: Option<Position> = None;

        for row in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                if self.board[row][column].number == number {
                    position = Some(Position { row, column });
                }
            }
        }

        match position {
            Some(position) => {
                self[position].called = true;
                self.check_win(position)
            }
            None => None,
        }
    }

    fn sum_unmarked(&self) -> u64 {
        let mut sum: u64 = 0;

        for row in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                let cell = self.board[row][column];

                if !cell.called {
                    sum += cell.number as u64;
                }
            }
        }

        sum
    }
}

fn get_draws(input: &str) -> Vec<u8> {
    return input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(",")
        .map(|draw| draw.parse().unwrap())
        .collect();
}

fn get_boards(input: &str) -> Vec<BingoBoard> {
    let boards_string = input.split("\n").skip(2).collect::<Vec<&str>>().join("\n");

    return boards_string
        .split("\n\n")
        .map(|board| BingoBoard::from_str(board).unwrap())
        .collect::<Vec<BingoBoard>>();
}

fn part1(input: &str) -> u64 {
    let draws = get_draws(input);
    let mut boards = get_boards(input);

    for draw in &draws {
        for (board_number, board) in boards.iter_mut().enumerate() {
            match board.draw(*draw) {
                Some(_winning_conditions) => {
                    println!("[Part 1]\n-------------------------------------");
                    println!(
                        "The first winning board is {}, with the call of {}. Final board:\n{}",
                        board_number, draw, board
                    );

                    let sum = board.sum_unmarked();
                    let score = sum * *draw as u64;

                    println!("Sum of unmarked numbers: {}; Final score: {}", sum, score);
                    return score;
                }
                _ => (),
            }
        }
    }

    println!("No winners, looks like we didn't handle the input correctly.");
    dbg!(draws, boards);
    return 0;
}

fn part2(input: &str) -> u64 {
    let draws = get_draws(input);
    let mut boards = get_boards(input);
    let final_board_count = boards.len();
    let mut winning_boards = 0;

    for draw in &draws {
        for (board_number, board) in boards.iter_mut().enumerate() {
            if board.bingo {
                continue;
            }

            match board.draw(*draw) {
                Some(_winning_conditions) => {
                    board.bingo = true;
                    winning_boards += 1;

                    // last board
                    if winning_boards == final_board_count {
                        println!("[Part 2]\n-------------------------------------");
                        println!(
                            "The last winning board is {}, with the call of {}. Final board:\n{}",
                            board_number, draw, board
                        );

                        let sum = board.sum_unmarked();
                        let score = sum * *draw as u64;

                        println!("Sum of unmarked numbers: {}; Final score: {}", sum, score);
                        return score;
                    }
                }
                _ => (),
            }
        }
    }

    println!("No winners, looks like we didn't handle the input correctly.");
    dbg!(draws, boards);
    return 0;
}

fn main() {
    part1(INPUT_FILE);
    part2(INPUT_FILE);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part_1_test_input() {
        let test_input = include_str!("test.txt");
        assert_eq!(part1(test_input), 4512);
    }

    #[test]
    fn part_2_test_input() {
        let test_input = include_str!("test.txt");
        assert_eq!(part2(test_input), 1924);
    }
}
