use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

fn max_p(bytes: &[u8]) -> (u8, isize) {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (*b, -1 * i as isize))
        .max()
        .expect("failed to find max digit")
}

#[aoc(day3, part1)]
fn part1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let l = l.as_bytes();
            let (first, idx) = max_p(&l[..l.len() - 1]);
            let (second, _) = max_p(&l[(-idx as usize) + 1..]);
            ((first - b'0') * 10 + second - b'0') as usize
        })
        .sum::<usize>()
        .to_string()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "357");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}
