//! https://adventofcode.com/2024/day/1
//! sum the differences of the sorted entries in two lists.
//! part 2: similarity score
//!
//! ```
//! use advent_of_code_202x::generated::year2024day01::run;
//! assert!(run().contains(
//!     "sum of differences: 1889772\nsimilarity score: 23228917"));
//! ```

use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("input");

/// example answer 11, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day01::run_example;
/// assert!(run_example().contains(
///     "sum of differences: 11\nsimilarity score: 31"));
/// ```
const EXAMPLE_INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

fn str_to_i32(a_str: &str) -> i32 {
    a_str.parse::<i32>().unwrap()
}

fn count_occ_in_vec(the_vec: &[i32], the_num: i32) -> i32 {
    the_vec.iter().filter(|&n| *n == the_num).count() as i32
}

pub fn process_input(input: &str) -> String {
    let (a_list, b_list): (Vec<_>, Vec<_>) = input
        .trim()
        .split('\n')
        .map(|e| {
            e.split_whitespace()
                .map(str_to_i32)
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .unzip();
    //println!("a_list: {:?}\nb_list: {:?}", a_list, b_list);
    // sort, and then sum the differences
    let summed_diffs: i32 = a_list
        .iter()
        .sorted()
        .zip_eq(b_list.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum();
    let a_set: HashSet<_> = a_list.iter().cloned().collect();
    let b_set: HashSet<_> = b_list.iter().cloned().collect();
    let similarity_score: i32 = a_set
        .intersection(&b_set)
        .map(|&num| num * count_occ_in_vec(&a_list, num) * count_occ_in_vec(&b_list, num))
        .sum();

    format!(
        "sum of differences: {}\nsimilarity score: {}",
        summed_diffs, similarity_score,
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
