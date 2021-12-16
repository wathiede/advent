use std::{
    fmt::{Debug, Error, Formatter},
    io::Read,
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

fn hex(b: &u8) -> u8 {
    if *b >= b'A' {
        10 + b - b'A'
    } else {
        b - b'0'
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    tmp: u64,
    tmp_len: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &str) -> Parser {
        Parser {
            bytes: input.as_bytes(),
            tmp: 0,
            tmp_len: 0,
        }
    }
    fn read(&mut self, n: usize) -> u64 {
        assert!(n < 32, "can't read more than 32 bits at time");
        print!(
            "   BEGIN n {0} tmp 0b{1:b} len {2}    - ",
            n, self.tmp, self.tmp_len
        );
        while self.tmp_len < n {
            let mut buf = [0; 1];
            self.bytes.read_exact(&mut buf).expect("EOF");
            // Convert the byte from hexdecimal to binary and merge with any leftover bits.
            self.tmp = (self.tmp << 4) | hex(&buf[0]) as u64;
            self.tmp_len += 4;
        }

        let mask = (1 << n) - 1;
        self.tmp_len -= n;
        let v = (self.tmp >> self.tmp_len) & mask;

        let mask = (1 << self.tmp_len) - 1;
        self.tmp = self.tmp & mask;

        println!(
            "   END n {0} tmp 0b{2:b} len {3} v 0b{1:00$b} ",
            n, v, self.tmp, self.tmp_len
        );
        v as u64
    }
}
fn read_packet(p: &mut Parser) -> (Vec<u64>, u64) {
    let mut versions = Vec::new();
    let mut bits_processed: u64 = 0;
    let version = p.read(3);
    versions.push(version);
    bits_processed += 3;
    let typ = p.read(3);
    bits_processed += 3;
    println!("version {} type {}", version, typ);
    if typ == 4 {
        // Literal, read 5 bits at a time until MSB is 0
        println!("type 4 literal");
        loop {
            let l = p.read(5);
            bits_processed += 5;
            println!("literal 0b{:05b}", l);
            if 0b10000 & l == 0 {
                /*
                // Read trailing 0s
                let n = 4 - ((bits_processed) % 4) as usize;
                println!(
                    "bits processed {}, draining {} trailing 0s",
                    bits_processed, n
                );
                let _ = p.read(n);
                bits_processed += n as u64;
                */

                break;
            }
        }
    } else {
        // length type ID
        let ltid = p.read(1);
        bits_processed += 1;
        if ltid == 0 {
            // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
            let len = p.read(15);
            bits_processed += 15;
            println!("{} bits in subpacket", len);
            let mut sub_bits = 0;
            while sub_bits < len {
                let (vs, bp) = read_packet(p);
                versions.extend(vs.iter());
                bits_processed += bp;
                sub_bits += bp;
            }
        } else {
            // If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
            let num = p.read(11);
            bits_processed += 11;
            println!("{} subpackets", num);
            for _ in 0..num {
                let (vs, bp) = read_packet(p);
                versions.extend(vs.iter());
                bits_processed += bp;
            }
        }
    }
    return (versions, bits_processed);
}

#[aoc(day16, part1)]
fn part1(input: &str) -> Result<u64> {
    let mut p = Parser::new(input);
    let (versions, _) = read_packet(&mut p);
    Ok(versions.iter().sum())
}

/*
#[aoc(day16, part2)]
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
        let input = vec![
            ("D2FE28", 6),
            //("38006F45291200", 1 + 0 + 0),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        for (inp, want) in input {
            print!("\nTesting '{}'\n - ", inp);
            inp.as_bytes().iter().for_each(|c| print!("{:04b}", hex(c)));
            println!();
            assert_eq!(part1(inp)?, want);
            println!("Passed '{}'", inp);
        }
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
