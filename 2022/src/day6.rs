use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let marker_len = 4;
    let v = {
        input
            .as_bytes()
            .windows(marker_len)
            .position(|chars| {
                let uniq: HashSet<_> = chars.iter().collect();
                uniq.len() == 4
            })
            .expect("not found")
            + marker_len
    };
    v
}

#[test]
fn p1() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
}
#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let v = {
        let marker_len = 14;
        for (i, chars) in input.as_bytes().windows(marker_len).enumerate() {
            let uniq: HashSet<_> = chars.iter().collect();
            if uniq.len() == marker_len {
                return i + marker_len;
            }
        }
        0
    };
    assert_eq!(v, 3380);
    v
}
