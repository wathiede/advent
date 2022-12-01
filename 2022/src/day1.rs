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
        .fold([0; 3], |mut top, cal| {
            if cal > top[0] {
                top[0] = cal;
                return top;
            } else if cal > top[1] {
                top[1] = cal;
                return top;
            } else if cal > top[2] {
                top[2] = cal;
                return top;
            }
            top
        })
        .iter()
        .sum()
}
