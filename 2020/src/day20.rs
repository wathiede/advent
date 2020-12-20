//! --- Day 20: Jurassic Jigsaw ---
//! The high-speed train leaves the forest and quickly carries you south. You can even see a desert in the distance! Since you have some spare time, you might as well see if there was anything interesting in the image the Mythical Information Bureau satellite captured.
//!
//! After decoding the satellite messages, you discover that the data actually contains many small images created by the satellite's camera array. The camera array consists of many cameras; rather than produce a single square image, they produce many smaller square image tiles that need to be reassembled back into a single image.
//!
//! Each camera in the camera array returns a single monochrome image tile with a random unique ID number. The tiles (your puzzle input) arrived in a random order.
//!
//! Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and flipped to a random orientation. Your first task is to reassemble the original image by orienting the tiles so they fit together.
//!
//! To show how the tiles should be reassembled, each tile's image data includes a border that should line up exactly with its adjacent tiles. All tiles have this border, and the border lines up exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this border, but the outermost edges won't line up with any other tiles.
//!
//! For example, suppose you have the following nine tiles:
//!
//! Tile 2311:
//! ..##.#..#.
//! ##..#.....
//! #...##..#.
//! ####.#...#
//! ##.##.###.
//! ##...#.###
//! .#.#.#..##
//! ..#....#..
//! ###...#.#.
//! ..###..###
//!
//! Tile 1951:
//! #.##...##.
//! #.####...#
//! .....#..##
//! #...######
//! .##.#....#
//! .###.#####
//! ###.##.##.
//! .###....#.
//! ..#.#..#.#
//! #...##.#..
//!
//! Tile 1171:
//! ####...##.
//! #..##.#..#
//! ##.#..#.#.
//! .###.####.
//! ..###.####
//! .##....##.
//! .#...####.
//! #.##.####.
//! ####..#...
//! .....##...
//!
//! Tile 1427:
//! ###.##.#..
//! .#..#.##..
//! .#.##.#..#
//! #.#.#.##.#
//! ....#...##
//! ...##..##.
//! ...#.#####
//! .#.####.#.
//! ..#..###.#
//! ..##.#..#.
//!
//! Tile 1489:
//! ##.#.#....
//! ..##...#..
//! .##..##...
//! ..#...#...
//! #####...#.
//! #..#.#.#.#
//! ...#.#.#..
//! ##.#...##.
//! ..##.##.##
//! ###.##.#..
//!
//! Tile 2473:
//! #....####.
//! #..#.##...
//! #.##..#...
//! ######.#.#
//! .#...#.#.#
//! .#########
//! .###.#..#.
//! ########.#
//! ##...##.#.
//! ..###.#.#.
//!
//! Tile 2971:
//! ..#.#....#
//! #...###...
//! #.#.###...
//! ##.##..#..
//! .#####..##
//! .#..####.#
//! #..#.#..#.
//! ..####.###
//! ..#.#.###.
//! ...#.#.#.#
//!
//! Tile 2729:
//! ...#.#.#.#
//! ####.#....
//! ..#.#.....
//! ....#..#.#
//! .##..##.#.
//! .#.####...
//! ####.#.#..
//! ##.####...
//! ##..#.##..
//! #.##...##.
//!
//! Tile 3079:
//! #.#.#####.
//! .#..######
//! ..#.......
//! ######....
//! ####.#..#.
//! .#...#.##.
//! #.#####.##
//! ..#.###...
//! ..#.......
//! ..#.###...
//! By rotating, flipping, and rearranging them, you can find a square arrangement that causes all adjacent borders to line up:
//!
//! #...##.#.. ..###..### #.#.#####.
//! ..#.#..#.# ###...#.#. .#..######
//! .###....#. ..#....#.. ..#.......
//! ###.##.##. .#.#.#..## ######....
//! .###.##### ##...#.### ####.#..#.
//! .##.#....# ##.##.###. .#...#.##.
//! #...###### ####.#...# #.#####.##
//! .....#..## #...##..#. ..#.###...
//! #.####...# ##..#..... ..#.......
//! #.##...##. ..##.#..#. ..#.###...
//!
//! #.##...##. ..##.#..#. ..#.###...
//! ##..#.##.. ..#..###.# ##.##....#
//! ##.####... .#.####.#. ..#.###..#
//! ####.#.#.. ...#.##### ###.#..###
//! .#.####... ...##..##. .######.##
//! .##..##.#. ....#...## #.#.#.#...
//! ....#..#.# #.#.#.##.# #.###.###.
//! ..#.#..... .#.##.#..# #.###.##..
//! ####.#.... .#..#.##.. .######...
//! ...#.#.#.# ###.##.#.. .##...####
//!
//! ...#.#.#.# ###.##.#.. .##...####
//! ..#.#.###. ..##.##.## #..#.##..#
//! ..####.### ##.#...##. .#.#..#.##
//! #..#.#..#. ...#.#.#.. .####.###.
//! .#..####.# #..#.#.#.# ####.###..
//! .#####..## #####...#. .##....##.
//! ##.##..#.. ..#...#... .####...#.
//! #.#.###... .##..##... .####.##.#
//! #...###... ..##...#.. ...#..####
//! ..#.#....# ##.#.#.... ...##.....
//! For reference, the IDs of the above tiles are:
//!
//! 1951    2311    3079
//! 2729    1427    2473
//! 2971    1489    1171
//! To check that you've assembled the image correctly, multiply the IDs of the four corner tiles together. If you do this with the assembled tiles from the example above, you get 1951 * 3079 * 2971 * 1171 = 20899048083289.
//!
//! Assemble the tiles into an image. What do you get if you multiply together the IDs of the four corner tiles?

