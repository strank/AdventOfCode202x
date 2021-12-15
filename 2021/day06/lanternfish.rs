//! https://adventofcode.com/2021/day/6
//! given a list of time-to-spawn-new-fish ages
//! find the number of spawned fish after 80 days
//!
//! ```
//! use advent_of_code_202x::generated::year2021day06::run;
//! assert!(run().contains(
//!     "number of fish after 80 days: 352195\nnumber of fish after 256 days: 1600306001288"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 5934 after 80 days, 26 after 18 days
/// ```
/// use advent_of_code_202x::generated::year2021day06::run_example;
/// assert!(run_example().contains(
///     "number of fish after 80 days: 5934\nnumber of fish after 256 days: 26984457539"));
/// ```
const EXAMPLE_INPUT: &str = "3,4,3,1,2";

/// vec that records how many fish with a certain timer are present
type FishRegister = [usize; 9];

fn count_after_sim_steps(timers: &[usize], sim_steps: usize) -> usize {
    let mut simulation: FishRegister = [0; 9];
    for timer in timers {
        simulation[*timer] += 1;
    }
    for _step in 0..sim_steps {
        // rotate all counts one index down (which will move the 0-count to index 8),
        // and add the 0-count (now at index 8) once to 6:
        simulation.rotate_left(1);
        simulation[6] += simulation[8];
        // more efficient but much less clear would be to skip the rotating,
        // and use (_step + 7) % 9 to identify the count that needs to be increased:
        //simulation[(_step + 7) % 9] += simulation[_step % 9]
    }
    simulation.iter().sum()
}

fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}

pub fn process_input(input: &str) -> String {
    let timers: Vec<_> = input
        .trim()
        .split(',')
        .map(str_to_usize)
        .collect();
    //println!("timers:\n{:?}", timers);
    let sim_steps_1: usize = 80;
    let num_fish_1 = count_after_sim_steps(&timers, sim_steps_1);
    let sim_steps_2: usize = 256;
    let num_fish_2 = count_after_sim_steps(&timers, sim_steps_2);
    format!(
        "number of fish after {} days: {}\nnumber of fish after {} days: {}",
        sim_steps_1, num_fish_1, sim_steps_2, num_fish_2
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
