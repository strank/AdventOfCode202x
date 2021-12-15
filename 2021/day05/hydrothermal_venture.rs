//! https://adventofcode.com/2021/day/5
//! given a list of line segments
//! a,b -> x,y
//! record all points that the line segments touch.
//! first part: how many points get touched more than once?
//!
//! ```
//! use advent_of_code_202x::generated::year2021day05::run;
//! assert!(run().contains(
//!     "Number of doubly used points, straight-only: 6311\nNumber of doubly used points: 19929"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 5
/// ```
/// use advent_of_code_202x::generated::year2021day05::run_example;
/// assert!(run_example().contains(
///     "Number of doubly used points, straight-only: 5\nNumber of doubly used points: 12"));
/// ```
const EXAMPLE_INPUT: &str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

use num::{PrimInt, Unsigned};
use std::collections::HashMap;
use std::iter;

type Point = (usize, usize);
type LineSpec = (Point, Point);
/// hashmap that records how often a point is used by lines
type PosMap = HashMap<Point, usize>;

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

fn calc_points_use_count(line_specs: &[LineSpec], diag: bool) -> PosMap {
    let mut points_used = PosMap::new();
    for ((a, b), (x, y)) in line_specs {
        if a != x && b != y {
            assert_eq!(abs_diff(*x, *a), abs_diff(*y, *b)); // as noted in the task
            if !diag {
                continue;
            }
        }
        let a_range: Box<dyn Iterator<Item = usize>> = if a != x {
            if x > a {
                Box::new(*a..=*x)
            } else {
                Box::new((*x..=*a).rev())
            }
        } else {
            // repeat a as needed:
            Box::new(iter::repeat(*a))
        };
        let b_range: Box<dyn Iterator<Item = usize>> = if b != y {
            if y > b {
                Box::new(*b..=*y)
            } else {
                Box::new((*y..=*b).rev())
            }
        } else {
            // repeat b as needed:
            Box::new(iter::repeat(*b))
        };
        for (index_a, index_b) in a_range.zip(b_range) {
            *points_used.entry((index_a, index_b)).or_insert(0) += 1;
        }
    }
    points_used
}

fn get_doubly_used_points(points_used: &PosMap) -> usize {
    points_used.values().filter(|count| count > &&1).count()
}

fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}

fn line_splitter(line: &str) -> LineSpec {
    // very verbose, next time try itertools::next_tuple
    match line
        .trim()
        .split(&[',', ' ', '-', '>'][..])
        .collect::<Vec<&str>>()[..]
    {
        [startx, starty, _, _, _, endx, endy, ..] => (
            (str_to_usize(startx), str_to_usize(starty)),
            (str_to_usize(endx), str_to_usize(endy)),
        ),
        _ => panic!("No line-spec found!"),
    }
}

pub fn process_input(input: &str) -> String {
    let line_specs: Vec<_> = input
        .trim()
        .split('\n')
        .map(line_splitter)
        .collect();
    //println!("lines:\n{:?}", line_specs);
    let points_used_1 = calc_points_use_count(&line_specs, false);
    let points_used_2 = calc_points_use_count(&line_specs, true);
    format!(
        "Number of doubly used points, straight-only: {}\nNumber of doubly used points: {}",
        get_doubly_used_points(&points_used_1),
        get_doubly_used_points(&points_used_2)
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
