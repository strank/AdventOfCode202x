//! https://adventofcode.com/2021/day/16
//! Decode a binary packet format, representing numeric expressions, given as hex
//! exercise in bit-level parsing: let's try using the nom package!
//! (binread looks interesting too, but does not seem to be made for bit-level)
//!
//! ```
//! use advent_of_code_202x::generated::year2021day16::run;
//! assert!(run().contains("Version sum of packets: 893\nEvaluates to: 4358595186090"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day16::run_example;
/// assert!(run_example().contains("
/// Version sum of packets: 16\nEvaluates to: 15\n
/// Version sum of packets: 12\nEvaluates to: 46\n
/// Version sum of packets: 23\nEvaluates to: 46\n
/// Version sum of packets: 31\nEvaluates to: 54"));
/// ```
const EXAMPLE_INPUT: &str = "
D2FE28
38006F45291200
EE00D40C823060
8A004A801A8002F478
620080001611562C8802118E34
C0015000016115A2E0802F182340
A0016C880162017C3686B18A3D4780
C200B40A82
04005AC33890
880086C3E88112
CE00C43D881120
D8005AC2A8F0
F600BC2D8F
9C005AC2F8F0
9C0141080250320F1802104A08
"; // see AOC page for details

/// we have to deal with an expression tree - a recursive struct, expressions containing packets:
#[derive(Debug)]
pub enum Expr {
    Literal(u64),
    Operator(Vec<Packet>),
}

/// and packets containing expressions:
#[derive(Debug)]
pub struct Packet {
    version: u8,
    type_id: u8, // 4 for literals, otherwise it's an operator
    expr: Expr,
}

impl Packet {
    /// recursively sum all version numbers
    fn version_sum(&self) -> u32 {
        match &self.expr {
            Expr::Literal(_) => self.version as u32,
            Expr::Operator(pack_vec) => pack_vec
                .iter()
                .fold(self.version as u32, |acc, ele| acc + ele.version_sum()),
        }
    }

    /// recursively evaluate the tree of expressions
    fn evaluate(&self) -> u64 {
        match &self.expr {
            Expr::Literal(value) => *value,
            Expr::Operator(pack_vec) => {
                let mut evaluated_iter = pack_vec.iter().map(|ele| ele.evaluate());
                match self.type_id {
                    0 => evaluated_iter.sum(),
                    1 => evaluated_iter.product(),
                    2 => evaluated_iter.min().unwrap(),
                    3 => evaluated_iter.max().unwrap(),
                    5 => (evaluated_iter.next().unwrap() > evaluated_iter.next().unwrap()) as u64,
                    6 => (evaluated_iter.next().unwrap() < evaluated_iter.next().unwrap()) as u64,
                    7 => (evaluated_iter.next().unwrap() == evaluated_iter.next().unwrap()) as u64,
                    _ => panic!("Unknown packet type id {}", self.type_id),
                }
            }
        }
    }
}

/// All parsing code in a separate module:
pub mod parse {
    use hex::FromHex;
    use nom::bits::{bits, complete::tag, complete::take};
    use nom::branch::alt;
    use nom::error::{Error, ErrorKind};
    use nom::multi::{length_count, many_till};
    use nom::sequence::{preceded, tuple};
    use nom::{Err, IResult, InputLength};
    use std::cmp::Ordering;

    use super::{Expr, Packet};

