//! https://adventofcode.com/2024/day/5
//! check sorting order
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day05::run;
//! assert!(run().contains(
//!     "sum of middles: 5955\nsum of sorted middles: ???"));
//! ```

use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("input");

/// example answer 161, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day05::run_example;
/// assert!(run_example().contains(
///     "sum of middles: 143\nsum of sorted middles: 123"));
/// ```
const EXAMPLE_INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

/// rules for print queue ordering, an entry for a number lists the ones that need to be after it if present
type Rules = HashMap<u8, Vec<u8>>;

fn str_to_u8(a_str: &str) -> u8 {
    a_str.parse::<u8>().unwrap()
}

/// Return a Rules hashmap, with page numbers mapped to page numbers that follow
fn parse_rules(rules_str: &'static str) -> Rules {
    let mut result = Rules::new();
    for (a, b) in rules_str
        .split('\n')
        .map(|line| line.split("|").map(str_to_u8).next_tuple().unwrap())
    {
        result.entry(a).or_default().push(b);
    }
    result
}

/// Return a list of lists of numbers
fn parse_queues(queues_str: &'static str) -> Vec<Vec<u8>> {
    queues_str
        .split('\n')
        .map(|line| line.split(',').map(str_to_u8).collect())
        .collect()
}

/// Return true iff the queue conforms to the rules
fn check_queue(queue: &Vec<u8>, rules: &Rules) -> bool {
    //println!("\nChecking queue {:?}", queue);
    queue
        .iter()
        .enumerate()
        .all(|(index, ele)| match rules.get(ele) {
            Some(rulelist) => {
                //println!("Rulelist for {:?}: {:?}", ele, rulelist);
                rulelist
                    .iter()
                    .all(|&other| match queue.iter().position(|&e| e == other) {
                        Some(other_index) => index < other_index,
                        None => true,
                    })
            }
            _ => true,
        })
}

pub fn process_input(input: &'static str) -> String {
    let (rules_str, queues_str) = input.trim().split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let queues: Vec<Vec<u8>> = parse_queues(queues_str);
    //println!("rules {:?}\n\nqueues {:?}", rules, queues);
    let (correct_queues, wrong_queues): (Vec<Vec<u8>>, Vec<Vec<u8>>) =
        queues.into_iter().partition(|q| check_queue(q, &rules));
    // filter the correct ones and sum up the middle numbers:
    let sum_middles: u32 = correct_queues.iter().map(|q| q[q.len() / 2] as u32).sum();
    let sorted_queues: Vec<Vec<u8>> = wrong_queues
        .into_iter()
        .map(|q| {
            q.into_iter()
                .sorted_by(|a, b| match rules.get(a) {
                    Some(rulelist) => {
                        if rulelist.contains(b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    _ => Ordering::Greater,
                })
                .collect()
        })
        .collect();
    let sum_sorted_middles: u32 = sorted_queues.iter().map(|q| q[q.len() / 2] as u32).sum();
    format!(
        "sum of middles: {}\nsum of sorted middles: {}",
        sum_middles, sum_sorted_middles
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
