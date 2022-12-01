use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<usize>().expect("number"))
                .sum()
        })
        .max()
        .expect("max")
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut cals: Vec<_> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<usize>().expect("number"))
                .sum()
        })
        .collect();
    cals.sort();
    cals.iter().rev().take(3).sum()
}
