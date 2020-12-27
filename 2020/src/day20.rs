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

use crate::debug_println;

#[derive(Clone, Default, Hash, Eq, PartialEq)]
struct Tile {
    id: usize,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {} ({}x{}):\n", self.id, self.width, self.height)?;
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

#[cfg(any(debug_assertions, test))]
fn border_to_str(border: &[u8]) -> String {
    std::str::from_utf8(border).unwrap().to_string()
}

impl Tile {
    /// Copy `t` into self @ x_off,y_off.
    fn blit(&mut self, t: &Tile, x_off: usize, y_off: usize) {
        debug_println!(
            "blitting tile {} {}x{} @ {},{}",
            t.id,
            t.width,
            t.height,
            x_off,
            y_off
        );
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
fn reorient<F>(img: &Tile, predicate: F) -> Option<Tile>
where
    F: Fn(&Tile) -> bool,
{
    if predicate(&img) {
        return Some(img.clone());
    }
    let rotated = img.rotate90();
    if predicate(&rotated) {
        return Some(rotated);
    }

    let rotated = img.rotate180();
    if predicate(&rotated) {
        return Some(rotated);
    }

    let rotated = img.rotate270();
    if predicate(&rotated) {
        return Some(rotated);
    }

    let horiz = img.flip_horizontal();
    if predicate(&horiz) {
        return Some(horiz);
    }

    let rotated = horiz.rotate90();
    if predicate(&rotated) {
        return Some(rotated);
    }

    let rotated = horiz.rotate180();
    if predicate(&rotated) {
        return Some(rotated);
    }

    let rotated = horiz.rotate270();
    if predicate(&rotated) {
        return Some(rotated);
    }
    None
}

fn stitch(tiles: &[Tile]) -> Tile {
    // Make sure there's a square number of tiles.
    let sqrt = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(sqrt * sqrt, tiles.len());

    let width = sqrt * (tiles[0].width - 2);
    let height = sqrt * (tiles[0].height - 2);
    let mut image = Tile {
        id: 0,
        width,
        height,
        pixels: vec![b'X'; width * height],
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

    #[cfg(any(debug_assertions, test))]
    border_counts.iter().for_each(|(b, c)| {
        let _ = b;
        let _ = c;
        debug_println!("{}: {}", border_to_str(b), c);
    });

    let edge_borders: HashSet<_> = border_counts
        .iter()
        .filter(|(_b, c)| **c == 1)
        .map(|(b, _c)| b)
        .collect();
    // Count the number of borders that are in edge_borders.  The answer should be 0, 1 or 2
    // if the tile is a middle, edge or corner, respectively.
    let (corner_tiles, _edge_tiles, _center_tiles) = tiles.iter().fold(
        (vec![], vec![], vec![]),
        |(mut corner, mut edge, mut center), t| {
            let edge_count = vec![
                t.top_border(),
                t.right_border(),
                t.bottom_border(),
                t.left_border(),
            ]
            .into_iter()
            .filter(|b| edge_borders.contains(b))
            .count();
            match edge_count {
                0 => center.push(t),
                1 => edge.push(t),
                2 => corner.push(t),
                c => panic!(format!("unexpected edge_count for {:?}: {}", t, c)),
            };
            (corner, edge, center)
        },
    );

    let mut tile_map = vec![vec![None; sqrt]; sqrt];
    let corner = corner_tiles[0];
    // Make this the upper-left corner at 0,0.
    let corner = reorient(corner, |im| {
        edge_borders.contains(&im.left_border()) && edge_borders.contains(&im.top_border())
    })
    .expect("couldn't find proper orientation");
    let mut remaining_tiles: HashSet<_> = tiles.iter().filter(|t| t.id != corner.id).collect();
    let mut last = corner.clone();
    tile_map[0][0] = Some(corner);
    (0..sqrt)
        .map(|y| (0..sqrt).map(move |x| (x, y)))
        .flatten()
        .skip(1)
        .for_each(|(x, y)| {
            debug_println!("Solving for tile {},{}", x, y);
            let mut local_last = last.clone();
            let orientation_check: Box<dyn Fn(&Tile) -> bool> = if y == 0 {
                debug_println!("search for top row tiles");
                // Top row, tiles should be match the tile to the left and have their top_border in the
                // edge set.
                // Find a tile that matches last and reorient so it's edge is on top.
                Box::new(|im: &Tile| {
                    edge_borders.contains(&im.top_border())
                        && im.left_border() == local_last.right_border()
                })
            } else if x == 0 {
                debug_println!("search for left column tiles");
                // When we're in the first column, we need to match against the tile above, instead of
                // the last tile on the previous row.
                local_last = tile_map[0][y - 1]
                    .as_ref()
                    .expect(&format!("couldn't file tile above {},{}", x, y))
                    .clone();
                Box::new(|im: &Tile| {
                    edge_borders.contains(&im.left_border())
                        && im.top_border() == local_last.bottom_border()
                })
            } else {
                debug_println!("search for regular tiles");
                // Default, last is to the left match shared edge.
                Box::new(|im: &Tile| im.left_border() == local_last.right_border())
            };

            debug_println!("last tile {}", last.id);
            let mut found: Option<Tile> = None;
            for candidate in &remaining_tiles {
                match reorient(candidate, &orientation_check) {
                    Some(good) => {
                        debug_println!("found3 {}", good.id);
                        found = Some(good);
                        break;
                    }
                    None => continue,
                };
            }
            match found {
                Some(rm) => {
                    debug_println!(
                        "rm3 {} {:?}",
                        rm.id,
                        remaining_tiles.iter().map(|t| t.id).collect::<Vec<_>>()
                    );
                    last = rm.clone();
                    tile_map[x][y] = Some(last.clone());
                    let rm = remaining_tiles
                        .iter()
                        .filter(|t| t.id == rm.id)
                        .nth(0)
                        .expect(&format!("Couldn't find {} in remaining_tiles", rm.id))
                        .clone();
                    remaining_tiles.remove(rm);
                }
                None => panic!("couldn't find match for {},{}", x, y),
            };
        });
    debug_println!("Stitched titles");
    #[cfg(debug_assertions)]
    (0..sqrt).for_each(|y| {
        let row_ids: Vec<_> = (0..sqrt)
            .map(|x| tile_map[x][y].as_ref().unwrap().id)
            .collect();
        debug_println!("{:?}", row_ids);
    });
    (0..sqrt)
        .map(|y| (0..sqrt).map(move |x| (x, y)))
        .flatten()
        .for_each(|(x, y)| {
            let t = tile_map[x][y]
                .as_ref()
                .expect(&format!("missing tile {},{} in completed tile_map", x, y));
            let out = t.strip_border();
            image.blit(&out, x * out.width, y * out.height);
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
    let full_map = stitch(tiles);
    debug_println!("Full map\n{:?}", full_map);
    habitat(&reorient(&full_map, contains_seamonster).expect("couldn't find proper orientation"))
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
                .expect("couldn't find proper orientation")
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
        dbg!(img.count_hashes());
        dbg!(seamonster().count_hashes());
        assert_eq!(
            habitat(
                &reorient(&img, contains_seamonster).expect("couldn't find proper orientation")
            ),
            273
        );
    }
    #[test]
    fn test_stitch() {
        let want: Tile = OUTPUT_IMAGE.parse().expect("can't parse stitched input");
        let output = stitch(&generator(INPUT));
        let output = reorient(&output, contains_seamonster);

        match output {
            None => assert!(false, "Failed to reorient stitched image to reference"),
            Some(im) => {
                dbg!(&im);
                assert_eq!(
                    habitat(&im),
                    273,
                    "\n  im {}\nwant {}",
                    border_to_str(&im.pixels),
                    border_to_str(&want.pixels)
                );
            }
        }
    }
    #[test]
    fn test_solution2() {
        assert_eq!(solution2(&generator(&INPUT)), 273);
    }
}
