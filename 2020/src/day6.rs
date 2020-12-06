//! --- Day 6: Custom Customs ---
//! As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:
//!
//! abcx
//! abcy
//! abcz
//! In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
//!
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! This list represents answers from five groups:
//!
//! The first group contains one person who answered "yes" to 3 questions: a, b, and c.
//! The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
//! The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
//! The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
//! The last group contains one person who answered "yes" to only 1 question, b.
//! In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
//!
//! For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
//!
//! --- Part Two ---
//! As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
//!
//! You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!
//!
//! Using the same example as above:
//!
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! This list represents answers from five groups:
//!
//! In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
//! In the second group, there is no question to which everyone answered "yes".
//! In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
//! In the fourth group, everyone answered yes to only 1 question, a.
//! In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
//! In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
//!
//! For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn solution1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.chars().filter(|c| c != &'\n').collect::<HashSet<_>>())
        .map(|set| set.len())
        .sum()
}

#[aoc(day6, part2)]
fn solution2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let sets = group
                .split('\n')
                .map(|p| p.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            // Find letters common to all sets int this group.
            sets.iter().fold(sets.first().unwrap().clone(), |acc, s| {
                acc.intersection(s).cloned().collect()
            })
        })
        .map(|set| set.len())
        .sum()
}

#[aoc(day6, part2, faster)]
fn solution2_faster(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|group| {
            group.split('\n').fold(None, |acc: Option<HashSet<_>>, p| {
                match acc {
                    None => {
                        // Add all the letters for the first set.
                        Some(p.chars().collect())
                    }
                    Some(acc) => {
                        // Remove from acc any letters not in p.
                        let tmp = Some(
                            p.chars()
                                .filter(|c| acc.contains(c))
                                .collect::<HashSet<_>>(),
                        );
                        tmp
                    }
                }
            })
        })
        .map(|set| set.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn part1() {
        assert_eq!(solution1(INPUT), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solution2(INPUT), 6);
    }
}
