//! https://adventofcode.com/2020/day/18
//! parse numeric expression with parenthesis, plus and times.
//! the operators have the same precedence so evaluate left to right!
//!
//! ```
//! use advent_of_code_202x::generated::year2020day18::run;
//! assert!(run().contains("Sum part 1: 45283905029161\nSum part 2: 216975281211165"));
//! ```

const INPUT: &str = include_str!("input");

/// 2 * 3 + (4 * 5) becomes 26.
/// 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
/// 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
/// ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
/// ```
/// use advent_of_code_202x::generated::year2020day18::run_example;
/// assert!(run_example().contains("Sum part 1: 26351\nSum part 2: 693907"));
/// ```
const EXAMPLE_INPUT: &str = "
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
(2 * (2 * 3)) + 4
";

use std::iter::Peekable;

fn parse_expr(token_stream: &mut impl Iterator<Item = char>) -> Vec<char> {
    let mut polish: Vec<char> = Vec::new();
    while let Some(token) = token_stream.next() {
        match token {
            '+' | '*' => polish.insert(0, token),
            '(' => polish.extend(parse_expr(token_stream)),
            ')' => return polish,
            _ => polish.push(token),
        }
    }
    polish
}

/// parse with reversed operator precedence from normal:
fn parse_expr_op_prec(token_stream: &mut Peekable<impl Iterator<Item = char>>) -> Vec<char> {
    parse_expr_lhs(parse_primary(token_stream), token_stream)
}

/// parse primary expr, either a number or a parenthesized expr
fn parse_primary(token_stream: &mut Peekable<impl Iterator<Item = char>>) -> Vec<char> {
    if let Some(token) = token_stream.next() {
        //println!("primary {}", token);
        match token {
            '(' => {
                let parsed = parse_expr_lhs(parse_primary(token_stream), token_stream);
                if let Some(')') = token_stream.next() {
                    return parsed;
                } else {
                    panic!("Parenthesis was not balanced!")
                }
            }
            '+' | '*' | ')' => panic!(
                "Operator or closing paren at beginning of primary expr should never happen!"
            ),
            _ => return vec![token],
        }
    }
    panic!("No primary expression to parse!");
}

/// parse an expr with an operator, lhs already parsed, continue while multiplicative
fn parse_expr_lhs(
    mut lhs: Vec<char>, //min_precedence,
    token_stream: &mut Peekable<impl Iterator<Item = char>>,
) -> Vec<char> {
    let mut rhs: Vec<char>;
    while let Some(&token) = token_stream.peek() {
        if token == ')' {
            return lhs;
        }
        token_stream.next(); // consume peeked token
                             //println!("operator {} (lhs: {:?})", token, lhs);
        match token {
            '+' => rhs = parse_primary(token_stream),
            '*' => rhs = parse_expr_lhs(parse_primary(token_stream), token_stream),
            '(' | ')' => panic!("Opening parenthesis inside expr should never happen!"),
            _ => panic!("Number inside expr should never happen!"),
        }
        lhs.insert(0, token);
        lhs.extend(rhs);
        //println!("lhs now: {:?}", lhs);
    }
    lhs
}

#[allow(clippy::ptr_arg)]
fn eval_expr(p_expr: &Vec<char>) -> u64 {
    let mut operand_stack: Vec<u64> = Vec::new();
    for &op in p_expr.iter().rev() {
        match op {
            '+' => {
                let op1 = operand_stack.pop().unwrap();
                let op2 = operand_stack.pop().unwrap();
                operand_stack.push(op1 + op2)
            }
            '*' => {
                let op1 = operand_stack.pop().unwrap();
                let op2 = operand_stack.pop().unwrap();
                operand_stack.push(op1 * op2)
            }
            _ => operand_stack.push(op.to_digit(10).unwrap().into()),
        }
    }
    operand_stack.pop().unwrap()
}

pub fn process_input(input: &str) -> String {
    let input = input.trim().split('\n');
    //println!("input:\n{:?}\n", input);
    let token_streams = input.map(|a| a.chars().filter(|&c| c != ' '));
    // TODO: there should be a nicer way that doesn't create any intermediate Vec
    // but just passes through the initial chars in a different order...
    let mut parsed_exprs = Vec::new();
    let mut parsed_exprs2 = Vec::new();
    for ts in token_streams.into_iter() {
        parsed_exprs.push(parse_expr(&mut ts.clone()));
        parsed_exprs2.push(parse_expr_op_prec(&mut ts.into_iter().peekable()));
        //        parsed_exprs2.push(parse_primary(&mut ts.into_iter().peekable()));
    }
    let evaluated_sum_1: u64 = parsed_exprs.iter().map(eval_expr).sum();
    //println!("Parsed: {:?}", parsed_exprs2);
    let evaluated_sum_2: u64 = parsed_exprs2.iter().map(eval_expr).sum();
    format!(
        "Sum part 1: {}\nSum part 2: {}",
        evaluated_sum_1, evaluated_sum_2,
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
