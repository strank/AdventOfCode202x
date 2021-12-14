#![warn(clippy::all)]

use advent_of_code_202x::run_puzzles;

/// Two arguments possible: day year
/// both optional, select the latest one (year or day) by default
/// (year can only be specified if day is present too)
pub fn main() {
    run_puzzles(std::env::args().nth(2), std::env::args().nth(1));
}
