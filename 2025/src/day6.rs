use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Op {
    Plus,
    Multiply,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    let m: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.trim().split(' ').filter(|c| !c.is_empty()).collect())
        .collect();
    let ops: Vec<_> = m[m.len() - 1]
        .iter()
        .map(|c| match *c {
            "+" => Op::Plus,
            "*" => Op::Multiply,
            op => panic!("unknown op: {op}"),
        })
        .collect();
    let mut collector: Vec<i64> = ops
        .iter()
        .map(|op| match op {
            Op::Plus => 0,
            Op::Multiply => 1,
        })
        .collect();
    m.iter().take(m.len() - 1).for_each(|r| {
        r.iter().enumerate().for_each(|(idx, c)| {
            let i: i64 = c.parse().expect("failed to parse cell");
            match ops[idx] {
                Op::Plus => collector[idx] += i,
                Op::Multiply => collector[idx] *= i,
            }
        })
    });
    collector.into_iter().sum::<i64>().to_string()
}

fn process_rect(im: &Image, r: Range<usize>, h: usize) -> u64 {
    let mut nums = vec![];
    for x in r.start..r.end - 1 {
        let mut n = 0u64;
        for y in 0..h {
            let b = im[(x, y)];
            if b != b' ' {
                n = n * 10 + (b - b'0') as u64;
            }
        }
        nums.push(n);
    }
    let op = match im[(r.start, h)] {
        b'+' => Op::Plus,
        b'*' => Op::Multiply,
        c => panic!("unknown op {c}"),
    };
    match op {
        Op::Plus => nums.iter().sum::<u64>(),
        Op::Multiply => nums.iter().product::<u64>(),
    }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    let im: Image = input.parse().expect("parse image failed");
    let h = im.height - 1;
    let mut ranges = vec![];
    let mut start = 0;
    for x in 0..im.width {
        if im[(x, h)] != b' ' && x > 0 {
            ranges.push(start..x);
            start = x;
        }
    }
    ranges.push(start..im.width + 1);

    ranges
        .into_iter()
        .map(|r| process_rect(&im, r, h))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "4277556");
        assert_eq!(part1(&input_for(2025, 6)), "7229350537438");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "3263827");
        assert_eq!(part2(&input_for(2025, 6)), "11479269003550");
    }
}
