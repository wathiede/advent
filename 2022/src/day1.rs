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
