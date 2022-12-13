use crate::prelude::*;

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Image {
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
