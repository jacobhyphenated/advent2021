/*
Day 4: Giant Squid

Play a game of bingo with a giant squid.
You are given a list of numbers in the order they will be called and a list of bingo boards.

Part 1: Find the board that will win first.
Return the sum of all numbers on the board that were not called, multiplied by the last number called.

Part 2: Let the squid win by picking the board that wins last.
Return the score from part 1 of that last board when it wins.
*/

use std::fmt;
use std::fs;

#[derive(Clone)]
pub struct Tile {
    number: i32,
    called: bool
}

impl Tile {
    fn new(number: i32) -> Tile {
        Tile {number, called: false}
    }

    fn mark(&mut self) {
        self.called = true;
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.called {
            write!(f, "*{:2}*", self.number)
        } else {
            write!(f, "{:4}", self.number)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<Tile>>
}

impl Board {
    fn mark(&mut self, draw: &i32) {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                if &self.board[row][col].number == draw {
                    self.board[row][col].mark();
                    return;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        // check rows
        let row_winner = self.board.iter().any(|row| {
            row.iter().all(|tile| tile.called)
        });
        if row_winner {
            return true;
        }

        // check cols
        for c in 0..self.board.len() {
            let mut all_called = true;
            for r in 0..self.board.len() {
                if !self.board[r][c].called {
                    all_called = false;
                    break;
                }
            }
            if all_called {
                return true;
            }
        }
        return false;
    }

    fn sum_unmarked(&self) -> i32 {
        self.board.iter()
            .map(|row| row.iter()
                .filter(|tile| !tile.called)
                .map(|tile| tile.number)
                .sum::<i32>()
            )
            .sum()
    }
}

pub fn first_winner_score(mut boards: Vec<Board>, draws: &Vec<i32>) -> i32 {
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
            if board.is_winner() {
                return board.sum_unmarked() * draw;
            }
        }
    }
    return 0;
}

pub fn last_winner_score(mut boards: Vec<Board>, draws: &Vec<i32>) -> i32 {
    for draw in draws {
        let remaining = boards.len();
        for board in boards.iter_mut() {
            board.mark(draw);
            if remaining == 1 && board.is_winner() {
                return board.sum_unmarked() * draw;
            }
        }
        boards = boards.into_iter().filter(|board| !board.is_winner()).collect();
    }
    return 0;
}


fn parse_board(input: &str) -> Vec<Board> {
    input.split("\n\n")
        .map(|board_str| {
            Board { board: board_str.lines()
                .map(|line| line.trim().split_whitespace().map(|num| Tile::new(num.parse().unwrap())).collect())
                .collect()
            }
        })
        .collect()
}

pub fn read_input() -> (Vec<Board>, Vec<i32>) {
    let boards = fs::read_to_string("src/day4/boards.txt").expect("missing boards.txt");
    let draws = fs::read_to_string("src/day4/draws.txt").expect("missing draws.txt");
    (parse_board(&boards[..]), draws.split(",").map(|x| x.parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> (Vec<Board>, Vec<i32>) {
        let boards = "22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7";
    
        let draws = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        return (parse_board(boards), draws);
    }

    #[test]
    fn test_first_winner() {
        let (boards, draws) = get_test_data();
        assert_eq!(4512, first_winner_score(boards.clone(), &draws));
    }

    #[test]
    fn test_last_winner() {
        let (boards, draws) = get_test_data();
        assert_eq!(1924, last_winner_score(boards.clone(), &draws));
    }
}

