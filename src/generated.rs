// GENERATED BY CARGO BUILD SCRIPT ../build.rs
// DO NOT EDIT!

// year 2020
#[path = "../2020/day16/tickets.rs"]
mod year2020day16;
#[path = "../2020/day17/conway_cubes.rs"]
mod year2020day17;
#[path = "../2020/day18/expr_parser.rs"]
mod year2020day18;
#[path = "../2020/day19/peg_parser.rs"]
mod year2020day19;
#[path = "../2020/day20/image_tiles.rs"]
mod year2020day20;
#[path = "../2020/day21/allergens.rs"]
mod year2020day21;
#[path = "../2020/day22/crab_combat.rs"]
mod year2020day22;
#[path = "../2020/day23/crab_cups.rs"]
mod year2020day23;
#[path = "../2020/day24/hex_tiles.rs"]
mod year2020day24;
#[path = "../2020/day25/combo_breaker.rs"]
mod year2020day25;
// year 2021
#[path = "../2021/day05/hydrothermal_venture.rs"]
mod year2021day05;


pub fn get_years() -> [usize; 2] {
    [2020, 2021]
}


/// Return an array of 26 run functions
/// (26 rather than 25 for clean 1-based indexing)
pub fn get_days(year: usize) -> [Option<fn() -> ()>; 26] {
    let mut days : [Option<fn() -> ()>; 26] = [None; 26];
    match year {
        2020 => {
            days[16] = Some(year2020day16::run);
            days[17] = Some(year2020day17::run);
            days[18] = Some(year2020day18::run);
            days[19] = Some(year2020day19::run);
            days[20] = Some(year2020day20::run);
            days[21] = Some(year2020day21::run);
            days[22] = Some(year2020day22::run);
            days[23] = Some(year2020day23::run);
            days[24] = Some(year2020day24::run);
            days[25] = Some(year2020day25::run);
        },
        2021 => {
            days[05] = Some(year2021day05::run);
        },
        _ => {
        },
    }
    days
}