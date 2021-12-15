use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::aoc;

struct TupleWindow<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    prev: Option<T>,
    next: Option<T>,
}

impl<I, T> TupleWindow<I, T>
where
    I: Iterator<Item = T>,
{
    fn new(iter: I, rules: &HashMap<&[u8], u8>) -> Self {
        TupleWindow {
            iter,
            prev: None,
            next: None,
        }
    }
}

impl<I, T> Iterator for TupleWindow<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            self.prev = self.iter.next();
        }
        /*
           template.next() {
           template.flat_map(|y|
           let z = rules[xy];
           res[i * 2] = xy[0];
           res[i * 2 + 1] = z;
           res[i * 2 + 2] = xy[1];
           });
        //dbg!(String::from_utf8_lossy(&res));
        res
        */

        if let Some(next) = self.iter.next() {
            let prev = self.prev.take();
            self.prev = Some(next);
            return prev;
        }
        None
    }
}

fn expand_it<'a, I: 'a + Iterator<Item = &'a u8>>(
    template: I,
    rules: &HashMap<&[u8], u8>,
) -> impl Iterator<Item = &'a u8> {
    TupleWindow::new(template, rules)
}

fn forty_steps<'a, I: 'a + Iterator<Item = &'a u8>>(it: I, rules: &HashMap<&[u8], u8>) -> usize {
    return 0;
    //let it = (1..40).fold(it, |acc, _| expand_it(acc, &rules));
    expand_it(
        expand_it(
            expand_it(
                expand_it(
                    expand_it(
                        expand_it(
                            expand_it(
                                expand_it(
                                    expand_it(
                                        expand_it(
                                            expand_it(
                                                expand_it(
                                                    expand_it(
                                                        expand_it(
                                                            expand_it(
                                                                expand_it(
                                                                    expand_it(
                                                                        expand_it(
                                                                            expand_it(
                                                                                expand_it(
                                                                                    expand_it(
                                                                                        expand_it(
                                                                                            expand_it(
                                                                                                expand_it(
                                                                                                    expand_it(
                                                                                                        expand_it(
                                                                                                            expand_it(
                                                                                                                expand_it(
                                                                                                                    expand_it(
                                                                                                                        expand_it(
                                                                                                                            expand_it(
                                                                                                                                expand_it(
                                                                                                                                    expand_it(
                                                                                                                                        expand_it(
                                                                                                                                            expand_it(
                                                                                                                                                expand_it(
                                                                                                                                                    expand_it(
                                                                                                                                                        expand_it(
    expand_it(
        expand_it(it, &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
                                                                                                                                                                    &rules),
        &rules,
    )
    .count()
}

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
    let mut keys: Vec<_> = m.keys().collect();
    keys.sort_unstable();
    let mut s = "".to_string();
    for k in keys {
        s.push_str(&format!("{}: {} ", String::from_utf8_lossy(&[*k]), m[k]));
    }
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
        let s = String::from_utf8_lossy(&template);
        count(&template);
    }
    let (min, max) = count(&template);
    Ok(max - min)
}

// TODO
//#[aoc(day14, part2)]
fn part2(input: &str) -> Result<usize> {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<&[u8], u8> = rules
        .lines()
        .map(|l| {
            let (pair, insert) = l.split_once(" -> ").unwrap();
            (pair.as_bytes(), insert.as_bytes()[0])
        })
        .collect();
    let cnt = forty_steps(template.as_bytes().iter(), &rules);
    dbg!(cnt);
    //println!("After step {}: {}", i, String::from_utf8_lossy(&template));
    //let (min, max) = count(template);
    //Ok(max - min)
    Ok(0)
}

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

    // TODO
    //#[test]
    fn test_part2() -> Result<()> {
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
        assert_eq!(part2(input)?, 2188189693529);
        Ok(())
    }
}

// BB -> N   BN NB    BB NB NB BB
// BC -> B   BB BC    BN NB BB BC
// BH -> H   BH HH    BH HH HN NH
// BN -> B   BB NB    BN NB NB BB
// CB -> H   CH HB    \
// CC -> N   CN NC
// CH -> B   CB BH
// CN -> C   CC CN
// HB -> C   HC CB
// HC -> B   HB BC
// HH -> N   HN NH
// HN -> C   HC CN
// NB -> B   NB BB
// NC -> B   NC BC
// NH -> C   NC CH
// NN -> C   NC CN
//
//
//
