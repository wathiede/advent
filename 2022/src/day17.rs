use advent::prelude::*;
use aoc_runner_derive::aoc;

// Pieces
//
// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##

#[derive(Copy, Clone)]
enum Piece {
    Dash,
    Plus,
    L,
    I,
    Square,
}

impl Piece {
    fn width(&self) -> usize {
        use Piece::*;
        match self {
            Dash => 4,
            Plus => 3,
            L => 3,
            I => 1,
            Square => 2,
        }
    }
    // Returns offsets from the lower left corner representing this piece
    fn bits(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        use Piece::*;
        match self {
            Dash => Box::new((0..4).map(|x| (x, 0))),
            Plus => Box::new((0..3).map(|y| (1, y)).chain([(0, 1), (2, 1)])),
            L => Box::new((0..2).map(|x| (x, 0)).chain((0..3).map(|y| (2, y)))),
            I => Box::new((0..4).map(|y| (0, y))),
            Square => Box::new((0..2).flat_map(|x| (0..2).map(move |y| (x, y)))),
        }
    }
}

struct Chamber {
    stack: Vec<u8>,
    pieces: Box<dyn Iterator<Item = Piece>>,
    cur_piece: Option<Piece>,
    // Lower left corner of the bounding box of the piece.
    pos: (usize, usize),
}

impl Default for Chamber {
    fn default() -> Self {
        Chamber {
            stack: Default::default(),
            pieces: Box::new(
                [Piece::Dash, Piece::Plus, Piece::L, Piece::I, Piece::Square]
                    .into_iter()
                    .cycle(),
            ),
            cur_piece: None,
            pos: (0, 0),
        }
    }
}

enum State {
    Rest,
    Move,
}

impl Chamber {
    fn step(&mut self, jet: char) -> State {
        let p = match self.cur_piece {
            Some(p) => {
                println!("Continuing @ {:?}\n{}", self.pos, self);
                p
            }
            None => {
                let p = self.pieces.next().unwrap();
                self.cur_piece = Some(p);
                let zeros = self.stack.iter().rev().take_while(|&v| *v == 0).count();
                println!("{zeros} empty layers");
                let delta = if zeros <= 3 { 3 - zeros + 1 } else { 0 };
                println!("Adding {delta} empty layers");
                self.stack.extend((0..delta).map(|_| 0));
                self.pos = (2, self.stack.len() - 1);
                println!("A new rock begins falling @ {:?}\n{}", self.pos, self);
                p
            }
        };
        // Blow
        match jet {
            '<' => {
                if self.pos.0 > 0 && !self.hit(self.pos.0 - 1, self.pos.1) {
                    println!("Jet of gas pushes rock left:");
                    self.pos = (self.pos.0 - 1, self.pos.1);
                } else {
                    println!("Jet of gas pushes rock left, but nothing happens:");
                }
            }
            '>' => {
                if (self.pos.0 + p.width()) < 7 && !self.hit(self.pos.0 + 1, self.pos.1) {
                    println!("Jet of gas pushes rock right:");
                    self.pos = (self.pos.0 + 1, self.pos.1);
                } else {
                    println!("Jet of gas pushes rock right, but nothing happens:");
                }
            }
            c => panic!("Unknown {c}"),
        }
        println!("{}", self);
        // If at bottom, or hit an object, rest.
        if self.pos.1 == 0 || self.hit(self.pos.0, self.pos.1 - 1) {
            println!("Rock falls 1 unit, causing it to come to rest:");
            // fill in stack with bits.
            let max_y = self.stack.len();
            for (x, y) in p
                .bits()
                .map(|(x, y)| (x + self.pos.0, y + self.pos.1))
                .filter(|(_x, y)| y < &max_y)
            {
                println!("setting ({x},{y})");
                self.stack[y] |= (1 << x);
            }
            // Reset the piece
            self.cur_piece = None;
            State::Rest
        } else {
            // Else fall
            println!("Rock falls 1 unit:");
            self.pos = (self.pos.0, self.pos.1 - 1);
            State::Move
        }
    }
    fn hit(&self, x: usize, y: usize) -> bool {
        if let Some(p) = self.cur_piece {
            p.bits()
                .map(|(x2, y2)| (x + x2, y + y2))
                .filter(|(_x, y)| y < &self.stack.len())
                .any(|(x, y)| (self.stack[y] & (1 << x)) != 0)
        } else {
            panic!("hit called with no current piece");
        }
    }
    fn tallest(&self) -> usize {
        self.stack.iter().rev().skip_while(|&&l| l == 0).count()
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Uncomment to run fast enough for an answer
        return Ok(());
        let piece = if let Some(p) = self.cur_piece {
            p.bits()
                .map(|(x, y)| (self.pos.0 + x, self.pos.1 + y))
                .collect()
        } else {
            HashSet::new()
        };
        for (y, layer) in self.stack.iter().enumerate().rev() {
            write!(f, "|")?;
            for b in (0..7) {
                if piece.contains(&(b as usize, y)) {
                    write!(f, "@")?;
                } else if layer & (1 << b) == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    let mut ch = Chamber::default();
    let mut rocks = 0;
    let mut steps = 0;
    let mut last_rest = 0;
    let mut jets = input.chars().cycle();
    // TODO remove upper bound
    for steps in 0.. {
        //println!("Step {steps} Rocks {rocks} Last Rest {last_rest}");
        match ch.step(jets.next().unwrap()) {
            State::Rest => {
                rocks += 1;
                last_rest = 0;
                if rocks == 2022 {
                    break;
                }
            }
            State::Move => {
                last_rest += 1;
                if last_rest > ch.stack.len() {
                    panic!(
                        "Too many steps, {last_rest} since last rest in stack of {}",
                        ch.stack.len()
                    );
                }
            }
        }
    }
    // 3073 too low
    ch.tallest()
}

// #[aoc(day17, part2)]
// fn part2(input: &str) -> usize { }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 3068);
    }

    //#[test]
    //fn p2() {
    //    assert_eq!(part2(INPUT), 42);
    //}
}
