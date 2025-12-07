use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Image<u8> {
    input.parse().expect("failed to parse")
}

#[aoc(day7, part1)]
fn part1(im: &Image<u8>) -> String {
    part1_vec(im)
}

fn part1_vec(im: &Image<u8>) -> String {
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

fn part1_bitset(im: &Image<u8>) -> String {
    let mut rays = BitSet::new(im.width);
    let mut splits = 0;
    for x in 0..im.width {
        if im[(x, 0)] == b'S' {
            rays.set(x);
            break;
        }
    }
    for y in 1..im.height {
        for x in 0..im.width {
            if rays.is_set(x) {
                if im[(x, y)] == b'^' {
                    // Split
                    rays.clear(x);
                    rays.set(x - 1);
                    rays.set(x + 1);
                    splits += 1;
                }
            }
        }
    }

    splits.to_string()
}

fn explore(im: &Image<u8>, y: usize, mut rays: &mut [bool], mut memo: &mut Image<usize>) -> usize {
    //dbg!(y, rays_as_bits(rays), &timelines);
    if y >= im.height {
        return 1;
    }
    let mut timelines = 0;
    for x in 0..im.width {
        if rays[x] {
            if memo[(x, y)] == 0 {
                if im[(x, y)] == b'^' {
                    // Split
                    rays[x] = false;
                    rays[x - 1] = true;
                    memo[(x, y)] = explore(im, y + 1, &mut rays, &mut memo);
                    rays[x - 1] = false;
                    rays[x + 1] = true;
                    memo[(x, y)] += explore(im, y + 1, &mut rays, &mut memo);
                    rays[x + 1] = false;
                } else {
                    memo[(x, y)] = explore(im, y + 1, &mut rays, &mut memo);
                }
            }
            timelines += memo[(x, y)];
        }
    }
    timelines
}

#[aoc(day7, part2)]
fn part2(im: &Image<u8>) -> String {
    let mut rays = vec![false; im.width];
    for x in 0..im.width {
        if im[(x, 0)] == b'S' {
            rays[x] = true;
            break;
        }
    }
    let mut memo = Image::new(im.width, im.height, 0usize);
    explore(im, 1, &mut rays, &mut memo).to_string()
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
        assert_eq!(part1(&parse(&input_for(2025, 7))), "1555");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "40");
        assert_eq!(part2(&parse(&input_for(2025, 7))), "12895232295789");
    }
}

