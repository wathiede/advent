use std::{collections::VecDeque, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Op {
    Mul(usize),
    Add(usize),
    Sq,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test_div: usize,
    true_idx: usize,
    false_idx: usize,

    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        _ = it.next().expect("first");

        let s = it.next().expect("second");
        let items = s[s.find(':').expect(":") + 2..]
            .split(", ")
            .map(|n| n.parse().expect("item number"))
            .collect();

        let o: Vec<_> = it.next().expect("third").split(' ').skip(6).collect();
        let op = match o.as_slice() {
            ["*", "old"] => Op::Sq,
            ["*", n] => Op::Mul(n.parse().expect("op num")),
            ["+", n] => Op::Add(n.parse().expect("op num")),
            c => panic!("unknown {:?}", c),
        };

        let test_div = it
            .next()
            .expect("forth")
            .split_whitespace()
            .last()
            .expect("last div")
            .parse()
            .expect("div num");
        let true_idx = it
            .next()
            .expect("fifth")
            .split_whitespace()
            .last()
            .expect("last true_idx")
            .parse()
            .expect("true_idx num");
        let false_idx = it
            .next()
            .expect("six")
            .split_whitespace()
            .last()
            .expect("last false_idx")
            .parse()
            .expect("false_idx num");
        Ok(Monkey {
            items,
            op,
            test_div,
            true_idx,
            false_idx,

            inspect_count: 0,
        })
    }
}

impl Monkey {
    fn recv(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| s.parse().expect("couldn't parse monkey"))
        .collect();
    for round in 0..20 {
        for i in 0..monkeys.len() {
            let mut trades = Vec::new();
            let m = &mut monkeys[i];
            while let Some(item) = m.items.pop_front() {
                m.inspect_count += 1;
                use Op::*;
                let mut item = match m.op {
                    Sq => item * item,
                    Add(n) => item + n,
                    Mul(n) => item * n,
                };
                println!("r {} m {} i {}", round, i, item);
                item /= 3;
                if item % m.test_div == 0 {
                    trades.push((m.true_idx, item));
                } else {
                    trades.push((m.false_idx, item));
                }
            }
            for (idx, item) in trades.into_iter() {
                monkeys[idx].recv(item);
            }
        }
        println!("After round {}", round + 1);
        monkeys.iter().enumerate().for_each(|(i, m)| {
            println!(
                "Monkey {}: {}",
                i,
                m.items
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        });
    }
    monkeys.sort_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));
    monkeys
        .iter()
        .enumerate()
        .for_each(|(i, m)| println!("Monkey {} inspected items {} times.", i, m.inspect_count));
    dbg!(&monkeys);
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

const INPUT: &'static str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1 "#;

#[test]
fn p1() {
    assert_eq!(part1(INPUT), 10605);
}

// #[aoc(day11, part2)]
// fn part2(input: &str) -> usize { }
