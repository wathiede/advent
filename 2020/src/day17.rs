//! --- Day 17: Conway Cubes ---
//! As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.
//!
//! The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.
//!
//! The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.
//!
//! In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.
//!
//! The energy source then proceeds to boot up by executing six cycles.
//!
//! Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//!
//! During a cycle, all cubes simultaneously change their state according to the following rules:
//!
//! If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
//! If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
//! The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.
//!
//! For example, consider the following initial state:
//!
//! .#.
//! ..#
//! ###
//! Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):
//!
//! Before any cycles:
//!
//! z=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1
//! #..
//! ..#
//! .#.
//!
//! z=0
//! #.#
//! .##
//! .#.
//!
//! z=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=0
//! ##...
//! ##...
//! #....
//! ....#
//! .###.
//!
//! z=1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//!
//! After 3 cycles:
//!
//! z=-2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//!
//! z=-1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=0
//! ...#...
//! .......
//! #......
//! .......
//! .....##
//! .##.#..
//! ...#...
//!
//! z=1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//! After the full six-cycle boot process completes, 112 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?

//! --- Part Two ---
//! For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
//!
//! The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.
//!
//! Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
//!
//! The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.
//!
//! For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
//!
//! Before any cycles:
//!
//! z=0, w=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=0, w=0
//! #.#
//! .##
//! .#.
//!
//! z=1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//! After the full six-cycle boot process completes, 848 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Cube {
    Active = b'#',
    Inactive = b'.',
}
impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cube::Active => '#',
                Cube::Inactive => '.',
            }
        )
    }
}

#[derive(Default, Clone)]
struct Universe<T> {
    cells: Vec<T>,
    x_len: usize,
    y_len: usize,
    z_len: usize,
    w_len: usize,
    default: T,
}

impl<T> Universe<T> {
    fn dimensions(&self) -> String {
        let u = &self;
        format!("{}x{}x{}x{}", u.x_len, u.y_len, u.z_len, u.w_len)
    }
}

impl<T> fmt::Debug for Universe<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.dimensions())?;
        let u = &self;
        for w in 0..u.w_len {
            for z in 0..u.z_len {
                let hdr = format!(
                    "z={}, w={}",
                    z as isize - u.z_len as isize / 2,
                    w as isize - u.w_len as isize / 2
                );
                write!(f, "{:width$} | ", hdr, width = u.x_len)?;
            }
            write!(f, "\n")?;
            for y in 0..u.y_len {
                for z in 0..u.z_len {
                    for x in 0..u.x_len {
                        write!(f, "{:?}", u[(x, y, z, w)])?;
                    }
                    write!(f, " | ")?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

use std::ops::{Index, IndexMut};

impl<T> IndexMut<(usize, usize, usize, usize)> for Universe<T> {
    fn index_mut(&mut self, (x, y, z, w): (usize, usize, usize, usize)) -> &mut Self::Output {
        if x >= self.x_len || y >= self.y_len || z > self.z_len || w > self.w_len {
            panic!(
                "index_mut outside of bounds ({},{},{},{})",
                x, y, z, w
            );
        }
        &mut self.cells[x
            + y * self.y_len
            + z * self.x_len * self.y_len
            + w * self.x_len * self.y_len * self.z_len]
    }
}

impl<T> Index<(usize, usize, usize, usize)> for Universe<T> {
    type Output = T;

    /// Returns the value in 4-space given by x,y,z,w.  Values outside the active space this Universe covers will return the default for T;
    fn index(&self, (x, y, z, w): (usize, usize, usize, usize)) -> &Self::Output {
        if x >= self.x_len || y >= self.y_len || z > self.z_len || w > self.w_len {
            return &self.default;
        }
        &self.cells[x
            + y * self.y_len
            + z * self.x_len * self.y_len
            + w * self.x_len * self.y_len * self.z_len]
    }
}

impl<T> Index<(isize, isize, isize, isize)> for Universe<T> {
    type Output = T;

    /// Returns the value in 4-space given by x,y,z,w.  Values outside the active space this Universe covers will return self.default;
    fn index(&self, (x, y, z, w): (isize, isize, isize, isize)) -> &Self::Output {
        if x < 0 || y < 0 || z < 0 || w < 0 {
            return &self.default;
        }

        let x_len = self.x_len as isize;
        let y_len = self.y_len as isize;
        let z_len = self.z_len as isize;
        let w_len = self.w_len as isize;

        if x >= x_len || y >= y_len || z >= z_len || w >= w_len {
            return &self.default;
        }

        &self.cells[(x + y * y_len + z * x_len * y_len + w * x_len * y_len * z_len) as usize]
    }
}

#[derive(Clone)]
struct PocketDimension {
    universe: Universe<Cube>,
}

impl std::str::FromStr for PocketDimension {
    type Err = ();
    fn from_str(s: &str) -> Result<PocketDimension, ()> {
        let mut cells = Vec::new();
        let z_layers: Vec<_> = s.split("\n\n").collect();
        let z_len = z_layers.len();
        let mut x_len = 0;
        let mut y_len = 0;
        z_layers.iter().for_each(|layer| {
            let rows: Vec<_> = layer.split('\n').map(|s| s.trim()).collect();
            y_len = rows.len();
            rows.iter().for_each(|row| {
                x_len = row.len();
                // TODO(wathiede): Is there something better here given we're using an enum with a
                // repr(u8)?
                cells.extend(row.bytes().filter(|c| c != &b'\n').map(|c| match c {
                    b'#' => Cube::Active,
                    b'.' => Cube::Inactive,
                    c => panic!("Unknown state '{}'", c),
                }));
            });
        });
        let universe = Universe {
            cells,
            x_len,
            y_len,
            z_len,
            w_len: 1,
            default: Cube::Inactive,
        };

        Ok(PocketDimension { universe })
    }
}

impl fmt::Debug for PocketDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.universe)
    }
}

