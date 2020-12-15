//! --- Day 13: Shuttle Search ---
//! Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.
//!
//! Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.
//!
//! Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.
//!
//! The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!
//!
//! Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.
//!
//! To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)
//!
//! For example, suppose you have the following notes:
//!
//! 939
//! 7,13,x,x,59,x,31,19
//! Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:
//!
//! time   bus 7   bus 13  bus 59  bus 31  bus 19
//! 929      .       .       .       .       .
//! 930      .       .       .       D       .
//! 931      D       .       .       .       D
//! 932      .       .       .       .       .
//! 933      .       .       .       .       .
//! 934      .       .       .       .       .
//! 935      .       .       .       .       .
//! 936      .       D       .       .       .
//! 937      .       .       .       .       .
//! 938      D       .       .       .       .
//! 939      .       .       .       .       .
//! 940      .       .       .       .       .
//! 941      .       .       .       .       .
//! 942      .       .       .       .       .
//! 943      .       .       .       .       .
//! 944      .       .       D       .       .
//! 945      D       .       .       .       .
//! 946      .       .       .       .       .
//! 947      .       .       .       .       .
//! 948      .       .       .       .       .
//! 949      .       D       .       .       .
//! The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.
//!
//! What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?
//!
//! --- Part Two ---
//! The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)
//!
//! For example, suppose you have the same list of bus IDs as above:
//!
//! 7,13,x,x,59,x,31,19
//! An x in the schedule means there are no constraints on what bus IDs must depart at that time.
//!
//! This means you are looking for the earliest timestamp (called t) such that:
//!
//! Bus ID 7 departs at timestamp t.
//! Bus ID 13 departs one minute after timestamp t.
//! There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//! Bus ID 59 departs four minutes after timestamp t.
//! There are no requirements or restrictions on departures at five minutes after timestamp t.
//! Bus ID 31 departs six minutes after timestamp t.
//! Bus ID 19 departs seven minutes after timestamp t.
//! The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.
//!
//! In this example, the earliest timestamp at which this occurs is 1068781:
//!
//! time     bus 7   bus 13  bus 59  bus 31  bus 19
//! 1068773    .       .       .       .       .
//! 1068774    D       .       .       .       .
//! 1068775    .       .       .       .       .
//! 1068776    .       .       .       .       .
//! 1068777    .       .       .       .       .
//! 1068778    .       .       .       .       .
//! 1068779    .       .       .       .       .
//! 1068780    .       .       .       .       .
//! 1068781    D       .       .       .       .
//! 1068782    .       D       .       .       .
//! 1068783    .       .       .       .       .
//! 1068784    .       .       .       .       .
//! 1068785    .       .       D       .       .
//! 1068786    .       .       .       .       .
//! 1068787    .       .       .       D       .
//! 1068788    D       .       .       .       D
//! 1068789    .       .       .       .       .
//! 1068790    .       .       .       .       .
//! 1068791    .       .       .       .       .
//! 1068792    .       .       .       .       .
//! 1068793    .       .       .       .       .
//! 1068794    .       .       .       .       .
//! 1068795    D       D       .       .       .
//! 1068796    .       .       .       .       .
//! 1068797    .       .       .       .       .
//! In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.
//!
//! Here are some other examples:
//!
//! The earliest timestamp that matches the list 17,x,13,19 is 3417.
//! 67,7,59,61 first occurs at timestamp 754018.
//! 67,x,7,59,61 first occurs at timestamp 779210.
//! 67,7,x,59,61 first occurs at timestamp 1261476.
//! 1789,37,47,1889 first occurs at timestamp 1202161486.
//! However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!
//!
//! What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, PartialEq)]
struct Schedule {
    time: u32,
    buses: Vec<u32>,
}

#[aoc_generator(day13, part1)]
fn parse1(input: &str) -> Schedule {
    let mut it = input.split('\n');
    let time = it
        .next()
        .expect("Premature EOF")
        .parse()
        .expect("Can't parse time");
    let buses = it
        .next()
        .expect("Premature EOF")
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    Schedule { time, buses }
}

#[aoc(day13, part1)]
fn solution1(sch: &Schedule) -> u32 {
    let (bus, next) = sch
        .buses
        .iter()
        // Find the next bus time after sch.time.
        .map(|b| (b, b * ((sch.time / b) + 1)))
        // Find the earliest next bus time.
        .min_by(|i1, i2| i1.1.cmp(&i2.1))
        .unwrap();
    bus * (next - sch.time)
}

