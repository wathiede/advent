use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn solve(input: &str, marker_len: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_len)
        .position(|chars| {
            let uniq: HashSet<_> = chars.iter().collect();
            uniq.len() == marker_len
        })
        .expect("not found")
        + marker_len
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[test]
fn p1() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
}
#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let v = solve(input, 14);
    assert_eq!(v, 3380);
    v
}
