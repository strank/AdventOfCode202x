//! https://adventofcode.com/2021/day/12
//! Enumerate all paths in a graph. Probably breadth-first / dikstra / uniform-cost search
//!
//! ```
//! use advent_of_code_202x::generated::year2021day12::run;
//! assert!(run().contains("num paths from start to end: 5178\nnum paths from start to end: 130094"));
//! ```

const INPUT: &str = include_str!("input");

/// ```
/// use advent_of_code_202x::generated::year2021day12::run_example;
/// assert!(run_example().contains("num paths from start to end: 10\nnum paths from start to end: 36"));
/// ```
const EXAMPLE_INPUT: &str = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"; // 10 paths

const _EXAMPLE_INPUT2: &str = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"; // 19 paths

const _EXAMPLE_INPUT3: &str = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"; // 226 paths

use std::collections::HashMap;

/// need to construct a network mapping:
type Connections = HashMap<&'static str, Vec<&'static str>>;

/// Add a connection to the connections dictionary
/// filters out connections to 'start' and from 'end'
fn add_connection(connections: &mut Connections, from: &'static str, to: &'static str) {
    if from == "end" || to == "start" {
        return;
    }
    connections.entry(from).or_insert_with(Vec::new).push(to);
}

/// Return a dict from node name to list of node names, both directions are in the dict for every edge
fn parse_connections(puzzle_input: &'static str) -> Connections {
    let mut result = HashMap::new();
    for line in puzzle_input.split('\n') {
        let (origin, destination) = line.split_once('-').unwrap();
        add_connection(&mut result, origin, destination);
        add_connection(&mut result, destination, origin);
    }
    result
}

/// depth-first search with special cases based on the case of a node label. uppercase can be visited unlimited times
/// lowercase can only be visited once, unless `small_visited_twice` is false, then one can be visited twice
/// TODO: optimization after testing is in place: don't remember paths, just return a count!
/// possible further optimization: graph could be based on ints or even bit-based adjacency
fn dfs(
    connections: &Connections,
    node: &'static str,
    goal: &'static str,
    current_path: &mut Vec<&'static str>,
    paths: &mut Vec<Vec<&'static str>>,
    small_visited_twice: bool,
) {
    current_path.push(node);
    //println!("considering {:?}", current_path);
    if node == goal {
        paths.push(current_path.clone())
    } else {
        for neigh in &connections[node] {
            if neigh.chars().next().unwrap().is_ascii_lowercase() && current_path.contains(neigh) {
                // small cave that we've already visited, check if still possible:
                if !small_visited_twice {
                    dfs(connections, neigh, goal, current_path, paths, true)
                }
            } else {
                dfs(
                    connections,
                    neigh,
                    goal,
                    current_path,
                    paths,
                    small_visited_twice,
                )
            }
        }
    }
    current_path.pop();
}

/// Count the numer of paths from start to end, only allowing repeat visits to uppercase nodes in the connection graph
/// (assumes that there are no direct connections between uppercase nodes - that would result in infinite loops)
/// modified depth-first (or uniform-cost ?) search that counts all possible paths
fn count_paths(
    connections: &Connections,
    start: &'static str,
    end: &'static str,
    small_visited_twice: bool,
) -> usize {
    let mut paths = Vec::new();
    let mut path = Vec::new();
    dfs(
        connections,
        start,
        end,
        &mut path,
        &mut paths,
        small_visited_twice,
    );
    paths.len()
}

pub fn process_input(input: &'static str) -> String {
    let connections = parse_connections(input.trim());
    let num_paths = count_paths(&connections, "start", "end", true);
    let num_paths_small_twice = count_paths(&connections, "start", "end", false);
    format!(
        "num paths from start to end: {}\nnum paths from start to end: {}\n",
        num_paths, num_paths_small_twice
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
