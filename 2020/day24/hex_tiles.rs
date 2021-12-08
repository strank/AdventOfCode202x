//! https://adventofcode.com/2020/day/24
//! Hexagon tiles: identify from walking instructions -> choose good coordinate system
//! 
//! cube coordinates: tiles correspond to 3d cubes projected onto a plane,
//! every tile has an (x, y, z) coordinate where x+y+z=0
//! (so one coordinate is redundant)
//! 
//!        -z
//!     +y  |  +x
//!       \ ^ /
//!        | |
//!       / v \ 
//!     -x  |  -y
//!        +z
//! 
//! so going east, for example, increases x, but decreases y (z stays the same)

use std::collections::{HashSet, HashMap};
use phf::phf_map;


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


const _TEST_INPUT: &str = "
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
// --> answer 10 tiles black


fn flip_tiles(lines: &Vec<&str>) -> HashSet<HexCoord> {
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
    for day in 1..=100 {
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
        if day < 10 || day % 10 == 0 {
            println!("Day {}: {}", day, black_tiles.len());
        }
    }
    black_tiles
}

pub fn run() -> () {
    let input: Vec<_> = include_str!("input")
            .trim()
            .split("\n")
            .collect();
    let black_tiles = flip_tiles(&input);
    println!("Num black tiles, part 1: {}", black_tiles.len());
    let black_tiles = game_of_life(black_tiles);
    println!("Num black tiles, part 2: {}", black_tiles.len());
}
