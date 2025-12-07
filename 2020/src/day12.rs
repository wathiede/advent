//! --- Day 12: Rain Risk ---
//! Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:
//!
//! Action N means to move north by the given value.
//! Action S means to move south by the given value.
//! Action E means to move east by the given value.
//! Action W means to move west by the given value.
//! Action L means to turn left the given number of degrees.
//! Action R means to turn right the given number of degrees.
//! Action F means to move forward by the given value in the direction the ship is currently facing.
//! The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)
//!
//! For example:
//!
//! F10
//! N3
//! F7
//! R90
//! F11
//! These instructions would be handled as follows:
//!
//! F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
//! N3 would move the ship 3 units north to east 10, north 3.
//! F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
//! R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
//! F11 would move the ship 11 units south to east 17, south 8.
//! At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//!
//! Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

//! --- Part Two ---
//! Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.
//!
//! Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
//!
//! Action N means to move the waypoint north by the given value.
//! Action S means to move the waypoint south by the given value.
//! Action E means to move the waypoint east by the given value.
//! Action W means to move the waypoint west by the given value.
//! Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//! Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//! Action F means to move forward to the waypoint a number of times equal to the given value.
//! The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
//!
//! For example, using the same instructions as above:
//!
//! F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
//! N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
//! F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
//! R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
//! F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
//! After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
//!
//! Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
enum Action {
    North(u32),
    South(u32),
    East(u32),
    West(u32),

    Right(u32),
    Left(u32),

    Forward(u32),
}

#[derive(Clone, Copy)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl From<i32> for Orientation {
    fn from(i: i32) -> Orientation {
        assert_eq!(i % 90, 0);
        match ((i + 360) % 360) / 90 {
            0 => Orientation::North,
            1 => Orientation::East,
            2 => Orientation::South,
            3 => Orientation::West,
            c => panic!("Should never see orientation of {}", c),
        }
    }
}
impl Into<i32> for Orientation {
    fn into(self) -> i32 {
        match self {
            Orientation::North => 0,
            Orientation::East => 90,
            Orientation::South => 180,
            Orientation::West => 270,
        }
    }
}

use std::str::FromStr;

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, String> {
        let c = s
            .chars()
            .nth(0)
            .ok_or("Couldn't get first char".to_string())?;
        let v = s[1..]
            .parse::<u32>()
            .map_err(|e| format!("{}: '{}'", e, s))?;
        use Action::*;
        Ok(match c {
            'N' => North(v),
            'S' => South(v),
            'E' => East(v),
            'W' => West(v),

            'R' => Right(v),
            'L' => Left(v),

            'F' => Forward(v),
            c => return Err(format!("Unexpected action character '{}'", c)),
        })
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Action> {
    input
        .split('\n')
        .map(|l| l.parse().expect("Failed to parse action"))
        .collect()
}

struct Waypoint {
    // East is +, West is -.
    x: i32,
    // North is +, South is -.
    y: i32,
}

struct Ship {
    orientation: Orientation,
    // East is +, West is -.
    x: i32,
    // North is +, South is -.
    y: i32,
    waypoint: Option<Waypoint>,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            orientation: Orientation::East,
            x: 0,
            y: 0,
            waypoint: None,
        }
    }
}

impl Ship {
    fn new(waypoint_x_offset: i32, waypoint_y_offset: i32) -> Ship {
        Ship {
            waypoint: Some(Waypoint {
                x: waypoint_x_offset,
                y: waypoint_y_offset,
            }),
            ..Ship::default()
        }
    }

    fn act_part1(&mut self, action: &Action) {
        match action {
            Action::North(v) => self.y += *v as i32,
            Action::South(v) => self.y -= *v as i32,
            Action::East(v) => self.x += *v as i32,
            Action::West(v) => self.x -= *v as i32,

            Action::Right(v) => {
                self.orientation = {
                    let i: i32 = self.orientation.into();
                    (i + *v as i32).into()
                }
            }
            Action::Left(v) => {
                self.orientation = {
                    let i: i32 = self.orientation.into();
                    (i - *v as i32).into()
                }
            }
            Action::Forward(v) => match self.orientation {
                Orientation::North => self.y += *v as i32,
                Orientation::South => self.y -= *v as i32,
                Orientation::East => self.x += *v as i32,
                Orientation::West => self.x -= *v as i32,
            },
        };
    }

    fn act_part2(&mut self, action: &Action) {
        let mut wp = self.waypoint.take().unwrap();
        match action {
            Action::North(v) => wp.y += *v as i32,
            Action::South(v) => wp.y -= *v as i32,
            Action::East(v) => wp.x += *v as i32,
            Action::West(v) => wp.x -= *v as i32,

            Action::Right(v) => {
                assert_eq!(v % 90, 0);
                for _ in 0..(v / 90) {
                    std::mem::swap(&mut wp.x, &mut wp.y);
                    wp.y *= -1;
                }
            }
            Action::Left(v) => {
                assert_eq!(v % 90, 0);
                for _ in 0..(v / 90) {
                    std::mem::swap(&mut wp.x, &mut wp.y);
                    wp.x *= -1;
                }
            }

            Action::Forward(v) => {
                self.x += wp.x * *v as i32;
                self.y += wp.y * *v as i32;
            }
        };
        self.waypoint = Some(wp);
    }

    fn act(&mut self, action: &Action) {
        match self.waypoint {
            None => self.act_part1(action),
            Some(_) => self.act_part2(action),
        };
    }

    fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[aoc(day12, part1)]
fn solution1(actions: &[Action]) -> u32 {
    let mut s = Ship::default();
    actions.iter().for_each(|a| s.act(a));
    s.manhattan_distance()
}

#[aoc(day12, part2)]
fn solution2(actions: &[Action]) -> u32 {
    let mut s = Ship::new(10, 1);
    actions.iter().for_each(|a| s.act(a));
    s.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn parser() {
        use Action::*;
        assert_eq!(
            parse(INPUT),
            vec![Forward(10), North(3), Forward(7), Right(90), Forward(11),]
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&parse(INPUT)), 17 + 8);
    }
    #[test]
    fn part2() {
        assert_eq!(solution2(&parse(INPUT)), 214 + 72);
    }
}
