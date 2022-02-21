use aoc2020::read_file;

use nom::{bits::bits, bits::complete::take, IResult};

type BitResult<'a, T> = IResult<BitInput<'a>, T>;
type BitInput<'a> = (&'a [u8], usize);

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Debug, Clone)]
enum PacketData {
    Literal(i64),
    Operator { type_id: u8, packets: Vec<Packet> },
}

impl PacketData {
    fn eval(&self) -> i64 {
        match self {
            PacketData::Literal(x) => *x,
            PacketData::Operator { type_id, packets } => match type_id {
                t @ (0 | 1 | 2 | 3) => {
                    let nums = packets.iter().map(|x| x.eval());
                    match t {
                        0 => nums.sum(),
                        1 => nums.product(),
                        2 => nums.min().unwrap(),
                        3 => nums.max().unwrap(),
                        _ => unreachable!(),
                    }
                }
                t @ (5 | 6 | 7) => {
                    assert_eq!(packets.len(), 2);
                    let (x, y) = (packets[0].eval(), packets[1].eval());
                    let comp = match t {
                        5 => x > y,
                        6 => x < y,
                        7 => x == y,
                        _ => unreachable!(),
                    };
                    if comp {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("invalid packet type {}", type_id),
            },
        }
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        let mut sum = self.version as u32;
        if let PacketData::Operator { ref packets, .. } = self.data {
            sum += packets.iter().map(|x| x.version_sum()).sum::<u32>();
        }
        sum
    }

    fn eval(&self) -> i64 {
        self.data.eval()
    }
}

fn parse_packet(input: BitInput) -> BitResult<Packet> {
    let (input, version) = take(3usize)(input)?;
    let (input, type_id) = take(3usize)(input)?;
    let (input, data) = match type_id {
        4 => parse_lit(input)?,
        x => parse_operator(input, x)?,
    };
    Ok((input, Packet { version, data }))
}

fn parse_operator(input: BitInput, type_id: u8) -> BitResult<PacketData> {
    let (input, length_type): (_, u8) = take(1usize)(input)?;
    let mut packets = Vec::new();
    if length_type == 0 {
        let (mut input, sub_bit_len): (_, usize) = take(15usize)(input)?;
        let initial_bits_remaining = input.0.len() * 8 - input.1;
        while initial_bits_remaining - (input.0.len() * 8 - input.1) < sub_bit_len {
            let r = parse_packet(input)?;
            input = r.0;
            packets.push(r.1);
        }
        Ok((input, PacketData::Operator { packets, type_id }))
    } else {
        let (mut input, num_subs): (_, usize) = take(11usize)(input)?;
        for _ in 0..num_subs {
            let r = parse_packet(input)?;
            input = r.0;
            packets.push(r.1);
        }
        Ok((input, PacketData::Operator { packets, type_id }))
    }
}

fn parse_lit(mut input: BitInput) -> BitResult<PacketData> {
    let mut res = 0;
    loop {
        let (new_input, cur): (_, i64) = take(5usize)(input)?;
        input = new_input;
        res = res << 4 | (cur & 0xf);
        if cur & 0x10 == 0 {
            return Ok((input, PacketData::Literal(res)));
        }
    }
}

fn main() {
    let input = hex::decode(read_file("inputs/2021/p16.txt")).unwrap();
    let res: IResult<&[u8], Packet> = bits(parse_packet)(&input);
    let packet = res.unwrap().1;
    let p1 = packet.version_sum();
    let p2 = packet.eval();
    dbg!(p1, p2);
}
