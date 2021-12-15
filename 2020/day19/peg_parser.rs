/// https://adventofcode.com/2020/day/19
/// parse ab strings based on a grammar that only uses the sequence and choice operators.
/// This is probably meant to be done with just recursive descent parsing,
/// but I would like to use it to implement a PEG parser with memoization,
/// i.e. a packrat parser.
/// (Maybe later also implement left-recursion like in this paper:
/// https://web.cs.ucla.edu/~todd/research/pepm08.pdf
/// this is also roughtly what's implemented in python's new peg parser.)

const INPUT: &str = include_str!("input");

const EXAMPLE_INPUT: &str = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"; // --> answer 2

const _EXAMPLE_INPUT2: &str = "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

use std::collections::{HashMap, HashSet};

type PosSet = HashSet<usize>;

#[derive(Debug)]
struct ParseRun<'a> {
    rules: &'a HashMap<&'a str, &'a str>,
    msg: &'a str,
    memo: HashMap<(&'a str, usize), PosSet>,
}

impl<'a> ParseRun<'a> {
    fn new(rules: &'a HashMap<&'a str, &'a str>, msg: &'a str) -> ParseRun<'a> {
        ParseRun {
            rules,
            msg,
            memo: HashMap::new(),
        }
    }

    /// return true if the rule matches the whole msg:
    fn parse(&mut self, rule: &'a str) -> bool {
        let target_len = self.msg.len();
        return self
            .apply_rule(rule, 0)
            .iter()
            .any(|&pos| target_len == pos);
    }

    /// return a set of possible new positions after applying the rule
    /// returns an empty set if no match is possible
    fn apply_rule(&mut self, rule: &'a str, at_pos: usize) -> PosSet {
        //println!("apply_rule {} at pos {} (self.pos {})", rule, at_pos, self.pos);
        if let Some(answer) = self.memo.get(&(rule, at_pos)) {
            //println!("found memoized {} (setting self.pos {} to {})", answer, self.pos, position);
            return answer.clone();
        }
        let answer = self.eval_body(self.rules[rule], at_pos);
        let result = answer.clone();
        self.memo.insert((rule, at_pos), answer);
        result
    }

    /// All rule bodies have the form: a [b] [ | c d ]
    /// so possibly a choice, with both options being a sequence,
    /// any entry could be a rule name or a literal char "a"
    /// return a set of all possible next scanning positions (empty if no match)
    fn eval_body(&mut self, body: &'a str, at_pos: usize) -> PosSet {
        //println!("eval_body {}", body);
        let mut matched = PosSet::new();
        for option in body.split(" | ") {
            //println!("Checking option: {}", option);
            let mut current_positions = PosSet::new();
            current_positions.insert(at_pos);
            for seq_elem in option.split(' ') {
                //println!("Checking sequence element: {}", seq_elem);
                if seq_elem.len() == 3 && seq_elem.starts_with('"') && seq_elem.ends_with('"') {
                    let to_match = seq_elem.chars().nth(1);
                    // advance all positions that match:
                    current_positions = current_positions
                        .iter()
                        .filter(|&&pos| to_match == self.msg.chars().nth(pos))
                        .map(|&pos| pos + 1)
                        .collect();
                    if current_positions.is_empty() {
                        break; // sequence failed
                    }
                } else {
                    // it is another rule:
                    current_positions = current_positions
                        .iter()
                        .map(|&pos| self.apply_rule(seq_elem, pos))
                        .fold(PosSet::new(), |hs, next_hs| {
                            hs.union(&next_hs).cloned().collect()
                        });
                    if current_positions.is_empty() {
                        break; // sequence failed
                    }
                }
            }
            if !current_positions.is_empty() {
                assert!(!current_positions.contains(&at_pos));
                matched = matched.union(&current_positions).cloned().collect();
                // a true PEG parser would return here on the first option that already succeeded,
                // but to allow ambiguous rules, we will continue here
            }
        }
        matched
    }
}

// now the choice of PEG parsing is biting me,
// as these rules, especially the 8 rule seem to be ambiguous,
// and thus need backtracking...
const PART2_MODIFICATION: &str = "
8: 42 | 42 8
11: 42 31 | 42 11 31
";

fn rule_splitter(r: &str) -> (&str, &str) {
    // very verbose, next time try itertools::next_tuple
    match r.split(':').collect::<Vec<&str>>()[..] {
        [name, rule, ..] => (name, rule.trim()),
        _ => panic!("No rule found!"),
    }
}

pub fn process_input(input: &str) -> String {
    let input: Vec<_> = input.trim().split("\n\n").collect();
    let mut rules: HashMap<&str, &str> = input[0].split('\n').map(rule_splitter).collect();
    let messages: Vec<_> = input[1].split('\n').collect();
    //println!("rules:\n{:?}\nmessages:\n{:?}", rules, messages);
    // create a parser class that knows the rules and manages memoizing:
    // try applying rule 0 for each message and count successes:
    let matched_messages_count = messages
        .iter()
        .filter(|&&msg| ParseRun::new(&rules, msg).parse("0"))
        .count();
    println!("Number of matches: {}", matched_messages_count);
    for (rule, body) in PART2_MODIFICATION.trim().split('\n').map(rule_splitter) {
        rules.insert(rule, body);
    }
    let matched_messages: Vec<&&str> = messages
        .iter()
        .filter(|&&msg| ParseRun::new(&rules, msg).parse("0"))
        .collect();
    println!("Part 2, matches: {:?}", matched_messages);
    println!("Part 2, Number of matches: {}", matched_messages.len());
    format!("TODO")
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
