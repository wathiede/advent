use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

fn expand(template: &[u8], rules: &HashMap<&[u8], u8>) -> Vec<u8> {
    let mut res = vec![0u8; template.len() * 2 - 1];
    template.windows(2).enumerate().for_each(|(i, xy)| {
        let z = rules[xy];
        res[i * 2] = xy[0];
        res[i * 2 + 1] = z;
        res[i * 2 + 2] = xy[1];
    });
    //dbg!(String::from_utf8_lossy(&res));
    res
}

fn count(template: &[u8]) -> (usize, usize) {
    let m = template
        .iter()
        .fold(HashMap::<u8, usize>::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
    m.values()
        .fold((usize::MAX, 0), |(min, max), v| (min.min(*v), max.max(*v)))
}

#[aoc(day14, part1)]
fn part1(input: &str) -> Result<usize> {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<&[u8], u8> = rules
        .lines()
        .map(|l| {
            let (pair, insert) = l.split_once(" -> ").unwrap();
            (pair.as_bytes(), insert.as_bytes()[0])
        })
        .collect();
    let mut template = template.as_bytes().to_vec();
    for i in 1..11 {
        template = expand(&template, &rules);
        //println!("After step {}: {}", i, String::from_utf8_lossy(&template));
    }
    let (min, max) = count(&template);
    Ok(max - min)
}

/*
#[aoc(day14, part2)]
fn part2(input: &[u64]) -> Result<u64> {
todo!("part2");
Ok(0)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#
        .trim();
        assert_eq!(part1(input)?, 1588);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
    let input = r#"
    "#
    .trim();
    assert_eq!(part2(&parse(input)?)?, u64::MAX);
    Ok(())
    }
    */
}
