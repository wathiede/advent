use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<(usize, Vec<usize>, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let mut led = 0;
            let mut buttons = vec![];
            let jolts = vec![];
            for p in l.split(' ') {
                let p_b = p.as_bytes();
                match p_b[0] {
                    b'[' => {
                        for b in p_b[1..p_b.len() - 1].iter().rev() {
                            let bit = if *b == b'#' { 1 } else { 0 };
                            led = led << 1 | bit;
                        }
                    }
                    b'(' => {
                        let mut b = 0;
                        p[1..p.len() - 1].split(',').for_each(|v| {
                            let v: u8 = v
                                .parse()
                                .unwrap_or_else(|e| panic!("parse button {v}: {e}"));
                            b = b | (1 << v);
                        });
                        buttons.push(b);
                    }
                    b'{' => (), // TODO handle jolts,
                    c => panic!("unexpected part start {c}"),
                }
            }
            (led, buttons, jolts)
        })
        .collect()
}

fn format_buttons(buttons: &[&usize]) -> String {
    buttons
        .iter()
        .map(|b| format!("{b:b}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn find_match(led: usize, buttons: &[usize]) -> usize {
    //let bs = format_buttons(buttons); println!("b: {led:b} {bs}");
    if led == 0 {
        return 0;
    }
    for set in buttons.into_iter().powerset() {
        if set.is_empty() {
            continue;
        }
        //let bs = format_buttons(&set);
        //println!("Start ---- {bs}");
        let mut l = 0;
        for b in &set {
            l ^= *b;
            //println!("Toggling {b:b} to get {l:b} looking for {led:b}");
        }
        if l == led {
            return set.len();
        }
    }
    unreachable!("failed to find match")
}

#[aoc(day10, part1)]
fn part1(input: &[(usize, Vec<usize>, Vec<usize>)]) -> String {
    input
        .into_iter()
        .map(|(led, buttons, _)| find_match(*led, buttons))
        .sum::<usize>()
        .to_string()
}

#[aoc(day10, part2)]
fn part2(input: &[(usize, Vec<usize>, Vec<usize>)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "7");
        //assert_ne!(part1(&parse(&input_for(2025, 10))), "76");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "<RESULT>");
    }
    */
}

