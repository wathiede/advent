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

fn rays_as_bits(rays: &[bool]) -> String {
    rays.iter()
        .map(|r| if *r { "|" } else { "." })
        .collect::<Vec<_>>()
        .join("")
}
// TODO: bitset
// TODO: remove cloned
fn explore(im: &Image, y: usize, mut rays: &mut [bool], mut timelines: &mut usize) {
    //dbg!(y, rays_as_bits(rays), &timelines);
    if y >= im.height {
        return;
    }

    for x in 0..im.width {
        if rays[x] {
            if im[(x, y)] == b'^' {
                // Split
                rays[x] = false;
                rays[x - 1] = true;
                *timelines += 1;
                explore(im, y + 1, &mut rays, &mut timelines);
                rays[x - 1] = false;
                rays[x + 1] = true;
                explore(im, y + 1, &mut rays, &mut timelines);
                rays[x + 1] = false;
            } else {
                explore(im, y + 1, &mut rays, &mut timelines);
            }
        }
    }
}

#[aoc(day7, part2)]
fn part2(im: &Image) -> String {
    let mut rays = vec![false; im.width];
    for x in 0..im.width {
        if im[(x, 0)] == b'S' {
            rays[x] = true;
            break;
        }
    }
    let y_start = 1;
    let mut timelines = 1;
    explore(im, y_start, &mut rays, &mut timelines);
    timelines.to_string()
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "40");
    }
}

