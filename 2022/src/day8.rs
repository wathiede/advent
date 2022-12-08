use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let m: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut trees: Vec<u128> = (0..m.len())
        .map(|i| {
            if i == 0 || i == m.len() - 1 {
                (1 << m.len()) - 1
            } else {
                1 | (1 << m.len() - 1)
            }
        })
        .collect();
    for y in 1..m.len() - 1 {
        for x in 1..m.len() - 1 {
            // From left
            let h = m[y][x];
            let viz = (0..x).all(|x1| m[y][x1]<h)
                // From right
                ||(x+1..m.len()).all(|x1| m[y][x1]<h)
                // From top
                ||(0..y).all(|y1| m[y1][x]<h)
                // From bottom
                ||(y+1..m.len()).all(|y1| m[y1][x]<h);

            if viz {
                trees[y] |= 1 << x
            }
        }
    }
    //let w = m.len();
    //trees.iter().for_each(|r| println!("{r:w$b}"));
    trees.iter().map(|r| r.count_ones() as usize).sum::<usize>()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let m: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();
    let mut scores: Vec<Vec<usize>> = m.iter().map(|r| vec![0; r.len()]).collect();
    for y in 0..m.len() {
        for x in 0..m.len() {
            let h = m[y][x];
            // Up
            let up = (0..y)
                .rev()
                .position(|y1| m[y1][x] >= h)
                .map(|v| v + 1)
                .unwrap_or(y);
            // Left
            let left = (0..x)
                .rev()
                .position(|x1| m[y][x1] >= h)
                .map(|v| v + 1)
                .unwrap_or(x);
            // Right
            let right = (x + 1..m.len())
                .position(|x1| m[y][x1] >= h)
                .map(|v| v + 1)
                .unwrap_or(m.len() - x - 1);
            // Down
            let down = (y + 1..m.len())
                .position(|y1| m[y1][x] >= h)
                .map(|v| v + 1)
                .unwrap_or(m.len() - y - 1);

            scores[y][x] = up * left * right * down;
        }
    }
    scores
        .into_iter()
        .map(|r| r.into_iter().max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &'static str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT1), 21);
    }
    const INPUT2: &'static str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT2), 8);
    }
}
