use anyhow::Result;
use aoc_runner_derive::aoc;

fn char_to_bit(c: u8) -> u64 {
    let bit = if c >= b'a' {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    };
    1 << bit
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let answer = input
        .split('\n')
        .map(|l| {
            let l = l.as_bytes();
            let h = l.len() / 2;
            let r1 = l[..h].iter().fold(0_u64, |bits, b| bits | char_to_bit(*b));
            let r2 = l[h..].iter().fold(0_u64, |bits, b| bits | char_to_bit(*b));
            let intersection = r1 & r2;
            debug_assert_eq!(1, intersection.count_zeros());
            intersection.trailing_zeros()
        })
        .sum();
    debug_assert_eq!(answer, 8018);
    answer
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.split('\n').collect();

    let answer = lines
        .chunks(3)
        .map(|lines| {
            let a = lines[0]
                .as_bytes()
                .iter()
                .fold(0_u64, |bits, b| bits | char_to_bit(*b));
            let b = lines[1]
                .as_bytes()
                .iter()
                .fold(0_u64, |bits, b| bits | char_to_bit(*b));
            let c = lines[2]
                .as_bytes()
                .iter()
                .fold(0_u64, |bits, b| bits | char_to_bit(*b));
            let intersection = a & b & c;
            debug_assert_eq!(1, intersection.count_zeros());
            intersection.trailing_zeros()
        })
        .sum();
    debug_assert_eq!(answer, 2518);
    answer
}
