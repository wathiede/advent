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
//!
//! --- Part Two ---
//! Now, you're ready to check the image for sea monsters.
//!
//! The borders of each tile are not part of the actual image; start by removing them.
//!
//! In the example above, the tiles become:
//!
//! .#.#..#. ##...#.# #..#####
//! ###....# .#....#. .#......
//! ##.##.## #.#.#..# #####...
//! ###.#### #...#.## ###.#..#
//! ##.#.... #.##.### #...#.##
//! ...##### ###.#... .#####.#
//! ....#..# ...##..# .#.###..
//! .####... #..#.... .#......
//!
//! #..#.##. .#..###. #.##....
//! #.####.. #.####.# .#.###..
//! ###.#.#. ..#.#### ##.#..##
//! #.####.. ..##..## ######.#
//! ##..##.# ...#...# .#.#.#..
//! ...#..#. .#.#.##. .###.###
//! .#.#.... #.##.#.. .###.##.
//! ###.#... #..#.##. ######..
//!
//! .#.#.### .##.##.# ..#.##..
//! .####.## #.#...## #.#..#.#
//! ..#.#..# ..#.#.#. ####.###
//! #..####. ..#.#.#. ###.###.
//! #####..# ####...# ##....##
//! #.##..#. .#...#.. ####...#
//! .#.###.. ##..##.. ####.##.
//! ...###.. .##...#. ..#..###
//! Remove the gaps to form the actual image:
//!
//! .#.#..#.##...#.##..#####
//! ###....#.#....#..#......
//! ##.##.###.#.#..######...
//! ###.#####...#.#####.#..#
//! ##.#....#.##.####...#.##
//! ...########.#....#####.#
//! ....#..#...##..#.#.###..
//! .####...#..#.....#......
//! #..#.##..#..###.#.##....
//! #.####..#.####.#.#.###..
//! ###.#.#...#.######.#..##
//! #.####....##..########.#
//! ##..##.#...#...#.#.#.#..
//! ...#..#..#.#.##..###.###
//! .#.#....#.##.#...###.##.
//! ###.#...#..#.##.######..
//! .#.#.###.##.##.#..#.##..
//! .####.###.#...###.#..#.#
//! ..#.#..#..#.#.#.####.###
//! #..####...#.#.#.###.###.
//! #####..#####...###....##
//! #.##..#..#...#..####...#
//! .#.###..##..##..####.##.
//! ...###...##...#...#..###
//! Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster will look like this:
//!
//!                   #
//! #    ##    ##    ###
//!  #  #  #  #  #  #
//! When looking for this pattern in the image, the spaces can be anything; only the # need to match. Also, you might need to rotate or flip your image before it's oriented correctly to find sea monsters. In the above image, after flipping and rotating it to the appropriate orientation, there are two sea monsters (marked with O):
//!
//! .####...#####..#...###..
//! #####..#..#.#.####..#.#.
//! .#.#...#.###...#.##.O#..
//! #.O.##.OO#.#.OO.##.OOO##
//! ..#O.#O#.O##O..O.#O##.##
//! ...#.#..##.##...#..#..##
//! #.##.#..#.#..#..##.#.#..
//! .###.##.....#...###.#...
//! #.####.#.#....##.#..#.#.
//! ##...#..#....#..#...####
//! ..#.##...###..#.#####..#
//! ....#.##.#.#####....#...
//! ..##.##.###.....#.##..#.
//! #...#...###..####....##.
//! .#.##...#.##.#.#.###...#
//! #.###.#..####...##..#...
//! #.###...#.##...#.##O###.
//! .O##.#OO.###OO##..OOO##.
//! ..O#.O..O..O.#O##O##.###
//! #.#..##.########..#..##.
//! #.#####..#.#...##..#....
//! #....##..#.#########..##
//! #...#.....#..##...###.##
//! #..###....##.#...##.##.#
//! Determine how rough the waters are in the sea monsters' habitat by counting the number of # that are not part of a sea monster. In the above example, the habitat's water roughness is 273.
//!
//! How many # are not part of a sea monster?

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Hash, Eq, PartialEq)]
struct Tile {
    id: usize,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {}:\n", self.id)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)] as char)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Tile, ()> {
        let mut it = s.split('\n');
        let id = it
            .next()
            .expect("couldn't get first line of tile")
            .trim()
            .split(' ')
            .skip(1)
            .next()
            .expect("couldn't get second word of tile header")
            .strip_suffix(':')
            .expect("couldn't strip ':' from tile header")
            .parse()
            .expect("couldn't parse tile number");
        let rows: Vec<_> = it.map(|l| l.trim()).collect();
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

impl IndexMut<(usize, usize)> for Tile {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[x + y * self.width]
    }
}

