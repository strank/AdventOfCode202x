//! https://adventofcode.com/2024/day/8
//! find antinodes on a 2D map
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day08::run;
//! assert!(run().contains(
//!     "num antinodes: 276\nnum extended: 991"));
//! ```

use itertools::Itertools;
use num::integer::gcd;
use std::collections::{HashMap, HashSet};
use std::ops::Neg;

const INPUT: &str = include_str!("input");

/// example answer 14, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day08::run_example;
/// assert!(run_example().contains(
///     "num antinodes: 14\nnum extended: 34"));
/// ```
const EXAMPLE_INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

const DOT: char = '.';

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector(isize, isize);

impl Position {
    fn vector_from(&self, origin: &Position) -> Vector {
        Vector(
            self.0 as isize - origin.0 as isize,
            self.1 as isize - origin.1 as isize,
        )
    }

    fn add(&self, rhs: &Vector) -> Option<Position> {
        Some(Position(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }

    fn sub(&self, rhs: &Vector) -> Option<Position> {
        self.add(&-rhs)
    }
}

impl Vector {
    fn div_by_gcd(&self) -> Vector {
        let x_y_gcd = gcd(self.0, self.1);
        Vector(self.0 / x_y_gcd, self.1 / x_y_gcd)
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}

type PositionMap = HashMap<char, Vec<Position>>;

#[derive(Debug, Clone)]
struct AntennaMap {
    char_positions: PositionMap,
    max_row: usize,
    max_col: usize,
}

impl AntennaMap {
    fn from_str(map_str: &str) -> Self {
        let mut char_positions: PositionMap = HashMap::new();
        let (mut max_row, mut max_col): (usize, usize) = (0, 0);
        map_str
            .split('\n')
            .enumerate()
            .for_each(|(row_index, row_str)| {
                max_row = row_index;
                row_str.chars().enumerate().for_each(|(col_index, ch)| {
                    max_col = col_index;
                    if ch != DOT {
                        char_positions
                            .entry(ch)
                            .or_default()
                            .push(Position(row_index, col_index));
                    }
                })
            });
        Self {
            char_positions,
            max_row,
            max_col,
        }
    }

    fn inside_check(&self, pos: &Position) -> bool {
        pos.0 <= self.max_row && pos.1 <= self.max_col
    }

    fn get_antinodes(&self) -> HashSet<Position> {
        let mut result = HashSet::new();
        // for each letter,
        for (_, positions) in &self.char_positions {
            // get all pairs of antennas,
            for (pos_a, pos_b) in positions.iter().tuple_combinations() {
                // calculate distance vector between the two points
                let a_to_b = pos_b.vector_from(pos_a);
                // calculate both antinodes with checked addition/subtraction,
                // if they are inside the map-grid, add to result
                if let Some(anti_a) = pos_a.sub(&a_to_b) {
                    if self.inside_check(&anti_a) {
                        result.insert(anti_a);
                    }
                }
                if let Some(anti_b) = pos_b.add(&a_to_b) {
                    if self.inside_check(&anti_b) {
                        result.insert(anti_b);
                    }
                }
            }
        }
        result
    }

    fn get_extended_antinodes(&self) -> HashSet<Position> {
        let mut result = HashSet::new();
        // for each letter,
        for (_, positions) in &self.char_positions {
            // get all pairs of antennas,
            for (pos_a, pos_b) in positions.iter().tuple_combinations() {
                // calculate distance vector between the two points
                let a_to_b = pos_b.vector_from(pos_a);
                // divide the vector by the gcd of its axes:
                let a_to_b = a_to_b.div_by_gcd();
                // add a as first antinode,
                result.insert(*pos_a);
                // then calculate antinodes with checked addition/subtraction
                // repeatedly starting from a forwards/backwards and
                // while they are inside the map-grid, add to result
                let mut cur_pos = *pos_a;
                while let Some(anti) = cur_pos.add(&a_to_b) {
                    if !self.inside_check(&anti) {
                        break;
                    }
                    result.insert(anti);
                    cur_pos = anti;
                }
                let mut cur_pos = *pos_a;
                while let Some(anti) = cur_pos.sub(&a_to_b) {
                    if !self.inside_check(&anti) {
                        break;
                    }
                    result.insert(anti);
                    cur_pos = anti;
                }
            }
        }
        result
    }
}

pub fn process_input(input: &str) -> String {
    let antenna_map: AntennaMap = AntennaMap::from_str(input.trim());
    //println!("antennas: {:?}", antenna_map);
    // filter for equations tha could be correct and sum the test values:
    let num_antipodes: usize = antenna_map.get_antinodes().len();
    let num_extended: usize = antenna_map.get_extended_antinodes().len();
    format!(
        "num antinodes: {}\nnum extended: {}",
        num_antipodes, num_extended
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
