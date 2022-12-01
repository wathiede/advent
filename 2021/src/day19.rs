use advent::prelude::*;
use aoc_runner_derive::aoc;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Vec3([i64; 3]);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "<{:4},{:4},{:4}>", self.0[0], self.0[1], self.0[2])
    }
}

impl FromStr for Vec3 {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Vec3, Infallible> {
        let v: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Vec3(v.try_into().unwrap()))
    }
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    offset: Option<Vec3>,
    points: Vec<Vec3>,
}

impl Scanner {
    fn translate(&mut self, distance: Vec3, orientation: [usize; 3], signs: [i64; 3]) {
        for p in &mut self.points {
            *p = Vec3([
                signs[0] * p.0[orientation[0]] + distance.0[0],
                signs[1] * p.0[orientation[1]] + distance.0[1],
                signs[2] * p.0[orientation[2]] + distance.0[2],
            ]);
        }
    }
}

impl FromStr for Scanner {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Scanner, Infallible> {
        let mut it = input.lines();
        let id = it
            .next()
            .unwrap()
            .split(' ')
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        Ok(Scanner {
            id,
            offset: None,
            points: it.map(|l| l.parse().unwrap()).collect(),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Match {
    abs_points: Vec<Vec3>,
    distance: Vec3,
    orientation: [usize; 3],
    signs: [i64; 3],
}

// Returns overlap, and  in s1 space
fn find_overlap(s1: &Scanner, s2: &Scanner) -> Option<Match> {
    let mut counts: HashMap<(Vec3, [usize; 3], [i64; 3]), Vec<Vec3>> = HashMap::new();
    let orientations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let signs = [
        [-1, -1, -1],
        [1, -1, -1],
        [-1, 1, -1],
        [1, 1, -1],
        [-1, -1, 1],
        [1, -1, 1],
        [-1, 1, 1],
    ];
    for v1 in &s1.points {
        for v2 in &s2.points {
            for or in orientations {
                for sign in signs {
                    let [x, y, z] = sign;
                    let v = Vec3([x * v2.0[or[0]], y * v2.0[or[1]], z * v2.0[or[2]]]);
                    let diff = *v1 - v;
                    counts.entry((diff, or, sign)).or_default().push(*v1);
                }
            }
        }
    }
    if let Some(((distance, orientation, signs), list)) =
        counts.into_iter().find(|(_k, v)| v.len() >= 12)
    {
        // s1's points should already be in absolute coords.  s2 will be translated in
        // part1().
        return Some(Match {
            abs_points: list,
            distance,
            orientation,
            signs,
        });
    }
    None
}

fn parse(input: &str) -> Result<Vec<Scanner>> {
    input.split("\n\n").map(|s| Ok(s.parse()?)).collect()
}
#[aoc(day19, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut scanner = parse(input)?;
    // Assign the first scanner to the origin (0,0,0).
    // Put that in a list of recently registered scanners.
    // In a loop
    // - For each recently registered scanner, attempt to find overlap with each unregistered
    // scanner.
    // - Matches should be translated according to the offsets found during the match. This should
    // put them in absolute space.
    // - Each match should be added to the recently registered list for the next iteration.
    // - Do this until all scanners are registered.
    scanner[0].offset = Some(Vec3::default());
    let (mut registered, mut unregistered): (VecDeque<_>, VecDeque<_>) =
        scanner.into_iter().partition(|s| s.offset.is_some());

    let mut becons = HashSet::new();
    let mut done = Vec::new();
    while let Some(reg) = registered.pop_front() {
        let mut unregs = VecDeque::new();
        for mut unreg in unregistered {
            if let Some(mat) = find_overlap(&reg, &unreg) {
                unreg.offset = Some(mat.distance);
                unreg.translate(mat.distance, mat.orientation, mat.signs);
                println!(
                    "scanner {} @ {:?} found {} hits",
                    &unreg.id,
                    &unreg.offset.unwrap(),
                    mat.abs_points.len()
                );
                registered.push_back(unreg);
                for pt in mat.abs_points {
                    becons.insert(pt);
                }
            } else {
                unregs.push_back(unreg);
            }
        }
        done.push(reg);
        unregistered = unregs;
    }

    println!("before pass 2: {}", becons.len());
    for i in 0..registered.len() {
        for j in i..registered.len() {
            let s1 = &registered[i];
            let s2 = &registered[j];
            if let Some(mat) = find_overlap(s1, s2) {
                for pt in mat.abs_points {
                    becons.insert(pt);
                }
            }
        }
    }
    println!("after pass 2: {}", becons.len());

    //assert_eq!(done.len(), 12);
    let mut becons: Vec<_> = becons.iter().collect();
    becons.sort();
    dbg!(&becons);
    Ok(becons.len())
}

/*
#[aoc(day19, part2)]
fn part2(input: &str) -> Result<usize> {
todo!("part2");
Ok(0)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() -> Result<()> {
        use pretty_assertions::assert_eq;

        let input = r#"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390
"#
        .trim();
        let mut abs_points: Vec<Vec3> = r#"
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347
"#
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
        abs_points.sort();
        let orientation = [0, 1, 2];
        let signs = [-1, 1, -1];
        let distance = Vec3([68, -1246, -43]);
        let want = Match {
            distance,
            abs_points,
            orientation,
            signs,
        };

        let scanners = parse(input)?;
        let mut got = find_overlap(&scanners[0], &scanners[1]).unwrap();
        got.abs_points.sort();
        assert_eq!(want, got);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#
        .trim();
        assert_eq!(part1(input)?, 79);
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
