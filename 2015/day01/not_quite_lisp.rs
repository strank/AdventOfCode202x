//! https://adventofcode.com/2015/day/1
//! count parentheses
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2015day01::run;
//! assert!(run().contains(
//!     "floor: 74\nbasement index: 1795"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 3, and ??
/// ```
/// use advent_of_code_202x::generated::year2015day01::run_example;
/// assert!(run_example().contains(
///     "floor: 3\nbasement index: 7"));
/// ```
const EXAMPLE_INPUT: &str = "(()(()(";

pub fn process_input(input: &str) -> String {
    let parens: &str = input.trim();
    let ups: i32 = parens.matches('(').count() as i32;
    let downs: i32 = parens.len() as i32 - ups;
    let floor = ups - downs;
    let mut current_floor = 0;
    let mut basement_index = 0;
    for char in parens.chars() {
        basement_index += 1;
        current_floor += match char {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        };
        if current_floor < 0 {
            break;
        }
    }
    format!("floor: {}\nbasement index: {}", floor, basement_index)
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
