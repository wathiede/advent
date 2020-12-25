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

//! --- Part Two ---
//! Due to what you can only assume is a mistranslation (you're not exactly fluent in Crab), you are quite surprised when the crab starts arranging many cups in a circle on your raft - one million (1000000) in total.
//!
//! Your labeling is still correct for the first few cups; after that, the remaining cups are just numbered in an increasing fashion starting from the number after the highest number in your list and proceeding one by one until one million is reached. (For example, if your labeling were 54321, the cups would be numbered 5, 4, 3, 2, 1, and then start counting up from 6 until one million is reached.) In this way, every number from one through one million is used exactly once.
//!
//! After discovering where you made the mistake in translating Crab Numbers, you realize the small crab isn't going to do merely 100 moves; the crab is going to do ten million (10000000) moves!
//!
//! The crab is going to hide your stars - one each - under the two cups that will end up immediately clockwise of cup 1. You can have them if you predict what the labels on those cups will be when the crab is finished.
//!
//! In the above example (389125467), this would be 934001 and then 159792; multiplying these together produces 149245887792.
//!
//! Determine which two cups will end up immediately clockwise of cup 1. What do you get if you multiply their labels together?

use std::fmt;
use std::ops::{Index, IndexMut, Range, RangeFrom};

use aoc_runner_derive::aoc;

use crate::debug_println;

trait Hand {
    fn play(&mut self, rounds: usize) {
        use std::time::{Duration, Instant};
        let start = Instant::now();
        let mut last_report = Instant::now();
        (0..rounds).for_each(|i| {
            debug_println!("-- move {} --", i + 1);
            if last_report.elapsed() > Duration::new(1, 0) {
                let elapsed = start.elapsed();
                let runtime = elapsed * rounds as u32 / i as u32;
                let eta = runtime - elapsed;

                println!(
                    "{} steps ({}%) in {}s, Estimated runtime {}s, ETA {}s",
                    i,
                    100 * i / rounds,
                    elapsed.as_secs_f32(),
                    runtime.as_secs_f32(),
                    eta.as_secs_f32(),
                );
                last_report = Instant::now();
            }
            self.step();
        });
    }
    fn part1_answer(&self) -> String;
    fn part2_answer(&self) -> usize;
    fn step(&mut self);
    fn test_cur_to_end(&self) -> Vec<usize>;
}

#[derive(Debug)]
struct TargetCup {
    val: usize,
    idx: usize,
}

#[derive(Debug)]
struct FastHand {
    idx_to_val: Vec<usize>,
    val_to_idx: Vec<usize>,
    cur: usize,
    min: usize,
    max: usize,
}

impl FastHand {
    fn new(s: &str) -> FastHand {
        let data: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();
        let mut idx_to_val = vec![0; data.len()];
        let mut val_to_idx = vec![0; data.len()];
        data.into_iter().enumerate().for_each(|(idx, val)| {
            val_to_idx[val - 1] = idx;
            idx_to_val[idx] = val;
        });
        FastHand {
            idx_to_val,
            val_to_idx,
            cur: 0,
            min,
            max,
        }
    }
    fn new_part2(s: &str) -> FastHand {
        let mut data: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *data.iter().min().unwrap();
        let mut max = *data.iter().max().unwrap();
        data.extend(max + 1..=1000000);
        max = 1000000;
        let mut idx_to_val = vec![0; data.len()];
        let mut val_to_idx = vec![0; data.len()];
        data.into_iter().enumerate().for_each(|(idx, val)| {
            val_to_idx[val - 1] = idx;
            idx_to_val[idx] = val;
        });
        FastHand {
            idx_to_val,
            val_to_idx,
            cur: 0,
            min,
            max,
        }
    }

