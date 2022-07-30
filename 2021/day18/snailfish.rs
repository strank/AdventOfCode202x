//! https://adventofcode.com/2021/day/18
//! Nested pairs of numbers, need to be simplified ("exploded" if nested too deep or "split" if too large)
//! recursive parsing, depth-first mostly, maybe with a little clever backtracking?
//!
//! ```
//! use advent_of_code_202x::generated::year2021day18::run;
//! assert!(run().contains("Sum magnitude: 3884\nMax Magnitude: 4595"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day18::run_example;
/// assert!(run_example().contains("Sum magnitude: 4140\nMax Magnitude: 3993"));
/// ```
const EXAMPLE_INPUT: &str = "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"; // final sum: [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]] magnitude 4140

use std::{fmt, ops::Add, str::Chars};

use itertools::Itertools;

/// We need to deal with snailfish numbers: nested pairs of numbers, so we need a recursive type
#[derive(Debug, Clone)]
enum SnailfishNumber {
    Number(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

/// Display SnailfishNumbers in the same way as the input
impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailfishNumber::Number(num) => write!(f, "{}", num),
            SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

/// Addition for SnailfishNumbers is forming a new pair and then reducing it
impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other)).reduce()
    }
}

/// Possible results passed back up when dealing with an explosion
enum Explosion {
    No,
    Done,
    Both(u8, u8),
    Left(u8),
    Right(u8),
}

impl SnailfishNumber {
    /// Reduce the SnailfishNumber: Apply rules repeatedly, always a before b
    /// (a) If any pair is nested inside four pairs, the leftmost such pair explodes: (add left value to next on the left, right to the right)
    /// (b) If any regular number is 10 or greater, the leftmost such regular number splits. (new pair with [floor(num/2), ceil(num/2)])
    fn reduce(mut self) -> Self {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
        self
    }

    /// Explode, return whether an explosion occurred
    fn explode(&mut self) -> bool {
        !matches!(self.explode_depth(0u8), Explosion::No)
    }

    /// Explode recursively as if at depth depth, return Explosion type
    fn explode_depth(&mut self, depth: u8) -> Explosion {
        if depth < 4u8 {
            if let SnailfishNumber::Pair(left, right) = self {
                match left.explode_depth(depth + 1) {
                    Explosion::Both(a, b) => {
                        // explosion just happened, send a part to the right and one up
                        right.add_to_leftmost(b);
                        Explosion::Left(a)
                    }
                    Explosion::Right(b) => {
                        // explosion leftover that needs to be added to the leftmost number on the right
                        right.add_to_leftmost(b);
                        Explosion::Done
                    }
                    Explosion::No => {
                        // need to try exploding the right side instead:
                        match right.explode_depth(depth + 1) {
                            Explosion::Both(a, b) => {
                                // explosion just happened, send a part to the right and one up
                                left.add_to_rightmost(a);
                                Explosion::Right(b)
                            }
                            Explosion::Left(a) => {
                                // explosion leftover that needs to be added to the leftmost number on the right
                                left.add_to_rightmost(a);
                                Explosion::Done
                            }
                            other => other, // either no/done explosion, or Explosion::Right that needs to be passed up
                        }
                    }
                    other => other, // either explosion done, or Explosion::Left that needs to be passed up
                }
            } else {
                Explosion::No
            }
        } else {
            match self {
                SnailfishNumber::Number(_) => Explosion::No,
                SnailfishNumber::Pair(left, right) => {
                    // need to pass it up, then replace with 0
                    match (&**left, &**right) {
                        (SnailfishNumber::Number(left_num), SnailfishNumber::Number(right_num)) => {
                            let explosion = Explosion::Both(*left_num, *right_num);
                            *self = SnailfishNumber::Number(0u8);
                            explosion
                        }
                        _ => {
                            panic!("SnailfishNumber nested deeper than 4 levels!")
                        }
                    }
                }
            }
        }
    }

    /// Add u8 to leftmost element in the number tree
    fn add_to_leftmost(&mut self, summand: u8) {
        match self {
            SnailfishNumber::Number(num) => {
                *num += summand;
            }
            SnailfishNumber::Pair(left, _) => left.add_to_leftmost(summand),
        }
    }

    /// Add u8 to rightmost element in the number tree
    fn add_to_rightmost(&mut self, summand: u8) {
        match self {
            SnailfishNumber::Number(num) => {
                *num += summand;
            }
            SnailfishNumber::Pair(_, right) => right.add_to_rightmost(summand),
        }
    }

    /// Split, return true if a split occurred
    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::Number(num) => {
                if *num > 9u8 {
                    *self = SnailfishNumber::Pair(
                        Box::new(SnailfishNumber::Number(*num / 2u8)),
                        Box::new(SnailfishNumber::Number(*num / 2u8 + *num % 2u8)),
                    );
                    true
                } else {
                    false
                }
            }
            SnailfishNumber::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Number(num) => *num as u32,
            SnailfishNumber::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

/// Return the snailfishnumber from a mutable iterator of chars advancing as necessary
fn parse_snailfish_number_from_iter(char_iter: &mut Chars) -> SnailfishNumber {
    let next = char_iter.next().unwrap();
    if next == '[' {
        let left = parse_snailfish_number_from_iter(char_iter);
        assert!(char_iter.next().unwrap() == ',');
        let right = parse_snailfish_number_from_iter(char_iter);
        assert!(char_iter.next().unwrap() == ']');
        SnailfishNumber::Pair(Box::new(left), Box::new(right))
    } else {
        // must be a digit!
        SnailfishNumber::Number(next.to_digit(10).expect("Digit expected at this position!") as u8)
    }
}

/// Return the snailfishnumber represented in the string
fn parse_snailfish_number(puzzle_input: &'static str) -> SnailfishNumber {
    parse_snailfish_number_from_iter(&mut puzzle_input.chars())
}

pub fn process_input(input: &'static str) -> String {
    let snailfish_numbers = input.trim().split('\n').map(parse_snailfish_number);
    let result = snailfish_numbers.clone().reduce(|a, b| a + b).unwrap();
    let perms = snailfish_numbers.permutations(2);
    let max_magnitude = perms
        .map(|mut ele_vec| (ele_vec.pop().unwrap() + ele_vec.pop().unwrap()).magnitude())
        .max()
        .unwrap();
    format!(
        "Sum magnitude: {}\nMax Magnitude: {}\n",
        result.magnitude(),
        max_magnitude
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
