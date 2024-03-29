//! https://adventofcode.com/2021/day/7
//! based on a list of horizontal positions, find the position with minimum sum of distances
//! (the position to align crabs on using minimal fuel to get them there)
//! ```
//! use advent_of_code_202x::generated::year2021day07::run;
//! assert!(run().contains("fuel for pos 311: 347011\ntriangular fuel for pos 464: 98363777"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 37 fuel to align to position 2
/// ```
/// use advent_of_code_202x::generated::year2021day07::run_example;
/// assert!(run_example().contains("fuel for pos 2: 37\ntriangular fuel for pos 5: 168"));
/// ```
const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

use num::{PrimInt, Unsigned};

fn abs_diff<U>(slf: U, other: U) -> U
where
    U: PrimInt + Unsigned,
{
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

/// expects an already sorted array slice!
fn find_median(positions: &[usize]) -> usize {
    let len = positions.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (positions[mid - 1] + positions[mid]) / 2
    } else {
        positions[mid]
    }
}

fn find_average(positions: &[usize]) -> f32 {
    positions.iter().sum::<usize>() as f32 / positions.len() as f32
}

fn calculate_fuel_use(positions: &[usize], target_pos: usize) -> usize {
    positions.iter().map(|pos| abs_diff(*pos, target_pos)).sum()
}

fn calculate_fuel_use_triangular(positions: &[usize], target_pos: usize) -> usize {
    positions
        .iter()
        .map(|pos| {
            let diff = abs_diff(*pos, target_pos);
            diff * (diff + 1) / 2
        })
        .sum()
}

/// didn't bother with proving that, but the best case should be at the average,
/// or at least very close by
fn find_best_fuel_use_triangular(positions: &[usize]) -> (usize, usize) {
    let mut target_pos = find_average(positions).round() as usize;
    let mut best_case = calculate_fuel_use_triangular(positions, target_pos);
    for new_target in [target_pos - 1, target_pos + 1] {
        let new_sum = calculate_fuel_use_triangular(positions, new_target);
        if new_sum < best_case {
            best_case = new_sum;
            target_pos = new_target;
        }
    }
    (target_pos, best_case)
}

fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}

pub fn process_input(input: &str) -> String {
    let mut positions: Vec<_> = input.trim().split(',').map(str_to_usize).collect();
    positions.sort_unstable();
    //println!("positions:\n{:?}", positions);
    let median_position = find_median(&positions);
    let fuel_use = calculate_fuel_use(&positions, median_position);
    // different distance measure in part 2: triangular number
    let (best_position, fuel_use_t) = find_best_fuel_use_triangular(&positions);
    format!(
        "fuel for pos {}: {}\ntriangular fuel for pos {}: {}",
        median_position, fuel_use, best_position, fuel_use_t
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
