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
            let mut total = 0;
            // Check endpoints
            let s = r.start();
            let e = r.end();

            // Then iterate over the high half of digits
            let l10 = e.ilog10();
            let trunc = 10u32.pow((l10 + 1) / 2) as u64;
            let trunc_s = s / trunc;
            let trunc_e = (e + 1) / trunc;
            /*
            println!(
                "s:{:10} e:{:10} l10:{} trunc:{} {} {}",
                s, e, l10, trunc, trunc_s, trunc_e
            );
            */
            for upper in trunc_s..=trunc_e {
                if upper == 0 {
                    continue;
                }
                let v = upper * (10u32.pow(upper.ilog10() + 1) as u64) + upper;
                if !r.contains(&v) {
                    continue;
                }

                total += v;
            }
            total
        })
        .sum::<u64>()
        .to_string()
    // Answer: 18893502033
}

fn is_pattern_part2(s: &str) -> bool {
    let b = s.as_bytes();
    let l = b.len() / 2;
    for i in 1..=l {
        let needle = &b[..i];
        //println!("needle {}", String::from_utf8_lossy(b));
        if b.chunks(i).skip(1).all(|c| c == needle) {
            return true;
        }
    }
    false
}
#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> String {
    input
        .iter()
        .map(|r| {
            r.clone()
                .filter_map(|n| {
                    let s = n.to_string();
                    if is_pattern_part2(&s) {
                        return Some(n);
                    }
                    return None;
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
    // Answer: 26202168557
}

#[cfg(test)]
mod tests {
    use advent::prelude::*;

    use super::*;

    const INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "1227775554");
        assert_eq!(part1(&parse(&input_for(2025, 2))), "18893502033");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "4174379265");
        assert_eq!(part2(&parse(&input_for(2025, 2))), "26202168557");
    }
}
