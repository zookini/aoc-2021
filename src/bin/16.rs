use bitvec::prelude::*;
use itertools::Itertools;

fn main() {
    let packet = Packet::parse(&mut Parser::new(include_str!("../../input/16.txt")));

    println!("Part 1: {}", p1(&packet));
    println!("Part 2: {}", p2(&packet));
}

fn p1(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(version, _) => *version as usize,
        Packet::Op(version, _, packets) => *version as usize + packets.iter().map(p1).sum::<usize>(),
    }
}

fn p2(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(_, n) => *n,
        Packet::Op(_, type_id, packets) => match (type_id, packets.iter().map(|p| p2(p))) {
            (0, ps) => ps.sum(),
            (1, ps) => ps.product(),
            (2, ps) => ps.min().unwrap(),
            (3, ps) => ps.max().unwrap(),
            (5, mut ps) => (ps.next().unwrap() > ps.next().unwrap()) as usize,
            (6, mut ps) => (ps.next().unwrap() < ps.next().unwrap()) as usize,
            (7, mut ps) => (ps.next().unwrap() == ps.next().unwrap()) as usize,
        _ => unreachable!()
        }
    }
}

struct Parser { position: usize, bits: BitVec<Msb0, u8> }

impl Parser {
    fn new(hex: &str) -> Self {
        Self {
            position: 0,
            bits: (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
        }
    }

    fn read(&mut self, len: usize) -> u16 {
        self.position += len;
        self.bits[self.position - len..self.position].load_be()
    }
}

enum Packet { Literal(u8, usize), Op(u8, u8, Vec<Packet>) }

impl Packet {
    fn parse(parser: &mut Parser) -> Self {
        let version = parser.read(3) as u8;
        let type_id = parser.read(3) as u8;

        if type_id == 4 {
            Packet::Literal(version, std::iter::repeat_with(|| parser.read(5)).fold_while(0, |acc, n| if n >> 4 == 1 {
                itertools::FoldWhile::Continue((acc << 4) + (n as usize & 0xf))
            } else {
                itertools::FoldWhile::Done((acc << 4) + n as usize)
            }).into_inner())
        } else {
            Packet::Op(version, type_id, if parser.read(1) == 0 {
                let end = parser.read(15) as usize + parser.position;
                std::iter::from_fn(|| (parser.position != end).then(|| Self::parse(parser))).collect()
            } else {
                (0..parser.read(11) as usize).map(|_| Self::parse(parser)).collect()
            })
        }
    }
}
