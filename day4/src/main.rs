use std::{
    fmt,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Default)]
struct BingoCell {
    number: u8,
    called: bool,
}

#[derive(Default)]
struct BingoBoard {
    board: [[BingoCell; 5]; 5],
    bingo: bool,
}

#[derive(Debug)]
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
                    write!(f, "{:>4} ", cell.number).unwrap();
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
    type Output = BingoCell;
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
        let board = Self {
            ..Default::default()
        };

        return Ok(board);
    }
}

impl BingoBoard {
    fn draw(&mut self, number: u8) -> bool {
        // iterate over numbers
        // self.board
        false
    }
}

fn main() {
    let mut board = BingoBoard {
        ..Default::default()
    };

    println!("{}", board);
    dbg!(board);
}
