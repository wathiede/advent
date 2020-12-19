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
//!
//! --- Part Two ---
//! As you look over the list of messages, you realize your matching rules aren't quite right. To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
//!
//! 8: 42 | 42 8
//! 11: 42 31 | 42 11 31
//! This small change has a big impact: now, the rules do contain loops, and the list of messages they could hypothetically match is infinite. You'll need to determine how these changes affect which messages are valid.
//!
//! Fortunately, many of the rules are unaffected by this change; it might help to start by looking at which rules always match the same set of values and how those rules (especially rules 42 and 31) are used by the new versions of rules 8 and 11.
//!
//! (Remember, you only need to handle the rules you have; building a solution that could handle any hypothetical combination of rules would be significantly more difficult.)
//!
//! For example:
//!
//! 42: 9 14 | 10 1
//! 9: 14 27 | 1 26
//! 10: 23 14 | 28 1
//! 1: "a"
//! 11: 42 31
//! 5: 1 14 | 15 1
//! 19: 14 1 | 14 14
//! 12: 24 14 | 19 1
//! 16: 15 1 | 14 14
//! 31: 14 17 | 1 13
//! 6: 14 14 | 1 14
//! 2: 1 24 | 14 4
//! 0: 8 11
//! 13: 14 3 | 1 12
//! 15: 1 | 14
//! 17: 14 2 | 1 7
//! 23: 25 1 | 22 14
//! 28: 16 1
//! 4: 1 1
//! 20: 14 14 | 1 15
//! 3: 5 14 | 16 1
//! 27: 1 6 | 14 18
//! 14: "b"
//! 21: 14 1 | 1 14
//! 25: 1 1 | 1 14
//! 22: 14 14
//! 8: 42
//! 26: 14 22 | 1 20
//! 18: 15 15
//! 7: 14 5 | 1 21
//! 24: 14 1
//!
//! abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaaaabbaaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! babaaabbbaaabaababbaabababaaab
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba, ababaaaaaabaaab, and ababaaaaabbbaba.
//!
//! However, after updating rules 8 and 11, a total of 12 messages match:
//!
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! After updating rules 8 and 11, how many messages completely match rule 0?

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
        assert_ne!(e, &Entry::Rule(0));
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
                        rule.iter()
                            .map(|e| self.resolve(e))
                            .collect::<Vec<_>>()
                            .join("")
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
    // Hack
    let part2 = rule_map.len() > 8 && rule_map[&8].len() > 1;
    let mut r = Resolver {
        rule_map,
        resolved: HashMap::new(),
    };
    let re = if part2 {
        let hack = (1..10_usize)
            .map(|i| {
                format!(
                    "{}{}",
                    r.resolve(&Entry::Rule(42)).repeat(i),
                    r.resolve(&Entry::Rule(31)).repeat(i)
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("({})+({})", r.resolve(&Entry::Rule(42)), hack)
    } else {
        let rule_zero = r.rule_map[&0][0].clone();
        rule_zero
            .iter()
            .fold("".to_string(), |acc, e| format!("{}{}", acc, r.resolve(e)))
    };

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
                        Err(_) => Entry::Char(p[1..p.len() - 1].to_string()),
                    })
                    .collect()
            })
            .collect();
        rules.insert(k, sub);
    });
    expand_rulemap(rules)
}

#[aoc_generator(day19, part1)]
fn generator_part1(input: &str) -> Input {
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

#[aoc_generator(day19, part2)]
fn generator_part2(input: &str) -> Input {
    let mut it = input.split("\n\n");
    let rules = make_rules(
        it.next()
            .unwrap()
            .split('\n')
            .map(|s| s.trim())
            .map(|s| {
                if s.starts_with("8:") {
                    return "8: 42 | 42 8";
                }
                if s.starts_with("11:") {
                    return "11: 42 31 | 42 11 31";
                }
                s
            })
            .map(|s| s.to_string())
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

#[aoc(day19, part2)]
fn solution2(input: &Input) -> usize {
    input
        .messages
        .iter()
        .filter(|msg| input.rules.is_match(msg))
        .count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    const INPUT1: &'static str = r#"0: 4 1 5
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
        assert_eq!(solution1(&generator_part1(INPUT1)), 2);
    }

    #[test]
    fn parse1() {
        assert_eq!(
            generator_part1(INPUT1),
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
    const INPUT2: &'static str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn part2_matches() {
        let input = generator_part2(INPUT2);
        assert_eq!(
            input
                .messages
                .iter()
                .filter(|msg| input.rules.is_match(msg))
                .collect::<Vec<_>>(),
            vec![
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            ]
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solution2(&generator_part2(INPUT2)), 12);
    }
}
