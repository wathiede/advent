//! --- Day 9: Smoke Basin ---
//! These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.
//!
//! Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)
//!
//! In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.
//!
//! The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.
//!
//! Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
//!
//!

use std::{
    convert::Infallible,
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

struct HeightMap {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<u8> {
        let mut pts = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self[(x, y)];

                if (x == 0 || c < self[(x - 1, y)])
                    && (y == 0 || c < self[(x, y - 1)])
                    && (x == self.width - 1 || c < self[(x + 1, y)])
                    && (y == self.height - 1 || c < self[(x, y + 1)])
                {
                    pts.push(c);
                }
            }
        }
        pts
    }
}

impl Index<(usize, usize)> for HeightMap {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl FromStr for HeightMap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().collect();
        let width = rows[0].len();
        let height = rows.len();
        let pixels = rows
            .iter()
            .flat_map(|row| row.as_bytes().iter().map(|b| b - b'0'))
            .collect();

        Ok(HeightMap {
            width,
            height,
            pixels,
        })
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<HeightMap> {
    Ok(input.parse()?)
}

#[aoc(day9, part1)]
fn part1(input: &HeightMap) -> Result<u64> {
    Ok(input.low_points().iter().map(|b| (*b + 1) as u64).sum())
}

/*
#[aoc(day9, part2)]
fn part2(input: &[u64]) -> Result<u64> {
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
2199943210
3987894921
9856789892
8767896789
9899965678
"#
        .trim();
        let hm = parse(input)?;
        assert_eq!(hm.low_points(), vec![1, 0, 5, 5]);
        assert_eq!(part1(&hm)?, 15);
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
