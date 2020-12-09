//! --- Day 8: Handheld Halting ---
//! Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//!
//! Their handheld game console won't turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//!
//! The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
//!
//! acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
//! jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
//! nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
//! For example, consider the following program:
//!
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! These instructions are visited in this order:
//!
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
//!
//! This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
//!
//! Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
//!
//! Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?

use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}
impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');
        Ok(match it.next() {
            Some("nop") => Instruction::Nop(it.next().ok_or(())?.parse().map_err(|_| ())?),
            Some("acc") => Instruction::Acc(it.next().ok_or(())?.parse().map_err(|_| ())?),
            Some("jmp") => Instruction::Jmp(it.next().ok_or(())?.parse().map_err(|_| ())?),
            Some(c) => panic!(format!("unknown instruction '{}'", c)),
            None => panic!(format!("no space in '{}'", s)),
        })
    }
}

#[derive(Default, Debug, PartialEq)]
struct Program {
    ip: usize,
    acc: i32,
    intrs: Vec<Instruction>,
    executed: Vec<bool>,
}

impl Program {
    fn run(&mut self) -> i32 {
        loop {
            if self.executed[self.ip] {
                return self.acc;
            }
            self.executed[self.ip] = true;
            match self.intrs[self.ip] {
                Instruction::Nop(_) => self.ip += 1,
                Instruction::Acc(op) => {
                    self.acc += op;
                    self.ip += 1;
                }
                Instruction::Jmp(op) => self.ip = (self.ip as i32 + op) as usize,
            }
        }
    }
}

impl FromStr for Program {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let intrs: Vec<_> = s.split('\n').filter_map(|i| i.parse().ok()).collect();
        let executed = vec![false; intrs.len()];
        Ok(Program {
            ip: 0,
            acc: 0,
            intrs,
            executed,
        })
    }
}

#[aoc(day8, part1)]
fn solution1(input: &str) -> i32 {
    let mut p: Program = input.parse().expect("Failed to parse Program");
    p.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn make() {
        assert_eq!(
            INPUT1.parse::<Program>().expect("Failed to parse input"),
            Program {
                ip: 0,
                acc: 0,
                intrs: vec![
                    Instruction::Nop(0),
                    Instruction::Acc(1),
                    Instruction::Jmp(4),
                    Instruction::Acc(3),
                    Instruction::Jmp(-3),
                    Instruction::Acc(-99),
                    Instruction::Acc(1),
                    Instruction::Jmp(-4),
                    Instruction::Acc(6),
                ],
                executed: vec![false; 9],
            }
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solution1(&INPUT1), 5);
    }
}
