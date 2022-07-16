//! https://adventofcode.com/2021/day/13
//! Fold transparent paper checking for overlapping dots given as coordinates
//!
//! ```
//! use advent_of_code_202x::generated::year2021day13::run;
//! assert!(run().contains("num of dots after one fold: 669\nCODE\n\n#..#.####.####.####..##..#..#..##....##\n"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day13::run_example;
/// assert!(run_example().contains("num of dots after one fold: 17\nCODE\n\n#####\n"));
/// ```
const EXAMPLE_INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"; // 17 dots after first fold

use std::collections::HashSet;

/// need to deal with coordinates:
type Coord = [usize; 2];
/// instructions for folding, axis and value:
type FoldInstruction = (&'static str, usize);

/// Return a list of coordinates
fn parse_coords(puzzle_input: &'static str) -> HashSet<Coord> {
    puzzle_input
        .split('\n')
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()]
        })
        .collect()
}

/// Return a list of fold instructions
fn parse_instructions(puzzle_input: &'static str) -> Vec<FoldInstruction> {
    puzzle_input
        .split('\n')
        .map(|line| {
            let (axis, value) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            (axis, value.parse::<usize>().unwrap())
        })
        .collect()
}

fn fold_along(mut coords: HashSet<Coord>, axis: &str, value: usize) -> HashSet<Coord> {
    let axis_index = match axis {
        "x" => 0,
        "y" => 1,
        _ => panic!("axis that is not x or y"),
    };
    coords
        .drain()
        .map(|mut coord| {
            if coord[axis_index] > value {
                coord[axis_index] = 2 * value - coord[axis_index];
            }
            coord
        })
        .collect()
}

fn plot(coords: HashSet<Coord>) -> String {
    let mut lines = vec![];
    for coord in coords {
        while coord[1] >= lines.len() {
            lines.push(String::from("."));
        }
        let line = lines.get_mut(coord[1]).unwrap();
        if coord[0] >= line.len() {
            line.extend(std::iter::repeat(".").take(coord[0] + 1 - line.len()));
        }
        line.replace_range(coord[0]..coord[0] + 1, "#");
    }
    lines.join("\n")
}

pub fn process_input(input: &'static str) -> String {
    let (coords_str, instr_str) = input.trim().split_once("\n\n").unwrap();
    let mut coords = parse_coords(coords_str);
    let instructions = parse_instructions(instr_str);
    let mut instructions = instructions.iter();
    if let Some((axis, value)) = instructions.next() {
        coords = fold_along(coords, axis, *value)
    }
    let once_len = coords.len();
    for (axis, value) in instructions {
        coords = fold_along(coords, axis, *value);
    }
    let plotted_coords = plot(coords);
    format!(
        "num of dots after one fold: {}\nCODE\n\n{}\n",
        once_len, plotted_coords
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
