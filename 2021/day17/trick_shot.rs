//! https://adventofcode.com/2021/day/17
//! Calculate a trajectory to reach a target area, there is probably a closed-form solution
//! but we could of course also try reasonable values.
//! For part 1, x needs to be the minimum with a triangular number that reaches the target x
//!
//! ```
//! use advent_of_code_202x::generated::year2021day17::run;
//! assert!(run().contains("Max height: 19503\nvalid initial velocities: 5200"));
//! ```

use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet, ops::RangeInclusive};

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day17::run_example;
/// assert!(run_example().contains("Max height: 45\nvalid initial velocities: 112"));
/// ```
const EXAMPLE_INPUT: &str = "
target area: x=20..30, y=-10..-5
"; // highest point reached with 6,9: y of 45

/// Return the triangular number for a number, i.e. the sum of 1 to n, or equivalently n * (n+1) / 2
fn triangular_number(number: i32) -> i32 {
    number * (number + 1) / 2
}

/// Return the triangular root of a number, e.g. n for triangular number T(n) or in between
fn triangular_root(number: i32) -> f32 {
    ((number as f32 * 8f32 + 1f32).sqrt() - 1f32) / 2f32
}

/// Return the maximum initial y velocity that avoids missing the target area
fn find_max_initial_y(target: [[i32; 2]; 2]) -> i32 {
    // find y: y will increase and then decrease back to 0,
    // ideally we want the next step to land right in the target, so exactly the lower bound of the range,
    // (the one further away from 0), we therefore need to start with lowerbound - 1:
    target[1][0].abs() - 1
}

/// Return the maximum y value of the projectile so it hits the target area
fn find_max_height(target: [[i32; 2]; 2]) -> i32 {
    // maximum height is the triangular number of maximum y
    triangular_number(find_max_initial_y(target))
}

/// Return f(v,n) ... position after n steps for initial velocities v
/// This treats x and y the same, i.e. both subtract 1 on each step even beyond 0
/// Note: this is almost only used for debugging, not needed for actual solution!
#[allow(dead_code)]
fn pos_after_n_steps(velocity: (u32, i32), n: u32) -> [i32; 2] {
    // e.g. f(6,0) = 0 = T(6) - T(6-0), f(6,1) = 6 = T(6) - T(6-1), f(6,2) = 11 = T(6) - T(6-2)
    // expanded:
    // f(x,n) = x * (x + 1) / 2 - (x-n) * (x-n+1) / 2
    //        = 1/2 * (x * (x + 1) - (x-n) * (x-n+1))
    //        = 1/2 * (x2 + x - x2 + nx + nx - n2 - x + n)
    //        = 1/2 * (2nx + n - n2)
    //        = n/2 * (2x + 1 - n)
    // BUT: for x values, n cannot go above x as it would decrease the result again
    let val_calc = |val, n| (2 * val + 1 - n as i32) * n as i32 / 2;
    [
        val_calc(velocity.0 as i32, std::cmp::min(n, velocity.0)),
        val_calc(velocity.1, n),
    ]
}

/// Compare a position with a target area and return a comparison result
fn cmp_pos_target(pos: [i32; 2], target: [[i32; 2]; 2]) -> Ordering {
    if pos[0] > target[0][1] || pos[1] < target[1][0] {
        Ordering::Greater // pos overshot!
    } else if pos[0] < target[0][0] || pos[1] > target[1][1] {
        Ordering::Less // pos could still reach target
    } else {
        Ordering::Equal // pos is inside target area!
    }
}

/// Return whether the velocity pair reaches target area after n or more steps
fn validate_velocity(target: [[i32; 2]; 2], velocity: (u32, i32), mut n: u32) -> bool {
    loop {
        let pos = pos_after_n_steps(velocity, n);
        match cmp_pos_target(pos, target) {
            Ordering::Equal => return true,
            Ordering::Less => n += 1,
            Ordering::Greater => {
                // println!(
                //     "velocity {:?} after n {} at pos {:?} not in target {:?}",
                //     velocity, n, pos, target
                // );
                return false;
            }
        }
    }
}

/// Return the initial velocity component (as float) that reachest closest to target after n steps
fn xy_root_for_target_after_n_steps(target: i32, n: u32) -> f32 {
    // reversing f(x, n) see fn pos_after_n_steps:
    // x based on n and f (the resulting position)? (need >= and <= eventually)
    //   1/2 (2nx + n - n2) = f
    //   2nx = 2f - n + n2
    //   x   = (2f - n + n2) / 2n
    //   x   = f/n - 1/2 + n/2
    //   x   = f/n + (n-1)/2
    assert!(n > 0);
    target as f32 / n as f32 + (n - 1) as f32 / 2f32
}

/// Return a range of x-values that will reach x-target in n steps
fn x_values_in_target_after_n_steps(x_target: [i32; 2], n: u32) -> RangeInclusive<u32> {
    // use ceil for lower bound and floor for upper bound
    let lower = xy_root_for_target_after_n_steps(x_target[0], n).ceil() as u32;
    let upper = xy_root_for_target_after_n_steps(x_target[1], n).floor() as u32;
    lower..=upper
}

