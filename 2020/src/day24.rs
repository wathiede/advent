//! --- Day 24: Lobby Layout ---
//! Your raft makes it to the tropical island; it turns out that the small crab was an excellent navigator. You make your way to the resort.
//!
//! As you enter the lobby, you discover a small problem: the floor is being renovated. You can't even reach the check-in desk until they've finished installing the new tile floor.
//!
//! The tiles are all hexagonal; they need to be arranged in a hex grid with a very specific color pattern. Not in the mood to wait, you offer to help figure out the pattern.
//!
//! The tiles are all white on one side and black on the other. They start with the white side facing up. The lobby is large enough to fit whatever pattern might need to appear there.
//!
//! A member of the renovation crew gives you a list of the tiles that need to be flipped over (your puzzle input). Each line in the list identifies a single tile that needs to be flipped by giving a series of steps starting from a reference tile in the very center of the room. (Every line starts from the same reference tile.)
//!
//! Because the tiles are hexagonal, every tile has six neighbors: east, southeast, southwest, west, northwest, and northeast. These directions are given in your list, respectively, as e, se, sw, w, nw, and ne. A tile is identified by a series of these directions with no delimiters; for example, esenee identifies the tile you land on if you start at the reference tile and then move one tile east, one tile southeast, one tile northeast, and one tile east.
//!
//! Each time a tile is identified, it flips from white to black or from black to white. Tiles might be flipped more than once. For example, a line like esew flips a tile immediately adjacent to the reference tile, and a line like nwwswee flips the reference tile itself.
//!
//! Here is a larger example:
//!
//! sesenwnenenewseeswwswswwnenewsewsw
//! neeenesenwnwwswnenewnwwsewnenwseswesw
//! seswneswswsenwwnwse
//! nwnwneseeswswnenewneswwnewseswneseene
//! swweswneswnenwsewnwneneseenw
//! eesenwseswswnenwswnwnwsewwnwsene
//! sewnenenenesenwsewnenwwwse
//! wenwwweseeeweswwwnwwe
//! wsweesenenewnwwnwsenewsenwwsesesenwne
//! neeswseenwwswnwswswnw
//! nenwswwsewswnenenewsenwsenwnesesenew
//! enewnwewneswsewnwswenweswnenwsenwsw
//! sweneswneswneneenwnewenewwneswswnese
//! swwesenesewenwneswnwwneseswwne
//! enesenwswwswneneswsenwnewswseenwsese
//! wnwnesenesenenwwnenwsewesewsesesew
//! nenewswnwewswnenesenwnesewesw
//! eneswnwswnwsenenwnwnwwseeswneewsenese
//! neswnwewnwnwseenwseesewsenwsweewe
//! wseweeenwnesenwwwswnew
//! In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to black, then back to white). After all of these instructions have been followed, a total of 10 tiles are black.
//!
//! Go through the renovation crew's list and determine which tiles they need to flip. After all of the instructions have been followed, how many tiles are left with the black side up?
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TileCoord((isize, isize, isize));

#[derive(Debug, PartialEq)]
struct Tile {
    directions: Vec<Direction>,
}

impl std::str::FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Tile, ()> {
        let mut it = s.bytes();
        let mut directions = Vec::new();
        use Direction::*;
        while let Some(b) = it.next() {
            match b {
                b'n' => match it.next().unwrap() {
                    b'e' => directions.push(NorthEast),
                    b'w' => directions.push(NorthWest),
                    c => panic!(format!("unexpected tile direction {}", c)),
                },
                b's' => match it.next().unwrap() {
                    b'e' => directions.push(SouthEast),
                    b'w' => directions.push(SouthWest),
                    c => panic!(format!("unexpected tile direction {}", c)),
                },
                b'e' => directions.push(East),
                b'w' => directions.push(West),
                c => panic!(format!("unexpected tile direction {}", c)),
            }
        }
        Ok(Tile { directions })
    }
}
impl Tile {
    fn coord(&self) -> TileCoord {
        // Based on 'cube coordinates' from https://www.redblobgames.com/grids/hexagons/
        TileCoord(
            self.directions
                .iter()
                .fold((0, 0, 0), |(x, y, z), dir| match dir {
                    Direction::East => (x + 1, y - 1, z),
                    Direction::SouthEast => (x, y - 1, z + 1),
                    Direction::SouthWest => (x - 1, y, z + 1),
                    Direction::West => (x - 1, y + 1, z),
                    Direction::NorthWest => (x, y + 1, z - 1),
                    Direction::NorthEast => (x + 1, y, z - 1),
                }),
        )
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Tile> {
    input
        .split('\n')
        .map(|l| l.parse().expect("Failed to parse tile"))
        .collect()
}

#[aoc(day24, part1)]
fn solution1(tiles: &[Tile]) -> usize {
    let mut colors: HashMap<TileCoord, bool> = HashMap::new();
    tiles.iter().for_each(|t| {
        let v = colors.entry(t.coord()).or_insert(false);
        *v = !*v;
    });
    colors.values().filter(|v| **v).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
"#;

    #[test]
    fn tile() {
        use Direction::*;
        assert_eq!(
            "esenee".parse::<Tile>().expect("failed to parse tile"),
            Tile {
                directions: vec![East, SouthEast, NorthEast, East]
            }
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&parse(INPUT)), 10);
    }
}
