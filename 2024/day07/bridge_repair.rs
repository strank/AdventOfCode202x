//! https://adventofcode.com/2024/day/7
//! evaluate equations with missing operators
//! part 2:
//!
//! ```
//! use advent_of_code_202x::generated::year2024day07::run;
//! assert!(run().contains(
//!     "sum calibrated: 1289579105366\nsum extended: ???"));
//! ```

use itertools::{repeat_n, Itertools};

const INPUT: &str = include_str!("input");

/// example answer 41, and ??
/// ```
/// use advent_of_code_202x::generated::year2024day07::run_example;
/// assert!(run_example().contains(
///     "sum calibrated: 3749\nsum extended: "));
/// ```
const EXAMPLE_INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[derive(Debug, Clone)]
enum OP {
    ADD,
    MULTIPLY,
    CONCAT,
}

const ELE_OPS: [OP; 2] = [OP::ADD, OP::MULTIPLY];

const MORE_OPS: [OP; 3] = [OP::ADD, OP::MULTIPLY, OP::CONCAT];

impl OP {
    fn evaluate(&self, left: u64, right: u64) -> u64 {
        match self {
            OP::ADD => left + right,
            OP::MULTIPLY => left * right,
            OP::CONCAT => left * 10u64.pow(right.ilog10() + 1) + right,
        }
    }
}

fn str_to_u64(a_str: &str) -> u64 {
    a_str.parse::<u64>().unwrap()
}

#[derive(Debug, Clone)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn from_str(eq_str: &str) -> Self {
        let (val, nums) = eq_str.split(": ").next_tuple().unwrap();
        Equation {
            value: str_to_u64(val),
            numbers: nums.split(" ").map(str_to_u64).collect(),
        }
    }

    fn possible_total(&self, use_ops: &[OP]) -> u64 {
        // try all possible combinations of operators
        // if any returns the value, return that, otherwise 0
        let op_len = self.numbers.len() - 1;
        for ops in repeat_n(use_ops, op_len).multi_cartesian_product() {
            let mut nums_iter = self.numbers.clone().into_iter();
            // take the first element of nubmers as starting value
            let mut current_value: u64 = nums_iter.next().unwrap();
            // loop throught the operators zipped with the rest of numbers
            // if the value goes over value or ends up below it, continue loop
            // if we hit the target, break out of loop returning value
            for (op, num) in ops.iter().zip(nums_iter) {
                //println!("{} {:?} {} =", current_value, op, num);
                current_value = op.evaluate(current_value, num);
                if current_value > self.value {
                    break;
                }
            }
            if current_value == self.value {
                //println!("FOUND TOTAL {}", current_value);
                return current_value;
            }
        }
        //println!("failed TOTAL {}", self.value);
        0
    }
}

pub fn process_input(input: &str) -> String {
    let equations: Vec<_> = input.trim().split('\n').map(Equation::from_str).collect();
    //println!("equations: {:?}", equations);
    // filter for equations tha could be correct and sum the test values:
    let sum_calibrated: u64 = equations.iter().map(|eq| eq.possible_total(&ELE_OPS)).sum();
    let sum_extended: u64 = equations
        .iter()
        .map(|eq| eq.possible_total(&MORE_OPS))
        .sum();
    format!(
        "sum calibrated: {}\nsum extended: {}",
        sum_calibrated, sum_extended
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
