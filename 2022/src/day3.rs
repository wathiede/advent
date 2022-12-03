use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|l| {
            let l = l.as_bytes();
            let h = l.len() / 2;
            let r1: HashSet<_> = l[..h].iter().collect();
            let r2: HashSet<_> = l[h..].iter().collect();
            let m = *r1.intersection(&r2).next().unwrap();
            let b = if m >= &b'a' {
                m - &b'a' + 1
            } else {
                m - &b'A' + 27
            };
            b as usize
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.split('\n').collect();

    lines
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
        .sum()
}
