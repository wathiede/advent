use crate::prelude::*;

#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
}

impl Image {
    pub fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x as usize >= self.width {
            return None;
        }
        if y < 0 || y as usize >= self.height {
            return None;
        }
        Some(self[(x as usize, y as usize)])
    }
    /// Visits up to 8 neighbors, ignoring cells out of bounds
    pub fn visit_neighbors<MAP, REDUCE, T, U>(
        &self,
        (x, y): (isize, isize),
        compute: MAP,
        mut acc: REDUCE,
    ) where
        MAP: Fn(u8) -> T,
        REDUCE: FnMut(T) -> U,
    {
        for j in -1..=1 {
            for i in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                self.get(x + i, y + j).map(|b| acc(compute(b)));
            }
        }
    }
    pub fn kernel3x3_all<F>(&mut self, func: F)
    where
        F: Fn(u8) -> u8,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                self.kernel3x3((x, y), &func)
            }
        }
    }
    pub fn kernel3x3<F>(&mut self, (x, y): (usize, usize), func: F)
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
        self[(x, y)] = func(self[(x, y)]);
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
                write!(f, "{:2}", self[(x, y)] as char)?;
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
            .flat_map(|row| row.as_bytes().iter())
            .map(|b| *b)
            .collect();

        Ok(Image {
            width,
            height,
            pixels,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_neighbors() {
        let input = r#"111
111
111"#;
        let im: Image = input.parse().expect("failed to parse image");
        let mut sum = 0;
        im.visit_neighbors((1, 1), |b| b - b'0', |b| sum += b);
        assert_eq!(sum, 8);
    }
}
