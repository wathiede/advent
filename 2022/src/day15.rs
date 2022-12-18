use std::time::Instant;

use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Clone, Default, Eq, PartialEq)]
struct Bitset {
    sets: HashMap<isize, u64>,
}

impl fmt::Debug for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self
            .sets
            .keys()
            .fold((isize::MAX, isize::MIN), |(min, max), v| {
                (min.min(*v), max.max(*v))
            });
        for i in min..max {
            if i == 0 {
                write!(f, "|")?;
            }
            write!(f, "{:064b}", self.sets[&i])?;
        }
        Ok(())
    }
}

impl FromIterator<isize> for Bitset {
    fn from_iter<I: IntoIterator<Item = isize>>(iter: I) -> Self {
        let mut bs = Bitset::default();
        for i in iter {
            bs.set(i);
        }
        bs
    }
}

impl Bitset {
    fn is_empty(&self) -> bool {
        self.sets.is_empty()
    }
    fn set(&mut self, v: isize) {
        let idx = if v >= 0 { v / 64 } else { (v / 64) - 1 };
        let bit: u64 = 1 << ((v.abs() % 64) as u64);
        self.sets
            .entry(idx)
            .and_modify(|v| *v |= bit)
            .or_insert(bit);
    }
    // TODO(wathiede): add set_rng that can optimize by doing the divide once, and potentially set
    // 64 bits at a time.
    fn contains(&self, v: isize) -> bool {
        let idx = if v >= 0 { v / 64 } else { (v / 64) - 1 };
        let bit: u64 = 1u64 << ((v.abs() % 64) as u64);
        if !self.sets.contains_key(&idx) {
            return false;
        }
        //if v < 0 { println!( "v {idx} {} sets {:064b} bit {:064b}", v, self.sets[&idx], bit); }
        (self.sets[&idx] & bit) != 0
    }
    fn len(&self) -> usize {
        self.sets
            .values()
            .map(|v| v.count_ones())
            .sum::<u32>()
            .try_into()
            .unwrap()
    }
    fn truncate(&mut self, max: usize) {
        let idx: isize = (max / 64).try_into().unwrap();
        let mut sets: HashMap<_, _> = std::mem::take(&mut self.sets)
            .into_iter()
            .filter(|(k, v)| *k >= 0 && *k <= idx)
            .collect();
        let bit: u64 = 1u64 << ((max % 64) as u64);
        let mask = if bit == 63 { !0 } else { (bit) - 1 };
        println!("{max:2} {} {:064b} {:064b}", idx, bit, mask);
        sets.iter().for_each(|(idx, b)| {
            println!("sets {idx:2} {b:064b}");
        });
        sets.entry(idx).and_modify(|v| *v &= mask);
        // If none of the bits are set in the last element, truncate it too.
        if sets[&idx] == 0 {
            sets.remove(&idx);
        }
        self.sets = sets;
    }
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<(isize, isize), char>,
    not_becons: HashMap<isize, Bitset>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut cells: HashMap<(isize, isize), char> = HashMap::new();
        let mut not_becons: HashMap<isize, Bitset> = HashMap::new();
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
            {
                for y in 0..r + 1 {
                    let xd = r - y;
                    not_becons
                        .entry(sy - y)
                        .and_modify(|row| (sx - xd..sx + xd + 1).for_each(|x| row.set(x)))
                        .or_insert((sx - xd..sx + xd + 1).collect());
                    not_becons
                        .entry(sy + y)
                        .and_modify(|row| (sx - xd..sx + xd + 1).for_each(|x| row.set(x)))
                        .or_insert((sx - xd..sx + xd + 1).collect());
                }
            }
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
            let empty = Bitset::default();
            let row = self.not_becons.get(&y).unwrap_or(&empty);
            for x in width.start..width.end + 1 {
                match self.cells.get(&(x, y)) {
                    Some(c) => write!(f, "{}", *c)?,
                    None => {
                        if row.contains(x) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Grid {
    fn row_occupancy(&self, row: isize) -> usize {
        self.not_becons[&row].len() - self.cells.keys().filter(|(_, y)| *y == row).count()
    }
    fn find_hole(&self, max: isize) -> (isize, isize) {
        println!("self.not_becons {}", self.not_becons.len());
        for (y, bcns) in self.not_becons.iter() {
            //dbg!(&s.len(), &max);
            if bcns.len() == max.try_into().unwrap() {
                for x in 0..max {
                    if !bcns.contains(x) {
                        return (x, *y);
                    }
                }
            }
        }
        unreachable!();
    }
    fn tune(&self, max: isize) -> isize {
        let (x, y) = self.find_hole(max);
        x * 4000000 + y
    }
    fn truncate(&mut self, max: isize) {
        let rm_keys: Vec<_> = self
            .cells
            .keys()
            .filter(|(x, y)| *x < 0 || *x > max || *y < 0 || *y > max)
            .cloned()
            .collect();
        for k in rm_keys {
            self.cells.remove(&k);
        }

        let not_becons = self
            .not_becons
            .iter_mut()
            .filter(|(k, _v)| **k >= 0 && **k <= max)
            .map(|(k, bs)| {
                bs.truncate(max.try_into().map(|v: usize| v + 1).unwrap());
                (*k, bs)
            })
            .filter(|(_k, bs)| !bs.is_empty())
            .map(|(k, bs)| (k, bs.clone()))
            .collect();
        self.not_becons = not_becons;
        //dbg!(self.not_becons.keys());
    }
}

fn solve1(input: &str, row: isize) -> usize {
    let start = Instant::now();
    let g: Grid = input.parse().expect("parse");
    println!("Grid\n{g}");
    g.row_occupancy(row)
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    solve1(input, 2000000)
}

fn solve2(input: &str, max: isize) -> isize {
    let start = Instant::now();

    let mut g: Grid = input.parse().expect("parse");
    println!("After parse {}", start.elapsed().as_secs());
    println!("Grid\n{g}");
    g.truncate(max);
    println!("After truncate {}", start.elapsed().as_secs());
    println!("Grid\n{g}");
    let t = g.tune(max);
    println!("After tune {}", start.elapsed().as_secs());
    t
}

#[aoc(day15, part2)]
fn part2(input: &str) -> isize {
    solve2(input, 4000000)
}

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

    #[test]
    fn p2() {
        assert_eq!(solve2(INPUT, 20), 56000011);
    }

    #[test]
    fn bitset_truncate() {
        let mut bs = Bitset::from_iter(0..64);
        bs.truncate(0);
        assert_eq!(Bitset::default(), bs);

        let mut bs = Bitset::from_iter(0..64);
        bs.truncate(20);
        assert_eq!(Bitset::from_iter(0..20), bs);

        let mut bs = Bitset::from_iter(0..128);
        bs.truncate(20);
        assert_eq!(Bitset::from_iter(0..20), bs);

        let mut bs = Bitset::from_iter(0..256);
        bs.truncate(128);
        assert_eq!(Bitset::from_iter(0..128), bs);

        let mut bs = Bitset::from_iter(-256..256);
        bs.truncate(128);
        assert_eq!(Bitset::from_iter(0..128), bs);
    }
}
