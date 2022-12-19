use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Node {
    rate: usize,
    outbound: u64,
}

#[derive(Debug)]
struct Map {
    valves: Vec<Node>,
}

fn name_to_letter(s: &str) -> char {
    s.chars().next().unwrap().to_ascii_lowercase()
}
impl Map {
    /*
    fn print_graphviz(&self) {
    println!(
    r#"
    digraph G {{
    "#
    );
    for name in valves.enumerate() {
    let node = &self.valves[name];
    let mut outbound = node.outbound.clone();
    outbound.sort();
    for n in outbound {
    println!(
    "{}{} -> {}{};",
    name.to_ascii_uppercase(),
    name.to_ascii_uppercase(),
    n.to_ascii_uppercase(),
    n.to_ascii_uppercase()
    );
    }
    }
    for (name, node) in self.valves.iter() {
    println!(
    r#"{}{} [label="{}{} [{}]"]"#,
    name.to_ascii_uppercase(),
    name.to_ascii_uppercase(),
    name.to_ascii_uppercase(),
    name.to_ascii_uppercase(),
    node.rate
    );
    }
    println!(
    r#"
    AA [shape=Mdiamond];
    }}
    "#
    );
    }
    */
}
impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut names: Vec<_> = s
            .lines()
            .inspect(|l| {
                println!("l: {l}");
            })
            .map(|l| l[6..8].to_string())
            .collect();
        names.sort();
        dbg!(&names);
        let name_map: HashMap<String, usize> = names
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();
        dbg!(&name_map);
        let mut valves: Vec<_> = s
            .lines()
            .map(|l| {
                let l = l.replace(|c| c == '=' || c == ';', " ").replace(',', "");

                let mut it = l.split(' ');
                let name = it.nth(1).expect("name");
                let rate = it.nth(3).expect("rate").parse().expect("rate number");
                let outbound = it
                    .skip(5)
                    .fold(0, |outbound, name| outbound | 1 << name_map[name]);
                let node = Node { rate, outbound };
                (name_map[name], node)
            })
            .collect();
        valves.sort_by(|a, b| a.0.cmp(&b.0));
        let valves = valves.into_iter().map(|(_, v)| v).collect();

        Ok(Map { valves })
    }
}

fn solve1(
    m: &Map,
    cur: u64,
    ttl: usize,
    open: &mut u64,
    mut pressure: usize,
    solution: &mut usize,
) {
    if ttl == 0 || open.count_ones() == m.valves.len().try_into().unwrap() {
        println!("open = {:064b}\t\tpressure = {}", open, pressure,);
        *solution = *solution.max(&mut pressure);
        return;
    }

    // At each step we can open, if not already open, or move. Each action cost 1.
    let node: &Node = &m.valves[cur as usize];

    if open.count_ones() == m.valves.len().try_into().unwrap() {
        return;
    }

    // Try opening
    if *open & (1 << cur) == 0 {
        *open |= 1 << cur;
        pressure += ttl * node.rate;
        solve1(m, cur, ttl - 1, open, pressure, solution);
        pressure -= ttl * node.rate;
        *open &= !(1 << cur);
    }

    // Try moving to each neighbor
    for out in node.outbound.trailing_zeros()..64 - node.outbound.leading_zeros() {
        println!("{:064b} {}", node.outbound, out);
        solve1(m, out.into(), ttl - 1, open, pressure, solution);
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let m: Map = input.parse().expect("map");
    //m.print_graphviz();
    //dbg!(&m);
    let mut solution = 0;
    let mut open = 0;
    let pressure = 0;
    solve1(&m, 0, 30, &mut open, pressure, &mut solution);
    solution
}

// #[aoc(day16, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 1651);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
