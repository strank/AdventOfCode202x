//! https://adventofcode.com/2024/day/2
//! check that reports are safe or unsafe, count safe ones
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day02::run;
//! assert!(run().contains(
//!     "safe reports: 526\ndampened: xxx"));
//! ```

use itertools::Itertools;

const INPUT: &str = include_str!("input");

/// example answer 2, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day02::run_example;
/// assert!(run_example().contains(
///     "safe reports: 2\ndampened: xxx"));
/// ```
const EXAMPLE_INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn str_to_i32(a_str: &str) -> i32 {
    a_str.parse::<i32>().unwrap()
}

fn is_dampened_safe_report(the_vec: &[i32]) -> bool {
    if is_safe_report(the_vec) {
        return true;
    }
    let mut one_dropped_combinations = the_vec.iter().cloned().combinations(the_vec.len() - 1);
    one_dropped_combinations.any(|rep| is_safe_report(&rep))
}

fn is_safe_report(the_vec: &[i32]) -> bool {
    let all_diffs: Vec<i32> = the_vec
        .windows(2)
        .map(|pair| pair.first().unwrap() - pair.last().unwrap())
        .collect();
    all_diffs.iter().all(|&d| d > 0 && d < 4) || all_diffs.iter().all(|&d| d < 0 && d > -4)
}

pub fn process_input(input: &str) -> String {
    let report_list: Vec<_> = input
        .trim()
        .split('\n')
        .map(|e| e.split_whitespace().map(str_to_i32).collect::<Vec<_>>())
        .collect();
    //println!("report_list: {:?}", report_list);
    // filter only safe reports and count
    // a "safe" report is either all increasing or all decreasing
    let safe_count: usize = report_list
        .iter()
        .filter(|&rep| is_safe_report(rep))
        .count();
    let dampened_safe_count: usize = report_list
        .iter()
        .filter(|&rep| is_dampened_safe_report(rep))
        .count();
    format!(
        "safe reports: {}\ndampened: {}",
        safe_count, dampened_safe_count
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
