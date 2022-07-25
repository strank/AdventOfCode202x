//! https://adventofcode.com/2021/day/15
//! Pathfinding with cost on a grid, good excuse to use astar
//! First try the pathfinding crate, then possibly implement astar from scratch
//!
//! ```
//! use advent_of_code_202x::generated::year2021day15::run;
//! assert!(run().contains("Lowest total risk path: 685\ntiled 5 times: 2995"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day15::run_example;
/// assert!(run_example().contains("Lowest total risk path: 40\ntiled 5 times: 315"));
/// ```
const EXAMPLE_INPUT: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"; // 40

use pathfinding::prelude::astar; // TODO: implement here as an exercise

/// need positions on a grid
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, grid: &Grid) -> Vec<(Pos, u32)> {
        grid.neighbours_of(self)
            .into_iter()
            .map(|p| {
                let cost = grid.value_at(&p) as u32;
                (p, cost)
            })
            .collect()
    }
}

#[derive(Debug)]
struct Grid {
    vals: Vec<Vec<u8>>,
    len_x: usize,
    len_y: usize,
    repeats: usize,
}

impl Grid {
    /// Return a Grid based on string input
    fn new(puzzle_input: &'static str) -> Self {
        let vals: Vec<Vec<u8>> = puzzle_input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect();
        let len_x = vals.len() as usize;
        let len_y = vals.get(0).unwrap().len() as usize;
        Self {
            vals,
            len_x,
            len_y,
            repeats: 1,
        }
    }

    fn value_at(&self, pos: &Pos) -> u8 {
        let (x_repeat, x_index) = (pos.0 / self.len_x, pos.0 % self.len_x);
        let (y_repeat, y_index) = (pos.1 / self.len_y, pos.1 % self.len_y);
        let base_val = self.vals[x_index][y_index];
        ((base_val - 1 + x_repeat as u8 + y_repeat as u8) % 9) + 1
    }

    fn neighbours_of(&self, pos: &Pos) -> Vec<Pos> {
        let &Pos(x, y) = pos;
        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push(Pos(x - 1, y))
        }
        if y > 0 {
            result.push(Pos(x, y - 1))
        }
        if x < (self.len_x * self.repeats) - 1 {
            result.push(Pos(x + 1, y))
        }
        if y < (self.len_y * self.repeats) - 1 {
            result.push(Pos(x, y + 1))
        }
        result
    }

    #[allow(dead_code)]
    fn plot(&self) {
        for x in 0..(self.len_x * self.repeats) {
            for y in 0..(self.len_y * self.repeats) {
                print!("{}", self.value_at(&Pos(x, y)));
            }
            println!();
        }
        println!();
    }
}

pub fn process_input(input: &'static str) -> String {
    let mut grid = Grid::new(input.trim());
    //grid.plot();
    let goal_1 = Pos(grid.len_x - 1, grid.len_y - 1);
    let result_1 = astar(
        &Pos(0, 0),
        |p| p.successors(&grid),
        |p| p.distance(&goal_1),
        |p| *p == goal_1,
    )
    .unwrap();
    grid.repeats = 5;
    //grid.plot();
    let goal_5 = Pos(grid.len_x * grid.repeats - 1, grid.len_y * grid.repeats - 1);
    let result_5 = astar(
        &Pos(0, 0),
        |p| p.successors(&grid),
        |p| p.distance(&goal_5),
        |p| *p == goal_5,
    )
    .unwrap();
    format!(
        "Lowest total risk path: {}\ntiled 5 times: {:?}\n",
        result_1.1, result_5.1
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
