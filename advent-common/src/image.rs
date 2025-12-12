use std::hash::Hash;

use itertools::iproduct;

use crate::prelude::*;

#[derive(Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Image<T>
where
    T: Copy + Hash + Ord + PartialOrd,
{
    pub width: usize,
    pub height: usize,
    pixels: Vec<T>,
}

impl<T> Image<T>
where
    T: Copy + Default + Eq + Hash + Ord + PartialOrd,
{
    pub fn new(width: usize, height: usize, init: T) -> Image<T> {
        Image {
            width,
            height,
            pixels: vec![init; width * height],
        }
    }
    /// Sets all pixels to the default value for T
    pub fn clear(&mut self) {
        self.pixels.fill(T::default());
    }

    /// Compares all pixels in im @ offset x,y in self, if no pixels in self are currently is_set,
    /// the image can be blit
    pub fn can_blit(&self, (x_off, y_off): (usize, usize), im: &Image<T>, is_set: T) -> bool {
        for (x, y) in iproduct!(0..im.width, 0..im.height) {
            if im[(x, y)] == is_set && self[(x + x_off, y + y_off)] == is_set {
                return false;
            }
        }
        true
    }

    /// Copies im into self @ offset x,y where im is is_set
    pub fn blit(&mut self, (x_off, y_off): (usize, usize), im: &Image<T>, is_set: T) {
        for (x, y) in iproduct!(0..im.width, 0..im.height) {
            if im[(x, y)] == is_set {
                self[(x + x_off, y + y_off)] = im[(x, y)];
            }
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        if x < 0 || x as usize >= self.width {
            return None;
        }
        if y < 0 || y as usize >= self.height {
            return None;
        }
        Some(self[(x as usize, y as usize)])
    }
    /// Rotates 90 degrees clockwise and returns new Image
    pub fn rot90(&self) -> Self {
        let mut im = Image::new(self.height, self.width, T::default());
        for x in 0..self.width {
            for y in 0..self.height {
                im[(y, x)] = self[(self.width - x - 1, y)];
            }
        }
        im
    }
    /// Visits up to 8 neighbors, ignoring cells out of bounds
    pub fn visit_neighbors<MAP, REDUCE, U, V>(
        &self,
        (x, y): (isize, isize),
        compute: MAP,
        mut acc: REDUCE,
    ) where
        MAP: Fn(T) -> U,
        REDUCE: FnMut(U) -> V,
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
        F: Fn(T) -> T,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                self.kernel3x3((x, y), &func)
            }
        }
    }
    pub fn kernel3x3<F>(&mut self, (x, y): (usize, usize), func: F)
    where
        F: Fn(T) -> T,
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

impl<T> PartialEq for Image<T>
where
    T: PartialEq + Copy + Hash + Ord + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixels == other.pixels
    }
}

/// Draws the image as a grid. Default print has a space between columns, alternate ('#') format
/// prints a grid densly.
impl<T> Display for Image<T>
where
    T: Display + Copy + Hash + Ord + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                if f.alternate() {
                    write!(f, "{}", self[(x, y)])?;
                } else {
                    write!(f, "{:2}", self[(x, y)])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Debug for Image<T>
where
    T: Display + Copy + Hash + Ord + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{self}")?;
        Ok(())
    }
}

impl FromStr for Image<u8> {
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

impl FromStr for Image<char> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().collect();
        let width = rows[0].len();
        let height = rows.len();
        let pixels = rows.iter().flat_map(|row| row.chars()).collect();

        Ok(Image {
            width,
            height,
            pixels,
        })
    }
}

impl<T> Index<(usize, usize)> for Image<T>
where
    T: Copy + Hash + Ord + PartialOrd,
{
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Image<T>
where
    T: Copy + Hash + Ord + PartialOrd,
{
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
        let im: Image<u8> = input.parse().expect("failed to parse image");
        let mut sum = 0;
        im.visit_neighbors((1, 1), |b| b - b'0', |b| sum += b);
        assert_eq!(sum, 8);
    }
}