use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Debug)]
struct Tile {
    id: usize,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Tile, ()> {
        let mut it = s.split('\n');
        let id = it
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .expect("couldn't parse tile number");
        let rows: Vec<_> = it.collect();
        let height = rows.len();
        let mut width = 0;
        let mut pixels = Vec::with_capacity(height * height);
        rows.iter().for_each(|row| {
            width = row.len();
            pixels.extend(row.bytes());
        });
        Ok(Tile {
            id,
            pixels,
            height,
            width,
        })
    }
}
impl Index<(usize, usize)> for Tile {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x % self.width + y * self.width]
    }
}
impl Tile {
    fn border_set(&self) -> HashSet<Vec<u8>> {
        let mut set = HashSet::new();
        set.insert(self.top_border());
        set.insert(self.right_border());
        set.insert(self.bottom_border());
        set.insert(self.left_border());

        let rev_set: HashSet<_> = set
            .iter()
            .map(|b| {
                let mut b = b.clone();
                b.reverse();
                b
            })
            .collect();
        set.union(&rev_set).cloned().collect()
    }
    fn top_border(&self) -> Vec<u8> {
        (0..self.width).map(|x| self[(x, 0)]).collect()
    }
    fn right_border(&self) -> Vec<u8> {
        (0..self.height)
            .map(|y| self[(self.width - 1, y)])
            .collect()
    }
    fn bottom_border(&self) -> Vec<u8> {
        (0..self.width)
            .map(|x| self[(x, self.height - 1)])
            .collect()
    }
    fn left_border(&self) -> Vec<u8> {
        (0..self.height).map(|y| self[(0, y)]).collect()
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|s| s.parse().expect("failed to parse tile"))
        .collect()
}
#[aoc(day20, part1)]
fn solution1(tiles: &[Tile]) -> usize {
    let mut border_counts = HashMap::new();
    tiles.iter().for_each(|t| {
        t.border_set().iter().for_each(|b| {
            let c = border_counts.entry(b.clone()).or_insert(0);
            *c += 1;
        })
    });

    let corner_tiles: Vec<_> = tiles
        .iter()
        .filter(|t| {
            let matches: usize = t.border_set().iter().map(|b| border_counts[b]).sum();
            matches == 12
        })
        .collect();
    corner_tiles.iter().map(|t| t.id).product()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    const INPUT: &'static str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    #[test]
    fn test_generator() {
        assert_eq!(
            generator(&INPUT).iter().map(|t| t.id).collect::<Vec<_>>(),
            vec![2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079,]
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&generator(&INPUT)), 1951 * 3079 * 2971 * 1171);
    }
}
