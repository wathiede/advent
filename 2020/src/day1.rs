//! --- Day 1: Report Repair ---
//! After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//!
//! Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
//!
//! -- Part Two ---
//! The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
//!
//! In your expense report, what is the product of the three entries that sum to 2020?

use aoc_runner_derive::{aoc, aoc_generator};

/// Reads text file containing one integer per line, and parses them into `Vec<u32>`.  Any
/// non-number will result in a panice.
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}

/// Finds pairs of numbers in `nums` that sum to 2020.  If no pairs are found, the function panics.
/// TODO(wathiede): make a version that sorts or uses a hash for finding the match to compare
/// benchmarks.
#[aoc(day1, part1)]
fn find_pair_2020(nums: &[u32]) -> u32 {
    for (idx, first) in nums.iter().enumerate() {
        for second in nums.iter().skip(idx + 1) {
            if first + second == 2020 {
                return first * second;
            }
        }
    }
    panic!("Couldn't find pair");
}

/// Finds triple of numbers in `nums` that sum to 2020.  If no triple is found, the function
/// panics.
#[aoc(day1, part2)]
fn find_triple_2020(nums: &[u32]) -> u32 {
    for (idx1, first) in nums.iter().enumerate() {
        for (idx2, second) in nums.iter().enumerate().skip(idx1 + 1) {
            for third in nums.iter().skip(idx2 + 1) {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    panic!("Couldn't find triple");
}
