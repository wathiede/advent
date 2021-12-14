use std::{
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

#[aoc(dayX, part1)]
fn part1(input: &str) -> Result<usize> {
    todo!("part1");
    Ok(0)
}

/*
#[aoc(dayX, part2)]
fn part2(input: &str) -> Result<usize> {
    todo!("part2");
    Ok(0)
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
        assert_eq!(part1(input)?, usize::MAX);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
        let input = r#"
    "#
        .trim();
        assert_eq!(part2(input)?, usize::MAX);
    Ok(())
    }
    */
}
