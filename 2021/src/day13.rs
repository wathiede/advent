use advent::prelude::*;
use aoc_runner_derive::aoc;

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        let pixels = vec![0; width * height];
        Image {
            width,
            height,
            pixels,
        }
    }
    fn new_with_pts(width: usize, height: usize, pts: &[(usize, usize)]) -> Image {
        let pixels = vec![0; width * height];
        let mut im = Image {
            width,
            height,
            pixels,
        };
        dbg!(&width, &height);
        pts.iter().for_each(|xy| im[*xy] = 1);
        im
    }
    fn fold_y(&self, y_axis: usize) -> Image {
        println!("fold_y @ {}", y_axis);
        let mut im = Image::new(self.width, y_axis);
        let odd = self.height % 2;
        for y in 0..self.height {
            for x in 0..self.width {
                //dbg!( self.width, self.height, x, y, y_axis, (y % y_axis), self.pixels.len(), im.pixels.len());
                if self[(x, y)] > 0 {
                    if y > y_axis {
                        im[(x, self.height - y - odd)] = self[(x, y)];
                    } else {
                        im[(x, y)] = self[(x, y)];
                    }
                }
            }
        }
        im
    }
    fn fold_x(&self, x_axis: usize) -> Image {
        let odd = self.width % 2;
        println!("fold_x @ {}", x_axis);
        for y in 0..self.height {
            assert_eq!(
                self[(x_axis, y)],
                0,
                "w,h {},{} x_axis {}",
                self.width,
                self.height,
                x_axis,
            );
        }
        let mut im = Image::new(x_axis, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] > 0 {
                    if x > x_axis {
                        im[(self.width - x - odd, y)] = self[(x, y)];
                    } else {
                        im[(x, y)] = self[(x, y)];
                    }
                }
            }
        }
        im
    }

    fn count(&self) -> usize {
        self.pixels.iter().filter(|&n| *n != 0).count()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] > 0 {
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

impl Index<(usize, usize)> for Image {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        //dbg!(self.width, self.height, x, y, self.pixels.len());
        &mut self.pixels[x + y * self.width]
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> Result<usize> {
    let (pts, folds) = input.split_once("\n\n").unwrap();
    let pts: Vec<(usize, usize)> = pts
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<_> = folds
        .lines()
        .map(|l| l.split(' ').nth(2).unwrap().split_once('=').unwrap())
        .map(|(axis, idx)| (axis, idx.parse().unwrap()))
        .collect();
    let (maxx, maxy) = pts
        .iter()
        .fold((0, 0), |(maxx, maxy), (x, y)| (maxx.max(*x), maxy.max(*y)));
    let mut im = Image::new_with_pts(maxx + 1, maxy + 1, &pts);
    //dbg!(&im);

    for (axis, idx) in folds.iter().take(1) {
        im = if *axis == "y" {
            im.fold_y(*idx)
        } else {
            im.fold_x(*idx)
        };
    }
    //assert!(im.count() < 896);
    dbg!(&im);
    Ok(im.count())
}

#[aoc(day13, part2)]
fn part2(input: &str) -> Result<usize> {
    let (pts, folds) = input.split_once("\n\n").unwrap();
    let pts: Vec<(usize, usize)> = pts
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<_> = folds
        .lines()
        .map(|l| l.split(' ').nth(2).unwrap().split_once('=').unwrap())
        .map(|(axis, idx)| (axis, idx.parse().unwrap()))
        .collect();
    let (maxx, maxy) = pts
        .iter()
        .fold((0, 0), |(maxx, maxy), (x, y)| (maxx.max(*x), maxy.max(*y)));
    let mut im = Image::new_with_pts(maxx + 1, maxy + 1, &pts);
    //dbg!(&im);

    for (axis, idx) in folds.iter() {
        im = if *axis == "y" {
            im.fold_y(*idx)
        } else {
            im.fold_x(*idx)
        };
    }
    dbg!(&im);
    Ok(im.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5

"#
        .trim();
        assert_eq!(part1(input)?, 17);
        Ok(())
    }

    #[test]
    fn test_fold_x() -> Result<()> {
        let input = r#"
0,0
1,1
3,3
4,4

fold along x=2
fold along y=2

"#
        .trim();
        let (pts, folds) = input.split_once("\n\n").unwrap();
        let pts: Vec<(usize, usize)> = pts
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        let folds: Vec<_> = folds
            .lines()
            .map(|l| l.split(' ').nth(2).unwrap().split_once('=').unwrap())
            .map(|(axis, idx)| (axis, idx.parse().unwrap()))
            .collect();
        let (maxx, maxy) = pts
            .iter()
            .fold((0, 0), |(maxx, maxy), (x, y)| (maxx.max(*x), maxy.max(*y)));
        let mut im = Image::new_with_pts(maxx + 1, maxy + 1, &pts);
        dbg!(&im);
        for (axis, idx) in folds.iter() {
            im = if *axis == "y" {
                im.fold_y(*idx)
            } else {
                im.fold_x(*idx)
            };
        }
        dbg!(&im);
        //assert_eq!(im.count(), 17);
        Ok(())
    }
    /*
    #[test]
    fn test_part2()->Result<()> {
    let input = r#"
    "#
    .trim();
    assert_eq!(part2(input)?, usize::MAX);
    Ok(())
    }
    */
}