impl PocketDimension {
    /// Applies the rules of the puzzle one iteration and returns a new PocketDimension
    /// representing the new state.
    fn step(&self, expand_w: bool) -> PocketDimension {
        let u = &self.universe;
        let x_len = u.x_len as isize;
        let y_len = u.y_len as isize;
        let z_len = u.z_len as isize;
        let w_len = u.w_len as isize;

        let (new_w_len, w_range, w_off) = if expand_w {
            (u.w_len + 2, -1..w_len + 1, 1)
        } else {
            (u.w_len, 0..w_len, 0)
        };

        let mut counts = Universe::<usize> {
            x_len: u.x_len + 2,
            y_len: u.y_len + 2,
            z_len: u.z_len + 2,
            w_len: new_w_len,
            cells: vec![0; (u.x_len + 2) * (u.y_len + 2) * (u.z_len + 2) * (new_w_len)],
            default: 0,
        };
        let mut universe = Universe::<Cube> {
            x_len: u.x_len + 2,
            y_len: u.y_len + 2,
            z_len: u.z_len + 2,
            w_len: new_w_len,
            cells: vec![
                Cube::Inactive;
                (u.x_len + 2) * (u.y_len + 2) * (u.z_len + 2) * (new_w_len)
            ],
            default: Cube::Inactive,
        };
        for w in w_range {
            for z in -1..z_len + 1 {
                for y in -1..y_len + 1 {
                    for x in -1..x_len + 1 {
                        let adj = self.adjacency((x, y, z, w));
                        let dst = (
                            (x + 1) as usize,
                            (y + 1) as usize,
                            (z + 1) as usize,
                            (w + w_off) as usize,
                        );
                        counts[dst] = adj;
                        match self.universe[(x, y, z, w)] {
                            Cube::Active => {
                                if adj == 2 || adj == 3 {
                                    universe[dst] = Cube::Active;
                                } else {
                                    universe[dst] = Cube::Inactive;
                                }
                            }
                            Cube::Inactive => {
                                if adj == 3 {
                                    universe[dst] = Cube::Active;
                                }
                            }
                        };
                    }
                }
            }
        }
        //dbg!(&counts, &universe);
        PocketDimension { universe }
    }
    fn active(&self) -> usize {
        self.universe
            .cells
            .iter()
            .filter(|c| c == &&Cube::Active)
            .count()
    }
    /// Counts active neighbors.
    fn adjacency(&self, (x, y, z, w): (isize, isize, isize, isize)) -> usize {
        let mut sum = 0;
        for w_off in -1..=1 {
            for z_off in -1..=1 {
                for y_off in -1..=1 {
                    for x_off in -1..=1 {
                        if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
                            // Skip the requested cell
                            continue;
                        }
                        if self.universe[(x + x_off, y + y_off, z + z_off, w + w_off)]
                            == Cube::Active
                        {
                            sum += 1;
                        }
                    }
                }
            }
        }
        sum
    }
}

#[aoc_generator(day17)]
fn generator(input: &str) -> PocketDimension {
    input.parse().expect("Couldn't parse initial state")
}

#[aoc(day17, part1)]
fn solution1(pd: &PocketDimension) -> usize {
    (0..6).fold(pd.clone(), |acc, _| acc.step(false)).active()
}

#[aoc(day17, part2)]
fn solution2(pd: &PocketDimension) -> usize {
    (0..6).fold(pd.clone(), |acc, _| acc.step(true)).active()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &'static str = r#".#.
        ..#
###"#;

    const STEPS1: &'static str = r#".#.
    ..#
###


#..
    ..#
    .#.

#.#
    .##
    .#.

#..
    ..#
    .#.


    .....
    .....
    ..#..
    .....
    .....

    ..#..
    .#..#
    ....#
    .#...
    .....

##...
##...
#....
    ....#
    .###.

    ..#..
    .#..#
    ....#
    .#...
    .....

    .....
    .....
    ..#..
    .....
    .....


    .......
    .......
    ..##...
    ..###..
    .......
    .......
    .......

    ..#....
    ...#...
#......
    .....##
    .#...#.
    ..#.#..
    ...#...

    ...#...
    .......
#......
    .......
    .....##
    .##.#..
    ...#...

    ..#....
    ...#...
#......
    .....##
    .#...#.
    ..#.#..
    ...#...

    .......
    .......
    ..##...
    ..###..
    .......
    .......
    ......."#;

    #[test]
    fn parse_and_count() {
        for (idx, ((input, active), dimensions)) in STEPS1
            .split("\n\n\n")
            .zip(vec![5, 11, 21, 38])
            .zip(vec!["3x3x1x1", "3x3x3x1", "5x5x5x1", "7x7x5x1"])
            .enumerate()
        {
            let pd = generator(input);
            assert_eq!(pd.active(), active);
            assert_eq!(
                pd.universe.dimensions(),
                dimensions,
                "idx {}: {:?}",
                idx,
                pd,
            );
        }
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&generator(INPUT1)), 112);
    }
    #[test]
    fn step_exand_w() {
        let pd = generator(INPUT1);
        assert_eq!(pd.active(), 5);
        let pd = pd.step(true);
        assert_eq!(pd.active(), 29);
        let pd = pd.step(true);
        assert_eq!(pd.active(), 60);
    }

    #[test]
    fn part2() {
        assert_eq!(solution2(&generator(INPUT1)), 848);
    }
}
