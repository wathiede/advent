use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Image {
    input.parse().expect("failed to parse")
}

#[aoc(day7, part1)]
fn part1(im: &Image) -> String {
    let mut rays = vec![false; im.width];
    let mut splits = 0;
    for x in 0..im.width {
        if im[(x, 0)] == b'S' {
            rays[x] = true;
            break;
        }
    }
    for y in 1..im.height {
        for x in 0..im.width {
            if rays[x] {
                if im[(x, y)] == b'^' {
                    // Split
                    rays[x] = false;
                    rays[x - 1] = true;
                    rays[x + 1] = true;
                    splits += 1;
                }
            }
        }
    }

    splits.to_string()
}

#[aoc(day7, part2)]
fn part2(im: &Image) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "21");
        // 1555
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}

