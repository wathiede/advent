use std::{fmt::Debug, ops::RangeInclusive, str::FromStr};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {}

/// # Examples
///
/// ```
/// use advent::parsers::range_inclusive;
/// let s1 = "1-2,5-10,15-100";
/// let res = range_inclusive(s1).unwrap();
/// assert_eq!(res.0.collect::<Vec<_>>(), vec![1..=2, 5..=10, 15..=100]);
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
/// assert_eq!(res.0.collect::<Vec<_>>(), vec![1..=2, 5..=10, 15..=100]);
/// assert_eq!(
///     res.1,
///     r#"1
/// 2
/// 3"#
/// );
/// ```
pub fn range_inclusive<Idx>(
    input: &str,
) -> Result<
    (
        impl Iterator<Item = RangeInclusive<Idx>> + use<'_, Idx>,
        &str,
    ),
    ParserError,
>
where
    Idx: FromStr + Debug,
    Idx::Err: Debug,
{
    let (range_str, remainder) = match input.split_once("\n\n") {
        Some((s1, s2)) => (s1, s2),
        None => (input, ""),
    };
    Ok((
        range_str.split([',', '\n']).map(|seg| {
            println!("seg {seg}");
            let (lo, hi) = seg.split_once('-').expect("failed to split -");
            lo.parse::<Idx>().expect("lo failed")..=hi.parse::<Idx>().expect("hi failed")
        }),
        remainder,
    ))
}
