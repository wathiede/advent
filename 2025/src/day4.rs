use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Image {
    input.parse().expect("couldn't parse image")
}

#[aoc(day4, part1)]
fn part1(im: &Image) -> String {
    let mut rolls = 0;
    for y in 0..im.height {
        for x in 0..im.width {
            //println!("***** found roll *****");
            if im[(x, y)] == b'@' {
                let mut cnt = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        /*
                        println!(
                            "{:} {:?}",
                            im[(x, y)] as char,
                            im.get(x as isize + i, y as isize + j).map(|b| b as char)
                        );
                        */
                        if im.get(x as isize + i, y as isize + j) == Some(b'@') {
                            cnt += 1;
                        }
                    }
                }
                if cnt < 4 {
                    rolls += 1;
                }
            }
        }
    }
    rolls.to_string()
}

#[aoc(day4, part2)]
fn part2(input: &Image) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "13");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}
