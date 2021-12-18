use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Target {
    fn hit(&self, x: isize, y: isize) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

fn shoot(x: isize, y: isize, tgt: &Target) -> bool {
    let mut x_inc = x;
    let mut y_inc = y;
    let mut x_cur = 0;
    let mut y_cur = 0;
    while x_cur <= tgt.x_max && y_cur >= tgt.y_min {
        x_cur += x_inc;
        y_cur += y_inc;
        if x_inc > 0 {
            x_inc -= 1;
        }
        y_inc -= 1;
        if tgt.hit(x_cur, y_cur) {
            return true;
        }
    }
    false
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Target, Infallible> {
        let parts: Vec<_> = input.split(' ').collect();
        let x = &parts[2][2..].strip_suffix(',').unwrap();
        let y = &parts[3][2..];
        let (x_min, x_max) = x
            .split_once("..")
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .unwrap();
        let (y_min, y_max) = y
            .split_once("..")
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .unwrap();

        Ok(Target {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> Result<isize> {
    let tgt: Target = input.parse()?;
    let n = tgt.y_min.abs() - 1;
    Ok(n * (n + 1) / 2)
}

#[aoc(day17, part2)]
fn part2(input: &str) -> Result<usize> {
    let tgt: Target = input.parse()?;
    let mut cnt = 0;
    let y_range = tgt.y_min.abs().max(tgt.y_min.abs());
    for y in -y_range..=y_range {
        for x in 1..=tgt.x_max {
            if shoot(x, y, &tgt) {
                cnt += 1;
            }
        }
    }
    Ok(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
target area: x=20..30, y=-10..-5
"#
        .trim();
        assert_eq!(part1(input)?, 45);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
target area: x=20..30, y=-10..-5
"#
        .trim();
        assert_eq!(part2(input)?, 112);
        Ok(())
    }
}
