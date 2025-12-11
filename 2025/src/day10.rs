use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<(usize, Vec<Vec<usize>>, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            //println!("parse line");
            let mut led = 0;
            let mut buttons = vec![];
            let mut jolts = vec![];
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
                        let mut b = vec![];
                        //println!("button parsing {p}");
                        p[1..p.len() - 1].split(',').for_each(|v| {
                            b.push(
                                v.parse()
                                    .unwrap_or_else(|e| panic!("parse button {v}: {e}")),
                            );
                        });
                        buttons.push(b);
                    }
                    b'{' => {
                        jolts = p[1..p.len() - 1]
                            .split(',')
                            .map(|s| s.parse().unwrap_or_else(|e| panic!("parse jolt {s}: {e}")))
                            .collect()
                    }
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
fn part1(input: &[(usize, Vec<Vec<usize>>, Vec<usize>)]) -> String {
    /*  TODO convert buttons to format find_match expects
    input
        .into_iter()
        .map(|(led, buttons, _)| find_match(*led, buttons))
        .sum::<usize>()
        .to_string()
    */
    0.to_string()
}

fn buttons_bits_to_idx(buttons: &[usize]) -> Vec<Vec<usize>> {
    //println!("buttons {buttons:x?}");
    let res = buttons
        .iter()
        .map(|b| {
            let n = (64 - b.leading_zeros()) as usize;
            let mut v = vec![];
            for i in 0..n {
                if 1 << i & b > 0 {
                    v.push(i)
                }
            }
            v
        })
        .collect();
    res
}

fn valid_buttons(buttons: &[Vec<usize>], jolts: &[usize]) -> Vec<Vec<usize>> {
    //println!("buttons: {buttons:?} jolts: {jolts:?}");
    buttons
        .iter()
        .filter(|b| b.iter().all(|idx| jolts[*idx] > 0))
        .cloned()
        .collect()
}
// Sort buttons by largest number of jolts toggled
// Attempt to subtract the largest button set as many times as would remain positive
// When any jolt goes negative, rollback and move to the next button set
//
// If this solution doesn't work, do some sort of dynamic programming things where all the
// combinations are tried combinatorially
//
// Use DP to solve
// 1. determine what buttons are feasible to press (pressing them one cause any jolts to go
//      negative
// 2. Recurse with a memo that checks if we've gone done this path before and returns flip count
// 3. On cache miss, for each valid button, subtract and branch
fn solve_jolt_presses<'a>(
    buttons: Vec<Vec<usize>>,
    jolts: &'a [usize],
    mut memo: &mut HashMap<(Vec<Vec<usize>>, Vec<usize>), usize>,
    depth: usize,
) -> usize {
    let depth_str = format!("{depth}{:depth$}", " ");
    // Find button presses that won't go negative on jolts
    let btns = valid_buttons(&buttons, jolts);

    // Done
    if jolts.iter().sum::<usize>() == 0 {
        memo.insert((btns.clone(), jolts.to_vec()), 0);
        //println!("{depth_str} FOUND A SOLUTION!");
        return 0;
    }

    //println!("Memo len {}: {memo:?}", memo.len());

    // No valid buttons to press, Failed to match, this branch should have infinity cost
    if btns.is_empty() {
        memo.insert((btns.clone(), jolts.to_vec()), usize::MAX);
        //println!("{depth_str} NO SOLUTION, ran out of valid buttons");
        return usize::MAX;
    }
    // If we already have a value for this branch return it
    if let Some(flips) = memo.get(&(btns.clone(), jolts.to_vec())) {
        //println!("{depth_str} 1. Memo hit!");
        return *flips;
    }

    //println!("{depth_str} buttons {btns:?} jolts {jolts:?}");
    let sub_jolts: Vec<usize> = jolts.iter().cloned().collect();

    let mut min_flips = usize::MAX;
    for (i, btn) in btns.iter().enumerate() {
        // Try each button recursively
        let mut sub_jolts = sub_jolts.clone();
        for b in btn {
            sub_jolts[*b] -= 1;
        }

        let flips = solve_jolt_presses(btns.clone(), &sub_jolts, &mut memo, depth + 1);
        //println!( "{depth_str} {i} of {} min_flips {min_flips} flips {flips}", btns.len());
        if flips < min_flips {
            min_flips = flips;
        }
        memo.insert((btns.clone(), jolts.to_vec()), min_flips.saturating_add(1));
    }

    //println!("{depth_str} RETURNING {min_flips}");
    // Return the lowest route we've found
    memo[&(btns.clone(), jolts.to_vec())]
}

#[aoc(day10, part2)]
fn part2(input: &[(usize, Vec<Vec<usize>>, Vec<usize>)]) -> String {
    input
        .into_iter()
        .map(|(_, buttons, jolts)| {
            let mut memo = HashMap::new();
            solve_jolt_presses(buttons.clone(), jolts, &mut memo, 0)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    /*
        #[test]
        fn part1_example() {
            assert_eq!(part1(&parse(INPUT)), "7");
            assert_eq!(part1(&parse(&input_for(2025, 10))), "491");
        }
    */

    #[test]
    fn part2_example() {
        let input = parse(&INPUT);
        /*
        let mut memo = HashMap::new();
        let buttons = buttons_bits_to_idx(&input[0].1);
        println!("TEST 1");
        assert_eq!(solve_jolt_presses(buttons, &input[0].2, &mut memo, 0), 10);
        let mut memo = HashMap::new();
        let buttons = buttons_bits_to_idx(&input[1].1);
        println!("TEST 2");
        assert_eq!(solve_jolt_presses(buttons, &input[1].2, &mut memo, 0), 12);
        */
        let mut memo = HashMap::new();
        let buttons = &input[2].1;
        println!("TEST 3");
        assert_eq!(
            solve_jolt_presses(buttons.clone(), &input[2].2, &mut memo, 0),
            11
        );
        assert_eq!(part2(&input), "33");
    }
}

