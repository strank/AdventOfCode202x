//! https://adventofcode.com/2020/day/23
//! Cups game with a crab
//!
//! part 1 implemented directly by holding the cup numbers in a vec
//! part 2 needed a more efficient representation: neighbour links

const _TEST_INPUT: &str = "389125467";
// --> answer after 100 moves: (1) 67384529

const INPUT: &str = "614752839";

type Cups = Vec<usize>;

/// play 100 moves of the cups game and return the new cups order
#[allow(clippy::needless_collect)]
fn play_game(mut cups: Cups) -> Cups {
    let max_cup = *cups.iter().max().unwrap();
    let min_cup = *cups.iter().min().unwrap();
    for _ in 0..100 {
        // need to collect here, as the items are later used to mutate cups again:
        let to_be_moved: Cups = cups.drain(1..4).collect();
        let mut target = cups[0] - 1;
        let target_index: usize;
        loop {
            if target < min_cup {
                target = max_cup;
            }
            if let Some(found_index) = cups.iter().position(|&c| c == target) {
                target_index = found_index;
                break;
            }
            target -= 1;
        }
        to_be_moved
            .into_iter()
            .rev()
            .for_each(|ele| cups.insert(target_index + 1, ele));
        cups.rotate_left(1);
    }
    cups
}

/// struct tracks the clockwise neighbour of every number on a circle
/// minimum number is always 1 and the circle is filled up with nums up to max
#[derive(Debug)]
struct Circle {
    cw_neighbour: Vec<usize>, // here used as a list of links to the clockwise neighbour
    current: usize,
    max: usize,
}

impl Circle {
    fn new(starting_cups: Cups, max: usize) -> Self {
        let start = *starting_cups.first().unwrap();
        // use a vec with max + 1 entries, to use the num directly as index,
        // 0th entry is meaningless. initialize with default sequence:
        let mut neighbours: Vec<usize> = (1..max + 2).collect();
        neighbours[max] = start; // close circle
        neighbours[0] = 0; // take meaningless element out of circle
        for a_b in starting_cups.windows(2) {
            neighbours[a_b[0]] = a_b[1];
        }
        neighbours[*starting_cups.last().unwrap()] = *starting_cups.iter().max().unwrap() + 1;
        Circle {
            cw_neighbour: neighbours,
            current: start,
            max,
        }
    }

    /// play 10_000_000 moves of the cups game and return the new cups order
    fn play_game(&mut self) {
        for _ in 0..10_000_000 {
            let a = self.cw_neighbour[self.current];
            let b = self.cw_neighbour[a];
            let c = self.cw_neighbour[b];
            let mut target = self.current - 1;
            while [0, a, b, c].contains(&target) {
                if target == 0 {
                    target = self.max;
                } else {
                    target -= 1;
                }
            }
            // cut a,b,c out from circle
            self.cw_neighbour[self.current] = self.cw_neighbour[c];
            // insert a,b,c clockwise from target:
            self.cw_neighbour[c] = self.cw_neighbour[target];
            self.cw_neighbour[target] = a;
            self.current = self.cw_neighbour[self.current];
        }
    }
}

fn cups_splitter(line: &str) -> Cups {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn run() -> String {
    let starting_cups: Cups = cups_splitter(INPUT);
    //println!("Input: {:?}", &starting_cups);
    let part1_cups = play_game(starting_cups.clone());
    // part 2: extend cups to 1 million, do 10 million moves
    // -> need a specialized structure, a linked-list in an array
    let mut circle = Circle::new(starting_cups, 1_000_000);
    println!(
        "Circle first 12: {:?} last 5: {:?}",
        &circle.cw_neighbour[..12],
        &circle.cw_neighbour[circle.max - 4..]
    );
    circle.play_game();
    println!(
        "Circle first 12: {:?} last 5: {:?}",
        &circle.cw_neighbour[..12],
        &circle.cw_neighbour[circle.max - 4..]
    );
    let a = circle.cw_neighbour[1];
    let b = circle.cw_neighbour[a];
    format!(
        "Part one final cups: {:?}\nProduct of 1-neighbours: {} * {} = {}",
        &part1_cups,
        a,
        b,
        a as u64 * b as u64
    )
}
