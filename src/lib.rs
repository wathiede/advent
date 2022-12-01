pub mod prelude {
    pub use std::{
        collections::{HashMap, HashSet, VecDeque},
        convert::Infallible,
        fmt::{Debug, Display, Error, Formatter},
        io::Read,
        num::ParseIntError,
        ops::{Index, IndexMut, RangeInclusive},
        str::FromStr,
    };

    pub use anyhow::Result;
    pub use thiserror::Error;

    pub use crate::image::Image;
}

mod image;
