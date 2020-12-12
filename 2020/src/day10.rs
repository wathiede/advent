//! --- Day 10: Adapter Array ---
//! Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans, however, your device suddenly turns off!
//!
//! Its battery is dead.
//!
//! You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of jolts. Always prepared, you make a list of all of the joltage adapters in your bag.
//!
//! Each of your joltage adapters is rated for a specific output joltage (your puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still produce its rated output joltage.
//!
//! In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your device's built-in adapter would be rated for 12 jolts.)
//!
//! Treat the charging outlet near your seat as having an effective joltage rating of 0.
//!
//! Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!
//!
//! If you use every adapter in your bag at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?
//!
//! For example, suppose that in your bag, you have adapters with the following joltage ratings:
//!
//! 16
//! 10
//! 15
//! 5
//! 1
//! 11
//! 7
//! 19
//! 6
//! 12
//! 4
//! With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
//!
//! Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:
//!
//! The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect to it directly would need to have a joltage rating of 1, 2, or 3 jolts. Of these, only one you have is an adapter rated 1 jolt (difference of 1).
//! From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
//! From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated 5 jolts (difference of 1).
//! Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7 (with difference of 1 and 1).
//! The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
//! From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
//! After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then 19 (difference of 3).
//! Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
//! In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.
//!
//! Here is a larger example:
//!
//! 28
//! 33
//! 18
//! 42
//! 31
//! 14
//! 46
//! 20
//! 48
//! 47
//! 24
//! 23
//! 49
//! 45
//! 19
//! 38
//! 39
//! 11
//! 1
//! 32
//! 25
//! 35
//! 8
//! 17
//! 7
//! 9
//! 4
//! 2
//! 34
//! 10
//! 3
//! In this larger example, in a chain that uses all of the adapters, there are 22 differences of 1 jolt and 10 differences of 3 jolts.
//!
//! Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet, the adapters, and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
//!
//! --- Part Two ---
//! To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.
//!
//! The first example above (the one that starts with 16, 10, 15) supports the following arrangements:
//!
//! (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
//! (The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is 8.
//!
//! The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 48, 49, (52)
//! In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.
//!
//! You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be an efficient way to count the arrangements.
//!
//! What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?

use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<usize> {
    let mut jolts: Vec<_> = input.split('\n').map(|s| s.parse().unwrap()).collect();
    // Add outlet
    jolts.push(0);
    // Add device power adapter
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();
    jolts
}

#[aoc(day10, part1)]
fn solution1(jolts: &[usize]) -> usize {
    let (one, three) = jolts.windows(2).fold((0, 0), |(one, three), pair| {
        let first = pair[0];
        let second = pair[1];
        match second - first {
            1 => (one + 1, three),
            3 => (one, three + 1),
            d => panic!(format!("unexpected diff: {} - {} = {}", second, first, d)),
        }
    });
    one * three
}

#[derive(Copy, Clone, PartialEq)]
struct Node(usize);

use std::fmt;
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.0)
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Based on code from https://www.geeksforgeeks.org/count-possible-paths-two-vertices/
#[derive(Debug, Default)]
struct Graph {
    /// adj holds a
    adj: Vec<Vec<Edge>>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new(nodes: &[usize]) -> Graph {
        let name_to_idx: HashMap<_, _> = nodes.iter().enumerate().map(|(i, n)| (n, i)).collect();
        let nodes: Vec<_> = nodes.iter().map(|n| Node(*n)).collect();
        let adj = nodes[..nodes.len()]
            .iter()
            .enumerate()
            .map(|(i, cur)| {
                nodes[i + 1..]
                    .iter()
                    .enumerate()
                    .take_while(|(_, &t)| t.0 - cur.0 <= 3)
                    .map(|(j, t)| Edge {
                        node: (i + j + 1),
                        cost: t.0 - cur.0,
                    })
                    .collect()
            })
            .collect();

        Graph { adj, nodes }
    }
    fn print_adj(&self) {
        for (cur, edges) in self.adj.iter().enumerate() {
            print!("{:?}", self.nodes[cur]);
            for (i, edge) in edges.iter().enumerate() {
                print!(" -> {:?}[{}]", self.nodes[edge.node], edge.cost);
            }
            println!();
        }
    }

    fn count_paths_part2(&self) -> usize {
        self.adj
            .iter()
            .map(|adj| match adj.len() {
                0 | 1 => 0,
                2 => 2,
                3 => 3,
                c => panic!(format!("unexpected cost {}", c)),
            })
            .sum::<usize>()
            + 1
    }

    fn count_paths(&self, src: usize, dst: usize) -> usize {
        self.count_paths_rec(src, dst, 0)
    }

    fn count_paths_rec(&self, cur: usize, dst: usize, count: usize) -> usize {
        if cur == dst {
            count + 1
        } else {
            dbg!(cur, &self.adj[cur]);
            self.adj[cur].iter().fold(count, |acc, n| {
                assert!(cur < n.node);
                self.count_paths_rec(n.node, dst, acc)
            })
        }
    }
}

/*
fn count_permutations(jolts: &[usize]) -> usize {
for chunk in jolts.iter().windows(2) {
if chunk[1] - chunk[0] > 3 {
return false;
}
}
true
}
*/

#[aoc(day10, part2)]
fn solution2(jolts: &[usize]) -> usize {
    //   count_permutations(jolts)
    // Store permutations at each node.
    let mut paths = vec![1; jolts.len()];

    dbg!(&paths);
    jolts.iter().enumerate().skip(1).for_each(|(cur, jolt)| {
        let p = (cur.saturating_sub(3)..cur)
            .filter(|idx| (jolt - jolts[*idx]) <= 3)
            .map(|idx| paths[idx])
            .sum();
        paths[cur] = p;
    });
    dbg!(&paths);
    paths[paths.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 4 5 6 7 10 11 12 15 16 19
    const INPUT1: &'static str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    // 1 2 3 4 7 8 9 10 11 14 17 18 19 20 23 24 25 31 32 33 34 35 38 39 42 45 46 47 48 49
    const INPUT2: &'static str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
    #[test]
    fn graph() {
        let g = Graph {
            adj: vec![
                vec![
                    Edge { node: 1, cost: 1 },
                    Edge { node: 2, cost: 1 },
                    Edge { node: 3, cost: 1 },
                ],
                vec![Edge { node: 3, cost: 1 }, Edge { node: 4, cost: 1 }],
                vec![Edge { node: 3, cost: 1 }, Edge { node: 4, cost: 1 }],
                vec![],
                vec![],
            ],
            nodes: vec![0, 1, 2, 3, 4].iter().map(|i| Node(*i)).collect(),
        };
        dbg!(&g);
        assert_eq!(g.count_paths(0, 3), 3);
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&parse(&INPUT1)), 7 * 5);
        assert_eq!(solution1(&parse(&INPUT2)), 22 * 10);
    }

    #[test]
    fn part2() {
        assert_eq!(solution2(&parse(&INPUT1)), 8);
        assert_eq!(solution2(&parse(&INPUT2)), 19208);
    }
}
