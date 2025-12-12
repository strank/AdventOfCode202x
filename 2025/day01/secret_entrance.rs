//! https://adventofcode.com/2025/day/01
//! Find password on rotary dial
//!
//! ```
//! use advent_of_code_202x::generated::year2025day01::run;
//! assert!(run().contains("Password: 1177\nMethod 0x43: 6768"));
//! ```

use std::ops::{Add, Neg, Sub};

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2025day01::run_example;
/// assert!(run_example().contains("Password:  3\nMethod 0x43: 6"));
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
struct NumberMod<const N: u8> {
    value: u8,
}

impl<const N: u8> NumberMod<N> {
    fn new(value: i32) -> Self {
        Self {
            value: value.rem_euclid(N as i32) as u8,
        }
    }

    fn zero() -> Self {
        Self { value: 0 }
    }
}

impl<const N: u8> Add for NumberMod<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value).rem_euclid(N),
        }
    }
}

impl<const N: u8> Neg for NumberMod<N> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: (N - self.value).rem_euclid(N),
        }
    }
}

impl<const N: u8> Sub for NumberMod<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<const N: u8> NumberMod<N> {
    fn new_zero_trans(value: i32) -> (Self, u32) {
        (
            Self {
                value: value.rem_euclid(N as i32) as u8,
            },
            (value / N as i32) as u32,
        )
    }

    fn add_zero_trans(self, other: Self) -> (Self, u32) {
        (
            self + other,
            if self.value + other.value > N { 1 } else { 0 },
        )
    }

    fn sub_zero_trans(self, other: Self) -> (Self, u32) {
        (
            self - other,
            if self.value != 0 && (self.value as i16) - (other.value as i16) < 0 {
                1
            } else {
                0
            },
        )
    }
}

fn str_to_i32(a_str: &str) -> i32 {
    a_str.parse::<i32>().unwrap()
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
    println!("rotations: {:?}", rotations);
    let mut passw = 0u32;
    let mut current = NumberMod::<100>::new(50);
    for rot in rotations.clone().iter() {
        if rot.0 == 'R' {
            current = current + NumberMod::<100>::new(str_to_i32(rot.1));
        } else {
            current = current - NumberMod::<100>::new(str_to_i32(rot.1));
        }
        print!(" {:?} ", current.value);
        if current == NumberMod::<100>::zero() {
            passw += 1;
        }
    }
    let mut passw_method_0x = 0u32;
    let mut current = NumberMod::<100>::new(50);
    print!("Starting with {:?}\n", current.value);
    for rot in rotations.iter() {
        let (next_num, mut zero_trans_count) = NumberMod::<100>::new_zero_trans(str_to_i32(rot.1));
        passw_method_0x += zero_trans_count;
        print!(" {:?}{:?} (z{:?})", rot.0, next_num.value, zero_trans_count);
        if rot.0 == 'R' {
            (current, zero_trans_count) = current.add_zero_trans(next_num);
        } else {
            (current, zero_trans_count) = current.sub_zero_trans(next_num);
        }
        passw_method_0x += zero_trans_count;
        print!(" -> {:?} (z{:?})", current.value, zero_trans_count);
        if current == NumberMod::<100>::zero() {
            passw_method_0x += 1;
            print!(" z1");
        }
    }
    format!(
        "Password: {:?}\nMethod 0x43: {:?}\n",
        passw, passw_method_0x
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
