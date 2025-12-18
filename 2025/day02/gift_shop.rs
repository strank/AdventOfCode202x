//! https://adventofcode.com/2025/day/02
//! Find invalid IDs in the gift shop system
//!
//! ```
//! use advent_of_code_202x::generated::year2025day02::run;
//! assert!(run().contains("Sum of invalid IDs: 5398419778\nNew rules: 15704845910"));
//! ```

use itertools::Itertools;

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2025day02::run_example;
/// assert!(run_example().contains("Sum of invalid IDs: 1227775554\nNew rules: 4174379265"));
/// ```
const EXAMPLE_INPUT: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"; // 3

fn str_to_u64(a_str: &str) -> u64 {
    a_str.parse::<u64>().unwrap()
}

/// Return all invalid IDs in the given range
/// Invalid means a repeat of the same digit pattern twice, e.g. 6464
fn find_invalid_ids(range_start: &str, range_end: &str) -> Vec<u64> {
    // first check if the range_start and range_end have the same length
    let range_len = range_start.len();
    if range_len == range_end.len() {
        if range_len % 2 != 0 {
            //println!("Range starting at {} cannot contain any", range_start);
            return vec![];
        }
        // even length so split both start and end in two halfs:
        let range_midpoint = range_len / 2;
        let range_start_left = &range_start[..range_midpoint];
        let range_start_right = &range_start[range_midpoint..];
        let range_end_left = &range_end[..range_midpoint];
        let range_end_right = &range_end[range_midpoint..];
        //println!(
        //    "checking: {}|{} to {}|{}",
        //    range_start_left, range_start_right, range_end_left, range_end_right,
        //);
        // for every unique possibility for the left side, there is one possible invalid ID,
        // but the first and the last one of those is only in the range if range_start_right
        // is not too high and range_end_right is not too low respectively:
        let mut begin = str_to_u64(range_start_left);
        if begin < str_to_u64(range_start_right) {
            begin += 1;
        }
        let mut end = str_to_u64(range_end_left);
        if end <= str_to_u64(range_end_right) {
            end += 1;
        }
        (begin..end)
            .map(|left_side| left_side * 10u64.pow(range_midpoint as u32) + left_side)
            .collect()
    } else {
        // if not, split it into multiple ranges with the same length and recursively call this method
        let mut result = find_invalid_ids(range_start, &"9".repeat(range_len));
        result.extend(find_invalid_ids(
            &("1".to_owned() + &"0".repeat(range_len)),
            range_end,
        ));
        result
    }
}

/// Return all invalid IDs in the given range
/// This time Invalid means any repeated digit pattern, e.g. 6464, 111, 130130130
fn find_invalid_ids_new_rules(range_start: &str, range_end: &str) -> Vec<u64> {
    // first check if the range_start and range_end have the same length
    let range_len = range_start.len();
    if range_len == range_end.len() {
        // find all possible divisors of the length,
        // for each length generate all possible repeat pattern numbers
        // and check if they are within the range
        let start = str_to_u64(range_start);
        let end = str_to_u64(range_end);
        //println!("\nchecking: {} to {}", start, end);
        // each divisor up to half the len can be a pattern length:
        (1..=range_len / 2)
            // but only exact divisors:
            .filter(|pl| range_len % pl == 0)
            // take possible patterns from the range_start and range_end strings:
            .flat_map(|pl| {
                (str_to_u64(&range_start[..pl])..=str_to_u64(&range_end[..pl]))
                    .map(move |p_start| str_to_u64(&p_start.to_string().repeat(range_len / pl)))
                    // and filter out the ones that are too big or too small:
                    .filter(|&pti| pti >= start && pti <= end)
            })
            // and no duplicates
            .unique()
            .collect()
    } else {
        // if not, split it into multiple ranges with the same length and recursively call this method
        let mut result = find_invalid_ids_new_rules(range_start, &"9".repeat(range_len));
        result.extend(find_invalid_ids_new_rules(
            &("1".to_owned() + &"0".repeat(range_len)),
            range_end,
        ));
        result
    }
}

pub fn process_input(input: &'static str) -> String {
    let ranges: Vec<_> = input
        .trim()
        .split(',')
        .map(|r| r.split('-').collect::<Vec<_>>())
        .collect();
    //println!("ranges: {:?}", ranges);
    let sum_of_invalids = ranges
        .iter()
        .map(|r| {
            find_invalid_ids(r.first().unwrap(), r.last().unwrap())
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>();
    let sum_of_invalids_new = ranges
        .iter()
        .map(|r| {
            find_invalid_ids_new_rules(r.first().unwrap(), r.last().unwrap())
                .iter()
                //.inspect(|ii| print!(" {} ", ii))
                .sum::<u64>()
        })
        .sum::<u64>();
    format!(
        "Sum of invalid IDs: {:?}\nNew rules: {:?}\n",
        sum_of_invalids, sum_of_invalids_new
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
