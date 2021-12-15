//! https://adventofcode.com/2020/day/25
//! break cryptographic handshake
//!
//! two entities (card and door) generate a public key:
//! 7 ^ loopsize_of_entity ==> public_key (modulo 20201227)
//!
//! 7 and 20201227 are prime
//!
//! We need a "discrete logarithm" to calculate the loopsizes

const INPUT: &str = include_str!("input");

const EXAMPLE_INPUT: &str = "
5764801
17807724
";

const BASE: i32 = 7;
const MODULO: i32 = 20201227;

/// find the discrete log of num with base (modulo modulo)
/// (extended euclidean algorithm)
fn find_discrete_log(num: i32, base: i32, modulo: i32) -> i32 {
    //if num % base == 0 {
    //    return num / base;
    //}
    // do it by "trial multiplication" first (brute force search):
    let mut value = 1;
    for ll in 1..modulo {
        value *= base;
        value %= modulo;
        if value == num {
            return ll;
        }
    }
    value // should be unreachable
}

fn transform_subject_number(subject: i32, loop_size: i32, modulo: i32) -> i32 {
    let mut value: i64 = 1;
    let subject = i64::from(subject);
    let modulo = i64::from(modulo);
    for _ll in 1..=loop_size {
        value *= subject;
        value %= modulo;
    }
    value as i32
}

pub fn process_input(input: &str) -> String {
    let input: Vec<_> = input.trim().split('\n').collect();
    let public_keys: Vec<_> = input.iter().map(|pk| pk.parse::<i32>().unwrap()).collect();
    //println!("Input keys: {:?}", &public_keys);
    // find discrete logarithm with base 7 in modular arithmetic to get the loop-sizes:
    let loop_sizes: Vec<_> = public_keys
        .iter()
        .map(|&pk| find_discrete_log(pk, BASE, MODULO))
        .collect();
    //println!("Loop sizes: {:?}", &loop_sizes);
    // check that it's true:
    for loop_s in &loop_sizes {
        println!(
            "transform subject number {} -> {}",
            BASE,
            transform_subject_number(7, *loop_s, MODULO)
        );
    }
    // calculate the encryption key
    format!(
        "Encryption key, part 1: {}",
        transform_subject_number(public_keys[0], loop_sizes[1], MODULO)
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
