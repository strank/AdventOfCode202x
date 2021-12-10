use std::collections::HashMap;
use std::iter;
use num::{PrimInt, Unsigned};

/// https://adventofcode.com/2021/day/5
/// given a list of line segments
/// a,b -> x,y
/// record all points that the line segments touch.
/// first part: how many points get touched more than once?

/// hashmap that records how often a point is used by lines
type PosMap = HashMap<(usize, usize), usize>;

const _TEST_INPUT: &str = "
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
"; // --> answer 5


fn abs_diff<U>(slf: U, other: U)  -> U
    where U: PrimInt + Unsigned
{
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}


fn calc_points_use_count(line_specs: &Vec<((usize, usize), (usize, usize))>, diag: bool) -> PosMap {
    let mut points_used: PosMap = HashMap::new();
    for ((a, b), (x, y)) in line_specs {
        if a != x && b != y {
            assert_eq!(abs_diff(*x, *a), abs_diff(*y, *b)); // as noted in the task
            if !diag {
                continue;
            }
        }
        let a_range: Box<dyn Iterator<Item = usize>> = if a != x {
            if x > a { Box::new(*a..=*x) } else { Box::new((*x..=*a).rev()) }
        } else { // repeat a as needed:
            Box::new(iter::repeat(*a))
        };
        let b_range: Box<dyn Iterator<Item = usize>> = if b != y {
            if y > b { Box::new(*b..=*y) } else { Box::new((*y..=*b).rev()) }
        } else { // repeat b as needed:
            Box::new(iter::repeat(*b))
        };
        for (index_a, index_b) in a_range.zip(b_range) {
            *points_used.entry((index_a, index_b)).or_insert(0) += 1;
        }
    }
    points_used
}


fn get_doubly_used_points(points_used: &PosMap) -> usize {
    points_used.values()
        .filter(|count| count > &&1)
        .count()
}


fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}


fn line_splitter(line: &str) -> ((usize, usize), (usize, usize)) {
    // very verbose, next time try itertools::next_tuple
    match &line.trim().split(&[',', ' ', '-', '>'][..]).collect::<Vec<&str>>()[..] {
        &[startx, starty, _, _, _, endx, endy, ..] => {
            ((str_to_usize(startx), str_to_usize(starty)),
             (str_to_usize(endx), str_to_usize(endy)))
        },
        _ => panic!("No line-spec found!"),
    }
}

pub fn run() -> () {
    let line_specs: Vec<_> = include_str!("input")
            .trim()
            .split("\n")
            .map(line_splitter)
            .collect();
    //println!("lines:\n{:?}", line_specs);
    let points_used = calc_points_use_count(&line_specs, false);
    println!("Number of doubly used points: {}", get_doubly_used_points(&points_used));
    let points_used = calc_points_use_count(&line_specs, true);
    //println!("points used:\n{:?}", points_used);
    println!("Number of doubly used points: {}", get_doubly_used_points(&points_used));
}
