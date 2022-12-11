use std::{collections::VecDeque, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Op {
    Mul(Factor),
    Add(Factor),
    Sq,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Factor>,
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
            .map(|n| Factor::new(n))
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
    fn recv(&mut self, item: Factor) {
        self.items.push_back(item);
    }
}

//#[aoc(day11, part1)]
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
                //item /= 3;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Factor {
    // Factors [2,3,5,7,11,13,17,19]
    prime_powers: [u32; 8],
    remainder: usize,
}

const PRIMES: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
impl Factor {
    fn new(i: usize) -> Factor {
        let remainder = i;
        let prime_powers = [0; 8];
        let mut f = Factor {
            prime_powers,
            remainder,
        };
        f.reduce();
        f
    }
    fn reduce(&mut self) {
        for (i, p) in PRIMES.into_iter().enumerate() {
            loop {
                if self.remainder % p == 0 {
                    self.prime_powers[i] += 1;
                    self.remainder /= p;
                    if self.remainder == 1 {
                        self.remainder = 0;
                        return;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

impl FromStr for Factor {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Factor::new(s.parse::<usize>()?))
    }
}

use std::fmt;
impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = PRIMES
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, b)| acc + b.pow(self.prime_powers[i]));
        write!(f, "{}", v)
    }
}

use std::ops::Mul;
impl Mul for Factor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut prime_powers = [0; 8];
        for i in 0..8 {
            prime_powers[i] = self.prime_powers[i] + rhs.prime_powers[i];
        }
        let remainder = self.remainder + rhs.remainder;
        let mut f = Factor {
            prime_powers,
            remainder,
        };
        f.reduce();
        f
    }
}

impl Mul<usize> for Factor {
    type Output = Factor;

    fn mul(self, rhs: usize) -> Factor {
        todo!("mul usize")
    }
}

use std::ops::Add;
impl Add for Factor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!("add Self")
    }
}

use std::ops::Rem;
impl Rem<usize> for Factor {
    type Output = usize;

    fn rem(self, rhs: usize) -> usize {
        if let Some(idx) = PRIMES.into_iter().position(|v| v == rhs) {
            if self.prime_powers[idx] > 0 {
                return 0;
            }
        }
        self.remainder % rhs
    }
}

#[test]
fn factors() {
    assert_eq!(Factor::new(2).prime_powers[0], 1);
    assert_eq!(Factor::new(4).prime_powers[0], 2);
    assert_eq!(Factor::new(6).prime_powers[0..2], [1, 1]);
    assert_eq!(Factor::new(5).prime_powers[2], 1);
    assert_eq!(Factor::new(126).prime_powers[..], [1, 2, 0, 1, 0, 0, 0, 0]);
    let two = Factor::new(2);
    assert_eq!(two * two, Factor::new(4));
    assert_eq!(two * 3, Factor::new(6));
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| s.parse().expect("couldn't parse monkey"))
        .collect();
    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut trades = Vec::new();
            let m = &mut monkeys[i];
            while let Some(item) = m.items.pop_front() {
                //let item = prime_reduce(item);
                m.inspect_count += 1;
                use Op::*;
                let mut item = match m.op {
                    Sq => item * item,
                    Add(n) => item + n,
                    Mul(n) => item * n,
                };
                //println!("r {} m {} i {}", round, i, item);
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
        {
            let round = round + 1;
            if round == 1 || round == 20 || round % 1000 == 0 {
                println!("After round {}", round);
                monkeys.iter().enumerate().for_each(|(i, m)| {
                    println!("Monkey {} inspected items {} times.", i, m.inspect_count)
                });
            }
        }
    }
    monkeys.sort_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));
    //dbg!(&monkeys);
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

#[test]
fn p2() {
    assert_eq!(part2(INPUT), 2713310158);
}
