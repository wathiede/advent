use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayX)]
fn parse(input: &str) -> Result<Vec<u32>> {
    todo!("parse");
    Ok(Vec::new())
}

#[aoc(dayX, part1)]
fn part1(depths: &[u32]) -> Result<u32> {
    todo!("part1");
    Ok(())
}

/*
#[aoc(dayX, part2)]
fn part2(depths: &[u32]) -> Result<u32> {
    todo!("part2")
    Ok(())
}
*/

#[test]
fn test_part1() -> Result<()> {
    let input = r#"
"#
    .trim();
    assert_eq!(part1(&parse(input)?)?, TODO);
    Ok(())
}

/*
#[test]
fn test_part2()->Result<()> {
    let input = r#"
"#
    .trim();
    assert_eq!(part2(&parse(input)?)?, TODO);
Ok(())
}
*/
