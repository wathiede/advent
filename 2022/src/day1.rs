use anyhow::Result;
use aoc_runner_derive::aoc;

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
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<usize>().expect("number"))
                .sum()
        })
        .fold([0; 3], |mut top, v| {
            let min = *top.iter().min().expect("empty list");
            let v: usize = v;
            if min < v {
                top[top.iter().position(|v| *v == min).unwrap()] = v;
            }
            top
        })
        .iter()
        .sum()
}
