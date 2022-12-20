use advent::prelude::*;
use aoc_runner_derive::aoc;

#[macro_use]
use advent::vprint;
const VERBOSE: bool = false;

#[aoc(day20, part1)]
fn part1(input: &str) -> isize {
    let mut ring: VecDeque<isize> = input.lines().map(|l| l.parse().expect("number")).collect();
    vprint!("{ring:?}");
    let orig_ring = ring.clone();
    for o in orig_ring {
        let src: isize = ring
            .iter()
            .position(|v: &isize| *v == o)
            .unwrap()
            .try_into()
            .unwrap();

        ring.rotate_left(src as usize);
        vprint!("after rotate1 {ring:?}");
        let v = ring.pop_front().unwrap();
        vprint!("after pop {ring:?}");
        let shft = v.rem_euclid(ring.len().try_into().unwrap());
        ring.rotate_left(shft.try_into().unwrap());
        vprint!("after rotate2 {ring:?}");
        ring.push_front(v);
        vprint!("{o} shl {src} shr {shft}: {v}");

        vprint!("**********");
        vprint!("{ring:?}");
    }

    let off = ring.iter().position(|&v| v == 0).unwrap();
    let v = ring[(off + 1000) % ring.len()]
        + ring[(off + 2000) % ring.len()]
        + ring[(off + 3000) % ring.len()];
    assert_ne!(-9792, v);
    assert_ne!(-9344, v);
    assert_ne!(2832, v);
    //assert_ne!(3, v);
    v
}

// #[aoc(day20, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"1
2
-3
3
-2
0
4
"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 3);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
