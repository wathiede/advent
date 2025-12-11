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

fn bfs2<'a>(
    g: &'a Graph,
    node: &'a str,
    stop: &str,
    mut memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if node == stop {
        return 1;
    }
    if !g.g.contains_key(node) {
        return 0;
    }
    if let Some(c) = memo.get(node) {
        return *c;
    }
    let res = g.g[node].iter().map(|n| bfs2(g, n, stop, &mut memo)).sum();
    memo.insert(node, res);
    res
}

#[aoc(day11, part2)]
fn part2(input: &Graph) -> String {
    // My data has fft before dac, so this solution works
    let mut memo = HashMap::new();
    let v1 = bfs2(input, "svr", "fft", &mut memo);
    let mut memo = HashMap::new();
    let v2 = bfs2(input, "fft", "dac", &mut memo);
    let mut memo = HashMap::new();
    let v3 = bfs2(input, "dac", "out", &mut memo);
    (v1 * v2 * v3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &'static str = r#"aaa: you hhh
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
        assert_eq!(part1(&parse(INPUT1)), "5");
        assert_eq!(part1(&parse(&input_for(2025, 11))), "470");
    }

    const INPUT2: &'static str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT2)), "2");
        assert_eq!(part2(&parse(&input_for(2025, 11))), "384151614084875");
    }
}

