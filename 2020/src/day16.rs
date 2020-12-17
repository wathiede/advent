//! --- Day 16: Ticket Translation ---
//! As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.
//!
//! Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.
//!
//! You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).
//!
//! The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//!
//! Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:
//!
//! .--------------------------------------------------------.
//! | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
//! |                                                        |
//! | ??: 301  ??: 302             ???????: 303      ??????? |
//! | ??: 401  ??: 402           ???? ????: 403    ????????? |
//! '--------------------------------------------------------'
//! Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!
//!
//! Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.
//!
//! For example, suppose you have the following notes:
//!
//! class: 1-3 or 5-7
//! row: 6-11 or 33-44
//! seat: 13-40 or 45-50
//!
//! your ticket:
//! 7,1,14
//!
//! nearby tickets:
//! 7,3,47
//! 40,4,50
//! 55,2,20
//! 38,6,12
//! It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.
//!
//! Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
//!
//! --- Part Two ---
//! Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.
//!
//! Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.
//!
//! For example, suppose you have the following notes:
//!
//! class: 0-1 or 4-19
//! row: 0-5 or 8-19
//! seat: 0-13 or 16-19
//!
//! your ticket:
//! 11,12,13
//!
//! nearby tickets:
//! 3,9,18
//! 15,1,5
//! 5,14,9
//! Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
//!
//! Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?

use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct Rule {
    name: String,
    low: Range<usize>,
    high: Range<usize>,
}

