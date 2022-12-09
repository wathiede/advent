use std::{collections::HashSet, fmt};

use aoc_runner_derive::aoc;

#[derive(Debug, Default)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Default)]
struct Grid {
    t: Pos,
    h: Pos,
    places: HashSet<(isize, isize)>,
}

impl Grid {
    fn places(&self) -> usize {
        self.places.len()
    }
    fn walk(&mut self, d: &str, n: isize) {
        for _ in 0..n {
            self.step(d);
        }
    }
    fn step(&mut self, d: &str) {
        // Move head.
        match d {
            "R" => self.h.x += 1,
            "L" => self.h.x -= 1,
            "U" => self.h.y -= 1,
            "D" => self.h.y += 1,
            _ => panic!("unexpected dir"),
        }
        let x_d = self.h.x - self.t.x;
        let y_d = self.h.y - self.t.y;
        //dbg!(&self.h, &self.t, x_d, y_d);
        if x_d.abs() > 1 {
            self.t.x += x_d.signum();
            if y_d != 0 {
                self.t.y = self.h.y;
            }
        }
        if y_d.abs() > 1 {
            self.t.y += y_d.signum();
            if x_d != 0 {
                self.t.x = self.h.x;
            }
        }
        self.places.insert((self.t.x, self.t.y));
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::cmp::{max, min};
        let (x_min, x_max, y_min, y_max) = self.places.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(xi, xa, yi, ya), (x, y)| (min(*x, xi), max(*x, xa), min(*y, yi), max(*y, ya)),
        );
        //dbg!((x_min, x_max, y_min, y_max));
        for y in y_min - 2..y_max + 2 {
            for x in x_min - 2..x_max + 2 {
                if self.places.contains(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut g = Grid::default();
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(d, n)| (d, n.parse::<isize>().expect("couldn't parse number")))
                .expect("couldn't split line")
        })
        .for_each(|(d, n)| {
            g.walk(d, n);
            //println!("{} {}\n{}", d, n, g);
        });
    g.places()
}

#[test]
fn p1() {
    let s = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(part1(s), 13);
}

// #[aoc(day9, part2)]
// fn part2(input: &str) -> usize { }
