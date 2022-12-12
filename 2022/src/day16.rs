use advent::prelude::*;
use aoc_runner_derive::aoc;

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    0
}

// #[aoc(day16, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"SOMETHING
"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 42);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
