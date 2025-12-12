use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[derive(Debug)]
struct Command {
    region: Image<char>,
    qtys: Vec<u8>,
}

#[derive(Debug)]
struct Present {
    // Store image in all orientations
    images: Vec<Image<char>>,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> (Vec<Present>, Vec<Command>) {
    let mut presents = Vec::new();
    let mut commands = Vec::new();
    for chunk in input.split("\n\n") {
        let l = chunk.as_bytes();
        if l[1] == b':' {
            // present image
            let im: Image<char> = chunk
                .split_once(":\n")
                .map(|(_, data)| data.parse().expect("failed to parse image"))
                .expect("failed to split region header");
            let p1 = im;
            let p2 = p1.rot90();
            let p3 = p2.rot90();
            let p4 = p3.rot90();
            let orients: HashSet<_> = [p1, p2, p3, p4].into_iter().collect();
            let mut images: Vec<_> = orients.into_iter().collect();
            // Sort so things are consistent run to run
            images.sort();
            presents.push(Present { images });
        } else {
            for l in chunk.lines() {
                // command
                let (res, qtys) = l.split_once(": ").expect("failed to split command");
                let (w, h) = res
                    .split_once('x')
                    .map(|(w, h)| {
                        (
                            w.parse().expect("failed to parse width"),
                            h.parse().expect("failed to parse width"),
                        )
                    })
                    .expect("failed to split res on x");
                let region = Image::new(w, h, '.');
                let qtys = qtys
                    .split(' ')
                    .map(|q| q.parse().expect("failed to parse qty"))
                    .collect();

                commands.push(Command { region, qtys });
            }
        }
    }
    (presents, commands)
}

fn will_it_fit(presents: &[Present], c: &Command) -> bool {
    //println!("c.qtys {:?}", c.qtys);
    let mut floor = c.region.clone();
    for (i, q) in c.qtys.iter().enumerate() {
        if *q == 0 {
            continue;
        }
        let p = &presents[i];
        /*
        println!("Processing {}x{} i:{i}x{q}", floor.width, floor.height);
        for o in &p.images {
            println!("{o:#}");
        }
        */
        for n in 0..*q {
            let mut found_fit = false;
            for (x, y) in iproduct!(0..floor.width - 2, 0..floor.height - 2) {
                //println!( "checking [{},{}] p{i},{n}/{q} @ {x},{y}", floor.width, floor.height);

                if p.images.iter().any(|p| {
                    /*
                    println!("Checking @ {x},{y}:{:#}", &p);
                    println!("{floor:#}");
                    */
                    if floor.can_blit((x, y), &p, '#') {
                        floor.blit((x, y), &p, '#');
                        true
                    } else {
                        false
                    }
                }) {
                    /*
                    println!("found fit");
                    println!("{floor:#}");
                    */
                    found_fit = true;
                    break;
                };
            }
            if !found_fit {
                /*
                println!("FAILED TO FIND FIT");
                println!("[{},{}] p{i},{n}/{q}", floor.width, floor.height);
                println!("Present\n{:#}", &p.images[0]);
                println!("{floor:#}");
                */
                return false;
            }
        }
    }
    //println!("SUCCESS");
    true
}

#[aoc(day12, part1)]
fn part1((presents, commands): &(Vec<Present>, Vec<Command>)) -> String {
    //dbg!(presents, commands);
    commands
        .iter()
        .filter(|c| will_it_fit(presents, c))
        .count()
        .to_string()
}

#[aoc(day12, part2)]
fn part2((presents, commands): &(Vec<Present>, Vec<Command>)) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), "2");
    }

    /*
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
    */
}

