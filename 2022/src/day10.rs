use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let mut it = input.lines();
    let mut x = 1;
    let mut sum = 0;
    let mut add: Option<i32> = None;
    for cycle in 1i32.. {
        //if cycle > 180 { println!("{sum} {cycle} {x} {add:?}"); }
        if (cycle - 20) % 40 == 0 {
            //dbg!((sum, cycle, x, x * cycle));
            sum += cycle * x;
            if cycle == 220 {
                break;
            }
        }
        if let Some(add) = add.take() {
            x += add;
            continue;
        }

        let s = it.next();
        //if cycle > 180 { println!("{s:?}"); }
        match s {
            Some(s) => match s {
                "noop" => (),
                addx => {
                    let v = addx[addx.find(" ").expect("no space") + 1..]
                        .parse()
                        .expect("not a number");
                    add = Some(v);
                }
            },
            None => break,
        }
    }
    sum
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    let mut it = input.lines();
    let mut x = 1;
    let mut output = Vec::with_capacity(4096);
    let mut add: Option<i32> = None;
    for cycle in 1i32.. {
        let beam = (cycle - 1) % 40;
        if (beam - x).abs() <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }
        if let Some(add) = add.take() {
            x += add;
            continue;
        }

        match it.next() {
            Some(s) => match s {
                "noop" => (),
                addx => {
                    let v = addx[addx.find(" ").expect("no space") + 1..]
                        .parse()
                        .expect("not a number");
                    add = Some(v);
                }
            },
            None => break,
        }
    }
    output.truncate(output.len() - 1);
    format!(
        "\n{}",
        output
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 13140);
    }

    #[test]
    fn p2() {
        let want = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;
        assert_eq!(part2(INPUT), want);
    }
    const INPUT: &'static str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
}
