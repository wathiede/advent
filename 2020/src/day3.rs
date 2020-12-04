//! --- Day 3: Toboggan Trajectory ---
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
//!
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! In this example, traversing the map using this slope would cause you to encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
//!
//! --- Part Two ---
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! Right 1, down 1.
//! Right 3, down 1. (This is the slope you already checked.)
//! Right 5, down 1.
//! Right 7, down 1.
//! Right 1, down 2.
//! In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

use std::ops::Index;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct Bitmap {
    width: usize,
    height: usize,
    bits: Vec<bool>,
}

impl Index<(usize, usize)> for Bitmap {
    type Output = bool;

    // Perform 2-dimensional indexing of bits, wrapping the X coordinates if it is larger than the
    // width of the Bitmap.
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.bits[x % self.width + y * self.width]
    }
}
#[aoc_generator(day3)]
fn parse(input: &str) -> Bitmap {
    let mut height = 0;
    let bits: Vec<bool> = input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            '\n' => {
                height += 1;
                None
            }
            _ => panic!("Unexpected character"),
        })
        .collect();

    let width = bits.len() / height;
    Bitmap {
        width,
        height,
        bits,
    }
}

#[aoc(day3, part1)]
fn answer_part1(map: &Bitmap) -> usize {
    (0..map.height).filter(|y| map[(*y * 3, *y)]).count()
}

#[aoc(day3, part2)]
fn answer_part2(map: &Bitmap) -> usize {
    (0..map.height).filter(|y| map[(*y, *y)]).count()
        * (0..map.height).filter(|y| map[(*y * 3, *y)]).count()
        * (0..map.height).filter(|y| map[(*y * 5, *y)]).count()
        * (0..map.height).filter(|y| map[(*y * 7, *y)]).count()
        * (0..map.height / 2).filter(|y| map[(*y, *y * 2)]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_bitmap() {
        assert_eq!(
            parse("..##\n##..\n"),
            Bitmap {
                width: 4,
                height: 2,
                bits: vec![false, false, true, true, true, true, false, false]
            }
        );
    }

    const INPUT :&'static str="..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";
    #[test]
    fn part1() {
        assert_eq!(answer_part1(&parse(INPUT)), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(answer_part2(&parse(INPUT)), 336);
    }
}
