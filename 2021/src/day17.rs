use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Target, Infallible> {
        let parts: Vec<_> = input.split(' ').collect();
        let x = &parts[2][2..].strip_suffix(',').unwrap();
        let y = &parts[3][2..];
        let (x_min, x_max) = x
            .split_once("..")
            .and_then(|(min, max)| Some((min.parse().unwrap(), max.parse().unwrap())))
            .unwrap();
        let (y_min, y_max) = y
            .split_once("..")
            .and_then(|(min, max)| Some((min.parse().unwrap(), max.parse().unwrap())))
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
fn part1(input: &str) -> Result<usize> {
    let tgt: Target = input.parse()?;
    dbg!(&tgt);
    Ok((0..(tgt.y_min).abs() as usize).sum())
}

/*
#[aoc(day17, part2)]
fn part2(input: &str) -> Result<usize> {
    todo!("part2");
    Ok(0)
}
*/

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
