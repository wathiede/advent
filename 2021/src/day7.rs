//! --- Day 7: The Treachery of Whales ---
//! A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!
//!
//! Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!
//!
//! The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?
//!
//! There's one major catch - crab submarines can only move horizontally.
//!
//! You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.
//!
//! For example, consider the following horizontal positions:
//!
//! 16,1,2,0,4,2,7,1,2,14
//! This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.
//!
//! Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:
//!
//! Move from 16 to 2: 14 fuel
//! Move from 1 to 2: 1 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 0 to 2: 2 fuel
//! Move from 4 to 2: 2 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 7 to 2: 5 fuel
//! Move from 1 to 2: 1 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 14 to 2: 12 fuel
//! This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).
//!
//! Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?
//!
//!  --- Part Two ---
//! The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?
//!
//! As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.
//!
//! As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:
//!
//! Move from 16 to 5: 66 fuel
//! Move from 1 to 5: 10 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 0 to 5: 15 fuel
//! Move from 4 to 5: 1 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 7 to 5: 3 fuel
//! Move from 1 to 5: 10 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 14 to 5: 45 fuel
//! This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.
//!
//! Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
use std::num::ParseIntError;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()
}

fn score1(nums: &[u64], mid: u64) -> u64 {
    nums.iter()
        .map(|n| ((*n as i64) - (mid as i64)).abs())
        .sum::<i64>() as u64
}

#[aoc(day7, part1)]
fn part1(input: &[u64]) -> Result<u64> {
    let mut input: Vec<_> = input.to_vec();
    input.sort_unstable();
    Ok(score1(&input, input[input.len() / 2]))
}

fn score2(nums: &[u64], mid: u64) -> u64 {
    nums.iter()
        .map(|n| {
            let d = ((*n as i64) - (mid as i64)).abs();
            (d * (d + 1)) / 2
        })
        .sum::<i64>() as u64
}

#[aoc(day7, part2)]
fn part2(input: &[u64]) -> Result<u64> {
    let mut input: Vec<_> = input.to_vec();
    let avg = input.iter().sum::<u64>() / input.len() as u64;

    let s = if avg > 10 { avg - 10 } else { 0 };
    let num = input.len() as u64;
    let e = if avg + 10 < num { avg + 10 } else { num };
    let answer = (s..e)
        .map(|i| score2(&input, i))
        .min()
        .expect("couldn't find min");
    if input.len() > 10 {
        // The real data needs an answer lower than our first attempt.
        assert!(answer < 94862126);
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score1() -> Result<()> {
        let nums: Vec<u64> = parse("16,1,2,0,4,2,7,1,2,14")?;
        assert_eq!(score1(&nums, 1), 41);
        assert_eq!(score1(&nums, 2), 37);
        assert_eq!(score1(&nums, 3), 39);
        assert_eq!(score1(&nums, 10), 71);
        Ok(())
    }
    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#.trim();
        assert_eq!(part1(&parse(input)?)?, 37);
        Ok(())
    }

    #[test]
    fn test_score2() -> Result<()> {
        let nums: Vec<u64> = parse("16,1,2,0,4,2,7,1,2,14")?;
        assert_eq!(score2(&nums, 5), 168);
        assert_eq!(score2(&nums, 2), 206);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#.trim();
        assert_eq!(part2(&parse(input)?)?, 168);
        Ok(())
    }
}
