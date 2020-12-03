//! --- Day 2: Password Philosophy ---
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//!
//! In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
//!
//! How many passwords are valid according to their policies?

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Policy> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (.*)").expect("Failed to compile regex");
    input
        .split('\n')
        .filter(|line| re.is_match(line))
        .map(|line| {
            let caps = re.captures(line).expect("Failed to match pattern");
            Policy {
                min: caps.get(1).unwrap().as_str().parse().unwrap(),
                max: caps.get(2).unwrap().as_str().parse().unwrap(),
                letter: caps.get(3).unwrap().as_str().to_string(),
                password: caps.get(4).unwrap().as_str().to_string(),
            }
        })
        .collect()
}

fn is_valid_policy(p: &Policy) -> bool {
    let c = p.password.matches(&p.letter).count();
    p.min <= c && c <= p.max
}

#[aoc(day2, part1)]
fn valid_policy_count(policies: &[Policy]) -> usize {
    policies.iter().filter(|p| is_valid_policy(p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
    #[test]
    fn parse_policies() {
        assert_eq!(
            parse(INPUT),
            vec![
                Policy {
                    min: 1,
                    max: 3,
                    letter: "a".to_string(),
                    password: "abcde".to_string(),
                },
                Policy {
                    min: 1,
                    max: 3,
                    letter: "b".to_string(),
                    password: "cdefg".to_string(),
                },
                Policy {
                    min: 2,
                    max: 9,
                    letter: "c".to_string(),
                    password: "ccccccccc".to_string(),
                },
            ]
        );
    }

    #[test]
    fn validate_count() {
        assert_eq!(valid_policy_count(&parse(INPUT)), 2);
    }
}