impl Index<(usize, usize)> for Tile {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

fn border_to_str(border: &[u8]) -> String {
    std::str::from_utf8(border).unwrap().to_string()
}

impl Tile {
    /// Copy `t` into self @ x_off,y_off.
    fn blit(&mut self, t: &Tile, x_off: usize, y_off: usize) {
        (0..t.height)
            .for_each(|y| (0..t.width).for_each(|x| self[(x_off + x, y_off + y)] = t[(x, y)]));
    }
    /// Builds a set containing all the borders of this tile and their reverse (useful if the tile
    /// is in the wrong orientation).
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
    fn strip_border(&self) -> Tile {
        let pixels = (1..self.height - 1)
            .map(|y| (1..self.width - 1).map(move |x| self[(x, y)]))
            .flatten()
            .collect();

        Tile {
            id: self.id,
            width: self.width - 2,
            height: self.height - 2,
            pixels,
        }
    }
    fn search(&self, needle: &Tile, x_off: usize, y_off: usize) -> bool {
        for n_y in 0..needle.height {
            for n_x in 0..needle.width {
                if needle[(n_x, n_y)] != b'#' {
                    continue;
                }
                if self[(x_off + n_x, y_off + n_y)] != b'#' {
                    return false;
                }
            }
        }
        true
    }
    fn count_hashes(&self) -> usize {
        self.pixels.iter().filter(|b| *b == &b'#').count()
    }
    fn rotate90(&self) -> Tile {
        let pixels = (0..self.height)
            .map(|y| (0..self.width).map(move |x| self[(y, self.height - x - 1)]))
            .flatten()
            .collect();

        Tile {
            id: self.id,
            width: self.width,
            height: self.height,
            pixels,
        }
    }
    /// Slow but easy to implement.
    fn rotate180(&self) -> Tile {
        self.rotate90().rotate90()
    }
    /// Slow but easy to implement.
    fn rotate270(&self) -> Tile {
        self.rotate180().rotate90()
    }
    fn flip_horizontal(&self) -> Tile {
        let pixels = (0..self.height)
            .map(|y| (0..self.width).map(move |x| self[(self.width - x - 1, y)]))
            .flatten()
            .collect();

        Tile {
            id: self.id,
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    /// Finds number of occurrences of needle in self.  A match requires all '#' in needle to be
    /// found in self.  Extra '#' in self are ignored. The returned vector is the x,y of the upper
    /// left pixel for the match.
    fn find_hashes(&self, needle: &Tile) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for y_off in 0..self.height - needle.height {
            for x_off in 0..self.width - needle.width {
                if self.search(needle, x_off, y_off) {
                    res.push((x_off, y_off));
                }
            }
        }
        res
    }
}

/// Tries various orientations, until predicate matches.
fn reorient<F>(img: &Tile, predicate: F) -> Tile
where
    F: Fn(&Tile) -> bool,
{
    let rotated = img.rotate90();
    if predicate(&rotated) {
        return rotated;
    }

    let rotated = img.rotate180();
    if predicate(&rotated) {
        return rotated;
    }

    let rotated = img.rotate270();
    if predicate(&rotated) {
        return rotated;
    }

    let horiz = img.flip_horizontal();

    let rotated = horiz.rotate90();
    if predicate(&rotated) {
        return rotated;
    }

    let rotated = horiz.rotate180();
    if predicate(&rotated) {
        return rotated;
    }

    let rotated = horiz.rotate270();
    if predicate(&rotated) {
        return rotated;
    }
    panic!("couldn't find an orientation matching predicate");
}

fn stitch(tiles: &[Tile]) -> Tile {
    // Make sure there's a sqare number of tiles.
    let sqrt = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(sqrt * sqrt, tiles.len());

    let width = sqrt * (tiles[0].width - 2);
    let height = sqrt * (tiles[0].width - 2);
    let mut image = Tile {
        id: 0,
        width,
        height,
        pixels: vec![b' '; width * height],
    };

    let mut border_counts = HashMap::new();
    let mut border_map = HashMap::new();
    tiles.iter().for_each(|t| {
        t.border_set().iter().for_each(|b| {
            border_map.insert(b.clone(), t.id);
            let c = border_counts.entry(b.clone()).or_insert(0);
            *c += 1;
        })
    });

    // Find a corner, and then stitch from there.
    let corner = tiles
        .iter()
        .filter(|t| {
            let matches: usize = t.border_set().iter().map(|b| border_counts[b]).sum();
            matches == 12
        })
        // Grab the min for repeatable results.
        .min_by(|l, r| l.id.cmp(&r.id))
        .expect("couldn't find corner");
    /*
       let corner = reorient(&corner.flip_horizontal(), |im| {
    // Reorient until the top and left borders are edges.  This has a 50/50 chance of being
    // right, and we can't verify it until the first neighbor is found.
    border_counts[&im.top_border()] == 2 && border_counts[&im.left_border()] == 2
    });
    */
    let mut map_ids = vec![vec![0; sqrt]; sqrt];

    map_ids[0][0] = corner.id;
    let id_to_tile: HashMap<_, _> = tiles.iter().map(|t| (t.id, t)).collect();
    let mut remaining_tiles: HashSet<_> = tiles.iter().collect();
    remaining_tiles.remove(&corner);
    let mut prev_tile = corner;

    (0..sqrt)
        .map(|y| (0..sqrt).map(move |x| (x, y)))
        .flatten()
        .skip(1)
        .for_each(|(x, y)| {
            if x == 0 {
                // Beginning of a new row, use the tile above as the previous tile.
                prev_tile = id_to_tile[&map_ids[x][y - 1]];
            }
            dbg!((x, y), prev_tile.id);
            let neighbor = remaining_tiles
                .iter()
                .filter(|t| t.border_set().intersection(&prev_tile.border_set()).count() > 0)
                .nth(0)
                .expect("couldn't find neighbor");
            map_ids[x][y] = neighbor.id;
            dbg!(&map_ids);
            prev_tile = neighbor;
            remaining_tiles.remove(prev_tile);
        });

    let corner = reorient(corner, |im| {
        let right_set = id_to_tile[&map_ids[1][0]].border_set();
        let bottom_set = id_to_tile[&map_ids[0][1]].border_set();
        right_set.contains(&im.right_border()) && bottom_set.contains(&im.bottom_border())
    });
    // Map tile id to correctly oriented Tile.
    let mut oriented = HashMap::new();
    oriented.insert(corner.id, corner);
    (0..sqrt)
        .map(|y| (0..sqrt).map(move |x| (x, y)))
        .flatten()
        .skip(1)
        .for_each(|(x, y)| {
            let t = id_to_tile[&map_ids[x][y]];
            let t = if x == 0 {
                // Beginning of a new row, use the tile above as the previous tile.
                let above_tile = id_to_tile[&map_ids[x][y - 1]];
                reorient(t, |im| {
                    dbg!(
                        border_to_str(&above_tile.right_border()),
                        border_to_str(&im.left_border())
                    );
                    above_tile.bottom_border() == im.top_border()
                })
            // TODO(wathiede): reorient and blit.
            } else {
                // Use the tile to the left as previous tile.
                let left_tile = id_to_tile[&map_ids[x - 1][y]];
                reorient(t, |im| {
                    dbg!(
                        border_to_str(&left_tile.right_border()),
                        border_to_str(&im.left_border())
                    );
                    left_tile.right_border() == im.left_border()
                })
            };
            let out = t.strip_border();
            image.blit(&out.strip_border(), x * out.width, y * out.height);
            oriented.insert(t.id, t);
        });

    // TODO(wathiede) paste oriented into image.
    image
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|s| s.parse().expect("failed to parse tile"))
        .collect()
}

fn seamonster() -> Tile {
    const MONSTER: &'static str = r#"Tile 666:
        ..................#.
#....##....##....###
        .#..#..#..#..#..#..."#;

    MONSTER.parse().expect("failed to parse seamonster")
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

fn habitat(img: &Tile) -> usize {
    let monster = seamonster();
    let num_monsters = img.find_hashes(&monster).len();
    img.count_hashes() - (num_monsters * monster.count_hashes())
}

fn contains_seamonster(t: &Tile) -> bool {
    let monster = seamonster();
    t.find_hashes(&monster).len() > 0
}

#[aoc(day20, part2)]
fn solution2(tiles: &[Tile]) -> usize {
    habitat(&reorient(&stitch(tiles), contains_seamonster))
}

#[cfg(test)]
mod tests {
    //use pretty_assertions::assert_eq;

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

    const OUTPUT_IMAGE: &'static str = r#"Tile 0:
        .#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
        ...########.#....#####.#
        ....#..#...##..#.#.###..
        .####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
        ...#..#..#.#.##..###.###
        .#.#....#.##.#...###.##.
###.#...#..#.##.######..
        .#.#.###.##.##.#..#.##..
        .####.###.#...###.#..#.#
        ..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
        .#.###..##..##..####.##.
        ...###...##...#...#..###"#;

    #[test]
    fn make_image() {
        let _: Tile = OUTPUT_IMAGE.parse().expect("failed to part want image");
    }
    #[test]
    fn find_monster() {
        let img: Tile = OUTPUT_IMAGE.parse().expect("failed to part want image");
        let monster = seamonster();
        dbg!(&img);
        dbg!(&monster);
        assert_eq!(img.find_hashes(&monster).len(), 0);
        assert_eq!(img.rotate90().find_hashes(&monster).len(), 0);
        assert_eq!(img.rotate180().find_hashes(&monster).len(), 0);
        assert_eq!(img.rotate270().find_hashes(&monster).len(), 0);

        let horiz = img.flip_horizontal();
        assert_eq!(horiz.rotate90().find_hashes(&monster).len(), 0);
        assert_eq!(horiz.rotate180().find_hashes(&monster).len(), 0);
        assert_eq!(horiz.rotate270().find_hashes(&monster).len(), 2);

        let correct = horiz.rotate270();
        dbg!(&correct);
        assert_eq!(correct.find_hashes(&monster), vec![(2, 2), (1, 16),]);
    }

    #[test]
    fn test_reorient() {
        let img: Tile = OUTPUT_IMAGE.parse().expect("failed to part want image");
        let monster = seamonster();
        assert_eq!(
            reorient(&img, contains_seamonster)
                .find_hashes(&monster)
                .len(),
            2
        );
    }

    const TEST_ROTATE: &'static str = "Tile 0:\n#.\n..";
    #[test]
    fn rotate90() {
        let img: Tile = TEST_ROTATE.parse().expect("failed to part rotate image");
        let want: Tile = "Tile 0:\n.#\n.."
            .parse()
            .expect("failed to parse rotate90 want");
        assert_eq!(img.rotate90(), want);
    }
    #[test]
    fn rotate180() {
        let img: Tile = TEST_ROTATE.parse().expect("failed to part rotate image");
        let want: Tile = "Tile 0:\n..\n.#"
            .parse()
            .expect("failed to parse rotate180 want");
        assert_eq!(img.rotate180(), want);
    }
    #[test]
    fn rotate270() {
        let img: Tile = TEST_ROTATE.parse().expect("failed to part rotate image");
        let want: Tile = "Tile 0:\n..\n#."
            .parse()
            .expect("failed to parse rotate270 want");
        assert_eq!(img.rotate270(), want);
    }
    #[test]
    fn flip_horizontal() {
        let img: Tile = TEST_ROTATE.parse().expect("failed to part rotate image");
        let want: Tile = "Tile 0:\n.#\n.."
            .parse()
            .expect("failed to parse flip_horizontal want");
        assert_eq!(img.flip_horizontal(), want);
    }
    #[test]
    fn test_habitat() {
        let img: Tile = OUTPUT_IMAGE.parse().expect("failed to part want image");
        // TODO(wathiede) Reorient img until you find a seamonster.
        dbg!(img.count_hashes());
        dbg!(seamonster().count_hashes());
        assert_eq!(habitat(&reorient(&img, contains_seamonster)), 273);
    }
    #[test]
    fn test_solution2() {
        assert_eq!(solution2(&generator(&INPUT)), 273);
    }
}
