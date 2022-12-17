use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct Grid {
    cells: HashMap<(usize, usize), char>,
    width: Range<usize>,
    height: Range<usize>,
    start: (usize, usize),
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let start = (500, 0);
        let mut cells: HashMap<(usize, usize), char> = HashMap::new();
        s.lines().for_each(|l| {
            println!("*******************");
            dbg!(&l);
            l.split(" -> ")
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|se| {
                    let s: (usize, usize) = se[0]
                        .split_once(',')
                        .map(|(x, y)| (x.parse().expect("s x"), y.parse().expect("s y")))
                        .expect("s");
                    let e: (usize, usize) = se[1]
                        .split_once(',')
                        .map(|(x, y)| ((x.parse().expect("e x"), y.parse().expect("e y"))))
                        .expect("e");
                    let start = (s.0.min(e.0), s.1.min(e.1));
                    let end = (s.0.max(e.0) + 1, s.1.max(e.1) + 1);

                    println!("drawing  {:?} -> {:?}", &start, &end);
                    for y in start.1..end.1 {
                        for x in start.0..end.0 {
                            println!("inserting @ {x:?} {y:?}");
                            cells.insert((x, y), '#');
                        }
                    }
                });
        });
        let (width, height) = cells.keys().chain([&start]).fold(
            ((usize::MAX..0), (usize::MAX..0)),
            |(w, h): (Range<usize>, Range<usize>), c: &(usize, usize)| {
                (
                    c.0.min(w.start)..c.0.max(w.end),
                    c.1.min(h.start)..c.1.max(h.end),
                )
            },
        );
        Ok(Grid {
            cells,
            width,
            height,
            start,
        })
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "w {:?} h {:?}", self.width, self.height)?;
        for y in self.height.start..self.height.end + 1 {
            for x in self.width.start..self.width.end + 1 {
                if (x, y) == self.start {
                    write!(f, "+")?;
                    continue;
                }
                match self.cells.get(&(x, y)) {
                    Some(c) => write!(f, "{}", *c)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Grid {
    fn drop(&mut self) -> bool {
        self.drop_rec(self.start).is_some()
    }

    // Returns
    //   None if we reach past the bottom
    //   Some(v) if we hit something, v indicates what was hit
    fn drop_rec(&mut self, xy: (usize, usize)) -> Option<char> {
        //println!("xy {xy:?}");
        if xy.1 > self.height.end {
            // Fell off the bottom
            return None;
        }

        let something_here = self.cells.contains_key(&xy);
        if something_here {
            return Some(self.cells[&xy]);
        }

        let down = (xy.0, xy.1 + 1);
        let down_left = (xy.0 - 1, xy.1 + 1);
        let down_right = (xy.0 + 1, xy.1 + 1);
        let something_below = self.cells.contains_key(&down);
        if something_below {
            let something_down_left = self.cells.contains_key(&down_left);
            if !something_down_left {
                return self.drop_rec(down_left);
            }

            let something_down_right = self.cells.contains_key(&down_right);
            if !something_down_right {
                return self.drop_rec(down_right);
            }

            self.cells.insert(xy, 'o');
            return Some('o');
        } else {
            return self.drop_rec(down);
        }

        None
    }

    fn count_sand(&self) -> usize {
        self.cells.values().filter(|&&c| c == 'o').count()
    }
}
#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let mut g: Grid = input.parse().expect("grid");
    println!("g:\n{g}");
    for i in 0.. {
        if !g.drop() {
            return i;
        }
        println!("g{i}\n{g}");
    }
    g.count_sand()
}

// #[aoc(day14, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 24);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
