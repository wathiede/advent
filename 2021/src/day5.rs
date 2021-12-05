//!
//! --- Day 5: Hydrothermal Venture ---
//! You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
//!
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
//!
//! An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//! An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//! For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
//!
//! To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
//!
//! Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
//!
//! --- Part Two ---
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//! An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//! Considering all lines from the above example would now produce the following diagram:
//!
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
//!
//! Consider all of the lines. At how many points do at least two lines overlap?

use std::{
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p0: Point,
    p1: Point,
}

#[derive(Debug, Error)]
enum LineError {
    #[error("couldn't parse number {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("input truncated")]
    PrematureEOL,
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(
            f,
            "{},{} -> {},{}",
            self.p0.x, self.p0.y, self.p1.x, self.p1.y,
        )
    }
}

impl FromStr for Line {
    type Err = LineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');
        let parse_point = |it: &mut dyn Iterator<Item = &str>| -> Result<Point, LineError> {
            let p = it.next().ok_or(LineError::PrematureEOL)?;
            let nums: Vec<_> = p
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<_, ParseIntError>>()?;
            Ok(Point {
                x: nums[0],
                y: nums[1],
            })
        };
        let p0 = parse_point(&mut it)?;
        let _ = it.next().ok_or(LineError::PrematureEOL)?;
        let p1 = parse_point(&mut it)?;
        Ok(Line { p0, p1 })
    }
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }
    fn answer(&self) -> u32 {
        self.pixels.iter().filter(|&v| *v > 1).count() as u32
    }
}

impl Index<(usize, usize)> for Image {
    type Output = u32;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[x + y * self.width]
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "({}, {})", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self[(x, y)];
                if v == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", v)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<Line>> {
    Ok(input
        .split('\n')
        .map(|l| l.parse())
        .collect::<Result<_, LineError>>()?)
}

fn draw(im: &mut Image, l: &Line) {
    let dx = l.p1.x - l.p0.x;
    let dy = l.p1.y - l.p0.y;

    if dx == 0 {
        let x = l.p0.x as usize;
        let sy = l.p0.y;
        let ey = l.p1.y;
        let (sy, ey) = if sy > ey { (ey, sy) } else { (sy, ey) };
        for y in sy..=ey {
            im[(x, y as usize)] += 1;
        }
    } else if dy == 0 {
        let y = l.p0.y as usize;
        let sx = l.p0.x;
        let ex = l.p1.x;
        let (sx, ex) = if sx > ex { (ex, sx) } else { (sx, ex) };
        for x in sx..=ex {
            im[(x as usize, y)] += 1;
        }
    } else {
        // We only support 45 degree angles.
        assert_eq!(dx.abs(), dy.abs());
        let dx = dx / dx.abs();
        let dy = dy / dy.abs();

        let mut x = l.p0.x;
        let mut y = l.p0.y;
        while x != l.p1.x && y != l.p1.y {
            im[(x as usize, y as usize)] += 1;
            x += dx;
            y += dy;
        }
        im[(x as usize, y as usize)] += 1;
    }
}

#[aoc(day5, part1)]
fn part1(lines: &[Line]) -> Result<u32> {
    let width = lines
        .iter()
        .map(|l| l.p0.x.max(l.p1.x) as usize)
        .max()
        .expect("couldn't find max width")
        + 1;
    let height = lines
        .iter()
        .map(|l| l.p0.y.max(l.p1.y) as usize)
        .max()
        .expect("couldn't find max height")
        + 1;
    let mut im = Image::new(width, height);
    for l in lines
        .iter()
        .filter(|l| l.p0.x == l.p1.x || l.p0.y == l.p1.y)
    {
        draw(&mut im, l);
    }
    Ok(im.answer())
}

#[aoc(day5, part2)]
fn part2(lines: &[Line]) -> Result<u32> {
    let width = lines
        .iter()
        .map(|l| l.p0.x.max(l.p1.x) as usize)
        .max()
        .expect("couldn't find max width")
        + 1;
    let height = lines
        .iter()
        .map(|l| l.p0.y.max(l.p1.y) as usize)
        .max()
        .expect("couldn't find max height")
        + 1;
    let mut im = Image::new(width, height);
    for l in lines {
        draw(&mut im, l);
    }
    Ok(im.answer())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
            "#
        .trim();
        assert_eq!(part1(&parse(input)?)?, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
    "#
        .trim();
        assert_eq!(part2(&parse(input)?)?, 12);
        Ok(())
    }
}
