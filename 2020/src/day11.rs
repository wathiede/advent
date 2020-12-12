//! --- Day 11: Seating System ---
//! Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//!
//! By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).
//!
//! The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:
//!
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:
//!
//! If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
//! If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
//! Otherwise, the seat's state does not change.
//! Floor (.) never changes; seats don't move, and nobody sits on the floor.
//!
//! After one round of these rules, every seat in the example layout becomes occupied:
//!
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! After a second round, the seats with four or more occupied adjacent seats become empty again:
//!
//! #.LL.L#.##
//! #LLLLLL.L#
//! L.L.L..L..
//! #LLL.LL.L#
//! #.LL.LL.LL
//! #.LLLL#.##
//! ..L.L.....
//! #LLLLLLLL#
//! #.LLLLLL.L
//! #.#LLLL.##
//! This process continues for three more rounds:
//!
//! #.##.L#.##
//! #L###LL.L#
//! L.#.#..#..
//! #L##.##.L#
//! #.##.LL.LL
//! #.###L#.##
//! ..#.#.....
//! #L######L#
//! #.LL###L.L
//! #.#L###.##
//!
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.L.L..#..
//! #LLL.##.L#
//! #.LL.LL.LL
//! #.LL#L#.##
//! ..L.L.....
//! #L#LLLL#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//!
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.#.L..#..
//! #L##.##.L#
//! #.#L.LL.LL
//! #.#L#L#.##
//! ..L.L.....
//! #L#L##L#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//!
//! At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.
//!
//! Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

use std::convert::TryFrom;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq)]
enum State {
    /// '.'
    Floor,
    /// 'L'
    Empty,
    /// '#'
    Occupied,
}

use std::fmt;
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Floor => write!(f, "."),
            State::Empty => write!(f, "L"),
            State::Occupied => write!(f, "#"),
        }
    }
}

impl FromStr for State {
    type Err = String;
    fn from_str(s: &str) -> Result<State, String> {
        match s {
            "." => Ok(State::Floor),
            "L" => Ok(State::Empty),
            "#" => Ok(State::Occupied),
            s => Err(format!("Unknown map character: '{}'", s)),
        }
    }
}

impl TryFrom<char> for State {
    type Error = String;
    fn try_from(c: char) -> Result<State, String> {
        match c {
            '.' => Ok(State::Floor),
            '#' => Ok(State::Occupied),
            'L' => Ok(State::Empty),
            c => Err(format!("Unknown map character: '{}'", c)),
        }
    }
}

#[derive(PartialEq)]
struct Map {
    cells: Vec<State>,
    width: usize,
    height: usize,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in self.cells.chunks(self.width) {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Map, String> {
        let mut cells = Vec::new();
        let rows: Vec<_> = s.split("\n").collect();
        for row in &rows {
            let c: Result<Vec<_>, _> = row.chars().map(|cell| State::try_from(cell)).collect();
            cells.extend(c?);
        }
        let height = rows.len();
        let width = cells.len() / height;
        Ok(Map {
            cells,
            height,
            width,
        })
    }
}

use std::ops::{Index, IndexMut};

impl Index<(usize, usize)> for Map {
    type Output = State;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[x + y * self.width]
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            width,
            height,
            cells: vec![State::Empty; width * height],
        }
    }
    fn adjacent_count(&self, x: usize, y: usize) -> usize {
        use std::cmp::min;
        let x_min = x.saturating_sub(1);
        let y_min = y.saturating_sub(1);
        let x_max = min(x + 1, self.width - 1);
        let y_max = min(y + 1, self.height - 1);

        let mut cnt = 0;
        for y_off in y_min..=y_max {
            for x_off in x_min..=x_max {
                // Skip the current cell
                if x == x_off && y == y_off {
                    continue;
                }
                if self[(x_off, y_off)] == State::Occupied {
                    cnt += 1
                }
            }
        }
        cnt
    }

    fn occupied_count(&self) -> usize {
        self.cells.iter().filter(|&c| c == &State::Occupied).count()
    }
}

fn step(map: &Map) -> Map {
    let mut new_m = Map::new(map.width, map.height);
    for y in 0..map.height {
        for x in 0..map.width {
            // Floor's never change
            if map[(x, y)] == State::Floor {
                new_m[(x, y)] = State::Floor;
                continue;
            }
            let new_cell = match map.adjacent_count(x, y) {
                0 => State::Occupied,
                c if c >= 4 => State::Empty,
                _ => map[(x, y)],
            };
            new_m[(x, y)] = new_cell;
        }
    }
    new_m
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Map {
    input.parse().expect("Failed to parse map")
}

#[aoc(day11, part1)]
fn solution1(map: &Map) -> usize {
    let mut prev = step(map);
    let mut cur = step(&prev);
    while prev != cur {
        // Show map animating.
        // println!("{}", cur);
        prev = cur;
        cur = step(&prev);
    }
    cur.occupied_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index() {
        let m: Map = "#.L\n.#L\n.L#".parse().expect("Failed to parse map");
        assert_eq!(m[(1, 0)], State::Floor);
        assert_eq!(m[(0, 1)], State::Floor);
        assert_eq!(m[(0, 2)], State::Floor);

        assert_eq!(m[(2, 0)], State::Empty);
        assert_eq!(m[(2, 1)], State::Empty);
        assert_eq!(m[(1, 2)], State::Empty);

        assert_eq!(m[(0, 0)], State::Occupied);
        assert_eq!(m[(1, 1)], State::Occupied);
        assert_eq!(m[(2, 2)], State::Occupied);
    }

    #[test]
    fn solution1() {
        let input = r#"L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL"#
            .replace(' ', "");
        let steps: Vec<_> = vec![
            r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
                ..#.#.....
##########
#.######.#
#.#####.##"#,
            r#"#.LL.L#.##
#LLLLLL.L#
            L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
            ..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"#,
            r#"#.##.L#.##
#L###LL.L#
            L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
            ..#.#.....
#L######L#
#.LL###L.L
#.#L###.##"#,
            r#"#.#L.L#.##
#LLL#LL.L#
            L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
            ..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##"#,
            r#"#.#L.L#.##
#LLL#LL.L#
            L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
            ..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"#,
        ]
        .iter()
        // Trim whitespace that rustfmt keeps introducing.
        .map(|m| m.replace(' ', ""))
        .collect();

        let mut m = input.parse().expect("Failed to parse map");
        for (i, want_input) in steps.iter().enumerate() {
            let want: Map = want_input
                .parse()
                .expect(&format!("Failed to parse step {}", i));
            let got = step(&m);
            assert_eq!(want, got, "step {}\nm {}", i, m);
            m = got;
        }
    }
}
