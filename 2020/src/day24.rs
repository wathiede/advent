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
//!
//! --- Part Two ---
//! The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all flipped according to the following rules:
//!
//! Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
//! Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
//! Here, tiles immediately adjacent means the six tiles directly touching the tile in question.
//!
//! The rules are applied simultaneously to every tile; put another way, it is first determined which tiles need to be flipped, then they are all flipped at the same time.
//!
//! In the above example, the number of black tiles that are facing up after the given number of days has passed is as follows:
//!
//! Day 1: 15
//! Day 2: 12
//! Day 3: 25
//! Day 4: 14
//! Day 5: 23
//! Day 6: 28
//! Day 7: 41
//! Day 8: 37
//! Day 9: 49
//! Day 10: 37
//!
//! Day 20: 132
//! Day 30: 259
//! Day 40: 406
//! Day 50: 566
//! Day 60: 788
//! Day 70: 1106
//! Day 80: 1373
//! Day 90: 1844
//! Day 100: 2208
//! After executing this process a total of 100 times, there would be 2208 black tiles facing up.
//!
//! How many tiles will be black after 100 days?

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
                    c => panic!("unexpected tile direction {}", c),
                },
                b's' => match it.next().unwrap() {
                    b'e' => directions.push(SouthEast),
                    b'w' => directions.push(SouthWest),
                    c => panic!("unexpected tile direction {}", c),
                },
                b'e' => directions.push(East),
                b'w' => directions.push(West),
                c => panic!("unexpected tile direction {}", c),
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

fn follow_instructions(instructions: &[Tile]) -> HashMap<TileCoord, bool> {
    // False == white
    // True == black
    // Default == white
    let mut tiles: HashMap<TileCoord, bool> = HashMap::new();
    instructions.iter().for_each(|t| {
        let v = tiles.entry(t.coord()).or_insert(false);
        *v = !*v;
    });
    tiles
}

#[aoc(day24, part1)]
fn solution1(instructions: &[Tile]) -> usize {
    let tiles = follow_instructions(instructions);
    count_black(&tiles)
}

const NEIGHBOR_OFFSETS: [(isize, isize, isize); 6] = [
    (-1, 1, 0),
    (1, -1, 0),
    (-1, 0, 1),
    (1, 0, -1),
    (0, -1, 1),
    (0, 1, -1),
];

fn count_neighbors(coord: &TileCoord, tiles: &HashMap<TileCoord, bool>) -> usize {
    let (x, y, z) = coord.0;
    NEIGHBOR_OFFSETS
        .iter()
        .filter(|(x_o, y_o, z_o)| {
            *tiles
                .get(&TileCoord((x + x_o, y + y_o, z + z_o)))
                .unwrap_or(&false)
        })
        .count()
}

fn count_black(tiles: &HashMap<TileCoord, bool>) -> usize {
    tiles.values().filter(|v| **v).count()
}

fn step(tiles: HashMap<TileCoord, bool>) -> HashMap<TileCoord, bool> {
    let mut output = HashMap::new();
    tiles
        .iter()
        .filter_map(|(k, v)| if *v { Some(k) } else { None })
        .for_each(|coord| {
            match count_neighbors(coord, &tiles) {
                1 | 2 => {
                    // Leave black
                    output.insert(*coord, true);
                }
                _ => {
                    // 0 or >=2, default is white, so don't set anything in new map.
                }
            };

            let (x, y, z) = coord.0;
            // TODO search white neighbors.
            NEIGHBOR_OFFSETS.iter().for_each(|(x_o, y_o, z_o)| {
                let coord = TileCoord((x + x_o, y + y_o, z + z_o));
                if *tiles.get(&coord).unwrap_or(&false) {
                    // Black, we can skip
                    return;
                }
                if count_neighbors(&coord, &tiles) == 2 {
                    output.insert(coord, true);
                }
            });
        });
    output
}

#[aoc(day24, part2)]
fn solution2(instructions: &[Tile]) -> usize {
    let tiles = follow_instructions(instructions);
    let tiles = (0..100).fold(tiles, |tiles, _| step(tiles));
    count_black(&tiles)
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

    #[test]
    fn test_step() {
        let instructions = parse(INPUT);
        let tiles = follow_instructions(&instructions);
        let wants = vec![15, 12, 25, 14, 23, 28, 41, 37, 49, 37];
        wants
            .iter()
            .enumerate()
            .fold(tiles, |mut tiles, (i, want)| {
                tiles = step(tiles);
                assert_eq!(count_black(&tiles), *want, "step {}", i);
                tiles
            });
    }

    #[test]
    fn part2() {
        assert_eq!(solution2(&parse(INPUT)), 2208);
    }
}
