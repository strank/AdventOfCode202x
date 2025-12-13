//! https://adventofcode.com/2025/day/01
//! Find password which is the count of hitting zero on a rotary dial
//!
//! ```
//! use advent_of_code_202x::generated::year2025day01::run;
//! assert!(run().contains("Password: 1177\nMethod 0x43: 6768"));
//! ```

use std::ops::{Add, Neg, Sub};

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2025day01::run_example;
/// assert!(run_example().contains("Password: 3\nMethod 0x43: 6"));
/// ```
const EXAMPLE_INPUT: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"; // 3

/// Need a u8 variant that wraps around modulo 100
#[derive(Debug, Copy, Clone, PartialEq)]
struct U8Mod<const N: u8> {
    value: u8,
}

impl<const N: u8> U8Mod<N> {
    fn new(value: i32) -> Self {
        Self {
            value: value.rem_euclid(N as i32) as u8,
        }
    }
}

impl<const N: u8> Add for U8Mod<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value).rem_euclid(N),
        }
    }
}

impl<const N: u8> Neg for U8Mod<N> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: (N - self.value).rem_euclid(N),
        }
    }
}

impl<const N: u8> Sub for U8Mod<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<const N: u8> U8Mod<N> {
    fn new_with_zero_trans_count(value: i32) -> (Self, u32) {
        (Self::new(value), (value / N as i32) as u32)
    }

    fn add_zero_trans(self, other: Self) -> (Self, u32) {
        (
            self + other,
            if self.value + other.value >= N { 1 } else { 0 },
        )
    }

    fn sub_zero_trans(self, other: Self) -> (Self, u32) {
        (
            self - other,
            if self.value != 0 && (self.value as i16) - (other.value as i16) <= 0 {
                1
            } else {
                0
            },
        )
    }
}

type DialPos = U8Mod<100>;

fn str_to_i32(a_str: &str) -> i32 {
    a_str.parse::<i32>().unwrap()
}

fn rotate_dial(dial_pos: DialPos, direction: char, amount: i32) -> DialPos {
    let amount = DialPos::new(amount);
    if direction == 'R' {
        dial_pos + amount
    } else {
        dial_pos - amount
    }
}

fn rotate_dial_with_zero_trans_count(
    mut dial_pos: DialPos,
    direction: char,
    amount: i32,
) -> (DialPos, u32) {
    let (amount, zero_trans_count_1) = DialPos::new_with_zero_trans_count(amount);
    let zero_trans_count_2;
    if direction == 'R' {
        (dial_pos, zero_trans_count_2) = dial_pos.add_zero_trans(amount);
    } else {
        (dial_pos, zero_trans_count_2) = dial_pos.sub_zero_trans(amount);
    }
    (dial_pos, zero_trans_count_1 + zero_trans_count_2)
}

pub fn process_input(input: &'static str) -> String {
    let rotations: Vec<_> = input
        .trim()
        .split('\n')
        .map(|r| {
            let mut chars = r.chars();
            let first = chars.next().unwrap();
            (first, chars.as_str())
        })
        .collect();
    //println!("rotations: {:?}", rotations);
    let mut zero_count = 0u32;
    let mut dial_pos = DialPos::new(50);
    for rot in rotations.clone().iter() {
        dial_pos = rotate_dial(dial_pos, rot.0, str_to_i32(rot.1));
        if dial_pos.value == 0 {
            zero_count += 1;
        }
    }
    let mut zero_count_method_0x = 0u32;
    let mut dial_pos = DialPos::new(50);
    for rot in rotations.iter() {
        let zero_trans_count;
        (dial_pos, zero_trans_count) =
            rotate_dial_with_zero_trans_count(dial_pos, rot.0, str_to_i32(rot.1));
        zero_count_method_0x += zero_trans_count;
    }
    format!(
        "Password: {:?}\nMethod 0x43: {:?}\n",
        zero_count, zero_count_method_0x
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
