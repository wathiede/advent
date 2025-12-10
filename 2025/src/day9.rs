use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').expect("missing ,");
            (
                x.parse().expect("failed to parse x"),
                y.parse().expect("failed to parse y"),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(isize, isize)]) -> String {
    part1_n2(input)
}

// Attempt to solve with brute force
fn part1_n2(input: &[(isize, isize)]) -> String {
    input
        .iter()
        .flat_map(|(x1, y1)| {
            input
                .iter()
                .map(move |(x2, y2)| (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs())
        })
        .max()
        .expect("couldn't find max")
        .to_string()
}

// Attempt to solve with min/max corners
fn part1_min_max(input: &[(isize, isize)]) -> String {
    let min_x = input.iter().min().expect("couldn't find min x");
    let max_x = input.iter().max().expect("couldn't find max x");
    let min_y = input
        .iter()
        .map(|(x, y)| (y, x))
        .min()
        .expect("couldn't find min y");
    let max_y = input
        .iter()
        .map(|(x, y)| (y, x))
        .max()
        .expect("couldn't find max y");
    dbg!(input, min_x, max_x, min_y, max_y);

    (((max_x.0 - min_x.0 + 1).abs() * (max_x.1 - min_x.1 + 1).abs())
        .max((min_y.0 - max_y.0 + 1).abs() * (min_y.1 - max_y.1 + 1).abs()))
    .to_string()
}

fn draw_line(im: &mut Image<char>, p1: (isize, isize), p2: (isize, isize)) {
    let x_min = p1.0.min(p2.0) as usize;
    let x_max = 1 + p1.0.max(p2.0) as usize;
    let y_min = p1.1.min(p2.1) as usize;
    let y_max = 1 + p1.1.max(p2.1) as usize;
    println!("draw_line {x_min}..{x_max} {y_min}..{y_max}");
    for y in y_min..y_max {
        for x in x_min..x_max {
            if im[(x, y)] != '#' {
                im[(x, y)] = 'X';
            }
        }
    }
}

fn draw_points(pts: &[(isize, isize)]) {
    let (max_x, max_y) = pts.iter().fold((0isize, 0isize), |acc, (x, y)| {
        (acc.0.max(*x), acc.1.max(*y))
    });
    println!("image {max_x}x{max_y}");
    let mut im = Image::new(2 + max_x as usize, 2 + max_y as usize, '.');
    im[(pts[0].0 as usize, pts[0].1 as usize)] = '#';
    println!("{im:#}");
    for idx in 1..pts.len() + 1 {
        draw_line(&mut im, pts[idx - 1], pts[idx % pts.len()]);
        let pt = pts[idx % pts.len()];
        im[(pt.0 as usize, pt.1 as usize)] = '#';
        println!("{im:#}");
    }
}

#[aoc(day9, part2)]
fn part2(input: &[(isize, isize)]) -> String {
    draw_points(input);
    // Walk points finding the nearest X neighbor and completing the rectangle
    //
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "50");
        assert_eq!(part1(&parse(&input_for(2025, 9))), "4754955192");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "24");
    }
}
