//! --- Day 19: Monster Messages ---
//! You land in an airport surrounded by dense forest. As you walk to your high-speed train, the Elves at the Mythical Information Bureau contact you again. They think their satellite has collected an image of a sea monster! Unfortunately, the connection to the satellite is having problems, and many of the messages sent back from the satellite have been corrupted.
//!
//! They sent you a list of the rules valid messages should obey and a list of received messages they've collected so far (your puzzle input).
//!
//! The rules for valid messages (the top part of your puzzle input) are numbered and build upon each other. For example:
//!
//! 0: 1 2
//! 1: "a"
//! 2: 1 3 | 3 1
//! 3: "b"
//! Some rules, like 3: "b", simply match a single character (in this case, b).
//!
//! The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means that to match rule 0, the text being checked must match rule 1, and the text after the part that matched rule 1 must then match rule 2.
//!
//! Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at least one list of sub-rules must match. (The ones that match might be different each time the rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.
//!
//! Fortunately, there are no loops in the rules, so the list of possible matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0 matches aab or aba.
//!
//! Here's a more interesting example:
//!
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//! Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the same (aa or bb), and rule 3 matches two letters that are different (ab or ba).
//!
//! Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of letters, one pair with matching letters and one pair with different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.
//!
//! Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
//!
//! The received messages (the bottom part of your puzzle input) need to be checked against the rules so you can determine which are valid and which are corrupted. Including the rules and the messages together, this might look like:
//!
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//!
//! ababbb
//! bababa
//! abbbab
//! aaabbb
//! aaaabbb
//! Your goal is to determine the number of messages that completely match rule 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer 2. The whole message must match all of rule 0; there can't be extra unmatched characters in the message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched b on the end.)
//!
//! How many messages completely match rule 0?
use std::collections::HashMap;
use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Input {
    rules: Regex,
    messages: Vec<String>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.rules.as_str() == other.rules.as_str() && self.messages == other.messages
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Entry {
    Rule(usize),
    Char(String),
}
impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Rule(n) => write!(f, "Entry({})", n)?,
            Entry::Char(c) => write!(f, r#"Entry("{}")"#, c)?,
        }
        Ok(())
    }
}

struct Resolver {
    rule_map: HashMap<usize, Vec<Vec<Entry>>>,
    resolved: HashMap<Entry, String>,
}

impl Resolver {
    fn resolve(&mut self, e: &Entry) -> String {
        if let Some(v) = self.resolved.get(e) {
            return v.to_string();
        }

        match e {
            Entry::Char(s) => {
                self.resolved.insert(e.clone(), s.to_string());
                s.to_string()
            }
            Entry::Rule(n) => {
                let subrules: Vec<String> = self.rule_map[n]
                    // TODO(wathiede): clone here seems inefficient, but it made the borrow checker
                    // happy given the &mut recursive call to self.resolve in the map closure.
                    .clone()
                    .iter()
                    .map(|rule| {
                        let letters: Vec<_> = rule.iter().map(|e| self.resolve(e)).collect();
                        letters.join("")
                    })
                    .collect();
                let s = subrules.join("|");

                if subrules.len() > 1 {
                    // More than one rule,
                    return format!("({})", s);
                }
                s
            }
        }
    }
}

fn expand_rulemap(rule_map: HashMap<usize, Vec<Vec<Entry>>>) -> Regex {
    let mut r = Resolver {
        rule_map,
        resolved: HashMap::new(),
    };
    let rule_zero = r.rule_map[&0][0].clone();
    let re = rule_zero
        .iter()
        .fold("".to_string(), |acc, e| format!("{}{}", acc, r.resolve(e)));

    Regex::new(&format!(r"^{}$", re)).unwrap()
}

fn make_rules(lines: Vec<String>) -> Regex {
    let mut rules = HashMap::new();

    lines.iter().for_each(|l| {
        let idx = l.find(':').expect("missing ':'");
        let k: usize = l[..idx].parse().expect("failed to parse number");

        let sub: Vec<Vec<Entry>> = l[idx + 2..]
            .split(" | ")
            .map(|sub| {
                sub.split(' ')
                    .map(|p| match p.parse() {
                        Ok(n) => Entry::Rule(n),
                        Err(_) => Entry::Char(
                            p.strip_prefix("\"")
                                .unwrap()
                                .strip_suffix("\"")
                                .unwrap()
                                .to_string(),
                        ),
                    })
                    .collect()
            })
            .collect();
        rules.insert(k, sub);
    });
    expand_rulemap(rules)
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Input {
    let mut it = input.split("\n\n");
    let rules = make_rules(
        it.next()
            .unwrap()
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect(),
    );

    let messages = it
        .next()
        .unwrap()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect();
    Input { rules, messages }
}

#[aoc(day19, part1)]
fn solution1(input: &Input) -> usize {
    input
        .messages
        .iter()
        .filter(|msg| input.rules.is_match(msg))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;

    #[test]
    fn part1() {
        assert_eq!(solution1(&generator(INPUT)), 2);
    }

    #[test]
    fn parse() {
        assert_eq!(
            generator(INPUT),
            Input {
                rules: Regex::new("^a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b$").unwrap(),
                messages: vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb",]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            }
        );
    }
    #[test]
    fn expand() {
        use Entry::*;
        let r: HashMap<usize, Vec<Vec<Entry>>> = vec![
            vec![vec![Rule(1)]],
            vec![vec![Rule(2), Rule(2)], vec![Rule(3), Rule(3)]],
            vec![vec![Char("a".to_string())]],
            vec![vec![Char("b".to_string())]],
        ]
        .into_iter()
        .enumerate()
        .collect();
        assert_eq!(
            expand_rulemap(r).as_str(),
            Regex::new("^(aa|bb)$").unwrap().as_str()
        );
    }
}
