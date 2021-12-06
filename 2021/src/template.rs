use std::{
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

#[aoc_generator(dayX)]
fn parse(input: &str) -> Result<Vec<u64>> {
    todo!("parse");
    Ok(Vec::new())
}

#[aoc(dayX, part1)]
fn part1(input: &[u64]) -> Result<u64> {
    todo!("part1");
    Ok(0)
}

/*
#[aoc(dayX, part2)]
fn part2(depths: &[u64]) -> Result<u64> {
    todo!("part2")
    Ok(())
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
"#
        .trim();
        assert_eq!(part1(&parse(input)?)?, u64::MAX);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
        let input = r#"
    "#
        .trim();
        assert_eq!(part2(&parse(input)?)?, u64::MAX);
    Ok(())
    }
    */
}
