use std::{
    collections::HashSet,
    convert::Infallible,
    fmt::{Debug, Error, Formatter},
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::aoc;

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    flashes: usize,
}

impl Image {
    fn kernel3x3<F>(&mut self, (x, y): (usize, usize), func: F)
    where
        F: Fn(u8) -> u8,
    {
        if x > 0 {
            self[(x - 1, y)] = func(self[(x - 1, y)]);
            if y > 0 {
                self[(x - 1, y - 1)] = func(self[(x - 1, y - 1)]);
            }
            if y < self.height - 1 {
                self[(x - 1, y + 1)] = func(self[(x - 1, y + 1)]);
            }
        }

        if y > 0 {
            self[(x, y - 1)] = func(self[(x, y - 1)]);
        }
        if y < self.height - 1 {
            self[(x, y + 1)] = func(self[(x, y + 1)]);
        }

        if x < self.width - 1 {
            self[(x + 1, y)] = func(self[(x + 1, y)]);
            if y > 0 {
                self[(x + 1, y - 1)] = func(self[(x + 1, y - 1)]);
            }
            if y < self.height - 1 {
                self[(x + 1, y + 1)] = func(self[(x + 1, y + 1)]);
            }
        }
    }

    fn step(&mut self) {
        self.pixels.iter_mut().for_each(|p| *p += 1);
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let mut flashes = 0;
            // Apply the effect of a flash on neighbors
            let mut need_to_flash = Vec::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    if self[(x, y)] > 9 && !flashed.contains(&(x, y)) {
                        need_to_flash.push((x, y));
                    }
                }
            }
            for (x, y) in need_to_flash {
                self.kernel3x3((x, y), |x| x + 1);
                flashed.insert((x, y));
                flashes += 1;
            }

            if flashes == 0 {
                break;
            }
            self.flashes += flashes;
        }

        self.pixels.iter_mut().for_each(|p| {
            if *p > 9 {
                *p = 0
            }
        });
    }

    fn sync(&self) -> bool {
        let sentinel = self[(0, 0)];
        for p in &self.pixels {
            if *p != sentinel {
                return false;
            }
        }
        true
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixels == other.pixels
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:3}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Image {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().collect();
        let width = rows[0].len();
        let height = rows.len();
        let pixels = rows
            .iter()
            .flat_map(|row| row.as_bytes().iter().map(|b| b - b'0'))
            .collect();

        Ok(Image {
            width,
            height,
            pixels,
            flashes: 0,
        })
    }
}

impl Index<(usize, usize)> for Image {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[x + y * self.width]
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut im: Image = input.parse()?;
    for _ in 0..100 {
        im.step();
    }
    if im.width > 11 {
        assert!(im.flashes > 1355);
    }
    Ok(im.flashes)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Result<usize> {
    let mut im: Image = input.parse()?;
    for i in 1.. {
        im.step();
        if im.sync() {
            return Ok(i);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#
        .trim();
        assert_eq!(part1(input)?, 1656);
        Ok(())
    }

    #[test]
    fn test_step() -> Result<()> {
        let mut im: Image = r#"
11111
19991
19191
19991
11111
"#
        .trim()
        .parse()?;
        let step1: Image = r#"
34543
40004
50005
40004
34543
"#
        .trim()
        .parse()?;

        let step2: Image = r#"
45654
51115
61116
51115
45654
"#
        .trim()
        .parse()?;

        im.step();
        assert_eq!(im, step1);
        im.step();
        assert_eq!(im, step2);
        Ok(())
    }

    #[test]
    fn test_many_iterations() -> Result<()> {
        let mut im: Image = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#
        .trim()
        .parse()?;

        let step1: Image = r#"
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"#
        .trim()
        .parse()?;

        let step2: Image = r#"
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
"#
        .trim()
        .parse()?;

        let step3: Image = r#"
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000
"#
        .trim()
        .parse()?;

        let step4: Image = r#"
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211
"#
        .trim()
        .parse()?;

        let step5: Image = r#"
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322
"#
        .trim()
        .parse()?;

        let step6: Image = r#"
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433
"#
        .trim()
        .parse()?;

        let step7: Image = r#"
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644
"#
        .trim()
        .parse()?;

        let step8: Image = r#"
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755
"#
        .trim()
        .parse()?;

        let step9: Image = r#"
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976
"#
        .trim()
        .parse()?;

        let step10: Image = r#"
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
"#
        .trim()
        .parse()?;
        let step10_flashes = 204;

        im.step();
        assert_eq!(im, step1, "step1");
        im.step();
        assert_eq!(im, step2, "step2");
        im.step();
        assert_eq!(im, step3, "step3");
        im.step();
        assert_eq!(im, step4, "step4");
        im.step();
        assert_eq!(im, step5, "step5");
        im.step();
        assert_eq!(im, step6, "step6");
        im.step();
        assert_eq!(im, step7, "step7");
        im.step();
        assert_eq!(im, step8, "step8");
        im.step();
        assert_eq!(im, step9, "step9");
        im.step();
        assert_eq!(im, step10, "step10");
        assert_eq!(im.flashes, step10_flashes, "step10 wrong flashes");
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
    "#
        .trim();
        assert_eq!(part2(input)?, 195);
        Ok(())
    }
}
