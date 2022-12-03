use std::collections::HashSet;

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
            let union = r1 & r2;
            debug_assert_eq!(1, union.count_zeros());
            union.trailing_zeros()
        })
        .sum();
    assert_eq!(answer, 8018);
    answer
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.split('\n').collect();

    let answer = lines
        .chunks(3)
        .map(|lines| {
            let a = lines[0];
            let b = lines[1];
            let c = lines[2];
            let r1: HashSet<_> = a.as_bytes().into_iter().collect();
            let r2: HashSet<_> = b.as_bytes().into_iter().collect();
            let r3: HashSet<_> = c.as_bytes().into_iter().collect();
            let m = *r1
                .intersection(&r2)
                .map(|u| *u)
                .collect::<HashSet<&u8>>()
                .intersection(&r3)
                .next()
                .unwrap();
            let b = if m >= &b'a' {
                m - &b'a' + 1
            } else {
                m - &b'A' + 27
            };
            b as usize
        })
        .sum();
    assert_eq!(answer, 2518);
    answer
}
