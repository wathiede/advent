pub mod prelude {
    pub use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet, VecDeque},
        convert::Infallible,
        fmt,
        fmt::{Debug, Display, Error, Formatter},
        io::Read,
        num::ParseIntError,
        ops::{Index, IndexMut, Range, RangeInclusive},
        str::FromStr,
    };

    pub use anyhow::Result;
    pub use thiserror::Error;

    pub use crate::{image::Image, input_for, parsers::range_inclusive, vprint, BitSet, Vec3};
}

use std::{
    fmt,
    ops::{Add, Sub},
    str::FromStr,
};

mod image;
pub mod parsers;

#[macro_export]
macro_rules! vprint {
    ($($x:tt)*) => { if VERBOSE { println!($($x)*); } }
}

pub fn input_for(year: u16, day: u16) -> String {
    let path = format!("input/{year}/day{day}.txt");
    let mut s =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    // String newline at end of file
    s.truncate(s.len() - 1);
    s
}

/// Example:
///
/// ```
/// use advent::BitSet;
///
/// let mut bs = BitSet::new(111);
///
/// bs.set(71);
/// assert!(bs.is_set(71));
///
/// assert!(!bs.is_set(70));
/// assert!(!bs.is_set(5));
///
/// assert!(bs.is_set(71));
/// assert_eq!(bs.to_string(), ".......................................................................+........................................................");
/// bs.clear(71);
/// assert!(!bs.is_set(71));
/// assert_eq!(bs.to_string(), "................................................................................................................................");
/// ```
pub struct BitSet {
    bits: Vec<u64>,
}

impl BitSet {
    pub fn new(num_bits: usize) -> Self {
        BitSet {
            bits: vec![0u64; num_bits.div_ceil(64)],
        }
    }
    pub fn set(&mut self, v: usize) {
        let idx = v / 64;
        let off = v % 64;
        self.bits[idx] |= 1 << off;
    }
    pub fn clear(&mut self, v: usize) {
        let idx = v / 64;
        let off = v % 64;
        self.bits[idx] &= !(1 << off);
    }
    pub fn is_set(&mut self, v: usize) -> bool {
        let idx = v / 64;
        let off = v % 64;
        self.bits[idx] & (1 << off) > 0
    }
    /*
    pub fn iter(&self) -> BitSetIter {
        let it = self.bits.iter().enumerate().flat_map(|(i, b)| {
            (0..64).filter_map(move |off| {
                if b & 1 << off > 0 {
                    Some(i * 64 + off)
                } else {
                    None
                }
            })
        });
        BitSetIter { it: Box::new(it) }
    }
    */
}

pub struct BitSetIter {
    it: Box<dyn Iterator<Item = usize>>,
}
/// Returns the bit index that are set
impl Iterator for BitSetIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bits {
            for i in 0..64 {
                let v = if b & 1 << i > 0 { "+" } else { "." };
                write!(f, "{v}")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Vec3([i64; 3]);

impl Vec3 {
    pub fn distance_squared(&self, rhs: &Vec3) -> i64 {
        ((rhs.0[0] - self.0[0]) * (rhs.0[0] - self.0[0])
            + (rhs.0[1] - self.0[1]) * (rhs.0[1] - self.0[1])
            + (rhs.0[2] - self.0[2]) * (rhs.0[2] - self.0[2]))
            .abs()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<{:4},{:4},{:4}>", self.0[0], self.0[1], self.0[2])
    }
}

impl FromStr for Vec3 {
    // TODO: make this a real type
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> std::result::Result<Vec3, std::convert::Infallible> {
        let v: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Vec3(v.try_into().unwrap()))
    }
}
