// GENERATED BY CARGO BUILD SCRIPT ../build.rs
// DO NOT EDIT!

// year 2020
#[path = "../2020/day16/tickets.rs"]
pub mod year2020day16;
#[path = "../2020/day17/conway_cubes.rs"]
pub mod year2020day17;
#[path = "../2020/day18/expr_parser.rs"]
pub mod year2020day18;
#[path = "../2020/day19/peg_parser.rs"]
pub mod year2020day19;
#[path = "../2020/day20/image_tiles.rs"]
pub mod year2020day20;
#[path = "../2020/day21/allergens.rs"]
pub mod year2020day21;
#[path = "../2020/day22/crab_combat.rs"]
pub mod year2020day22;
#[path = "../2020/day23/crab_cups.rs"]
pub mod year2020day23;
#[path = "../2020/day24/hex_tiles.rs"]
pub mod year2020day24;
#[path = "../2020/day25/combo_breaker.rs"]
pub mod year2020day25;
// year 2021
#[path = "../2021/day05/hydrothermal_venture.rs"]
pub mod year2021day05;
#[path = "../2021/day06/lanternfish.rs"]
pub mod year2021day06;
#[path = "../2021/day07/whale_crabs.rs"]
pub mod year2021day07;
#[path = "../2021/day08/seven_segment_search.rs"]
pub mod year2021day08;
#[path = "../2021/day12/passage_pathing.rs"]
pub mod year2021day12;
#[path = "../2021/day13/transparent_origami.rs"]
pub mod year2021day13;
#[path = "../2021/day14/extended_polymerization.rs"]
pub mod year2021day14;
#[path = "../2021/day15/chiton.rs"]
pub mod year2021day15;
#[path = "../2021/day16/packet_decoder.rs"]
pub mod year2021day16;

pub fn get_years() -> [usize; 2] {
    [2020, 2021]
}

pub type RunFn = fn() -> String;

#[derive(PartialEq, Copy, Clone)]
pub struct AOCRunFns {
    pub run: RunFn,
    pub example: RunFn,
}

/// Return an array of 26 tuples of functions (run, run_example)
/// (26 rather than 25 for clean 1-based indexing)
pub fn get_days(year: usize) -> [Option<AOCRunFns>; 26] {
    let mut days: [Option<AOCRunFns>; 26] = [None; 26];
    match year {
        2020 => {
            days[16] = Some(AOCRunFns {
                run: year2020day16::run,
                example: year2020day16::run_example,
            });
            days[17] = Some(AOCRunFns {
                run: year2020day17::run,
                example: year2020day17::run_example,
            });
            days[18] = Some(AOCRunFns {
                run: year2020day18::run,
                example: year2020day18::run_example,
            });
            days[19] = Some(AOCRunFns {
                run: year2020day19::run,
                example: year2020day19::run_example,
            });
            days[20] = Some(AOCRunFns {
                run: year2020day20::run,
                example: year2020day20::run_example,
            });
            days[21] = Some(AOCRunFns {
                run: year2020day21::run,
                example: year2020day21::run_example,
            });
            days[22] = Some(AOCRunFns {
                run: year2020day22::run,
                example: year2020day22::run_example,
            });
            days[23] = Some(AOCRunFns {
                run: year2020day23::run,
                example: year2020day23::run_example,
            });
            days[24] = Some(AOCRunFns {
                run: year2020day24::run,
                example: year2020day24::run_example,
            });
            days[25] = Some(AOCRunFns {
                run: year2020day25::run,
                example: year2020day25::run_example,
            });
        }
        2021 => {
            days[5] = Some(AOCRunFns {
                run: year2021day05::run,
                example: year2021day05::run_example,
            });
            days[6] = Some(AOCRunFns {
                run: year2021day06::run,
                example: year2021day06::run_example,
            });
            days[7] = Some(AOCRunFns {
                run: year2021day07::run,
                example: year2021day07::run_example,
            });
            days[8] = Some(AOCRunFns {
                run: year2021day08::run,
                example: year2021day08::run_example,
            });
            days[12] = Some(AOCRunFns {
                run: year2021day12::run,
                example: year2021day12::run_example,
            });
            days[13] = Some(AOCRunFns {
                run: year2021day13::run,
                example: year2021day13::run_example,
            });
            days[14] = Some(AOCRunFns {
                run: year2021day14::run,
                example: year2021day14::run_example,
            });
            days[15] = Some(AOCRunFns {
                run: year2021day15::run,
                example: year2021day15::run_example,
            });
            days[16] = Some(AOCRunFns {
                run: year2021day16::run,
                example: year2021day16::run_example,
            });
        }
        _ => {}
    }
    days
}
