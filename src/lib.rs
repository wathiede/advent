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

    pub use crate::{image::Image, input_for, parsers::range_inclusive, vprint};
}

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
