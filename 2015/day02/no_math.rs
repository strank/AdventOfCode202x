//! https://adventofcode.com/2015/day/2
//! wrapping paper needs
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2015day02::run;
//! assert!(run().contains(
//!     "area needed: 1588178\nlength needed: 3783758"));
//! ```

use itertools::Itertools;

const INPUT: &str = include_str!("input");

/// example answer 161, and ??
/// ```
/// use advent_of_code_202x::generated::year2015day02::run_example;
/// assert!(run_example().contains(
///     "area needed: 101\nlength needed: 48"));
/// ```
const EXAMPLE_INPUT: &str = "
2x3x4
1x1x10
";

fn str_to_i64(a_str: &str) -> i64 {
    a_str.parse::<i64>().unwrap()
}

fn strxxx_to_triple(dim_string: &str) -> (i64, i64, i64) {
    dim_string.split('x').map(str_to_i64).next_tuple().unwrap()
}

fn required_area(dims: &(i64, i64, i64)) -> i64 {
    let (a, b, c) = dims;
    let sides = [a * b, a * c, b * c];
    *sides.iter().min().unwrap() + sides.iter().sum::<i64>() * 2
}

fn required_length(dims: &(i64, i64, i64)) -> i64 {
    let (a, b, c) = dims;
    let (&min_a, &min_b) = [a, b, c].iter().sorted().next_tuple().unwrap();
    let volume: i64 = [*a, *b, *c].iter().product();
    volume + (min_a + min_b) * 2
}

pub fn process_input(input: &str) -> String {
    let dims_list: Vec<_> = input.trim().split('\n').map(strxxx_to_triple).collect();
    //println!("dims_list: {:?}", dims_list);
    let area_needed: i64 = dims_list.iter().map(required_area).sum();
    let length_needed: i64 = dims_list.iter().map(required_length).sum();
    format!(
        "area needed: {}\nlength needed: {}",
        area_needed, length_needed
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
