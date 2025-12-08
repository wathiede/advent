use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};

const INPUT: &'static str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<Vec3>, usize) {
    let top_n = if input == INPUT { 10 } else { 1000 };
    (
        input
            .lines()
            .map(|l| l.parse().expect("failed to parse vec3"))
            .collect(),
        top_n,
    )
}

fn nearest(v1: &Vec3, vecs: &[Vec3]) -> (i64, Vec3) {
    vecs.iter()
        .map(|v| (v.distance_squared(v1), v))
        .inspect(|(d, v)| {
            println!("v1:{v1:?} v:{v:?} d:{d}");
        })
        .min()
        .map(|(d, v)| (d, *v))
        .expect("couldn't find min")
}

#[aoc(day8, part1)]
fn part1((input, top_n): &(Vec<Vec3>, usize)) -> String {
    //println!("top_n {top_n}");
    let mut all_pairs = HashSet::new();
    for v1 in input {
        for v2 in input {
            if v1 == v2 {
                continue;
            }
            let (v1, v2) = if v1 > v2 { (v2, v1) } else { (v1, v2) };
            all_pairs.insert((v1, v2, v1.distance_squared(&v2)));
        }
    }
    let mut all_pairs: Vec<(Vec3, Vec3, i64)> = all_pairs
        .into_iter()
        .map(|(v1, v2, d)| (*v1, *v2, d))
        .collect();
    // Sort by distance
    all_pairs.sort_by_key(|(_, _, key)| *key);
    //for (v1, v2, d) in &all_pairs[..*top_n] { println!("v1:{v1:?} v2:{v2:?} d:{d}"); }

    let mut sets: Vec<HashSet<_>> = vec![];
    let mut it = all_pairs.iter();
    let mut connections = HashSet::new();
    while connections.len() < *top_n {
        let (v1, v2, _) = it.next().expect("ran out of pairs");
        //println!("v1:{v1:?} v2:{v2:?} d:{d}");
        // Can be 0, 1 or 2
        let sets_with_junction: Vec<_> = sets
            .iter()
            .enumerate()
            .filter(|(_i, s)| s.contains(&v1) || s.contains(&v2))
            .map(|(i, _s)| i)
            .collect();
        match sets_with_junction.len() {
            // Create new cluster
            0 => {
                connections.insert((v1, v2));

                //println!("NEW cluster {v1:?} - {v2:?}");
                let mut s = HashSet::new();
                s.insert(v1);
                s.insert(v2);
                sets.push(s);
            }
            // Add endpoint to existing cluster
            1 => {
                connections.insert((v1, v2));
                let s = &mut sets[sets_with_junction[0]];
                if s.contains(&v1) && s.contains(&v2) {
                    //println!("DUPE: {v1:?} - {v2:?}");
                    // Both already connected
                    continue;
                }
                if s.contains(v1) {
                    //println!("Adding new junction v2 {v2:?} to {s:?} | v1 {v1:?}");
                    s.insert(v2);
                }
                if s.contains(v2) {
                    //println!("Adding new junction v1 {v1:?} to {s:?} | v2 {v2:?}");
                    s.insert(v1);
                }
            }

            // Merge one cluster into the other
            2 => {
                connections.insert((v1, v2));
                let tmp: Vec<_> = sets[sets_with_junction[1]].drain().collect();
                let s1 = &mut sets[sets_with_junction[0]];
                s1.extend(tmp);
                if s1.contains(&v1) && s1.contains(&v2) {
                    //println!("DUPE: {v1:?} - {v2:?}");
                    // Both already connected
                    continue;
                }
                if s1.contains(v1) {
                    //println!("Adding new junction v2 {v2:?} to {s1:?} | v1 {v1:?}");
                    s1.insert(v2);
                }
                if s1.contains(v2) {
                    //println!("Adding new junction v1 {v1:?} to {s1:?} | v2 {v2:?}");
                    s1.insert(v1);
                }
            }

            c => panic!("Didn't expect {c} sets to have juntions"),
        }
    }
    sets.sort_by_key(|s| -(s.len() as isize));
    //println!("sets {sets:#?}");
    let sets = &sets[..3];
    //println!("sets top 3 {sets:#?}");
    sets.iter().map(|s| s.len()).product::<usize>().to_string()
}

#[aoc(day8, part2)]
fn part2((input, _top_n): &(Vec<Vec3>, usize)) -> String {
    //println!("top_n {top_n}");
    let mut all_pairs = HashSet::new();
    for v1 in input {
        for v2 in input {
            if v1 == v2 {
                continue;
            }
            let (v1, v2) = if v1 > v2 { (v2, v1) } else { (v1, v2) };
            all_pairs.insert((v1, v2, v1.distance_squared(&v2)));
        }
    }
    let mut all_pairs: Vec<(Vec3, Vec3, i64)> = all_pairs
        .into_iter()
        .map(|(v1, v2, d)| (*v1, *v2, d))
        .collect();
    // Sort by distance
    all_pairs.sort_by_key(|(_, _, key)| *key);
    //for (v1, v2, d) in &all_pairs[..*top_n] { println!("v1:{v1:?} v2:{v2:?} d:{d}"); }

    let mut sets: Vec<HashSet<_>> = vec![];
    let mut last_pair = (&all_pairs[0].0, &all_pairs[0].1);
    for (v1, v2, _) in &all_pairs {
        //println!("v1:{v1:?} v2:{v2:?} d:{d}");
        // Can be 0, 1 or 2
        let sets_with_junction: Vec<_> = sets
            .iter()
            .enumerate()
            .filter(|(_i, s)| s.contains(&v1) || s.contains(&v2))
            .map(|(i, _s)| i)
            .collect();
        match sets_with_junction.len() {
            // Create new cluster
            0 => {
                last_pair = (v1, v2);

                //println!("NEW cluster {v1:?} - {v2:?}");
                let mut s = HashSet::new();
                s.insert(v1);
                s.insert(v2);
                sets.push(s);
            }
            // Add endpoint to existing cluster
            1 => {
                let s = &mut sets[sets_with_junction[0]];
                if s.contains(&v1) && s.contains(&v2) {
                    //println!("DUPE: {v1:?} - {v2:?}");
                    // Both already connected
                    continue;
                }
                last_pair = (v1, v2);
                if s.contains(v1) {
                    //println!("Adding new junction v2 {v2:?} to {s:?} | v1 {v1:?}");
                    s.insert(v2);
                }
                if s.contains(v2) {
                    //println!("Adding new junction v1 {v1:?} to {s:?} | v2 {v2:?}");
                    s.insert(v1);
                }
            }

            // Merge one cluster into the other
            2 => {
                let tmp: Vec<_> = sets[sets_with_junction[1]].drain().collect();
                let s1 = &mut sets[sets_with_junction[0]];
                s1.extend(tmp);
                if s1.contains(&v1) && s1.contains(&v2) {
                    //println!("DUPE: {v1:?} - {v2:?}");
                    // Both already connected
                    continue;
                }
                if s1.contains(v1) {
                    //println!("Adding new junction v2 {v2:?} to {s1:?} | v1 {v1:?}");
                    s1.insert(v2);
                }
                if s1.contains(v2) {
                    //println!("Adding new junction v1 {v1:?} to {s1:?} | v2 {v2:?}");
                    s1.insert(v1);
                }
            }

            c => panic!("Didn't expect {c} sets to have juntions"),
        }
    }
    (last_pair.0.x() * last_pair.1.x()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "40");
        assert_eq!(part1(&parse(&input_for(2025, 8))), "47040");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "25272");
        assert_eq!(part2(&parse(&input_for(2025, 8))), "4884971896");
    }
}

