use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let (ranges, remainder) = range_inclusive(input).expect("failed to parse input");
    let mut cnt = 0;
    for id in remainder.lines() {
        let id: u64 = id.parse().expect("failed to parse id");
        for r in &ranges {
            if r.contains(&id) {
                cnt += 1;
                break;
            }
        }
    }
    cnt.to_string()
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut mr: Vec<RangeInclusive<u64>> = Vec::new();
    ranges.sort_by_key(|r| *r.start());
    for r in ranges {
        if let Some(last) = mr.last_mut() {
            if last.contains(r.start()) {
                *last = *last.start()..=*last.end().max(r.end());
            } else {
                mr.push(r);
            }
        } else {
            mr.push(r);
        }
    }
    mr
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let (ranges, _) = range_inclusive(input).expect("failed to parse input");
    let ranges = merge_ranges(ranges);
    let mut cnt = 0;
    for r in ranges {
        cnt += r.end() - r.start() + 1;
    }
    cnt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "14");
    }
}
