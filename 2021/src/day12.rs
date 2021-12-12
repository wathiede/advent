use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

struct Node {
    name: String,
    small: bool,
    neighbors: Vec<usize>,
}

struct Graph {
    nodes: Vec<String>,
}

fn search(node: &str, nodes: &HashMap<&str, Vec<&str>>, path: String, paths: &mut Vec<String>) {
    if node == "end" {
        paths.push(path);
        return;
    }
    for neighbor in &nodes[node] {
        // If lowercase.
        if neighbor.as_bytes()[0] & 0x20 != 0 {
            if path.contains(neighbor) {
                continue;
            }
        }
        search(neighbor, nodes, format!("{},{}", path, neighbor), paths);
    }
}

fn paths(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut paths = Vec::new();
    search("start", nodes, "start".to_string(), &mut paths);
    //dbg!(&paths);
    paths.len()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut nodes = HashMap::new();
    input.lines().for_each(|p| {
        let (n1, n2) = p.split_once('-').expect("missing dash");
        nodes.entry(n1).or_insert(Vec::new()).push(n2);
        nodes.entry(n2).or_insert(Vec::new()).push(n1);
    });
    Ok(paths(&nodes))
}

/*
#[aoc(day12, part2)]
fn part2(input: &[u64]) -> Result<u64> {
todo!("part2");
Ok(0)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#
        .trim();

        assert_eq!(part1(input)?, 10);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
    let input = r#"
    "#
    .trim();
    assert_eq!(part2(&parse(input)?)?, u64::MAX);
    Ok(())
    }
    */
}
