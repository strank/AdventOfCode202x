//! https://adventofcode.com/2020/day/24
//! Hexagon tiles: identify from walking instructions -> choose good coordinate system
//!
//! cube coordinates: tiles correspond to 3d cubes projected onto a plane,
//! every tile has an (x, y, z) coordinate where x+y+z=0
//! (so one coordinate is redundant)
//!
//!    -z
//! +y  |  +x
//!   \ ^ /
//!    | |
//!   / v \
//! -x  |  -y
//!    +z
//!
//! so going east, for example, increases x, but decreases y (z stays the same)
//!
//! ```
//! use advent_of_code_202x::generated::year2020day24::run;
//! assert!(run().contains("Num black tiles, part 1: 346\nNum black tiles, part 2: 3802"));
//! ```

const INPUT: &str = include_str!("input");

/// example answer 10 tiles black
/// ```
/// use advent_of_code_202x::generated::year2020day24::run_example;
/// assert!(run_example().contains("Num black tiles, part 1: 10\nNum black tiles, part 2: 2208"));
/// ```
const EXAMPLE_INPUT: &str = "
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

use phf::phf_map;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    x: i32,
    y: i32,
    //z = -x-y
}

impl HexCoord {
    //fn z(&self) -> i32 {
    //    - self.x - self.y
    //}

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const ORIGIN: HexCoord = HexCoord { x: 0, y: 0 };

static DIRS: phf::Map<&str, HexCoord> = phf_map! {
    "w"  => HexCoord { x: -1, y:  1 },
    "e"  => HexCoord { x:  1, y: -1 },
    "nw" => HexCoord { x:  0, y:  1 },
    "se" => HexCoord { x:  0, y: -1 },
    "ne" => HexCoord { x:  1, y:  0 },
    "sw" => HexCoord { x: -1, y:  0 },
};

fn flip_tiles(lines: &[&str]) -> HashSet<HexCoord> {
    let mut flipped_tiles = HashSet::new();
    for line in lines {
        //println!("Parsing line {}", line);
        let mut position = ORIGIN;
        let mut index = 0;
        while index < line.len() {
            let mut dir = &line[index..index + 1];
            if dir == "n" || dir == "s" {
                dir = &line[index..index + 2];
                index += 1;
            }
            //print!("Checking {}", dir);
            position = position.add(DIRS.get(dir).unwrap());
            index += 1;
        }
        //println!("Final position {:?}", position);
        // insert if new, remove if already present
        if !flipped_tiles.remove(&position) {
            flipped_tiles.insert(position);
        }
    }
    flipped_tiles
}

fn game_of_life(mut black_tiles: HashSet<HexCoord>) -> HashSet<HexCoord> {
    for _day in 1..=100 {
        let mut tiles_to_flip: HashSet<HexCoord> = HashSet::new();
        let mut whites_black_neighbour_count: HashMap<HexCoord, usize> = HashMap::new();
        for tile in &black_tiles {
            let mut black_count = 0;
            for neighbour in DIRS.values().map(|dir| tile.add(dir)) {
                if black_tiles.contains(&neighbour) {
                    black_count += 1;
                } else {
                    *whites_black_neighbour_count.entry(neighbour).or_insert(0) += 1
                }
            }
            if black_count == 0 || black_count > 2 {
                tiles_to_flip.insert(*tile);
            }
        }
        for (tile, count) in whites_black_neighbour_count {
            if count == 2 {
                tiles_to_flip.insert(tile);
            }
        }
        for tile in tiles_to_flip {
            if !black_tiles.remove(&tile) {
                black_tiles.insert(tile);
            }
        }
        // if day < 10 || day % 10 == 0 {
        //     println!("Day {}: {}", day, black_tiles.len());
        // }
    }
    black_tiles
}

pub fn process_input(input: &str) -> String {
    let input: Vec<_> = input.trim().split('\n').collect();
    let black_tiles_1 = flip_tiles(&input);
    let black_tiles_1_len = black_tiles_1.len();
    let black_tiles_2 = game_of_life(black_tiles_1);
    format!(
        "Num black tiles, part 1: {}\nNum black tiles, part 2: {}",
        black_tiles_1_len,
        black_tiles_2.len()
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
