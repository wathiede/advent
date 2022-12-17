use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct Grid {
    cells: HashMap<(isize, isize), char>,
    not_becons: HashMap<isize, Vec<Range<isize>>>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut cells: HashMap<(isize, isize), char> = HashMap::new();
        let mut not_becons: HashMap<isize, Vec<Range<isize>>> = HashMap::new();
        s.lines().for_each(|l| {
            let parts = l
                .split(|c: char| !c.is_digit(10) && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().expect("number"))
                .collect::<Vec<_>>();
            let sx = parts[0];
            let sy = parts[1];
            let bx = parts[2];
            let by = parts[3];
            cells.insert((sx, sy), 'S');
            cells.insert((bx, by), 'B');
            let r = manhattan_distance((sx, sy), (bx, by));
            //if sx == 8 && sy == 7 {
            for y in 0..r + 1 {
                let xd = (r - y);
                not_becons
                    .entry(sy - y)
                    .and_modify(|row| row.push(sx - xd..sx + xd + 1))
                    .or_insert(vec![sx - xd..sx + xd + 1]);
                not_becons
                    .entry(sy + y)
                    .and_modify(|row| row.push(sx - xd..sx + xd + 1))
                    .or_insert(vec![sx - xd..sx + xd + 1]);
            }
            //}
        });
        Ok(Grid { cells, not_becons })
    }
}

fn manhattan_distance(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_no_beacons, max_no_beacons) = self
            .not_becons
            .keys()
            .fold((isize::MAX, 0), |(min, max), k| (min.min(*k), max.max(*k)));
        let (width, height) = self.cells.keys().fold(
            ((isize::MAX..isize::MIN), (isize::MAX..isize::MIN)),
            |(w, h): (Range<isize>, Range<isize>), c: &(isize, isize)| {
                (
                    c.0.min(w.start)..c.0.max(w.end),
                    c.1.min(h.start).min(min_no_beacons)..c.1.max(h.end).max(max_no_beacons),
                )
            },
        );
        writeln!(f, "w {:?} h {:?}", width, height)?;
        for y in height.start..height.end + 1 {
            for x in width.start..width.end + 1 {
                match self.cells.get(&(x, y)) {
                    Some(c) => write!(f, "{}", *c)?,
                    None => match self.not_becons.get(&y) {
                        Some(row) => {
                            if row.iter().any(|r| r.contains(&x)) {
                                write!(f, "#")?;
                            } else {
                                write!(f, ".")?;
                            }
                        }
                        None => write!(f, ".")?,
                    },
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Grid {
    fn row_occupancy(&self, row: isize) -> usize {
        let mut s = HashSet::new();
        self.not_becons[&row].iter().for_each(|r| {
            for i in r.start..r.end {
                s.insert(i);
            }
        });
        s.len() - self.cells.keys().filter(|(_, y)| *y == row).count()
    }
}

fn solve1(input: &str, row: isize) -> usize {
    let g: Grid = input.parse().expect("parse");
    //println!("Grid\n{g}");
    g.row_occupancy(row)
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    solve1(input, 2000000)
}

// #[aoc(day15, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn p1() {
        assert_eq!(solve1(INPUT, 10), 26);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
