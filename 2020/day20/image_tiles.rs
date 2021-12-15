//! https://adventofcode.com/2020/day/20
//! Match image tiles based on their borders
//!
//! ```
//! use advent_of_code_202x::generated::year2020day20::run;
//! assert!(run().contains(
//!     "Product of corners: 59187348943703\nNOT PART OF SEA MONSTERS sum: 1565"
//! ));
//! ```

const INPUT: &str = include_str!("input");

// example answer
// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
// multiply the IDs of the four corner tiles: 1951 * 3079 * 2971 * 1171 = 20899048083289.
/// ```
/// use advent_of_code_202x::generated::year2020day20::run_example;
/// assert!(run_example().contains(
///     "Product of corners: 20899048083289\nNOT PART OF SEA MONSTERS sum: 273"
/// ));
/// ```
const EXAMPLE_INPUT: &str = "
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

// PART 1 solved without representing the tiles, just focusing on edges:

// for each tile get all four edges as strings,
// turn them into a type that compares backwards and forwards,
// then enter them one by one into a hashmap (mapping to the tile number)
// but if an entry is already inside, remove it instead of adding it
// (this should eliminate all entries that have a matching partner)
// the remaining entries should point to the edges
// the corners are the ones that have two entries for the tile!
// (so take the values and remove all unique ones)

// for PART 2 re-implemented with a full Tile struct that uses an ndarray::Array2
// and keep a full mapping of all flip-ignoring "unique" edges to their tile ids
// so we can then reconstruct a full image tile by tile

use ndarray::{prelude::*, Zip};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

const SEA_MONSTER: &str = "
                  # X
#    ##    ##    ###X
 #  #  #  #  #  #   X
";

