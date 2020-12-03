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
//!
//! --- Part Two ---
//! While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//! 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//! 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//! How many passwords are valid according to the new interpretation of the policies?

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
fn parse_regex(input: &str) -> Vec<Policy> {
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

#[aoc_generator(day2, part1, handrolled)]
fn parse_handrolled(input: &str) -> Vec<Policy> {
    // Example line:
    //   1-3 a: abcde
    input
        .split('\n')
        .filter_map(|line| {
            let start = 0;
            let end = line.find('-')?;
            let min: usize = line[start..end].parse().ok()?;

            let start = end + 1;
            let end = line.find(' ')?;
            let max: usize = line[start..end].parse().ok()?;

            let start = end + 1;
            let end = line.find(':')?;
            let letter = line[start..end].to_string();

            let start = end + 2;
            let password = line[start..].to_string();

            Some(Policy {
                min,
                max,
                letter,
                password,
            })
        })
        .collect()
}

fn is_valid_policy_part1(p: &Policy) -> bool {
    let c = p.password.matches(&p.letter).count();
    p.min <= c && c <= p.max
}

#[aoc(day2, part1)]
fn valid_policy_count_part1(policies: &[Policy]) -> usize {
    policies.iter().filter(|p| is_valid_policy_part1(p)).count()
}

#[aoc(day2, part1, handrolled)]
fn valid_policy_count_handrolled_part1(policies: &[Policy]) -> usize {
    policies.iter().filter(|p| is_valid_policy_part1(p)).count()
}

fn is_valid_policy_part2(p: &Policy) -> bool {
    let letter = Some(p.letter.as_str());
    // Password system uses it 1 based numbering, so we -1 to get zero based.
    let first = p.password.get(p.min - 1..p.min);
    let second = p.password.get(p.max - 1..p.max);
    let valid = (first == letter) ^ (second == letter);
    //dbg!((&p.password, &first, &second, &letter, &valid));
    valid
}

#[aoc(day2, part2)]
fn valid_policy_count_part2(policies: &[Policy]) -> usize {
    policies.iter().filter(|p| is_valid_policy_part2(p)).count()
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
        let want = vec![
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
        ];
        assert_eq!(parse_regex(INPUT), want);
        assert_eq!(parse_handrolled(INPUT), want);
    }

    #[test]
    fn validate_count_part1() {
        assert_eq!(valid_policy_count_part1(&parse_regex(INPUT)), 2);
    }

    #[test]
    fn validate_count_part2() {
        assert_eq!(valid_policy_count_part2(&parse_regex(INPUT)), 1);
    }
}
