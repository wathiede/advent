use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default)]
struct Graph {
    g: HashMap<String, Vec<String>>,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Graph {
    let mut g = Graph::default();
    input.lines().for_each(|l| {
        let label = l[..3].to_string();
        let outs = l[5..].split(' ').map(|s| s.to_string()).collect();
        g.g.insert(label, outs);
    });
    g
}

fn bfs(g: &Graph, node: &str) -> usize {
    if node == "out" {
        return 1;
    }
    g.g[node].iter().map(|n| bfs(g, n)).sum()
}

#[aoc(day11, part1)]
fn part1(input: &Graph) -> String {
    bfs(input, "you").to_string()
}

#[aoc(day11, part2)]
fn part2(input: &Graph) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "5");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}

