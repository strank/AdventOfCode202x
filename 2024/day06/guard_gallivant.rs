//! https://adventofcode.com/2024/day/6
//! moving guard on a grid
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day06::run;
//! assert!(run().contains(
//!     "squares visited: 5131\nblock options: 1784"));
//! ```

use ndarray::prelude::*;
use num::integer::div_floor;
use std::collections::HashSet;

const INPUT: &str = include_str!("input");

/// example answer 41, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day06::run_example;
/// assert!(run_example().contains(
///     "squares visited: 41\nblock options: 6"));
/// ```
const EXAMPLE_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const DOT: char = '.';
const BLOCKED: char = '#';
const VISITED: char = 'X';

#[derive(Debug, Clone)]
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

    fn walk_guard(&mut self) -> u32 {
        // find starting position/direction and change it to X:
        let (mut guard_pos, &guard_dir) = self
            .0
            .indexed_iter()
            .find(|(_, &ele)| ele != DOT && ele != BLOCKED)
            .unwrap();
        let mut guard_dir = guard_dir;
        self.0[guard_pos] = VISITED;
        let mut changed: u32 = 1;
        // loop walking the guard until they exit the grid
        while let Some(next_pos) = self.get_pos_in_dir(guard_pos, guard_dir) {
            match self.0[next_pos] {
                DOT => {
                    guard_pos = next_pos;
                    self.0[guard_pos] = VISITED;
                    changed += 1;
                }
                VISITED => {
                    guard_pos = next_pos; // already visited, just advance
                }
                BLOCKED => {
                    // don't advance, change direction towards the right:
                    guard_dir = self.turn_right(guard_dir);
                }
                _ => {
                    panic!("Found an unknown cell!")
                }
            }
        }
        changed
    }

    fn turn_right(&self, dir: char) -> char {
        match dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!("Found an unknown direction!"),
        }
    }

    fn block_guard(&mut self) -> usize {
        // find starting position/direction:
        let (start_pos, &guard_dir) = self
            .0
            .indexed_iter()
            .find(|(_, &ele)| ele != DOT && ele != BLOCKED)
            .unwrap();
        let (mut guard_pos, mut guard_dir) = (start_pos, guard_dir);
        let mut spots_checked = HashSet::new();
        let mut obstruction_candidates = HashSet::new();
        // loop walking the guard until they exit the grid
        // check every next_pos for possible blocking
        // by virtually walking right, checking for a previously trodden path:
        while let Some(next_pos) = self.get_pos_in_dir(guard_pos, guard_dir) {
            match self.0[next_pos] {
                DOT => {
                    // check if putting a blocker here would lead to a loop
                    if spots_checked.insert((next_pos, guard_dir)) {
                        if !obstruction_candidates.contains(&next_pos) {
                            self.0[next_pos] = BLOCKED;
                            if self.check_loop(
                                guard_pos,
                                self.turn_right(guard_dir),
                                spots_checked.clone(),
                            ) {
                                obstruction_candidates.insert(next_pos);
                            };
                        };
                    };
                    guard_pos = next_pos;
                    self.0[guard_pos] = guard_dir;
                }
                UP | DOWN | LEFT | RIGHT => {
                    // we walked through that square earlier and must have checked it
                    spots_checked.insert((next_pos, guard_dir));
                    guard_pos = next_pos;
                    self.0[guard_pos] = guard_dir;
                }
                BLOCKED => {
                    // don't advance, change direction towards the right:
                    guard_dir = self.turn_right(guard_dir);
                }
                _ => {
                    panic!("Found an unknown cell!")
                }
            }
        }
        obstruction_candidates.remove(&start_pos);
        obstruction_candidates.len()
    }

    fn check_loop(
        &self,
        mut pos: (usize, usize),
        mut dir: char,
        mut checked_squares: HashSet<((usize, usize), char)>,
    ) -> bool {
        if self.0[pos] == dir {
            return true;
        }
        checked_squares.insert((pos, dir));
        while let Some(next_pos) = self.get_pos_in_dir(pos, dir) {
            if checked_squares.contains(&(next_pos, dir)) {
                return true;
            }
            match self.0[next_pos] {
                pos_dir if pos_dir == dir => {
                    return true; // same direction as we are searching, blocker would have worked
                }
                DOT | UP | DOWN | RIGHT | LEFT => {
                    pos = next_pos; // no match, just advance
                    checked_squares.insert((pos, dir));
                }
                BLOCKED => {
                    // continue checking for loop
                    dir = self.turn_right(dir);
                    if self.0[pos] == dir {
                        return true;
                    }
                    checked_squares.insert((pos, dir));
                }
                _ => {
                    panic!("Found an unknown cell!")
                }
            }
        }
        false
    }

    fn get_pos_in_dir(&self, (pos_r, pos_c): (usize, usize), dir: char) -> Option<(usize, usize)> {
        match dir {
            UP => {
                if pos_r == 0 {
                    None
                } else {
                    Some((pos_r - 1, pos_c))
                }
            }
            DOWN => {
                if pos_r >= self.0.dim().0 - 1 {
                    None
                } else {
                    Some((pos_r + 1, pos_c))
                }
            }
            LEFT => {
                if pos_c == 0 {
                    None
                } else {
                    Some((pos_r, pos_c - 1))
                }
            }
            RIGHT => {
                if pos_c >= self.0.dim().1 - 1 {
                    None
                } else {
                    Some((pos_r, pos_c + 1))
                }
            }
            _ => {
                panic!("Found an unknown direction!")
            }
        }
    }
}

pub fn process_input(input: &str) -> String {
    let mut lab_grid: CharGrid = CharGrid::from_str(input.trim());
    //println!("lab grid: {:?}", lab_grid);
    // walk the guard changing all visited positions to X, returning the count of changes made:
    let num_changes = lab_grid.clone().walk_guard();
    let num_blockers = lab_grid.block_guard();
    //println!("lab grid: {:?}", lab_grid);
    format!(
        "squares visited: {}\nblock options: {}",
        num_changes, num_blockers
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