/// Parses "2-4" into Range(2..5).
fn from_range(s: &str) -> Result<Range<usize>, String> {
    let mut it = s.split('-');
    let low = it
        .next()
        .ok_or("low range".to_string())?
        .parse()
        .map_err(|e| format!("{}", e))?;
    let high = it
        .next()
        .ok_or("high range".to_string())?
        .parse::<usize>()
        .map_err(|e| format!("{}", e))?;
    Ok(low..high + 1)
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Rule, String> {
        let c_idx = s.find(":").expect("missing :");
        let name = s[..c_idx].to_string();
        let mut it = s[c_idx + 2..].split(' ');
        let low = from_range(it.next().ok_or("get low")?)?;
        let _ = it.next().ok_or("missing 'or'".to_string())?;
        let high = from_range(it.next().ok_or("get high")?)?;
        Ok(Rule { name, low, high })
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    nums: Vec<usize>,
}

impl FromStr for Ticket {
    type Err = String;
    fn from_str(s: &str) -> Result<Ticket, String> {
        Ok(Ticket {
            nums: s
                .split(',')
                .map(|s| s.parse().map_err(|e| format!("{}", e)))
                .collect::<Result<Vec<_>, String>>()?,
        })
    }
}

fn reduce_possibilities(possibilities: HashMap<usize, Vec<String>>) -> HashMap<usize, String> {
    let mut p = possibilities.clone();
    let mut uniq = HashMap::new();

    loop {
        let mut rm = Vec::new();
        // Extract all the columns with only one possible answer.
        p.iter().for_each(|(k, v)| {
            if v.len() == 1 {
                let word = v[0].to_string();
                rm.push(word.to_string());
                uniq.insert(*k, word);
            }
        });

        // Remove all the assigned columns from columns that have multiple possibilities.
        p = p
            .into_iter()
            .filter_map(|(k, v)| {
                let v: Vec<_> = v.into_iter().filter(|w| !rm.contains(w)).collect();
                if v.is_empty() {
                    None
                } else {
                    Some((k, v))
                }
            })
            .collect();
        if p.is_empty() {
            break;
        }
    }

    uniq
}

#[derive(Debug, PartialEq)]
struct Notes {
    rules: Vec<Rule>,
    my: Ticket,
    nearby: Vec<Ticket>,
}

impl Notes {
    fn valid(&self, n: usize) -> bool {
        for r in &self.rules {
            if r.low.contains(&n) || r.high.contains(&n) {
                return true;
            }
        }
        false
    }

    fn valid_ticket(&self, t: &Ticket) -> bool {
        t.nums.iter().all(|n| self.valid(*n))
    }

    fn invalid_nums(&self, t: &Ticket) -> Vec<usize> {
        t.nums
            .iter()
            .filter(|n| !self.valid(**n))
            .cloned()
            .collect()
    }

    fn valid_column<'a, I>(&self, idx: usize, mut tickets: I, r: &Rule) -> bool
    where
        I: Iterator<Item = &'a &'a Ticket>,
    {
        tickets.all(|t| r.low.contains(&t.nums[idx]) || r.high.contains(&t.nums[idx]))
    }

    /// translate will apply `rules` to all valid tickets to compute which column maps to each
    /// rule.
    fn translate(&self) -> HashMap<String, usize> {
        let valid_tickets: Vec<_> = self
            .nearby
            .iter()
            .filter(|t| self.valid_ticket(t))
            .collect();
        use std::iter::once;
        let possibilities: HashMap<_, _> = (0..self.my.nums.len())
            .map(|i| {
                let possible = self
                    .rules
                    .iter()
                    .filter(|r| {
                        self.valid_column(i, once(&&self.my).chain(valid_tickets.iter()), r)
                    })
                    .map(|r| r.name.to_string())
                    .collect::<Vec<_>>();
                (i, possible)
            })
            .collect();

        reduce_possibilities(possibilities)
            .into_iter()
            .map(|(i, name)| (name, i))
            .collect()
    }
}

#[aoc_generator(day16)]
fn parse1(input: &str) -> Notes {
    let mut it = input.split("\n\n");
    let rules: Vec<Rule> = it
        .next()
        .expect("EOF1")
        .split('\n')
        .map(|l| l.parse().expect("rules"))
        .collect();
    let my: Ticket = it
        .next()
        .expect("EOF2")
        .split('\n')
        .skip(1)
        .nth(0)
        .map(|l| l.parse().expect("my parse"))
        .expect("my");
    let nearby: Vec<Ticket> = it
        .next()
        .expect("EOF3")
        .split('\n')
        .skip(1)
        .map(|l| l.parse().expect("rules"))
        .collect();
    Notes { rules, my, nearby }
}

#[aoc(day16, part1)]
fn solution1(notes: &Notes) -> usize {
    notes
        .nearby
        .iter()
        .flat_map(|t| notes.invalid_nums(t))
        .sum()
}

#[aoc(day16, part2)]
fn solution2(notes: &Notes) -> usize {
    let t = notes.translate();
    let idxs: Vec<_> = t
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(v)
            } else {
                None
            }
        })
        .collect();
    idxs.into_iter().map(|i| notes.my.nums[*i]).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &'static str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse1(INPUT1),
            Notes {
                rules: vec![
                    Rule {
                        name: "class".to_string(),
                        low: 1..4,
                        high: 5..8,
                    },
                    Rule {
                        name: "row".to_string(),
                        low: 6..12,
                        high: 33..45,
                    },
                    Rule {
                        name: "seat".to_string(),
                        low: 13..41,
                        high: 45..51,
                    },
                ],
                my: Ticket {
                    nums: vec![7, 1, 14],
                },
                nearby: vec![
                    Ticket {
                        nums: vec![7, 3, 47]
                    },
                    Ticket {
                        nums: vec![40, 4, 50]
                    },
                    Ticket {
                        nums: vec![55, 2, 20]
                    },
                    Ticket {
                        nums: vec![38, 6, 12]
                    },
                ],
            }
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&parse1(INPUT1)), 4 + 55 + 12);
    }

    const INPUT2: &'static str = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    #[test]
    fn translate() {
        let notes = parse1(&INPUT2);
        assert_eq!(
            notes.translate(),
            vec![
                ("class".to_string(), 1),
                ("row".to_string(), 0),
                ("seat".to_string(), 2),
            ]
            .into_iter()
            .collect::<HashMap<String, usize>>()
        );
    }
}
