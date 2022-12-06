use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    for (i, chars) in input.as_bytes().windows(4).enumerate() {
        let uniq: HashSet<_> = chars.iter().collect();
        dbg!(&chars, &uniq);
        if uniq.len() == 4 {
            return i + 4;
        }
    }
    0
}

#[test]
fn p1() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
}
// #[aoc(day6, part2)]
// fn part2(input: &str) -> usize { }
