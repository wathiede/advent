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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Usage: 1 <path to expense report>"))?;
    let nums = parse(path)?;
    let pair = find_pair_2020(&nums).ok_or(anyhow!("Couldn't find pairs summing to 2020"))?;
    println!("Product of {} x {} = {}", pair.0, pair.1, pair.0 * pair.1);
    let triple = find_triple_2020(&nums).ok_or(anyhow!("Couldn't find triples summing to 2020"))?;
    println!(
        "Product of {} x {} x {} = {}",
        triple.0,
        triple.1,
        triple.2,
        triple.0 * triple.1 * triple.2
    );
    Ok(())
}

/// Finds pairs of numbers in `nums` that sum to 2020.  If no pairs are found, `None` is returned.
fn find_pair_2020(nums: &Vec<u32>) -> Option<(u32, u32)> {
    for (idx, first) in nums.iter().enumerate() {
        for second in nums.iter().skip(idx + 1) {
            if first + second == 2020 {
                return Some((*first, *second));
            }
        }
    }
    None
}

/// Finds triple of numbers in `nums` that sum to 2020.  If no triple is found, `None` is returned.
fn find_triple_2020(nums: &Vec<u32>) -> Option<(u32, u32, u32)> {
    for (idx1, first) in nums.iter().enumerate() {
        for (idx2, second) in nums.iter().enumerate().skip(idx1 + 1) {
            for third in nums.iter() {
                if first + second + third == 2020 {
                    return Some((*first, *second, *third));
                }
            }
        }
    }
    None
}

/// Reads text file containing one integer per line, and parses them into `Vec<u32>`.  Any
/// non-number will result in an error returned.
fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<u32>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut nums = Vec::new();
    for line in f.lines() {
        let num: u32 = line?.parse()?;
        nums.push(num)
    }
    Ok(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let nums = parse("src/bin/1-test.txt").expect("failed to parse");
        assert_eq!(nums, vec![1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    fn test_find_pair_2020() {
        let nums = parse("src/bin/1-test.txt").expect("failed to parse");
        assert_eq!(find_pair_2020(&nums), Some((1721, 299)));
    }

    #[test]
    fn test_find_triple_2020() {
        let nums = parse("src/bin/1-test.txt").expect("failed to parse");
        assert_eq!(find_triple_2020(&nums), Some((979, 366, 675)));
    }
}
