use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Copy, Clone, Debug)]
struct Player {
    tally: usize,
    score: usize,
}

impl Player {
    fn space(&self) -> usize {
        (self.tally % 10) + 1
    }
}

#[derive(Debug, Default)]
struct Die {
    roll_count: usize,
}

impl Die {
    fn roll(&mut self) -> usize {
        let val = (self.roll_count % 100) + 1;
        self.roll_count += 1;
        val
    }
}

fn take_turn(p: &mut Player, die: &mut Die) -> bool {
    p.tally += die.roll() + die.roll() + die.roll();
    p.score += p.space();

    if p.score >= 1000 {
        return true;
    }
    false
}

#[aoc(day21, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut p: Vec<_> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(_, space)| space.parse().expect("couldn't parse starting spaceition"))
        .map(|space: usize| Player {
            tally: space - 1,
            score: 0,
        })
        .collect();
    let mut die = Die::default();
    loop {
        if take_turn(&mut p[0], &mut die) {
            return Ok(die.roll_count * p[1].score);
        }
        //println!( "Player 1 space {} for a total score of {}.", p[0].space(), p[0].score);

        if take_turn(&mut p[1], &mut die) {
            return Ok(die.roll_count * p[0].score);
        }
        //println!( "Player 2 space {} for a total score of {}.", p[1].space(), p[1].score);
    }
}

fn play_part2(p1: Player, p2: Player) -> (usize, usize) {
    fn play_part2_rec(
        mut p1: Player,
        mut p2: Player,
        r1: usize,
        r2: usize,
        r3: usize,
        r4: usize,
        r5: usize,
        r6: usize,
    ) -> (usize, usize) {
        //println!( "p1 {} {} p2 {} {} die {} {} {} {} {} {}", p1.score, p1.space(), p2.score, p2.space(), r1, r2, r3, r4, r5, r6,);
        p1.tally += r1 + r2 + r3;
        p1.score += p1.space();
        if p1.score >= 21 {
            return (1, 0);
        }

        p2.tally += r4 + r5 + r6;
        p2.score += p2.space();
        if p2.score >= 21 {
            return (0, 1);
        }

        let mut p1_score = 0;
        let mut p2_score = 0;

        for i in [1, 2, 3] {
            for j in [1, 2, 3] {
                for k in [1, 2, 3] {
                    for x in [1, 2, 3] {
                        for y in [1, 2, 3] {
                            for z in [1, 2, 3] {
                                let (p1s, p2s) =
                                    play_part2_rec(p1.clone(), p2.clone(), i, j, k, x, y, z);
                                p1_score += p1s;
                                p2_score += p2s;
                            }
                        }
                    }
                }
            }
        }
        (p1_score, p2_score)
    }

    let mut p1_score = 0;
    let mut p2_score = 0;

    for i in [1, 2, 3] {
        for j in [1, 2, 3] {
            for k in [1, 2, 3] {
                for x in [1, 2, 3] {
                    for y in [1, 2, 3] {
                        for z in [1, 2, 3] {
                            let (p1s, p2s) =
                                play_part2_rec(p1.clone(), p2.clone(), i, j, k, x, y, z);
                            p1_score += p1s;
                            p2_score += p2s;
                            println!("Running score {} vs {}", p1_score, p2_score);
                        }
                    }
                }
            }
        }
    }
    (p1_score, p2_score)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> Result<usize> {
    let mut p: Vec<_> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(_, space)| space.parse().expect("couldn't parse starting spaceition"))
        .map(|space: usize| Player {
            tally: space - 1,
            score: 0,
        })
        .collect();
    let (p1_wins, p2_wins) = play_part2(p[0], p[1]);
    Ok(if p1_wins > p2_wins { p1_wins } else { p2_wins })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
Player 1 starting position: 4
Player 2 starting position: 8
"#
        .trim();
        assert_eq!(part1(input)?, 739785);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
Player 1 starting position: 4
Player 2 starting position: 8
"#
        .trim();
        assert_eq!(part2(input)?, 444356092776315);
        Ok(())
    }
}
