use std::{collections::HashSet, fmt};

use aoc_runner_derive::aoc;

#[derive(Clone, Debug, Default)]
struct Pos {
    x: isize,
    y: isize,
}

struct Grid {
    knots: Vec<Pos>,
    places: HashSet<(isize, isize)>,
}

impl Grid {
    fn new(num_knots: usize) -> Grid {
        Grid {
            knots: (0..num_knots).map(|_| Pos::default()).collect(),
            places: HashSet::default(),
        }
    }
    fn places(&self) -> usize {
        self.places.len()
    }
    fn walk(&mut self, d: &str, n: isize) {
        //println!("== {} {} ==", d, n);
        for _ in 0..n {
            self.step(d);
        }
    }
    fn step(&mut self, d: &str) {
        // Move head.
        match d {
            "R" => self.knots[0].x += 1,
            "L" => self.knots[0].x -= 1,
            "U" => self.knots[0].y -= 1,
            "D" => self.knots[0].y += 1,
            _ => panic!("unexpected dir"),
        }
        //dbg!(&self.knots);
        for i in 1..self.knots.len() {
            let h = &self.knots[i - 1];
            let mut t = self.knots[i].clone();
            let x_d = h.x - t.x;
            let y_d = h.y - t.y;
            //println!("{}: {:?} {:?} {:?} {:?}", i - 1, &h, &t, x_d, y_d);
            // 2,-2 + 0,0 => 1,-1
            // d=2,-2
            if x_d.abs() > 1 && y_d.abs() > 1 {
                t.x += x_d.signum();
                t.y += y_d.signum();
            } else {
                if x_d.abs() > 1 {
                    t.x += x_d.signum();
                    if y_d != 0 {
                        t.y = h.y;
                    }
                }
                if y_d.abs() > 1 {
                    t.y += y_d.signum();
                    if x_d != 0 {
                        t.x = h.x;
                    }
                }
            }
            // Only update the last knot's location.
            //dbg!(i, self.knots.len(), i == self.knots.len());
            if i == self.knots.len() - 1 {
                self.places.insert((t.x, t.y));
            }
            //println!("s: {} {:?}", i, &t);
            debug_assert!((h.x - t.x).abs() <= 1);
            debug_assert!((h.y - t.y).abs() <= 1);
            self.knots[i] = t;
        }
        //println!("Step:\n{}", &self);
    }
    fn fmt_visits(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::cmp::{max, min};
        let (x_min, x_max, y_min, y_max) = self.places.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(xi, xa, yi, ya), (x, y)| (min(*x, xi), max(*x, xa), min(*y, yi), max(*y, ya)),
        );
        //dbg!((x_min, x_max, y_min, y_max));
        for y in y_min - 2..y_max + 2 {
            for x in x_min - 2..x_max + 2 {
                if x == 0 && y == 0 {
                    write!(f, "s")?;
                } else if self.places.contains(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
    fn fmt_rope(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::cmp::{max, min};
        let (x_min, x_max, y_min, y_max) = self.knots.iter().map(|p| (p.x, p.y)).fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(xi, xa, yi, ya), (x, y)| (min(x, xi), max(x, xa), min(y, yi), max(y, ya)),
        );
        //dbg!((x_min, x_max, y_min, y_max));
        for y in y_min - 2..y_max + 2 {
            for x in x_min - 2..x_max + 2 {
                if let Some(p) = self.knots.iter().position(|k| k.x == x && k.y == y) {
                    if p == 0 {
                        write!(f, "H")?;
                    } else {
                        write!(f, "{}", p)?;
                    }
                } else if x == 0 && y == 0 {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.fmt_rope(f)
        } else {
            self.fmt_visits(f)
        }
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut g = Grid::new(2);
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(d, n)| (d, n.parse::<isize>().expect("couldn't parse number")))
                .expect("couldn't split line")
        })
        .for_each(|(d, n)| {
            g.walk(d, n);
            //println!("Walk\n{} {}\n{}", d, n, g);
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

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut g = Grid::new(10);
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(d, n)| (d, n.parse::<isize>().expect("couldn't parse number")))
                .expect("couldn't split line")
        })
        .for_each(|(d, n)| {
            g.walk(d, n);
        });
    g.places()
}

#[test]
fn p2() {
    let s1 = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(part2(s1), 1);
    let s2 = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    assert_eq!(part2(s2), 36);
}
