//! https://adventofcode.com/2024/day/4
//! word search for XMAS
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day04::run;
//! assert!(run().contains(
//!     "num of xmas's: 2633\nnum of x-mas's: 1936"));
//! ```

use itertools::iproduct;
use ndarray::prelude::*;
use num::integer::div_floor;

const INPUT: &str = include_str!("input");

/// example answer 161, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day04::run_example;
/// assert!(run_example().contains(
///     "num of xmas's: 18\nnum of x-mas's: 9"));
/// ```
const EXAMPLE_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const DIRS: [isize; 3] = [0, -1, 1];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug)]
struct CharGrid(Array2<char>);

impl CharGrid {
    fn from_str(grid_str: &str) -> Self {
        let row_length = grid_str.find('\n').unwrap();
        let num_rows = div_floor(grid_str.len(), row_length);
        println!("Row length: {:?}, num rows: {:?}", row_length, num_rows);
        CharGrid {
            0: Array::from_shape_vec(
                (num_rows, row_length),
                grid_str.chars().filter(|&c| c != '\n').collect(),
            )
            .unwrap(),
        }
    }

    fn count_xmases(&self) -> u32 {
        let mut result = 0;
        for row in 0..self.0.shape()[0] {
            for col in 0..self.0.shape()[1] {
                result += self.count_xmases_at([row, col])
            }
        }
        result
    }

    fn count_xmases_at(&self, pos: [usize; 2]) -> u32 {
        iproduct!(&DIRS, &DIRS)
            .skip(1)
            //.clone()
            .map(|(&i, &j)| self.count_xmases_at_towards(pos, [i, j]))
            .sum()
    }

    fn count_xmases_at_towards(&self, mut pos: [usize; 2], dir: [isize; 2]) -> u32 {
        let max_row = (self.0.shape()[0] - 1) as isize;
        let max_col = (self.0.shape()[1] - 1) as isize;
        let mut next_row = pos[0] as isize;
        let mut next_col = pos[1] as isize;
        for c in XMAS {
            if next_row < 0 || next_row > max_row || next_col < 0 || next_col > max_col {
                return 0;
            }
            pos = [next_row as usize, next_col as usize];
            if self.0[pos] != c {
                return 0;
            }
            next_row = pos[0] as isize + dir[0];
            next_col = pos[1] as isize + dir[1];
        }
        1
    }

    fn count_crossmases(&self) -> u32 {
        let mut result = 0;
        for row in 1..self.0.shape()[0] - 1 {
            for col in 1..self.0.shape()[1] - 1 {
                result += self.count_crossmases_at([row, col])
            }
        }
        result
    }

    fn count_crossmases_at(&self, pos: [usize; 2]) -> u32 {
        if self.0[pos] == 'A' {
            let up_l = self.0[[pos[0] - 1, pos[1] - 1]];
            let up_r = self.0[[pos[0] - 1, pos[1] + 1]];
            let dn_l = self.0[[pos[0] + 1, pos[1] - 1]];
            let dn_r = self.0[[pos[0] + 1, pos[1] + 1]];
            if (up_l == 'M' && dn_r == 'S' || up_l == 'S' && dn_r == 'M')
                && (dn_l == 'M' && up_r == 'S' || dn_l == 'S' && up_r == 'M')
            {
                return 1;
            }
        }
        0
    }
}

pub fn process_input(input: &str) -> String {
    let xmas_grid: CharGrid = CharGrid::from_str(input.trim());
    //println!("XMAS grid: {:?}", xmas_grid);
    // sum up the xmases found on every possible starting position:
    let num_xmases = xmas_grid.count_xmases();
    let num_crossmases = xmas_grid.count_crossmases();
    format!(
        "num of xmas's: {}\nnum of x-mas's: {}",
        num_xmases, num_crossmases
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
