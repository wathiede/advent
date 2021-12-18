use advent::prelude::*;
use aoc_runner_derive::aoc;

fn hex(b: &u8) -> u8 {
    if *b >= b'A' {
        10 + b - b'A'
    } else {
        b - b'0'
    }
}

fn sum_version(packet: &Packet) -> u64 {
    fn sum_packets(packets: &[Packet]) -> u64 {
        packets.iter().map(sum_version).sum()
    }
    packet.version as u64
        + match &packet.packet_type {
            PacketType::Sum(packets) => sum_packets(packets),
            PacketType::Product(packets) => sum_packets(packets),
            PacketType::Minimum(packets) => sum_packets(packets),
            PacketType::Maximum(packets) => sum_packets(packets),
            PacketType::Literal(_) => 0,
            PacketType::GreaterThan(packets) => sum_packets(packets),
            PacketType::LessThan(packets) => sum_packets(packets),
            PacketType::Equal(packets) => sum_packets(packets),
        }
}

fn interpret(packet: &Packet) -> u64 {
    match &packet.packet_type {
        PacketType::Sum(packets) => packets.iter().map(interpret).sum(),
        PacketType::Product(packets) => packets.iter().map(interpret).product(),
        PacketType::Minimum(packets) => packets.iter().map(interpret).min().unwrap(),
        PacketType::Maximum(packets) => packets.iter().map(interpret).max().unwrap(),
        PacketType::Literal(v) => *v,
        PacketType::GreaterThan(packets) => {
            if interpret(&packets[0]) > interpret(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketType::LessThan(packets) => {
            if interpret(&packets[0]) < interpret(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketType::Equal(packets) => {
            if interpret(&packets[0]) == interpret(&packets[1]) {
                1
            } else {
                0
            }
        }
    }
}

#[derive(Debug)]
enum PacketType {
    // 0
    Sum(Vec<Packet>),
    // 1
    Product(Vec<Packet>),
    // 2
    Minimum(Vec<Packet>),
    // 3
    Maximum(Vec<Packet>),
    // 4
    Literal(u64),
    // 5
    GreaterThan(Vec<Packet>),
    // 6
    LessThan(Vec<Packet>),
    // 7
    Equal(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    bit_size: u64,
    packet_type: PacketType,
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
        //print!( "   BEGIN n {0} tmp 0b{1:b} len {2}    - ", n, self.tmp, self.tmp_len);
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
        self.tmp &= mask;

        //println!( "   END n {0} tmp 0b{2:b} len {3} v 0b{1:00$b} ", n, v, self.tmp, self.tmp_len);
        v as u64
    }
}

fn parse_packet(p: &mut Parser) -> Packet {
    let mut bit_size: u64 = 0;
    let version = p.read(3);
    bit_size += 3;
    let packet_type_id = p.read(3);
    bit_size += 3;
    let packet_type = if packet_type_id == 4 {
        // Literal, read 5 bits at a time until MSB is 0
        let mut v = 0;
        loop {
            let l = p.read(5);
            v = (v << 4) | (l & 0b1111);
            bit_size += 5;
            if 0b10000 & l == 0 {
                break;
            }
        }
        PacketType::Literal(v)
    } else {
        // length type ID
        let ltid = p.read(1);
        bit_size += 1;
        let mut packets = Vec::new();
        if ltid == 0 {
            // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
            let len = p.read(15);
            bit_size += 15;
            let mut sub_bits = 0;
            while sub_bits < len {
                let sub_p = parse_packet(p);
                bit_size += sub_p.bit_size;
                sub_bits += sub_p.bit_size;
                packets.push(sub_p);
            }
        } else {
            // If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
            let num = p.read(11);
            bit_size += 11;
            for _ in 0..num {
                let sub_p = parse_packet(p);
                bit_size += sub_p.bit_size;
                packets.push(sub_p);
            }
        }
        match packet_type_id {
            0 => PacketType::Sum(packets),
            1 => PacketType::Product(packets),
            2 => PacketType::Minimum(packets),
            3 => PacketType::Maximum(packets),
            5 => PacketType::GreaterThan(packets),
            6 => PacketType::LessThan(packets),
            7 => PacketType::Equal(packets),
            _ => panic!("unknown packet type ID {}", packet_type_id),
        }
    };
    Packet {
        version,
        bit_size,
        packet_type,
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> Result<u64> {
    let mut p = Parser::new(input);
    let packet = parse_packet(&mut p);
    Ok(sum_version(&packet))
}

#[aoc(day16, part2)]
fn part2(input: &str) -> Result<u64> {
    let mut p = Parser::new(input);
    let packet = parse_packet(&mut p);
    Ok(interpret(&packet))
}

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

    #[test]
    fn test_part2() -> Result<()> {
        let input = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (inp, want) in input {
            print!("\nTesting '{}'\n - ", inp);
            inp.as_bytes().iter().for_each(|c| print!("{:04b}", hex(c)));
            println!();
            assert_eq!(part2(inp)?, want);
            println!("Passed '{}'", inp);
        }
        Ok(())
    }
}
