//! https://adventofcode.com/2021/day/8
//! identify seven-segment display numbers based on randomly shuffled segment labels
//!
//! ```
//! use advent_of_code_202x::generated::year2021day08::run;
//! assert!(run().contains("num of unique len outputs: 375\nsum of outputs: 1019355"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 26 instances of digits 1,4,7,8 in the output part
/// example answer 61229 for summing up 4-digit outputs
/// ```
/// use advent_of_code_202x::generated::year2021day08::run_example;
/// assert!(run_example().contains("num of unique len outputs: 26\nsum of outputs: 61229"));
/// ```
const EXAMPLE_INPUT: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

use std::collections::{BTreeSet, HashMap};

/// pattern of segments is a set of segment labels (char) unordered. Set operations are helpful:
type SegmentPattern = BTreeSet<char>;
/// an entry has the 10 unique segment patterns observed, and 4 specific output values seen:
type DisplayEntry = ([SegmentPattern; 10], [SegmentPattern; 4]);
/// need to find a mapping from pattern to digit:
type DigitMapping = HashMap<SegmentPattern, usize>;

fn count_unique_len_outputs(entries: &[DisplayEntry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .1
                .iter()
                .filter(|patt| [2, 3, 4, 7].contains(&patt.len()))
                .count()
        })
        .sum()
}

fn extract_unique_digit(
    digit: usize,
    it: &mut dyn Iterator<Item = &SegmentPattern>,
    mapping: &mut DigitMapping,
    digits: &mut [Option<SegmentPattern>; 10],
) {
    let dig_patt = it.next().unwrap();
    mapping.insert(dig_patt.clone(), digit);
    digits[digit] = Some(dig_patt.clone());
    assert_eq!(None, it.next());
}

fn create_digit_mapping(patterns: &[SegmentPattern; 10]) -> DigitMapping {
    let mut mapping = DigitMapping::new();
    let mut digits: [Option<SegmentPattern>; 10] = Default::default();
    // four of the digits use a unique number of segments:
    for (digit, num_segments) in [(1, 2), (7, 3), (4, 4), (8, 7)] {
        let mut dig_iter = patterns.iter().filter(|pat| pat.len() == num_segments);
        extract_unique_digit(digit, &mut dig_iter, &mut mapping, &mut digits);
    }
    // digits 6, 9 and 0 have 6 segments:
    // 6 is missing a segment that 1 has, even more: 1 is not a subset
    let mut dig_iter = patterns
        .iter()
        .filter(|pat| pat.len() == 6 && !digits[1].as_ref().unwrap().is_subset(pat))
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(6, &mut dig_iter, &mut mapping, &mut digits);
    // 9 has all segments that 4 has
    let mut dig_iter = patterns
        .iter()
        .filter(|pat| pat.len() == 6 && digits[4].as_ref().unwrap().is_subset(pat))
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(9, &mut dig_iter, &mut mapping, &mut digits);
    // 0 is the other one
    let mut dig_iter = patterns
        .iter()
        .filter(|&pat| {
            pat.len() == 6
                && digits[6].as_ref().unwrap() != pat
                && digits[9].as_ref().unwrap() != pat
        })
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(0, &mut dig_iter, &mut mapping, &mut digits);
    // digits 2, 3 and 5 have 5 segments:
    // 3 has 1 as a subset
    let mut dig_iter = patterns
        .iter()
        .filter(|pat| pat.len() == 5 && digits[1].as_ref().unwrap().is_subset(pat))
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(3, &mut dig_iter, &mut mapping, &mut digits);
    // 2 intersects uniquely with 4 to leave 2 segments
    let mut dig_iter = patterns
        .iter()
        .filter(|pat| pat.len() == 5 && digits[4].as_ref().unwrap().intersection(pat).count() == 2)
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(2, &mut dig_iter, &mut mapping, &mut digits);
    // 5 is the other one
    let mut dig_iter = patterns
        .iter()
        .filter(|&pat| {
            pat.len() == 5
                && digits[2].as_ref().unwrap() != pat
                && digits[3].as_ref().unwrap() != pat
        })
        .collect::<Vec<_>>() // necessary to avoid double borrowing digits!
        .into_iter();
    extract_unique_digit(5, &mut dig_iter, &mut mapping, &mut digits);
    // done!
    mapping
}

fn find_output_value(outputs: &[SegmentPattern; 4], digit_mapping: &DigitMapping) -> usize {
    1000 * digit_mapping.get(&outputs[0]).unwrap()
        + 100 * digit_mapping.get(&outputs[1]).unwrap()
        + 10 * digit_mapping.get(&outputs[2]).unwrap()
        + digit_mapping.get(&outputs[3]).unwrap()
}

fn sum_outputs(entries: &[DisplayEntry]) -> usize {
    let mut sum = 0;
    for entry in entries {
        let (patterns, outputs) = entry;
        // create a mapping from set of segments to digit based on the patterns:
        let digit_mapping = create_digit_mapping(patterns);
        // apply map to output values, add to sum:
        sum += find_output_value(outputs, &digit_mapping);
    }
    sum
}

fn str_to_display_entry(a_str: &str) -> DisplayEntry {
    let (ten_patterns, four_outputs) = a_str.split_once(" | ").unwrap();
    let make_display_entries = |patterns: &str| {
        patterns
            .split_ascii_whitespace()
            .map(|pt| pt.chars().collect())
            .collect::<Vec<_>>()
    };
    (
        make_display_entries(ten_patterns).try_into().unwrap(),
        make_display_entries(four_outputs).try_into().unwrap(),
    )
}

pub fn process_input(input: &str) -> String {
    let input = input.trim().split('\n');
    let displays: Vec<DisplayEntry> = input.map(str_to_display_entry).collect();
    //println!("displays:\n{:?}", displays);
    let num_of_unique_outputs = count_unique_len_outputs(&displays);
    let sum_of_outputs = sum_outputs(&displays);
    format!(
        "num of unique len outputs: {}\nsum of outputs: {}",
        num_of_unique_outputs, sum_of_outputs
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
