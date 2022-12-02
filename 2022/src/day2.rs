use std::str::FromStr;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn points(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    fn play(&self, them: &Play) -> usize {
        use Play::*;
        match (self, them) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 6,
        }
    }
}

impl FromStr for Play {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(format!("Unknown play: {}", s)),
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n")
        .map(|l| {
            let (them, me) = l.split_once(' ').unwrap();
            let (them, me): (Play, Play) = (
                them.parse().expect("couldn't parse them"),
                me.parse().expect("couldn't parse me"),
            );
            me.play(&them) + me.points()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    0
}
