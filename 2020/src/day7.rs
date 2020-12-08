//! --- Day 7: Handy Haversacks ---
//! You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//!
//! You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! A bright white bag, which can hold your shiny gold bag directly.
//! A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//! A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
//!
//! How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
//!
use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Color = String;

#[derive(Debug, Default)]
struct Node {
    color: Color,
    parents: Vec<Color>,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<Color, Node>,
}

impl Graph {
    fn add_node(&mut self, line: &str) {
        let parts: Vec<_> = line.split(" bags contain ").collect();
        match parts.len() {
            0 | 1 => panic!(format!("line '{}' fails assumptions", line)),
            _ => {
                let parent_color = parts[0].to_string();
                // Get or create this parent color
                let _ = self.nodes.entry(parent_color.clone()).or_insert(Node {
                    color: parent_color.clone(),
                    parents: Vec::new(),
                });

                if parts[1] != "no other bags." {
                    for chunk in parts[1].split(' ').collect::<Vec<_>>().chunks(4) {
                        // [0] quantity
                        // [1] color1
                        // [2] color2
                        // [3] bag/bags[,.]
                        let color = format!("{} {}", chunk[1], chunk[2]);
                        let c = self.nodes.entry(color.clone()).or_insert(Node {
                            color,
                            parents: Vec::new(),
                        });
                        c.parents.push(parent_color.clone());
                    }
                }
            }
        }
    }

    fn top_level(&self, color: &Color) -> HashSet<Color> {
        let n = self.nodes.get(color).expect("Couldn't find node");
        self.top_level_rec(n.parents.clone())
    }
    fn top_level_rec(&self, parents: Vec<Color>) -> HashSet<Color> {
        if parents.is_empty() {
            return HashSet::new();
        }

        let mut set = HashSet::new();
        set.extend(parents.clone());
        parents.iter().for_each(|color| {
            let n = self.nodes.get(color).expect("Couldn't find node");
            set.extend(self.top_level_rec(n.parents.clone()));
        });
        set
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Graph {
    let mut g = Graph::default();
    input.split('\n').for_each(|line| g.add_node(line));
    g
}

#[aoc(day7, part1)]
fn solution1(g: &Graph) -> usize {
    g.top_level(&"shiny gold".to_string()).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    #[test]
    fn part1() {
        assert_eq!(solution1(&parse(INPUT)), 4);
    }
}
