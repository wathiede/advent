use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Debug, Eq)]
enum Value {
    List(Vec<Value>),
    Int(usize),
}

impl PartialEq for Value {
    fn eq(&self, r: &Self) -> bool {
        self.cmp(r) == Ordering::Equal
    }
}

impl Ord for Value {
    fn cmp(&self, r: &Self) -> Ordering {
        use Value::*;
        let l = self;
        match (l, r) {
            (Int(l), Int(r)) => l.cmp(r),
            (List(l), List(r)) => l.cmp(r),
            (Int(l), List(_)) => List(vec![Int(*l)]).cmp(&r),
            (List(_), Int(r)) => l.cmp(&List(vec![Int(*r)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fmt_v(f: &mut fmt::Formatter<'_>, v: &Value) -> fmt::Result {
    use Value::*;
    match v {
        List(n) => {
            write!(f, "[")?;
            for (i, e) in n.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                fmt_v(f, e)?;
            }
            write!(f, "]")?;
        }
        Int(n) => write!(f, "{n}")?,
    }
    Ok(())
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_v(f, self)
    }
}

use std::iter::Peekable;
fn from_it<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Value {
    let mut vals = Vec::new();
    while let Some(v) = it.next() {
        match v {
            b'[' => vals.push(from_it(it)),
            b',' => continue,
            b']' => return Value::List(vals),
            n => {
                let mut n = n - b'0';
                let p = it.peek().expect("peek failed");
                if **p >= b'0' && **p <= b'9' {
                    n = n * 10 + (it.next().unwrap() - b'0');
                }
                vals.push(Value::Int(n.into()));
            }
        }
    }
    return Value::List(vals);
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Value, ()> {
        let mut it = s.as_bytes().into_iter().peekable();
        // Hack, bug in parser, top level has extra nested list
        if let Value::List(l) = from_it(&mut it) {
            Ok(l.into_iter().nth(0).expect("first"))
        } else {
            unreachable!("bug in parser")
        }
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let v = input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, l)| {
            let (l, r) = l.split_once('\n').expect("no tuple");
            let l: Value = l.parse().expect("couldn't parse l");
            let r: Value = r.parse().expect("couldn't parse r");
            //println!("== Pair {} ==", i + 1);
            if l.cmp(&r) != Ordering::Greater {
                //println!("Sorted");
                Some(i)
            } else {
                //println!("NOT sorted");
                None
            }
        })
        // Puzzle uses 1-based indexing
        .map(|i| i + 1)
        .sum();
    assert_ne!(448, v);
    assert_ne!(453, v);
    v
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    let two = Value::from_str("[[2]]").expect("parse 2");
    let six = Value::from_str("[[6]]").expect("parse 6");

    let mut v: Vec<_> = input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }

            Some(Value::from_str(l).expect("couldn't parse l"))
        })
        .chain([two, six])
        .collect();

    let two = Value::from_str("[[2]]").expect("parse 2");
    let six = Value::from_str("[[6]]").expect("parse 6");
    v.sort();
    let v = v
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v == &two || v == &six {
                Some(i)
            } else {
                None
            }
        })
        // Puzzle uses 1-based indexing
        .map(|i| i + 1)
        .product();
    assert_ne!(v, 33434);
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 140);
    }
}
