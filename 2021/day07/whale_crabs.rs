use num::{PrimInt, Unsigned};

/// https://adventofcode.com/2021/day/7
/// based on a list of horizontal positions, find the position with minimum sum of distances 
/// (the position to align crabs on using minimal fuel to get them there)

const _TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
// --> answer 37 fuel to align to position 2


fn abs_diff<U>(slf: U, other: U)  -> U
    where U: PrimInt + Unsigned
{
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}


fn find_median(positions: &Vec<usize>) -> usize {
    let len = positions.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (positions[mid - 1] + positions[mid]) / 2
    } else {
        positions[mid]
    }
}


fn calculate_fuel_use(positions: &Vec<usize>, target_pos: usize) -> usize {
    positions.iter().map(|pos| abs_diff(*pos, target_pos)).sum()
}


fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}


pub fn run() -> () {
    let mut positions: Vec<_> = include_str!("input")
            .trim()
            .split(",")
            .map(str_to_usize)
            .collect();
    positions.sort();
    //println!("positions:\n{:?}", positions);
    let median_position = find_median(&positions);
    let fuel_use = calculate_fuel_use(&positions, median_position);
    println!("fuel use for position {}: {}", median_position, fuel_use);
}
