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
//!
//! --- Part Two ---
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your shiny gold bag and the rules from the above example:
//!
//! faded blue bags contain 0 other bags.
//! dotted black bags contain 0 other bags.
//! vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//! dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//! So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//!
//! Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! In this example, a single shiny gold bag must contain 126 other bags.
//!
//! How many individual bags are required inside your single shiny gold bag?

use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Color = String;

#[derive(Debug, Default)]
struct Node {
    color: Color,
    parents: Vec<Color>,
    children: Vec<(usize, Color)>,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<Color, Node>,
}

impl Graph {
    fn add_node(&mut self, line: &str) {
        let parts: Vec<_> = line.split(" bags contain ").collect();
        match parts.len() {
            0 | 1 => panic!("line '{}' fails assumptions", line),
            _ => {
                let parent_color = parts[0].to_string();
                let mut children = Vec::new();
                if parts[1] != "no other bags." {
                    for chunk in parts[1].split(' ').collect::<Vec<_>>().chunks(4) {
                        // [0] quantity
                        // [1] color1
                        // [2] color2
                        // [3] bag/bags[,.]
                        let color = format!("{} {}", chunk[1], chunk[2]);
                        let c = self.nodes.entry(color.clone()).or_insert(Node {
                            color: color.clone(),
                            parents: Vec::new(),
                            children: Vec::new(),
                        });
                        c.parents.push(parent_color.clone());
                        let count = chunk[0].parse::<usize>().expect("couldn't parse bag count");
                        children.push((count, color.clone()));
                    }
                }
                // Get or create this parent color
                let p = self.nodes.entry(parent_color.clone()).or_insert(Node {
                    color: parent_color.clone(),
                    parents: Vec::new(),
                    children: Vec::new(),
                });
                p.children = children;
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

    fn bag_count(&self, color: &Color) -> usize {
        let n = self.nodes.get(color).expect("Couldn't find node");
        if n.children.is_empty() {
            // No children.
            return 0;
        } else {
            // Number of children bags and multiple the number of child bags by the transitive
            // closure of the child's sub bags.
            n.children
                .iter()
                // Return the number of sub
                .map(|(cnt, color)| cnt + cnt * self.bag_count(color))
                .sum()
        }
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
    let answer = g.top_level(&"shiny gold".to_string()).len();

    /*
    // Ensure we don't break part 1 while working on part 2.
    let correct_answer = 222;
    assert_eq!(answer, correct_answer);
    */
    answer
}

#[aoc(day7, part2)]
fn solution2(g: &Graph) -> usize {
    g.bag_count(&"shiny gold".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
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
        assert_eq!(solution1(&parse(INPUT1)), 4);
    }

    const INPUT2: &'static str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

    #[test]
    fn part2() {
        assert_eq!(solution2(&parse(INPUT1)), 32);
        assert_eq!(solution2(&parse(INPUT2)), 126);
    }
}
