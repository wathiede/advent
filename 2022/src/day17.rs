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

#[derive(Copy, Clone, PartialEq)]
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
    // encodes each row as a nibble
    fn bits(&self) -> u32 {
        use Piece::*;
        match self {
            Dash => 0b1111,
            Plus => 0b0010_0111_0010,
            L => 0b0100_0100_0111,
            I => 0b0001_0001_0001_0001,
            Square => 0b0011_0011,
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
    Rest(Piece),
    Move,
}

impl Chamber {
    fn step(&mut self, jet: char) -> State {
        let p = match self.cur_piece {
            Some(p) => {
                vprint!("Continuing @ {:?}\n{}", self.pos, self);
                p
            }
            None => {
                let p = self.pieces.next().unwrap();
                self.cur_piece = Some(p);
                let zeros = self.stack.iter().rev().take_while(|&v| *v == 0).count();
                vprint!("{zeros} empty layers");
                let delta = if zeros <= 3 { 3 - zeros + 1 } else { 0 };
                vprint!("Adding {delta} empty layers");
                self.stack.extend((0..delta).map(|_| 0));
                self.pos = (2, self.stack.len() - 1);
                vprint!("A new rock begins falling @ {:?}\n{}", self.pos, self);
                p
            }
        };
        // Blow
        match jet {
            '<' => {
                if self.pos.0 > 0 && !self.hit(self.pos.0 - 1, self.pos.1) {
                    vprint!("Jet of gas pushes rock left:");
                    self.pos = (self.pos.0 - 1, self.pos.1);
                } else {
                    vprint!("Jet of gas pushes rock left, but nothing happens:");
                }
            }
            '>' => {
                if (self.pos.0 + p.width()) < 7 && !self.hit(self.pos.0 + 1, self.pos.1) {
                    vprint!("Jet of gas pushes rock right:");
                    self.pos = (self.pos.0 + 1, self.pos.1);
                } else {
                    vprint!("Jet of gas pushes rock right, but nothing happens:");
                }
            }
            c => panic!("Unknown {c}"),
        }
        vprint!("{}", self);
        // If at bottom, or hit an object, rest.
        if self.pos.1 == 0 || self.hit(self.pos.0, self.pos.1 - 1) {
            vprint!("Rock falls 1 unit, causing it to come to rest:");
            // fill in stack with bits.
            let max_y = self.stack.len();
            let pat = p.bits();
            let y0: u8 = ((pat >> 0) & 0b1111).try_into().unwrap();
            let y1: u8 = ((pat >> 4) & 0b1111).try_into().unwrap();
            let y2: u8 = ((pat >> 8) & 0b1111).try_into().unwrap();
            let y3: u8 = ((pat >> 12) & 0b1111).try_into().unwrap();

            self.stack[self.pos.1 + 0] |= y0 << self.pos.0;
            self.stack[self.pos.1 + 1] |= y1 << self.pos.0;
            self.stack[self.pos.1 + 2] |= y2 << self.pos.0;
            self.stack[self.pos.1 + 3] |= y3 << self.pos.0;
            // Reset the piece
            self.cur_piece = None;
            State::Rest(p)
        } else {
            // Else fall
            vprint!("Rock falls 1 unit:");
            self.pos = (self.pos.0, self.pos.1 - 1);
            State::Move
        }
    }
    fn hit(&self, x: usize, y: usize) -> bool {
        let x: u8 = x.try_into().expect("x");
        if let Some(p) = self.cur_piece {
            let pat = p.bits();
            let y0: u8 = ((pat >> 0) & 0b1111).try_into().unwrap();
            let y1: u8 = ((pat >> 4) & 0b1111).try_into().unwrap();
            let y2: u8 = ((pat >> 8) & 0b1111).try_into().unwrap();
            let y3: u8 = ((pat >> 12) & 0b1111).try_into().unwrap();
            let s = &self.stack[y..];
            match s.len() {
                0 => panic!("empty stack"),
                1 => s[0] & (y0 << x) != 0,
                2 => (s[0] & (y0 << x) | s[1] & (y1 << x)) != 0,
                3 => (s[0] & (y0 << x) | s[1] & (y1 << x) | s[2] & (y2 << x)) != 0,
                _ => {
                    (s[0] & (y0 << x) | s[1] & (y1 << x) | s[2] & (y2 << x) | s[3] & (y3 << x)) != 0
                }
            }
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
        //return Ok(());
        let (y0, y1, y2, y3) = if let Some(p) = self.cur_piece {
            let pat = p.bits();
            let y0: u8 = ((pat >> 0) & 0b1111).try_into().unwrap();
            let y1: u8 = ((pat >> 4) & 0b1111).try_into().unwrap();
            let y2: u8 = ((pat >> 8) & 0b1111).try_into().unwrap();
            let y3: u8 = ((pat >> 12) & 0b1111).try_into().unwrap();
            (y0, y1, y2, y3)
        } else {
            (0, 0, 0, 0)
        };

        println!("pos.1 {}", self.pos.1);
        println!("y0 {y0:08b}");
        println!("y1 {y1:08b}");
        println!("y2 {y2:08b}");
        println!("y3 {y3:08b}");
        let y0 = y0 << self.pos.0;
        let y1 = y1 << self.pos.0;
        let y2 = y2 << self.pos.0;
        let y3 = y3 << self.pos.0;

        for (y, layer) in self.stack.iter().enumerate().rev() {
            write!(f, "|")?;
            for b in 0..7 {
                if y == self.pos.1 && (y0 & (1 << b)) != 0 {
                    write!(f, "@")?;
                } else if y == self.pos.1 + 1 && (y1 & (1 << b)) != 0 {
                    write!(f, "@")?;
                } else if y == self.pos.1 + 2 && (y2 & (1 << b)) != 0 {
                    write!(f, "@")?;
                } else if y == self.pos.1 + 3 && (y3 & (1 << b)) != 0 {
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
        vprint!("Step {steps} Rocks {rocks} Last Rest {last_rest}");
        match ch.step(jets.next().unwrap()) {
            State::Rest(_last_piece) => {
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

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    let mut ch = Chamber::default();
    let mut rocks = 0usize;
    let mut steps = 0;
    let mut last_rest = 0;
    let input_len = input.len();
    dbg!(&input_len);
    let mut jets = input.chars().cycle();
    // TODO remove upper bound
    for steps in 0.. {
        vprint!("Step {steps} Rocks {rocks} Last Rest {last_rest}");
        match ch.step(jets.next().unwrap()) {
            State::Rest(last_piece) => {
                assert!(
                    !(((steps % input_len) == 0) && last_piece == Piece::Square),
                    "last piece was a square when input restarted at {steps}"
                );
                rocks += 1;
                last_rest = 0;
                if rocks % 100000000 == 0 {
                    println!("Rocks {rocks}");
                }
                if rocks == 1000000000000 {
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
    ch.tallest()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
