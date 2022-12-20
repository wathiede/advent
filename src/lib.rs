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

    pub use crate::{image::Image, vprint};
}

mod image;
#[macro_export]
macro_rules! vprint {
    ($($x:tt)*) => { if VERBOSE { println!($($x)*); } }
}
