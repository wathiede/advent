use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    let numbers: Result<Vec<_>, ParseIntError> = input
        .split_whitespace()
        .map(|l| l.replace('R', "+").replace('L', "-").parse::<i64>())
        .collect();
    numbers.expect("Failed to parse number")
}

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> String {
    let mut cur = 50;
    let mut res = 0;
    for i in input {
        cur = cur + i;
        while cur < 0 {
            cur += 100;
        }
        cur %= 100;
        if cur == 0 {
            res += 1;
        }
    }
    // Answer for my data: 1048
    res.to_string()
}

#[aoc(day1, part2)]
fn part2(input: &[i64]) -> String {
    let mut cur = 50;
    let mut res = 0;
    for i in input {
        let dir = i.signum();
        for _ in 0..i.abs() {
            cur += dir;
            if cur < 0 {
                cur += 100;
            }
            if cur > 0 {
                cur -= 100;
            }
            if cur == 0 {
                res += 1;
            }
        }
    }
    // Answer for my data: 6498
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "3");
        assert_eq!(part1(&parse(&input_for(2025, 1))), "1048");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "6");
        assert_eq!(part2(&parse(&input_for(2025, 1))), "6498");
    }
}
