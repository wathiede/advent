//! --- Day 23: Crab Cups ---
//! The small crab challenges you to a game! The crab is going to mix up some cups, and you have to predict where they'll end up.
//!
//! The cups will be arranged in a circle and labeled clockwise (your puzzle input). For example, if your labeling were 32415, there would be five cups in the circle; going clockwise around the circle from the first cup, the cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.
//!
//! Before the crab starts, it will designate the first cup in your list as the current cup. The crab is then going to do 100 moves.
//!
//! Each move, the crab does the following actions:
//!
//! The crab picks up the three cups that are immediately clockwise of the current cup. They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
//! The crab selects a destination cup: the cup with a label equal to the current cup's label minus one. If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up. If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
//! The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
//! The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
//! For example, suppose your cup labeling were 389125467. If the crab were to do merely 10 moves, the following changes would occur:
//!
//! -- move 1 --
//! cups: (3) 8  9  1  2  5  4  6  7
//! pick up: 8, 9, 1
//! destination: 2
//!
//! -- move 2 --
//! cups:  3 (2) 8  9  1  5  4  6  7
//! pick up: 8, 9, 1
//! destination: 7
//!
//! -- move 3 --
//! cups:  3  2 (5) 4  6  7  8  9  1
//! pick up: 4, 6, 7
//! destination: 3
//!
//! -- move 4 --
//! cups:  7  2  5 (8) 9  1  3  4  6
//! pick up: 9, 1, 3
//! destination: 7
//!
//! -- move 5 --
//! cups:  3  2  5  8 (4) 6  7  9  1
//! pick up: 6, 7, 9
//! destination: 3
//!
//! -- move 6 --
//! cups:  9  2  5  8  4 (1) 3  6  7
//! pick up: 3, 6, 7
//! destination: 9
//!
//! -- move 7 --
//! cups:  7  2  5  8  4  1 (9) 3  6
//! pick up: 3, 6, 7
//! destination: 8
//!
//! -- move 8 --
//! cups:  8  3  6  7  4  1  9 (2) 5
//! pick up: 5, 8, 3
//! destination: 1
//!
//! -- move 9 --
//! cups:  7  4  1  5  8  3  9  2 (6)
//! pick up: 7, 4, 1
//! destination: 5
//!
//! -- move 10 --
//! cups: (5) 7  4  1  8  3  9  2  6
//! pick up: 7, 4, 1
//! destination: 3
//!
//! -- final --
//! cups:  5 (8) 3  7  4  1  9  2  6
//! In the above example, the cups' values are the labels as they appear moving clockwise around the circle; the current cup is marked with ( ).
//!
//! After the crab is done, what order will the cups be in? Starting after the cup labeled 1, collect the other cups' labels clockwise into a single string with no extra characters; each number except 1 should appear exactly once. In the above example, after 10 moves, the cups clockwise from 1 are labeled 9, 2, 6, 5, and so on, producing 92658374. If the crab were to complete all 100 moves, the order after cup 1 would be 67384529.
//!
//! Using your labeling, simulate 100 moves. What are the labels on the cups after cup 1?

use std::fmt;

use aoc_runner_derive::aoc;
struct Hand {
    cups: Vec<usize>,
    cur: usize,
    min: usize,
    max: usize,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, cup) in self.cups.iter().enumerate() {
            if i == self.cur {
                write!(f, "({}) ", cup)?;
            } else {
                write!(f, "{} ", cup)?;
            };
        }
        Ok(())
    }
}

impl Hand {
    fn new(s: &str) -> Hand {
        let cups: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *cups.iter().min().unwrap();
        let max = *cups.iter().max().unwrap();
        Hand {
            cups,
            cur: 0,
            min,
            max,
        }
    }

    fn play(&mut self, rounds: usize) -> String {
        (0..rounds).for_each(|i| {
            println!("-- move {} --", i + 1);
            self.step();
        });
        self.answer()
    }
    fn step(&mut self) {
        println!("{}", self);
        let cur = self.cups[self.cur];
        let mut pickups = Vec::new();
        let mut destination = self.cups[self.cur] - 1;
        let mut rm_idx = (self.cur + 1) % self.cups.len();
        (0..3).for_each(|_| {
            pickups.push(self.cups.remove(rm_idx));
            if rm_idx >= self.cups.len() {
                rm_idx -= self.cups.len();
            }
        });
        let cur = self.cups.iter().position(|i| i == &cur).unwrap();
        let next = self.cups[(cur + 1) % self.cups.len()];

        while pickups.contains(&destination) {
            destination -= 1;
        }
        if destination < self.min {
            destination = self.max;
            while pickups.contains(&destination) {
                destination -= 1;
            }
        }
        //dbg!(&pickups, &self.cups, destination);
        let idx = self.cups.iter().position(|i| i == &destination).unwrap();
        println!("pick up: {:?}", pickups);
        println!("destination: {}({})", destination, idx);
        println!("next destination: {}", next);

        pickups
            .into_iter()
            .rev()
            .for_each(|v| self.cups.insert(idx + 1, v));

        self.cur = self.cups.iter().position(|i| i == &next).unwrap();
    }
    fn answer(&self) -> String {
        let idx = self.cups.iter().position(|i| i == &1).unwrap();
        let s = self.cups[idx + 1..]
            .iter()
            .fold("".to_string(), |acc, c| format!("{}{}", acc, c));
        self.cups[..idx]
            .iter()
            .fold(s, |acc, c| format!("{}{}", acc, c))
    }
}

#[aoc(day23, part1)]
fn solution1(input: &str) -> String {
    let mut hand = Hand::new(input);
    hand.play(100)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "389125467";

    #[test]
    fn part1_10step() {
        let mut hand = Hand::new(INPUT);
        assert_eq!(hand.play(10), "92658374");
    }
    //#[test]
    fn part1() {
        assert_eq!(solution1(INPUT), "67384529");
    }
}
