//! https://adventofcode.com/2021/day/14
//! Grammar for extending string
//! initial naive approach: store the whole string and apply rules, then count.
//! this runs out of memory for part 2 (40 vs 10 iterations)
//! better: only track something that doesn't grow: counts of pairs in the string, there is a finite number of pairs!
//! and interpret the rules as producing two new pairs from an input pair, keep track of char counts along the way
//!
//! ```
//! use advent_of_code_202x::generated::year2021day14::run;
//! assert!(run().contains("Difference of most common and least common element after 10 rounds: 2170\nafter 40 rounds: 2422444761283"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day14::run_example;
/// assert!(run_example().contains("Difference of most common and least common element after 10 rounds: 1588\nafter 40 rounds: 2188189693529"));
/// ```
const EXAMPLE_INPUT: &str = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"; // most common (B, 1749) minus least common element (H, 161) produces 1749 - 161 = 1588

use std::collections::HashMap;

/// need to deal with pairs of chars, and tracking counts of pairs and individual chars:
/// assume that chars are always upppercase characters, so we can use fixed-size arrays instead of hashmaps
const MAX_CHAR_INDEX: usize = 26;
const A_INDEX: usize = 'A' as usize;

fn char_to_index(c: char) -> usize {
    c as usize - A_INDEX
}

#[derive(Debug, Default, Clone)]
struct Polymer {
    pair_counts: [[u64; MAX_CHAR_INDEX]; MAX_CHAR_INDEX],
    char_counts: [u64; MAX_CHAR_INDEX],
}

/// rules for extending: a pair is replaced by two new pairs and increase the count of one char's index
type Pair = (usize, usize);
type Rules = HashMap<Pair, (Pair, Pair, usize)>;

impl Polymer {
    /// Create a new polymer from a string specification
    fn new(polymer_str: &str) -> Self {
        let mut polymer = Self::default();
        for c in polymer_str.chars() {
            polymer.char_counts[char_to_index(c)] += 1;
        }
        for pair in polymer_str.chars().collect::<Vec<char>>()[..].windows(2) {
            let [left, right]: [char; 2] = pair.try_into().unwrap();
            polymer.pair_counts[char_to_index(left)][char_to_index(right)] += 1;
        }
        polymer
    }

    /// apply the rules once, returning a new instance of Polymer
    fn apply_rules(self, rules: &Rules) -> Self {
        let mut new_polymer = self.clone();
        for (pair, (left, right, new_char)) in rules {
            let pair_count = self.pair_counts[pair.0][pair.1];
            if pair_count > 0 {
                // fill new_polymer with updated values and increase the char count
                new_polymer.pair_counts[pair.0][pair.1] -= pair_count;
                new_polymer.pair_counts[left.0][left.1] += pair_count;
                new_polymer.pair_counts[right.0][right.1] += pair_count;
                new_polymer.char_counts[*new_char] += pair_count;
            }
        }
        new_polymer
    }

    /// Return the max-count minus the min-count (that is non-zero)
    fn count_diff_most_least(&self) -> u64 {
        let max = self.char_counts.iter().max().unwrap();
        let min = self
            .char_counts
            .iter()
            .filter(|ele| **ele > 0)
            .min()
            .unwrap();
        max - min
    }
}

/// Return a Rules hashmap, with chars already transformed to indices
fn parse_rules(puzzle_input: &'static str) -> Rules {
    let mut result = Rules::new();
    for (a, b, c) in puzzle_input.split('\n').map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (mut left, mut right) = (left.chars(), right.chars());
        (
            char_to_index(left.next().unwrap()),
            char_to_index(left.next().unwrap()),
            char_to_index(right.next().unwrap()),
        )
    }) {
        result.entry((a, b)).or_insert(((a, c), (c, b), c));
    }
    result
}

pub fn process_input(input: &'static str) -> String {
    let (polymer_str, rules_str) = input.trim().split_once("\n\n").unwrap();
    let mut polymer: Polymer = Polymer::new(polymer_str);
    let rules = parse_rules(rules_str);
    for _ in 0..10 {
        polymer = polymer.apply_rules(&rules);
    }
    let count_diff_10 = polymer.count_diff_most_least();
    for _ in 0..30 {
        polymer = polymer.apply_rules(&rules);
    }
    let count_diff_40 = polymer.count_diff_most_least();
    format!(
        "Difference of most common and least common element after 10 rounds: {}\nafter 40 rounds: {:?}\n",
        count_diff_10, count_diff_40
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
