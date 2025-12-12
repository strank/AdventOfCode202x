//! https://adventofcode.com/2024/day/3
//! look for uncorrupted mul instructions
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day03::run;
//! assert!(run().contains(
//!     "mul result: 164730528\ndodont result: 70478672"));
//! ```

use regex::Regex;

const INPUT: &str = include_str!("input");

/// example answer 161, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day03::run_example;
/// assert!(run_example().contains(
///     "mul result: 161\ndodont result: 48"));
/// ```
const EXAMPLE_INPUT: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn str_to_i32(a_str: &str) -> i32 {
    a_str.parse::<i32>().unwrap()
}

fn get_mul_sum(mul_string: &str) -> i32 {
    // regexp for mul(ddd,ddd)
    let re = Regex::new(r"mul\(([0-9]{1,3}?),([0-9]{1,3}?)\)").unwrap();
    re.captures_iter(mul_string)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            str_to_i32(a) * str_to_i32(b)
        })
        .sum()
}

pub fn process_input(input: &str) -> String {
    let mul_string: &str = input.trim();
    let mul_result = get_mul_sum(mul_string);
    // part two: initially iterated over more complicted regex with do and dont matches
    // turning evaluation on and off, but just extracting the disabled parts is better:
    // (important: could also be a don't() at the end of the input without matching do!)
    let re_dont_do = Regex::new(r"don\'t\(\)[\s\S]*?(?:do\(\)|$)").unwrap();
    let shortened_mul_string = re_dont_do.replace_all(mul_string, "");
    let mul_do_dont_result = get_mul_sum(&shortened_mul_string);
    //println!("muls: {:?}", muls);
    format!(
        "mul result: {}\ndodont result: {}",
        mul_result, mul_do_dont_result
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