/// Return a range of y-values that will reach y-target in n steps
fn y_values_in_target_after_n_steps(y_target: [i32; 2], n: u32) -> RangeInclusive<i32> {
    // use ceil for lower bound and floor for upper bound
    let lower = xy_root_for_target_after_n_steps(y_target[0], n).ceil() as i32;
    let upper = xy_root_for_target_after_n_steps(y_target[1], n).floor() as i32;
    lower..=upper
}

/// Return the number of valid initial velocities, i.e. the one that hit the target area
/// we are assuming that the target area is always in the positive x and negative y direction!
fn count_valid_initial_v(target: [[i32; 2]; 2]) -> i32 {
    // we have to distinguish some cases based on x values:
    // (A) direct hits that get to the target in one go, i.e. x-values in the target range,
    //     y-values can therefore also be from the whole target range:
    let direct_hits =
        ((target[0][0] - target[0][1]).abs() + 1) * ((target[1][0] - target[1][1]).abs() + 1);
    //println!("Direct hits, in target after 1 step: {}", direct_hits);
    // (B) reaching the target in the x dimension after 2 or more steps
    //     either staying in the target area eventually or overshooting.
    //     calculate the bounds for both cases, find matching y-values,
    //     collect resulting pairs in a set to eliminate duplicates
    let mut result_set: HashSet<(u32, i32)> = HashSet::new();
    // (B1) x-values that reach and then stay in the target area, i.e. triangular roots.
    //     This allows higher y, specifically up to the one used in part 1, max_y,
    //     but the possible positive y-values have to be found.
    //     first find lowest triangular number inside x range!
    let min_stay_x = triangular_root(target[0][0]).ceil() as u32;
    let max_stay_x = triangular_root(target[0][1]).floor() as u32;
    let max_y = find_max_initial_y(target);
    // println!(
    //     "X values that end up staying in target range (after x steps): {:?}",
    //     min_stay_x..=max_stay_x
    // );
    for n in min_stay_x..=max_stay_x {
        let valid_y_range = y_values_in_target_after_n_steps(target[1], n);
        // println!(
        //     "Considering for x/n {} y_range {:?} max_y {}",
        //     n, valid_y_range, max_y
        // );
        // The following overestimates the y-range considerably and there is a way of getting the
        // exact range (full y-target range but offset by 1, then probably several smaller ranges depending on target-range)
        // but by now the search space is massively reduced, so just validate all:
        // extend the range up to max_y and combine with n as x
        for v in (n..n + 1).cartesian_product(*valid_y_range.start()..=max_y) {
            if validate_velocity(target, v, n) {
                result_set.insert(v);
            }
        }
    }
    //println!("Len of result set: {}", result_set.len());
    // (B2) enumerate x-values that reach the target area in n>1 steps but then overshoot,
    //     then find y values with the same steps.
    //     iterate up to and not including n that reaches and then stays in the target area
    //println!("Remaining steps-range to test: {:?}", 2..min_stay_x);
    for n in 2u32..min_stay_x {
        let valid_x_range = x_values_in_target_after_n_steps(target[0], n);
        let valid_x_range_clone = valid_x_range.clone();
        //println!("valid x values for {} steps: {:?}", n, valid_x_range);
        let valid_y_range = y_values_in_target_after_n_steps(target[1], n);
        //println!("valid y values for {} steps: {:?}", n, valid_y_range);
        let y_range_end = valid_y_range.end() + 1;
        result_set.extend(valid_x_range.cartesian_product(valid_y_range));
        // unfortunately the previous misses a small amount of lucky shots, so we extend the range here too and validate:
        for v in valid_x_range_clone.cartesian_product(y_range_end..=max_y) {
            if validate_velocity(target, v, n) {
                result_set.insert(v);
            }
        }
    }
    //println!("Len of result set: {}", result_set.len());

    // TODO other cases, x values that can reach with negative or zero y, cannot be smaller than triangular root of lower x,
    // because that would never reach the target in the x dimension,
    // must not overshoot the target e.g. bigger than max_x_target / 2 + 2
    // cases with positive y probably need to be close to the triangular roots of the x target bounds for x!

    // x will be triangular number of x, so
    // find y: y will increase and then decrease back to 0,
    // ideally we want the next step to land right in the target, so exactly the lower bound of the range,
    // (further away from 0), we therefore need to start with lowerbound - 1:

    direct_hits + result_set.len() as i32
}

/// Return an x range and a y range from the string input
fn parse_target_area(puzzle_input: &'static str) -> [[i32; 2]; 2] {
    let values: Vec<[i32; 2]> = puzzle_input
        .trim_start_matches("target area: x=")
        .split(", y=")
        .map(|ele| {
            ele.split("..")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect();
    values.try_into().unwrap()
}

pub fn process_input(input: &'static str) -> String {
    let target_area = parse_target_area(input.trim());
    let max_height = find_max_height(target_area);
    let num_valid = count_valid_initial_v(target_area);
    format!(
        "Max height: {}\nvalid initial velocities: {}\n",
        max_height, num_valid
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