    fn destination_cup_idx(&self, skip_vals: &[usize]) -> usize {
        let mut search_val = self.idx_to_val[self.cur] - 1;
        while skip_vals.contains(&search_val) {
            search_val -= 1;
        }

        if search_val < self.min {
            search_val = self.max;
        }
        while skip_vals.contains(&search_val) {
            search_val -= 1;
        }

        self.val_to_idx[search_val - 1]
    }
}

impl fmt::Display for FastHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, val) in self.idx_to_val.iter().enumerate() {
            if idx == self.cur {
                write!(f, "({}) ", val)?;
            } else {
                write!(f, "{} ", val)?;
            };
        }
        Ok(())
    }
}

#[derive(Debug)]
struct CircleVec<T> {
    data: Vec<T>,
}

impl<T> CircleVec<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

// TODO(wathiede): Index<Range> and Index<RangeFrom>?
impl<T> Index<usize> for CircleVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index % self.data.len()]
    }
}

impl<T> IndexMut<usize> for CircleVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.data.len();
        &mut self.data[index % len]
    }
}

impl Hand for FastHand {
    fn step(&mut self) {
        let n_cups = self.idx_to_val.len();
        let right_idx = self.cur + 1;
        let mut three = vec![0; 3];
        (0..3).for_each(|dst| three[dst] = self.idx_to_val[(right_idx + dst) % n_cups]);
        let dst_idx = self.destination_cup_idx(&three);

        // TODO
        debug_println!(
            "before {} three {:?} target {}",
            self,
            three,
            self.idx_to_val[dst_idx]
        );

        //dbg!(right, &dst);
        let end_idx = if dst_idx < right_idx {
            n_cups + dst_idx - 3 + 1
        } else {
            dst_idx + 1 - 3
        };
        debug_println!("moving window {}.. to {}..{}", dst_idx, right_idx, end_idx);
        (right_idx..end_idx)
            // Allow wrap around.
            .zip((right_idx + 3..).chain(0..))
            .for_each(|(dst_idx, src_idx)| {
                let src_idx = src_idx % n_cups;
                let dst_idx = dst_idx % n_cups;
                let v = self.idx_to_val[src_idx];
                debug_println!(
                    "moving {}({}) -> {}({})",
                    v,
                    src_idx,
                    self.idx_to_val[dst_idx],
                    dst_idx
                );
                self.idx_to_val[dst_idx] = v;
                self.val_to_idx[v - 1] = dst_idx;
            });
        (0..3).for_each(|i| {
            let dst_idx = (end_idx + i) % n_cups;
            self.idx_to_val[dst_idx] = three[i];
            self.val_to_idx[three[i] - 1] = dst_idx;
        });
        self.cur = (self.cur + 1) % n_cups;
        debug_println!(" after {}", self);
    }
    fn test_cur_to_end(&self) -> Vec<usize> {
        self.idx_to_val[self.cur..]
            .iter()
            .chain(self.idx_to_val[..self.cur].iter())
            .cloned()
            .collect()
    }
    fn part1_answer(&self) -> String {
        let one_idx = self.val_to_idx[1 - 1];
        let s = self.idx_to_val[one_idx + 1..]
            .iter()
            .fold("".to_string(), |acc, c| format!("{}{}", acc, c));
        self.idx_to_val[..one_idx]
            .iter()
            .fold(s, |acc, c| format!("{}{}", acc, c))
    }
    fn part2_answer(&self) -> usize {
        let one_idx = self.val_to_idx[1 - 1];
        self.idx_to_val[one_idx + 1] * self.idx_to_val[one_idx + 2]
    }
}

struct SlowHand {
    cups: Vec<usize>,
    cur: usize,
    min: usize,
    max: usize,
}

impl fmt::Display for SlowHand {
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

impl SlowHand {
    fn new(s: &str) -> SlowHand {
        let cups: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *cups.iter().min().unwrap();
        let max = *cups.iter().max().unwrap();
        SlowHand {
            cups,
            cur: 0,
            min,
            max,
        }
    }

