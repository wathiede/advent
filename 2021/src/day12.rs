use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::aoc;

fn search(node: &str, nodes: &HashMap<&str, Vec<&str>>, path: String, paths: &mut Vec<String>) {
    if node == "end" {
        paths.push(path);
        return;
    }
    for neighbor in &nodes[node] {
        // If lowercase.
        if neighbor.as_bytes()[0] & 0x20 != 0 && path.contains(neighbor) {
            continue;
        }
        search(neighbor, nodes, format!("{},{}", path, neighbor), paths);
    }
}

fn paths(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut paths = Vec::new();
    search("start", nodes, "start".to_string(), &mut paths);
    paths.len()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut nodes = HashMap::new();
    input.lines().for_each(|p| {
        let (n1, n2) = p.split_once('-').expect("missing dash");
        nodes.entry(n1).or_insert_with(Vec::new).push(n2);
        nodes.entry(n2).or_insert_with(Vec::new).push(n1);
    });
    Ok(paths(&nodes))
}

fn search2<'a>(
    node: &str,
    nodes: &HashMap<&'a str, Vec<&'a str>>,
    path: &[&'a str],
    paths: &mut Vec<Vec<&'a str>>,
    double: &'a str,
    smalls: &[&'a str],
) {
    if node == "end" {
        paths.push(path.to_vec());
        return;
    }
    for neighbor in &nodes[node] {
        // If lowercase.
        if neighbor.as_bytes()[0] & 0x20 != 0 {
            if neighbor == &double {
                // Allow two passes for this small node.
                if path.iter().filter(|p| p == &neighbor).count() >= 2 {
                    continue;
                }
            } else {
                // Only allow one pass for this small node.
                if path.contains(neighbor) {
                    continue;
                }
            }
        }
        let mut child_path = path.to_vec();
        child_path.push(neighbor);
        search2(neighbor, nodes, &child_path, paths, double, smalls);
    }
}
fn paths2(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut paths = Vec::new();
    let smalls: Vec<_> = nodes
        .keys()
        .filter(|n| n.as_bytes()[0] & 0x20 != 0)
        .filter(|&n| n != &"start" && n != &"end")
        .cloned()
        .collect();

    for double in &smalls {
        search2(
            "start",
            nodes,
            &["start"],
            &mut paths,
            double,
            smalls.as_slice(),
        );
    }
    paths.sort();
    paths.dedup();
    paths.len()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> Result<usize> {
    let mut nodes = HashMap::new();
    input.lines().for_each(|p| {
        let (n1, n2) = p.split_once('-').expect("missing dash");
        nodes.entry(n1).or_insert_with(Vec::new).push(n2);
        nodes.entry(n2).or_insert_with(Vec::new).push(n1);
    });
    Ok(paths2(&nodes))
}

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

    #[test]
    fn test_part2() -> Result<()> {
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
        assert_eq!(part2(input)?, 36);
        Ok(())
    }
}