    /// nom's idiom for bits: a byte slice with an offset (0-7) into the first byte
    type Bits<'a> = (&'a [u8], usize);

    /// Return a vec of (n) bytes from a hexadecimal string (2n long)
    pub fn hex2bytes(input: &'static str) -> Vec<u8> {
        Vec::from_hex(input).expect("Invalid Hexadecimal String")
    }

    /// Parse a full packet given as bytes (also asserts there are no extra bytes left and panics on fail)
    pub fn bytes2packet(input: &[u8]) -> Packet {
        let result = bits::<_, _, Error<Bits>, Error<&[u8]>, _>(packet)(input);
        match result {
            Ok((unparsed, packet)) => {
                assert!(unparsed.is_empty());
                packet
            }
            Err(err) => {
                println!("Error {:?}", err);
                panic!("Parsing packet failed!")
            }
        }
    }

    /// Parse bit stream into a packet, i.e. a version number and either a literal or an operator
    fn packet(input: Bits) -> IResult<Bits, Packet> {
        let result = tuple((take(3usize), alt((literal, operator))))(input);
        let (unparsed, (version, (type_id, expr))) = result?;
        Ok((
            unparsed,
            Packet {
                version,
                type_id,
                expr,
            },
        ))
    }

    /// Parse bit stream that is tagged as a literal (unsigned) number
    fn literal(input: Bits) -> IResult<Bits, (u8, Expr)> {
        tuple((tag(4, 3usize), literal_value))(input)
    }

    /// Parse bit stream that holds the literal value, groups of 5 bits, the first bit being 0 on the last one,
    /// and calculate the resulting number as u64 (there is no defined max-length, so this could overflow!)
    fn literal_value(input: Bits) -> IResult<Bits, Expr> {
        let (unparsed, (parts_vec, final_part)) =
            many_till(literal_value_part(true), literal_value_part(false))(input)?;
        let parts_val = parts_vec
            .iter()
            .fold(0u64, |acc, ele| acc * 0b10000 + *ele as u64);
        Ok((
            unparsed,
            Expr::Literal(parts_val * 0b10000 + final_part as u64),
        ))
    }

    /// Create a parser for a part of the literal representation, 5 bits with the first being 1 or 0 (the `tag_value`)
    fn literal_value_part(to_be_cont: bool) -> impl Fn(Bits) -> IResult<Bits, u8> {
        move |input: Bits| preceded(tag(to_be_cont as u8, 1usize), take(4usize))(input)
    }

    /// Parse bit stream that is tagged as an operator with sub-packages
    fn operator(input: Bits) -> IResult<Bits, (u8, Expr)> {
        tuple((take(3usize), operands))(input)
    }

    /// Parse list of operands, either with length_value or length_count, based on the first bit
    fn operands(input: Bits) -> IResult<Bits, Expr> {
        let result = alt((
            preceded(tag(0u8, 1usize), length_value_packets),
            preceded(tag(1u8, 1usize), length_count_packets),
        ))(input)?;
        Ok((result.0, Expr::Operator(result.1)))
    }

    /// Parse list of operands where the first 15 bits encode the number of bits that hold packets
    fn length_value_packets(input: Bits) -> IResult<Bits, Vec<Packet>> {
        // the Bits representation doesn't and actually cannot implement the trait InputTake needed for using length_value
        // as you cannot split the bits-in-bytes-with-offset at a non-byte boundary,
        // also the consumed combinator doesn't work to keep track of length of parsed input (trait Offset not satisfied)
        // so we need to check length manually to avoid parsing too much:
        let (input, bit_length) = take::<_, usize, _, _>(15usize)(input)?;
        let target_bit_length = input.input_len() - bit_length;
        let (input, (packets_vec, _)) =
            many_till(packet, check_bit_length(target_bit_length))(input)?;
        Ok((input, packets_vec))
    }

    /// Parser that only matches if exactly `target_length` bits are left, if more are left Error to try again, if less are left return Failure
    fn check_bit_length(target_length: usize) -> impl Fn(Bits) -> IResult<Bits, ()> {
        move |input: Bits| match input.input_len().cmp(&target_length) {
            Ordering::Equal => Ok((input, ())),
            Ordering::Greater => Err(Err::Error(Error::new(input, ErrorKind::ManyTill))),
            Ordering::Less => Err(Err::Failure(Error::new(input, ErrorKind::ManyTill))),
        }
    }

    /// Parse list of operands where the first 11 bits encode the number of packets that follow
    fn length_count_packets(input: Bits) -> IResult<Bits, Vec<Packet>> {
        length_count(take::<_, u32, _, _>(11usize), packet)(input)
    }
}

pub fn process_input(input: &'static str) -> String {
    let byte_slice = parse::hex2bytes(input.trim());
    let test_result = parse::bytes2packet(&byte_slice[..]);
    format!(
        "Version sum of packets: {}\nEvaluates to: {}\n",
        test_result.version_sum(),
        test_result.evaluate()
    )
}

pub fn run_example() -> String {
    EXAMPLE_INPUT
        .trim()
        .split('\n')
        .map(process_input)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn run() -> String {
    process_input(INPUT)
}
