//! --- Day 4: Giant Squid ---
//! You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.
//!
//! Maybe it wants to play bingo?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
//!
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//! After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! Finally, 24 is drawn:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).
//!
//! The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
//!
//! To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
//!
//! --- Part Two ---
//! On the other hand, it might be wise to try a different strategy: let the giant squid win.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
//!
//! Figure out which board will win last. Once it wins, what would its final score be?

use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use ansi_term::Color::Green;
use anyhow::Result;
use aoc_runner_derive::aoc;
use thiserror::Error;

#[derive(Debug, Default)]
struct Game {
    numbers: Vec<u64>,
    boards: Vec<Board>,
    skip_boards: HashSet<usize>,
}

#[derive(Debug, Error)]
enum GameError {
    #[error("couldn't parse number {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("couldn't parse board {0}")]
    BoardError(#[from] BoardError),
}

impl Game {
    // If return not None, it contains a winning board
    fn apply_number(&mut self, number: u64) -> Option<&Board> {
        for b in &mut self.boards {
            b.mark(number);
            if b.is_bingo() {
                return Some(b);
            }
        }
        None
    }
    // If return not None, it contains a winning board. This will remove winning boards until only
    // one remains.
    fn apply_number_part2(&mut self, number: u64) -> Option<&Board> {
        let num_boards = self.boards.len();
        for (idx, b) in self.boards.iter_mut().enumerate() {
            if self.skip_boards.contains(&idx) {
                continue;
            }
            b.mark(number);
            if b.is_bingo() {
                self.skip_boards.insert(idx);
                if self.skip_boards.len() == num_boards {
                    return Some(b);
                }
            }
        }
        None
    }
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("\n\n");
        let numbers = it
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<_, ParseIntError>>()?;
        let boards: Vec<_> = it.map(|s| s.parse()).collect::<Result<_, BoardError>>()?;
        Ok(Game {
            numbers,
            boards,
            skip_boards: Default::default(),
        })
    }
}

#[derive(Default)]
struct Board {
    numbers: HashMap<(usize, usize), u64>,
    marked: HashSet<(usize, usize)>,
}

#[derive(Debug, Error)]
enum BoardError {
    #[error("couldn't parse number {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl Board {
    fn is_bingo(&self) -> bool {
        for y in 0..5 {
            if (0..5).all(|x| self.marked.contains(&(x, y))) {
                return true;
            }
        }
        for x in 0..5 {
            if (0..5).all(|y| self.marked.contains(&(x, y))) {
                return true;
            }
        }
        false
    }
    fn sum_uncovered(&self) -> u64 {
        self.numbers
            .iter()
            .map(|((x, y), v)| {
                if !self.marked.contains(&(*x, *y)) {
                    *v
                } else {
                    0
                }
            })
            .sum()
    }
    // Returns true if board has num.
    fn mark(&mut self, num: u64) -> bool {
        for ((x, y), v) in self.numbers.iter() {
            if *v == num {
                self.marked.insert((*x, *y));
                return true;
            }
        }
        false
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        for y in 0..5 {
            for x in 0..5 {
                if self.marked.contains(&(x, y)) {
                    let v = format!("{:3}", self.numbers[&(x, y)]);
                    write!(f, "{}", Green.bold().paint(v))?;
                } else {
                    write!(f, "{:3}", self.numbers[&(x, y)])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = BoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<Vec<_>> = s
            .split('\n')
            .map(|l| {
                l.split(' ')
                    // Remove the double space that happens before single digit cells.
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse())
                    .collect::<Result<_, ParseIntError>>()
            })
            .collect::<Result<_, ParseIntError>>()?;
        let numbers: HashMap<_, _> = numbers
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
            .collect();
        Ok(Board {
            numbers,
            marked: Default::default(),
        })
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> Result<u64> {
    let mut g: Game = input.parse()?;
    let numbers = g.numbers.clone();
    for n in numbers {
        if let Some(b) = g.apply_number(n) {
            println!("winning board {:?}", b);
            return Ok(n as u64 * b.sum_uncovered());
        }
    }
    unreachable!("We should have had a winner by now");
}

#[aoc(day4, part2)]
fn part2(input: &str) -> Result<u64> {
    let mut g: Game = input.parse()?;
    let numbers = g.numbers.clone();
    for n in numbers {
        if let Some(b) = g.apply_number_part2(n) {
            println!("winning board {:?}", b);
            return Ok(n as u64 * b.sum_uncovered());
        }
    }
    unreachable!("We should have had a winner by now");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() -> Result<()> {
        let input = r#"
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 "#
        .trim();
        let mut b = Board::from_str(input)?;
        assert!(!b.is_bingo());
        assert!(!b.mark(100));

        for num in &[7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
            assert!(b.mark(*num));
        }
        assert!(b.is_bingo());
        assert_eq!(b.sum_uncovered(), 188);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
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
 2  0 12  3  7
 "#
        .trim();
        assert_eq!(part1(input)?, 4512);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
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
 2  0 12  3  7
 "#
        .trim();
        assert_eq!(part2(input)?, 1924);
        Ok(())
    }
}