#[derive(Copy, Clone, Debug)]
struct Departure {
    bus: usize,
    delay: usize,
}

#[aoc_generator(day13, part2)]
fn parse2(input: &str) -> Vec<Departure> {
    let mut it = input.split('\n');
    let _ = it.next().expect("Premature EOF");
    it.next()
        .expect("Premature EOF")
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| Some((i, s.parse::<usize>().ok()?)))
        .map(|(delay, bus)| Departure { bus, delay })
        .collect()
}

fn inv_mod(a: usize, m: usize) -> usize {
    {
        let a = a % m;
        for i in 1..m {
            if (a * i) % m == 1 {
                return i;
            }
        }
    }
    panic!(format!("no inverse modulo found for {}^-1 % {}", a, m));
}

/// Based on http://homepages.math.uic.edu/~leon/mcs425-s08/handouts/chinese_remainder.pdf
/// a_m is a Vec with (a, m) as used in the above PDF. m are pairwise relatively prime positive
/// integers and a are any integers.
fn chinese_remainder(a_m: Vec<(usize, usize)>) -> usize {
    let a: Vec<_> = a_m.iter().map(|(a, _m)| a).collect();
    let m: Vec<_> = a_m.iter().map(|(_a, m)| m).collect();
    let m_all = m.iter().fold(1, |acc, m| *m * acc);
    let z: Vec<_> = m.iter().map(|m| m_all / *m).collect();
    let y: Vec<_> = m
        .iter()
        .zip(z.iter())
        .map(|(m, z)| inv_mod(*z, **m))
        .collect();
    let w: Vec<_> = y
        .iter()
        .zip(z.iter())
        .map(|(y, z)| (*y * *z) % m_all)
        .collect();

    let x = a
        .iter()
        .zip(w.iter())
        .fold(0, |acc, (a, w)| acc + (*a * *w));
    x % m_all
}

#[aoc(day13, part2)]
fn solution2(sch: &[Departure]) -> usize {
    let a_m: Vec<(_, _)> = sch.iter().map(|d| (d.bus - d.delay, d.bus)).collect();
    chinese_remainder(a_m)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn parsing1() {
        assert_eq!(
            parse1(INPUT),
            Schedule {
                time: 939,
                buses: vec![7, 13, 59, 31, 19],
            }
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&parse1(INPUT)), 295);
    }
    #[test]
    fn part2() {
        for (input, want) in vec![
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("7,13,x,x,59,x,31,19", 1068781),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ] {
            // Insert fake header '123\n' to make the parse2 function happy.
            assert_eq!(solution2(&parse2(&format!("123\n{}", input))), want);
        }
    }

    #[test]
    fn inverse_modulo() {
        assert_eq!(inv_mod(8400, 11), 8);
        assert_eq!(inv_mod(7, 11), 8);
        assert_eq!(inv_mod(5775, 16), 15);
        assert_eq!(inv_mod(15, 16), 15);
        assert_eq!(inv_mod(4400, 21), 2);
        assert_eq!(inv_mod(11, 21), 2);
        assert_eq!(inv_mod(3696, 25), 6);
        assert_eq!(inv_mod(21, 25), 6);
        assert_eq!(inv_mod(243257, 11), 4);
        assert_eq!(inv_mod(3, 11), 4);
        assert_eq!(inv_mod(243257, 13), 1);
    }

    #[test]
    fn chinese_remainder_theorem() {
        assert_eq!(chinese_remainder(vec![(2, 5), (3, 7)]), 17);
        assert_eq!(chinese_remainder(vec![(1, 3), (4, 5), (6, 7)]), 34);
        assert_eq!(chinese_remainder(vec![(3, 5), (2, 6), (4, 7)]), 158);
        assert_eq!(
            chinese_remainder(vec![(1, 5), (2, 7), (3, 9), (4, 11)]),
            1731
        );
        // http://homepages.math.uic.edu/~leon/mcs425-s08/handouts/chinese_remainder.pdf
        // says this answer is 51669 which doesn't check out.
        assert_eq!(
            chinese_remainder(vec![(6, 11), (13, 16), (9, 21), (19, 25),]),
            89469
        );
    }
}
