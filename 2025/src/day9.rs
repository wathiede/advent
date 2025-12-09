use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').expect("missing ,");
            (
                x.parse().expect("failed to parse x"),
                y.parse().expect("failed to parse y"),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(isize, isize)]) -> String {
    part1_n2(input)
}

// Attempt to solve with brute force
fn part1_n2(input: &[(isize, isize)]) -> String {
    input
        .iter()
        .flat_map(|(x1, y1)| {
            input
                .iter()
                .map(move |(x2, y2)| (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs())
        })
        .max()
        .expect("couldn't find max")
        .to_string()
}

// Attempt to solve with min/max corners
fn part1_min_max(input: &[(isize, isize)]) -> String {
    let min_x = input.iter().min().expect("couldn't find min x");
    let max_x = input.iter().max().expect("couldn't find max x");
    let min_y = input
        .iter()
        .map(|(x, y)| (y, x))
        .min()
        .expect("couldn't find min y");
    let max_y = input
        .iter()
        .map(|(x, y)| (y, x))
        .max()
        .expect("couldn't find max y");
    dbg!(input, min_x, max_x, min_y, max_y);

    (((max_x.0 - min_x.0 + 1).abs() * (max_x.1 - min_x.1 + 1).abs())
        .max((min_y.0 - max_y.0 + 1).abs() * (min_y.1 - max_y.1 + 1).abs()))
    .to_string()
}

#[aoc(day9, part2)]
fn part2(input: &[(isize, isize)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "50");
        assert_eq!(part1(&parse(&input_for(2025, 9))), "4754955192");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "24");
    }
}

