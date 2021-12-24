use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Instruction {
    on: bool,
    x_rng: RangeInclusive<i64>,
    y_rng: RangeInclusive<i64>,
    z_rng: RangeInclusive<i64>,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Instruction, Infallible> {
        // on x=11..13,y=11..13,z=11..13
        // off x=9..11,y=9..11,z=9..11
        let (verb, rest) = input.split_once(' ').unwrap();
        let on = match verb {
            "on" => true,
            "off" => false,
            _ => unreachable!("unexpected instruction type"),
        };

        let parts: Vec<_> = rest.split(',').collect();
        let parse_rng = |s: &str| -> RangeInclusive<i64> {
            s.split_once('=')
                .unwrap()
                .1
                .split_once("..")
                .map(|(lo, hi)| (lo.parse().unwrap(), hi.parse().unwrap()))
                .map(|(lo, hi)| lo..=hi)
                .unwrap()
        };
        let x_rng = parse_rng(parts[0]);
        let y_rng = parse_rng(parts[1]);
        let z_rng = parse_rng(parts[2]);
        Ok(Instruction {
            on,
            x_rng,
            y_rng,
            z_rng,
        })
    }
}

fn part1_apply(insts: Vec<Instruction>) -> usize {
    let mut grid = HashSet::new();
    for inst in &insts {
        dbg!(&inst);
        for x in inst.x_rng.clone() {
            for y in inst.y_rng.clone() {
                for z in inst.z_rng.clone() {
                    if inst.on {
                        grid.insert((x, y, z));
                    } else {
                        grid.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    grid.len()
}

fn inbounds(r: &RangeInclusive<i64>) -> bool {
    // lazy but good enough for part1
    r.start().abs() <= 50
}

#[aoc(day22, part1)]
fn part1(input: &str) -> Result<usize> {
    let insts: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse().expect("failed to parse instruction"))
        .filter(|i: &Instruction| inbounds(&i.x_rng) && inbounds(&i.y_rng) && inbounds(&i.z_rng))
        .collect();
    dbg!(&insts);
    Ok(part1_apply(insts))
}

/*
#[aoc(day22, part2)]
fn part2(input: &str) -> Result<usize> {
todo!("part2");
Ok(0)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
        on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
"#
        .trim();
        assert_eq!(part1(input)?, 39);

        let input = r#"
        on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
"#
        .trim();
        assert_eq!(part1(input)?, 590784);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
    let input = r#"
    "#
    .trim();
    assert_eq!(part2(input)?, usize::MAX);
    Ok(())
    }
    */
}
