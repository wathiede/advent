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

/// TODO(wathiede): redo based on this sentence from glenng:
/// `So a circular linked list containing 2,1,3 would be [3,1,2]`
#[derive(Debug)]
struct FastHand {
    // A cup labeled `1` will be represented by the index 0, in that cell will be the index of cup
    // clockwise to `1`.
    // Stores the next cup as indexed value (i.e. label-1).
    cups: Vec<usize>,
    cur: Cup,
    min: usize,
    max: usize,
}

/// Stores the label of a cup.  Use `as_idx` to compute the index into FastHand.cups. Use
/// `from_idx` to build a `Cup` from a given index into FastHand.cups.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Cup(usize);

impl Cup {
    fn new(val: usize) -> Cup {
        Cup(val)
    }
    fn from_idx(idx: usize) -> Cup {
        Cup(idx + 1)
    }
    fn as_idx(&self) -> usize {
        self.0 - 1
    }
}

impl FastHand {
    fn new(s: &str) -> FastHand {
        let data: Vec<_> = s.bytes().map(|s| (s - b'0') as usize).collect();
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();
        let mut cups = vec![0; max];
        let mut last = 0;
        data.windows(2).for_each(|nums| {
            let cur_cup = Cup::new(nums[0]);
            let next_cup = Cup::new(nums[1]);
            last = next_cup.as_idx();
            cups[cur_cup.as_idx()] = next_cup.as_idx();
        });
        let cur = Cup(data[0]);
        cups[last] = cur.as_idx();
        FastHand {
            cups,
            cur,
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
        let mut cups = vec![0; max];
        let mut last = 0;
        data.windows(2).for_each(|nums| {
            let cur_cup = Cup::new(nums[0]);
            let next_cup = Cup::new(nums[1]);
            last = next_cup.as_idx();
            cups[cur_cup.as_idx()] = next_cup.as_idx();
        });
        let cur = Cup(data[0]);
        cups[last] = cur.as_idx();
        FastHand {
            cups,
            cur,
            min,
            max,
        }
    }

    fn destination(&self, skip_vals: &[Cup]) -> Cup {
        let mut search_val = Cup::new(self.cur.0 - 1);
        while skip_vals.contains(&search_val) {
            search_val = Cup::new(search_val.0 - 1);
        }
        if search_val.0 < self.min {
            search_val = Cup::new(self.max);
        }
        while skip_vals.contains(&search_val) {
            search_val = Cup::new(search_val.0 - 1);
        }
        search_val
    }

    fn next(&self, c: Cup) -> Cup {
        //dbg!(c.as_idx(), self.cups[c.as_idx()]);
        Cup::from_idx(self.cups[c.as_idx()])
    }
}

impl fmt::Display for FastHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cur = self.cur;
        write!(f, "({}) ", cur.0)?;

        for _ in 1..self.cups.len() {
            cur = Cup::from_idx(self.cups[cur.as_idx()]);
            write!(f, "{} ", cur.0)?;
        }
        Ok(())
    }
}

impl Hand for FastHand {
    fn step(&mut self) {
        let mut cur = self.cur;
        let three: Vec<_> = (0..3)
            .map(|_| {
                cur = self.next(cur);
                cur
            })
            .collect();
        let dst = self.destination(&three);
        debug_println!(
            "cur {} cups {} three {:?} destination {:?}",
            self.cur.0,
            self,
            three,
            dst
        );
        debug_println!("cups (raw) {:?}", self.cups);

        // Cur points to whatever end of three used to.
        self.cups[self.cur.as_idx()] = self.cups[three[2].as_idx()];

        // End of three points to whatever dst used to point to.
        self.cups[three[2].as_idx()] = self.cups[dst.as_idx()];

        // Dst points to the beginning of three.
        self.cups[dst.as_idx()] = three[0].as_idx();

        // Cur points to whatever is next in the circle.
        self.cur = self.next(self.cur);
    }
    fn test_cur_to_end(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.cups.len());
        let mut cur = self.cur;
        (0..self.cups.len()).for_each(|_| {
            res.push(cur.0);
            cur = Cup::from_idx(self.cups[cur.as_idx()]);
        });
        res
    }
    fn part1_answer(&self) -> String {
        let mut cur = Cup::new(1);
        let mut s = "".to_string();
        for _ in 1..self.cups.len() {
            cur = self.next(cur);
            s = format!("{}{}", s, cur.0);
        }

        s
    }
    fn part2_answer(&self) -> usize {
        let one = Cup::new(1);
        let v1 = self.next(one);
        let v2 = self.next(v1);
        v1.0 * v2.0
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
    #[allow(dead_code)]
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
    let mut hand = FastHand::new(input);
    hand.play(100);
    hand.part1_answer()
}

#[aoc(day23, part2)]
fn solution2(input: &str) -> usize {
    let mut hand = FastHand::new_part2(input);
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
        let hand = SlowHand::new(INPUT);
        test_hand(hand);
    }

    #[test]
    fn fast_step() {
        let hand = FastHand::new(INPUT);
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
    // This is too slow in debug mode due to debug_println, build in release to run.
    #[cfg(not(debug_assertions))]
    #[test]
    fn part2() {
        assert_eq!(solution2("389125467"), 149245887792);
    }
}
