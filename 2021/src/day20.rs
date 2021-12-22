use advent::prelude::*;
use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

struct Image(HashSet<(isize, isize)>);

impl Image {
    fn new(input: &str) -> Image {
        let rows: Vec<_> = input.lines().collect();
        let width = rows[0].len();
        Image(
            rows.iter()
                .flat_map(|row| row.as_bytes().iter())
                .enumerate()
                .filter(|(_i, b)| *b == &b'#')
                .map(|(i, _b)| ((i % width) as isize, (i / width) as isize))
                .collect(),
        )
    }
    fn lookup(
        &self,
        x: isize,
        y: isize,
        algo: &[bool],
        odd: bool,
        x_rng: &RangeInclusive<isize>,
        y_rng: &RangeInclusive<isize>,
    ) -> usize {
        assert_eq!(algo.len(), 512);
        let mut idx = 0;
        for y_off in -1..=1 {
            for x_off in -1..=1 {
                let x_idx = x + x_off;
                let y_idx = y + y_off;
                let out_of_bounds = !(x_rng.contains(&x_idx) && y_rng.contains(&y_idx));
                let val = if (odd && out_of_bounds && algo[0]) || self.0.contains(&(x_idx, y_idx)) {
                    1
                } else {
                    0
                };
                idx <<= 1;
                idx |= val;
            }
        }
        idx
    }
    fn extents(&self) -> (isize, isize, isize, isize) {
        self.0.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            },
        )
    }

    fn enhance(&self, algo: &[bool], odd: bool) -> Image {
        let (min_x, max_x, min_y, max_y) = self.extents();
        let x_rng = min_x..=max_x;
        let y_rng = min_y..=max_y;
        let mut new_im = HashSet::new();
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let idx = self.lookup(x, y, algo, odd, &x_rng, &y_rng);
                if algo[idx] {
                    new_im.insert((x, y));
                }
            }
        }
        Image(new_im)
    }

    fn lights(&self) -> usize {
        self.0.len()
    }
    fn crop(&self, min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> Image {
        let x_rng = min_x..=max_x;
        let y_rng = min_y..=max_y;
        Image(
            self.0
                .iter()
                .filter(|(x, y)| x_rng.contains(x) && y_rng.contains(y))
                .cloned()
                .collect(),
        )
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let (min_x, max_x, min_y, max_y) = self.extents();
        writeln!(f, "({}..{})x({}..{})", min_x, max_x, min_y, max_y)?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.0.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn process(im: Image, algo: &[bool], num_steps: isize) -> Image {
    let mut im = im;
    for step in 0..num_steps {
        let (min_x, max_x, min_y, max_y) = im.extents();
        im = im.enhance(algo, step % 2 == 1);
        im = im.crop(min_x - 1, max_x + 1, min_y - 1, max_y + 1)
    }
    im
}

#[aoc(day20, part1)]
fn part1(input: &str) -> Result<usize> {
    let (algo, im) = input.split_once("\n\n").unwrap();
    let im = Image::new(im);
    let algo: Vec<bool> = algo.as_bytes().iter().map(|c| c == &b'#').collect();
    let im = process(im, &algo, 2);

    dbg!(&im, im.lights());
    let answer = im.lights();
    assert!(answer == 5268 || answer == 35);
    Ok(answer)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> Result<usize> {
    let (algo, im) = input.split_once("\n\n").unwrap();
    let im = Image::new(im);
    let algo: Vec<bool> = algo.as_bytes().iter().map(|c| c == &b'#').collect();
    let im = process(im, &algo, 50);

    dbg!(&im, im.lights());
    let answer = im.lights();
    assert!(answer < 19245);
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup() -> Result<()> {
        let input = r#"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#
.trim();
        let (algo, im) = input.split_once("\n\n").unwrap();
        let im = Image::new(im);
        let algo: Vec<bool> = algo.as_bytes().iter().map(|c| c == &b'#').collect();
        let (min_x, max_x, min_y, max_y) = im.extents();
        assert_eq!(
            im.lookup(2, 2, &algo, false, &(min_x..=max_x), &(min_y..=max_y)),
            34,
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#
.trim();
        assert_eq!(part1(input)?, 35);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#
.trim();
        assert_eq!(part2(input)?, 3351);
        Ok(())
    }
}
