use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> Image<u8> {
    input.parse().expect("couldn't parse image")
}

#[aoc(day4, part1)]
fn part1(im: &Image<u8>) -> String {
    let mut rolls = 0;
    for y in 0..im.height {
        for x in 0..im.width {
            //println!("***** found roll *****");
            if im[(x, y)] == b'@' {
                let mut cnt = 0;
                im.visit_neighbors(
                    (x as isize, y as isize),
                    |b| if b == b'@' { 1 } else { 0 },
                    |v| cnt += v,
                );

                if cnt < 4 {
                    rolls += 1;
                }
            }
        }
    }
    rolls.to_string()
}

#[aoc(day4, part2)]
fn part2(im: &Image<u8>) -> String {
    let mut im = im.clone();
    let mut rolls = 0;
    let mut done = false;
    while !done {
        let mut rm = Vec::new();

        for y in 0..im.height {
            for x in 0..im.width {
                //println!("***** found roll *****");
                if im[(x, y)] == b'@' {
                    let mut cnt = 0;
                    im.visit_neighbors(
                        (x as isize, y as isize),
                        |b| if b == b'@' { 1 } else { 0 },
                        |v| cnt += v,
                    );
                    if cnt < 4 {
                        rolls += 1;
                        done = false;
                        rm.push((x, y));
                    }
                }
            }
        }

        if rm.is_empty() {
            done = true;
        } else {
            for (x, y) in rm {
                im[(x, y)] = b'x';
            }
        }
    }
    rolls.to_string()
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
        assert_eq!(part1(&parse(&input_for(2025, 4))), "1578");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "43");
        assert_eq!(part2(&parse(&input_for(2025, 4))), "10132");
    }
}