/// A Edge tuple struct that simply changes the equality and hash of a String to consider
/// a reversed string identical
#[derive(Debug)]
struct Edge(Array1<u8>);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0 == other.0.slice(s![..;-1])
    }
}
impl Eq for Edge {}
impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let reversed = self.0.slice(s![..;-1]);
        let straight = self.0.slice(s![..]);
        if reversed.iter().lt(straight.iter()) {
            reversed.hash(state);
        } else {
            straight.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    pixels: Array2<u8>,
    id: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EdgeDir {
    Top,
    Bottom,
    Left,
    Right,
}

impl EdgeDir {
    fn next_ccw(&self) -> Self {
        match self {
            EdgeDir::Top => EdgeDir::Left,
            EdgeDir::Left => EdgeDir::Bottom,
            EdgeDir::Bottom => EdgeDir::Right,
            EdgeDir::Right => EdgeDir::Top,
        }
    }
}

impl Tile {
    fn from_str(tile_string: &str, tile_id: &str) -> Self {
        Tile::from_str_shape(tile_string, tile_id, (10, 10))
    }

    fn from_str_shape(tile_string: &str, tile_id: &str, shape: (usize, usize)) -> Self {
        Tile {
            pixels: Array::from_shape_vec(
                shape, // hard-coded size!
                tile_string
                    .chars()
                    .filter_map(|c| {
                        if c == '#' {
                            Some(1)
                        } else if " .".contains(c) {
                            Some(0)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
            .unwrap(),
            id: tile_id.to_owned(),
        }
    }

    fn get_edge(&self, edge_dir: &EdgeDir) -> Edge {
        match *edge_dir {
            EdgeDir::Top => Edge(self.pixels.row(0).to_owned()),
            EdgeDir::Bottom => Edge(self.pixels.row(self.pixels.nrows() - 1).to_owned()),
            EdgeDir::Left => Edge(self.pixels.column(0).to_owned()),
            EdgeDir::Right => Edge(self.pixels.column(self.pixels.ncols() - 1).to_owned()),
        }
    }

    fn get_edges(&self) -> Vec<(Edge, EdgeDir)> {
        [EdgeDir::Top, EdgeDir::Bottom, EdgeDir::Left, EdgeDir::Right]
            .iter()
            .map(|dir| (self.get_edge(dir), *dir))
            .collect()
    }

    /// rotate array counterclockwise, i.e. the final column becomes the first row
    fn rotate_ccw(&mut self) {
        let mut pix_vec = Vec::new();
        for col_index in (0..self.pixels.ncols()).rev() {
            pix_vec.extend(self.pixels.column(col_index));
        }
        let old_shape = self.pixels.shape();
        self.pixels = Array2::from_shape_vec((old_shape[1], old_shape[0]), pix_vec).unwrap();
    }

    /// rotate until from_dir is to_dir
    fn rotate_from_to(&mut self, from_dir: &EdgeDir, to_dir: &EdgeDir) {
        let mut cur_dir = *from_dir;
        while &cur_dir != to_dir {
            self.rotate_ccw();
            cur_dir = cur_dir.next_ccw();
        }
    }

    /// flip array, so that the indicated edge is reversed (and all other rows/cols)
    fn flip_along(&mut self, edge_dir: &EdgeDir) {
        let lane_iter = match edge_dir {
            &EdgeDir::Top | &EdgeDir::Bottom => self.pixels.rows_mut(),
            &EdgeDir::Left | &EdgeDir::Right => self.pixels.columns_mut(),
        };
        for mut lane in lane_iter {
            let lane_len = lane.len();
            for index in 0..(lane_len / 2) {
                lane.swap(index, lane_len - 1 - index);
            }
        }
    }
}

fn tile_splitter(r: &str) -> (&str, Tile) {
    // very verbose, next time try itertools::next_tuple
    match r.split(":\n").collect::<Vec<&str>>()[..] {
        [name, tile_data, ..] => (&name[5..], Tile::from_str(tile_data, &name[5..])),
        _ => panic!("No tile found!"),
    }
}

/// reconstruct image by starting from a corner and filling a matrix:
fn reconstruct_image(
    mut tiles: HashMap<&str, Tile>,
    edge_matches: &HashMap<Edge, Vec<(&str, EdgeDir)>>,
    corner: &str,
) -> Array2<u8> {
    let num_tiles = tiles.len();
    let row_length: usize = (num_tiles as f64).sqrt() as usize;
    assert!(num_tiles == row_length * row_length);
    let mut tile_id = corner;
    let mut tile = tiles.remove(tile_id).unwrap();
    //println!("Tile before:\n{:?}", tile);
    for rotation in 0..4 {
        if edge_matches[&tile.get_edge(&EdgeDir::Left)].len() == 1
            && edge_matches[&tile.get_edge(&EdgeDir::Top)].len() == 1
        {
            break;
        } else {
            if rotation == 3 {
                panic!("Top-Left Tile - still no fit after 3 rotations!")
            }
            tile.rotate_ccw();
            //println!("Tile rotated:\n{:?}", tile);
        }
    }
    // create a vec of tile references, and a row counter
    let mut tile_grid: Vec<Tile> = Vec::new();
    let mut edge_to_match: Edge;
    //let mut rows = 1; // no need for row counter, hard-coded for simplicity
    for tile_index in 1..num_tiles {
        // look for match to the right and rotate/flip until matched edges (flip if edge.0 unequal)
        // when no more match to the right, go back to first index (in row) and
        // look for match at bottom once, then to the right again
        // (easier: with fixed row_length, just look for the bottom match when at the beginning of row)
        let targt_edge_dir = if (tile_index % row_length) != 0 {
            edge_to_match = tile.get_edge(&EdgeDir::Right);
            EdgeDir::Left
        } else {
            let row_start_tile = &tile_grid[tile_index - row_length];
            //println!("Row-start at tile_index {}, matching bottom of index {}\nROWSTARTTILE {:?}",
            //    tile_index, tile_index - row_length, row_start_tile);
            edge_to_match = row_start_tile.get_edge(&EdgeDir::Bottom);
            tile_id = &row_start_tile.id;
            EdgeDir::Top
        };
        //println!("Pre-Match: tile_id {} at tile_index {} for targt_edge_dir {:?}, edge {:?},\n{:?}",
        //    tile_id, tile_index, targt_edge_dir, edge_to_match, &tile);
        let &(next_tile_id, next_tile_edge_dir) = match edge_matches[&edge_to_match]
            .iter()
            .find(|(tid, _)| tid != &tile_id)
        {
            Some(ematch) => ematch,
            None => panic!(
                "No edge match found for tile_id {}, tile_index {} targt_edge_dir {:?}\n{:?}",
                tile_id, tile_index, targt_edge_dir, edge_matches[&edge_to_match]
            ),
        };
        //println!("Match found: next_tile_id {} at tile_index {} for targt_edge_dir {:?}",
        //    next_tile_id, tile_index, targt_edge_dir);
        let mut next_tile = tiles.remove(next_tile_id).unwrap();
        // rotate to get correct orientation:
        next_tile.rotate_from_to(&next_tile_edge_dir, &targt_edge_dir);
        // flip if necessary:
        if edge_to_match.0 != next_tile.get_edge(&targt_edge_dir).0 {
            next_tile.flip_along(&targt_edge_dir);
        }
        assert!(edge_to_match.0 == next_tile.get_edge(&targt_edge_dir).0);
        // safety check that the top edge matches (borders could be symmetric and then no flip would be triggered)
        if tile_index > row_length && (tile_index % row_length) != 0 {
            let above_tile = &tile_grid[tile_index - row_length];
            if above_tile.get_edge(&EdgeDir::Bottom).0 != next_tile.get_edge(&EdgeDir::Top).0 {
                panic!(
                    "Tile does not match tile above \n{:?}\n\n{:?}",
                    above_tile, next_tile
                );
            }
        }
        // record tile for new image grid:
        tile_grid.push(tile);
        // switch to next_tile
        tile_id = next_tile_id;
        tile = next_tile;
    }
    tile_grid.push(tile); // final tile
                          // calculate dimensions of full image
                          // (row-length is the difference of length to remembered row-start index,
                          // every tile will change from 10x10 to 8x8 after removing borders)
    let image_dim = row_length * 8;
    // fill a new Array2 with the center 8X8 from all tiles in order
    let mut image: Array2<u8> = Array2::<u8>::zeros((image_dim, image_dim));
    //println!("TILE GRID:");
    for (index, tile) in tile_grid.iter().enumerate() {
        //if index % row_length == 0 { println!(); }
        //print!("{} ", tile.id);
        let row_index = 8 * (index / row_length);
        let col_index = 8 * (index % row_length);
        let img_slice = s![row_index..row_index + 8, col_index..col_index + 8];
        azip!((ipix in &mut image.slice_mut(img_slice), &tpix in tile.pixels.slice(s![1..9, 1..9])) *ipix = tpix);
    }
    image
}

fn count_value(array: ArrayView2<u8>, value: u8) -> usize {
    array.iter().filter(|&&num| num == value).count()
}

/// 2d cross-correlation of kernel on image with no padding, i.e. only multiply the kernel
/// with the matching part of the image where it fully fits. Therefore returns
/// an array with dimensions of image minus dimensions of kernel + 1
/// (Often also called convolve2d, but convolve first rotates the kernel 180)
fn crosscorrelate2d_unpadded(image: ArrayView2<u8>, kernel: ArrayView2<u8>) -> Array2<u8> {
    let kernel_rows = kernel.shape()[0];
    let kernel_cols = kernel.shape()[1];
    let num_rows = image.shape()[0] - kernel_rows + 1;
    let num_cols = image.shape()[1] - kernel_cols + 1;
    let mut result = Array2::<u8>::zeros((num_rows, num_cols));
    for row in 0..num_rows {
        for col in 0..num_cols {
            result[[row, col]] =
                Zip::from(image.slice(s![row..row + kernel_rows, col..col + kernel_cols]))
                    .and(&kernel)
                    .fold(0, |acc, &i, &k| acc + i * k);
        }
    }
    result
}

/// count occurrences of monster in image, but first the correct orientation (rotate/flip) has to be found
/// (rotate/flip the monster rather than the image for efficiency)
/// so do a convolve for every possible orientation until found
fn count_seamonster_hashes(image: ArrayView2<u8>, mut monster: Tile) -> usize {
    let monster_size = count_value(monster.pixels.view(), 1) as u8;
    let mut monster_count: usize;
    for trial in 0..8 {
        // 8 possible orientations
        // check if a convolution gets any result:
        let correlate = crosscorrelate2d_unpadded(image.view(), monster.pixels.view());
        monster_count = count_value(correlate.view(), monster_size);
        //println!("monster_count {} (target size {}) for monster {:?} correlate {:?}",
        //        monster_count, monster_size, &monster, &correlate);
        if monster_count > 0 {
            return monster_count * (monster_size as usize);
        }
        monster.rotate_ccw();
        if trial == 3 {
            monster.flip_along(&EdgeDir::Right);
        }
    }
    panic!("No monsters found in any orientation!");
}

pub fn process_input(input: &str) -> String {
    let input = input.trim().split("\n\n");
    let tiles: HashMap<&str, Tile> = input.map(tile_splitter).collect();
    //println!("Input: {:?}", &tiles["3371"]);
    let mut edge_matches: HashMap<Edge, Vec<(&str, EdgeDir)>> = HashMap::new();
    for (&tile_name, tile) in &tiles {
        for (edge, edge_dir) in tile.get_edges() {
            edge_matches
                .entry(edge)
                .or_insert_with(Vec::new)
                .push((tile_name, edge_dir));
        }
    }
    //println!("Edge_map {:?}", edge_matches);
    let mut outer_edge_freq = HashMap::new();
    for unique_edge_vec in edge_matches.values().filter(|&m_vec| m_vec.len() == 1) {
        let tile_id = unique_edge_vec[0].0;
        *outer_edge_freq.entry(tile_id).or_insert(0) += 1;
    }
    //println!("Outer Edge Frequencies: {:?}", outer_edge_freq);
    let corners: Vec<_> = outer_edge_freq
        .iter()
        .filter_map(|(&k, &v)| if v == 2 { Some(k) } else { None })
        .collect();
    //println!("Corners: {:?}", corners);
    let product_of_corners = corners
        .iter()
        .map(|&tid| tid.parse::<u64>().unwrap())
        .product::<u64>();
    // println!(
    //     "{} Tiles and {} Outer Tiles",
    //     tiles.len(),
    //     outer_edge_freq.len()
    // );
    // reconstruct image by starting from a corner and filling a matrix:
    let image = reconstruct_image(tiles, &edge_matches, corners[0]);
    let num_hashes = count_value(image.view(), 1);
    //println!("IMAGE (sum {})\n{:?}", num_hashes, image);
    // filter out monsters and count again:
    let monster = Tile::from_str_shape(SEA_MONSTER, "monster", (3, 20));
    //println!("the monster:\n{:?}", monster);
    let num_hashes_not_seamonster = num_hashes - count_seamonster_hashes(image.view(), monster);
    format!(
        "Product of corners: {}\nNOT PART OF SEA MONSTERS sum: {}",
        product_of_corners, num_hashes_not_seamonster,
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
