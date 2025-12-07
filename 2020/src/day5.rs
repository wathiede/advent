//! --- Day 5: Binary Boarding ---
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
//!
//! The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of FBFBBFFRLR:
//!
//! Start by considering the whole range, rows 0 through 127.
//! F means to take the lower half, keeping rows 0 through 63.
//! B means to take the upper half, keeping rows 32 through 63.
//! F means to take the lower half, keeping rows 32 through 47.
//! B means to take the upper half, keeping rows 40 through 47.
//! B keeps rows 44 through 47.
//! F keeps rows 44 through 45.
//! The final F keeps the lower of the two, row 44.
//! The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
//!
//! For example, consider just the last 3 characters of FBFBBFFRLR:
//!
//! Start by considering the whole range, columns 0 through 7.
//! R means to take the upper half, keeping columns 4 through 7.
//! L means to take the lower half, keeping columns 4 through 5.
//! The final R keeps the upper of the two, column 5.
//! So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//!
//! Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
//!
//! Here are some other boarding passes:
//!
//! BFFFBBFRRR: row 70, column 7, seat ID 567.
//! FFFBBBFRRR: row 14, column 7, seat ID 119.
//! BBFFBBFRLL: row 102, column 4, seat ID 820.
//! As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
//!
//! --- Part Two ---
//! Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//!
//! What is the ID of your seat?

use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r_start = 0;
        let mut r_size = 128 / 2;
        let mut c_start = 0;
        let mut c_size = 8 / 2;
        s.chars().for_each(|c| match c {
            'F' => r_size /= 2,
            'B' => {
                r_start += r_size;
                r_size /= 2;
            }
            'L' => c_size /= 2,
            'R' => {
                c_start += c_size;
                c_size /= 2;
            }
            c => panic!("unexpected character '{}'", c),
        });
        Ok(Seat {
            row: r_start,
            column: c_start,
        })
    }
}

#[aoc_generator(day5, part1, wathiede)]
#[aoc_generator(day5, part2, wathiede)]
fn parse(input: &str) -> Vec<Seat> {
    input
        .split('\n')
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day5, part1, wathiede)]
fn solution1(seats: &[Seat]) -> u32 {
    seats.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part1, glenng)]
fn solution1_glenng(input: &str) -> u32 {
    input
        .split('\n')
        .map(|s| {
            s.chars().fold(0, |s, c| match c {
                'F' | 'L' => s << 1,
                'B' | 'R' => s << 1 | 1,
                _ => panic!("unexpected character '{}'", c),
            })
        })
        .map(|s| {
            let r = s >> 3;
            let c = s & 0b111;
            r * 8 + c
        })
        .max()
        .unwrap()
}

#[aoc(day5, part2, wathiede)]
fn solution2(seats: &[Seat]) -> u32 {
    let mut seat_ids: Vec<_> = seats.iter().map(|s| s.id()).collect();
    seat_ids.sort();
    let mut last_id = seat_ids[0];
    for id in &seat_ids[1..] {
        if id - last_id != 1 {
            return id - 1;
        }
        last_id = *id;
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
    static WANT: &'static [Seat] = &[
        Seat { row: 70, column: 7 },
        Seat { row: 14, column: 7 },
        Seat {
            row: 102,
            column: 4,
        },
    ];

    #[test]
    fn parse_seats() {
        assert_eq!(parse(INPUT), WANT);
    }

    #[test]
    fn id() {
        assert_eq!(
            WANT.iter().map(|s| s.id()).collect::<Vec<u32>>(),
            vec![567, 119, 820]
        );
    }
}
