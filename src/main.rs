#![warn(clippy::all)]

use advent_of_code_202x::run_puzzles;

/// Two arguments possible: day year
/// both optional, select the latest one (year or day) by default
/// (year can only be specified if day is present too)
pub fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let use_example = if let Some(pos) = args.iter().position(|x| *x == "x") {
        args.remove(pos);
        true
    } else {
        false
    };
    run_puzzles(args.get(2), args.get(1), use_example);
}
