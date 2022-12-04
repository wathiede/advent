use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|l| {
            let (e1, e2) = l.split_once(',').unwrap();
            let (e1l, e1h) = e1
                .split_once('-')
                .map(|(l, h)| {
                    (
                        l.parse::<usize>().expect("not a number"),
                        h.parse::<usize>().expect("not a number"),
                    )
                })
                .unwrap();
            let (e2l, e2h) = e2
                .split_once('-')
                .map(|(l, h)| {
                    (
                        l.parse::<usize>().expect("not a number"),
                        h.parse::<usize>().expect("not a number"),
                    )
                })
                .unwrap();
            (e1l <= e2l && e1h >= e2h) || (e2l <= e1l && e2h >= e1h)
        })
        .count()
    // Not 18
}

// #[aoc(day4, part2)]
// fn part2(input: &str) -> usize { }

#[test]
fn test1() {
    assert_eq!(
        part1(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
        ),
        2
    );
}
