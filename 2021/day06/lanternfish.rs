/// https://adventofcode.com/2021/day/6
/// given a list of time-to-spawn-new-fish ages
/// find the number of spawned fish after 80 days

const _TEST_INPUT: &str = "3,4,3,1,2"; // --> answer 5934 after 80 days, 26 after 18 days


/// vec that records how many fish with a certain timer are present
type FishRegister = [usize; 9];


fn count_after_sim_steps(timers: &Vec<usize>, sim_steps: usize) -> usize {
    let mut simulation: FishRegister = [0; 9];
    for timer in timers {
        simulation[*timer] += 1;
    }
    for _step in 0..sim_steps {
        // shift all counts one index down, add the 0 count once to 6 and once to 8:
        let zero_count = simulation[0];
        for index in 0..8 {
            simulation[index] = simulation[index + 1];
        }
        simulation[6] += zero_count;
        simulation[8] = zero_count;
    }
    simulation.iter().sum()
}

fn str_to_usize(a_str: &str) -> usize {
    a_str.parse::<usize>().unwrap()
}


pub fn run() -> () {
    let timers: Vec<_> = include_str!("input")
            .trim()
            .split(",")
            .map(str_to_usize)
            .collect();
    //println!("timers:\n{:?}", timers);
    let sim_steps: usize = 80;
    let num_fish = count_after_sim_steps(&timers, sim_steps);
    println!("number of fish after {} days: {}", sim_steps, num_fish);
    let sim_steps: usize = 256;
    let num_fish = count_after_sim_steps(&timers, sim_steps);
    println!("number of fish after {} days: {}", sim_steps, num_fish);
}
