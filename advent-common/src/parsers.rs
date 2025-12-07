use std::{fmt::Debug, num::ParseIntError, ops::RangeInclusive, str::FromStr};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("failed to split '{0}' on '{1}'")]
    SplitError(String, char),
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("failed to parse range from '{0}': {1}")]
    ParseRangeValueError(String, Box<dyn std::error::Error>),
}

/// # Examples
///
/// ```
/// use advent::parsers::range_inclusive;
/// let s1 = "1-2,5-10,15-100";
/// let res = range_inclusive(s1).unwrap();
/// assert_eq!(res.0, vec![1..=2, 5..=10, 15..=100]);
/// assert_eq!(res.1, "");
///
/// let s2 = r#"1-2
/// 5-10
/// 15-100
///
/// 1
/// 2
/// 3"#;
/// let res = range_inclusive(s2).unwrap();
/// assert_eq!(res.0, vec![1..=2, 5..=10, 15..=100]);
/// assert_eq!(
///     res.1,
///     r#"1
/// 2
/// 3"#
/// );
/// ```
pub fn range_inclusive<Idx>(input: &str) -> Result<(Vec<RangeInclusive<Idx>>, &str), ParserError>
where
    Idx: FromStr + Debug,
    Idx::Err: Debug + std::error::Error,
    ParserError: From<<Idx as std::str::FromStr>::Err>,
{
    let (range_str, remainder) = match input.split_once("\n\n") {
        Some((s1, s2)) => (s1, s2),
        None => (input, ""),
    };

    let mut res = Vec::new();
    for seg in range_str.split([',', '\n']) {
        println!("seg {seg}");
        let (lo, hi) = seg
            .split_once('-')
            .ok_or_else(|| ParserError::SplitError(seg.to_string(), '-'))?;
        res.push(
            lo.parse::<Idx>().map_err(|e| {
                ParserError::ParseRangeValueError(lo.to_string(), Box::new(ParserError::from(e)))
            })?..=hi.parse::<Idx>().map_err(|e| {
                ParserError::ParseRangeValueError(hi.to_string(), Box::new(ParserError::from(e)))
            })?,
        );
    }

    Ok((res, remainder))
}