    fn new_part2(s: &str) -> SlowHand {
        let mut cups: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *cups.iter().min().unwrap();
        let mut max = *cups.iter().max().unwrap();
        cups.extend(max + 1..1000000);
        max = 1000000;
        SlowHand {
            cups,
            cur: 0,
            min,
            max,
        }
    }
}

impl Hand for SlowHand {
    fn part1_answer(&self) -> String {
        let idx = self.cups.iter().position(|i| i == &1).unwrap();
        let s = self.cups[idx + 1..]
            .iter()
            .fold("".to_string(), |acc, c| format!("{}{}", acc, c));
        self.cups[..idx]
            .iter()
            .fold(s, |acc, c| format!("{}{}", acc, c))
    }
    fn step(&mut self) {
        debug_println!("{}", self);
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
        debug_println!("pick up: {:?}", pickups);
        debug_println!("destination: {}({})", destination, idx);
        debug_println!("next destination: {}", next);

        pickups
            .into_iter()
            .rev()
            .for_each(|v| self.cups.insert(idx + 1, v));

        self.cur = self.cups.iter().position(|i| i == &next).unwrap();
    }

    /// Return internal state in a way unit tests can use
    fn test_cur_to_end(&self) -> Vec<usize> {
        self.cups[self.cur..]
            .iter()
            .chain(self.cups[..self.cur].iter())
            .cloned()
            .collect()
    }
    fn part2_answer(&self) -> usize {
        let one = self.cups.iter().position(|n| n == &1).unwrap();
        self.cups[one + 1] * self.cups[one + 2]
    }
}

#[aoc(day23, part1)]
fn solution1(input: &str) -> String {
    let mut hand = SlowHand::new(input);
    hand.play(100);
    hand.part1_answer()
}

#[aoc(day23, part2)]
fn solution2(input: &str) -> usize {
    let mut hand = FastHand::new_part2(input);
    //hand.play(1_000);
    hand.play(10_000_000);
    hand.part2_answer()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "389125467";

    fn test_hand<H: Hand>(mut hand: H) {
        let want = vec![
            [3, 8, 9, 1, 2, 5, 4, 6, 7],
            [2, 8, 9, 1, 5, 4, 6, 7, 3],
            [5, 4, 6, 7, 8, 9, 1, 3, 2],
            [8, 9, 1, 3, 4, 6, 7, 2, 5],
            [4, 6, 7, 9, 1, 3, 2, 5, 8],
            [1, 3, 6, 7, 9, 2, 5, 8, 4],
            [9, 3, 6, 7, 2, 5, 8, 4, 1],
            [2, 5, 8, 3, 6, 7, 4, 1, 9],
            [6, 7, 4, 1, 5, 8, 3, 9, 2],
            [5, 7, 4, 1, 8, 3, 9, 2, 6],
            [8, 3, 7, 4, 1, 9, 2, 6, 5],
        ];
        want.iter().enumerate().for_each(|(step, want)| {
            assert_eq!(hand.test_cur_to_end(), want, "step0 {}", step);
            hand.step();
        });
    }
    #[test]
    fn slow_step() {
        let mut hand = SlowHand::new(INPUT);
        test_hand(hand);
    }

    #[test]
    fn fast_step() {
        let mut hand = FastHand::new(INPUT);
        test_hand(hand);
    }

    #[test]
    fn part1_10step_slow() {
        let mut hand = SlowHand::new(INPUT);
        hand.play(10);
        assert_eq!(hand.part1_answer(), "92658374");
    }

    #[test]
    fn part1_10step_fast() {
        let mut hand = FastHand::new(INPUT);
        hand.play(10);
        assert_eq!(hand.part1_answer(), "92658374");
    }
    #[test]
    fn part1() {
        assert_eq!(solution1(INPUT), "67384529");
    }
    #[test]
    fn part2() {
        assert_eq!(solution2("389125467"), 149245887792);
    }
}
