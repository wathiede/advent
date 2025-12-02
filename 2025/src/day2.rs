use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split(',')
        .map(|seg| {
            let (lo, hi) = seg.split_once('-').expect("failed to split -");
            lo.parse::<u64>().expect("lo failed")..=hi.parse::<u64>().expect("hi failed")
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[RangeInclusive<u64>]) -> String {
    input
        .iter()
        .map(|r| {
            // TODO: why clone?
            r.clone()
                .filter_map(|n| {
                    let s = n.to_string();
                    let len = s.len();
                    if len % 2 == 1 {
                        return None;
                    };
                    if s[..len / 2] == s[len / 2..] {
                        return Some(n);
                    }
                    return None;
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
    // Answer: 18893502033
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "1227775554");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}
