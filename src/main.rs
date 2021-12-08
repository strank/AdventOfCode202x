#[path = "../2020/day16/tickets.rs"]
mod day16;
#[path = "../2020/day17/conway_cubes.rs"]
mod day17;
#[path = "../2020/day18/expr_parser.rs"]
mod day18;
#[path = "../2020/day19/peg_parser.rs"]
mod day19;
#[path = "../2020/day20/image_tiles.rs"]
mod day20;
#[path = "../2020/day21/allergens.rs"]
mod day21;
#[path = "../2020/day22/crab_combat.rs"]
mod day22;
#[path = "../2020/day23/crab_cups.rs"]
mod day23;
#[path = "../2020/day24/hex_tiles.rs"]
mod day24;
#[path = "../2020/day25/combo_breaker.rs"]
mod day25;

fn main() {
    let mut days : [Option<fn() -> ()>; 26] = [None; 26];
    days[16] = Some(day16::run);
    days[17] = Some(day17::run);
    days[18] = Some(day18::run);
    days[19] = Some(day19::run);
    days[20] = Some(day20::run);
    days[21] = Some(day21::run);
    days[22] = Some(day22::run);
    days[23] = Some(day23::run);
    days[24] = Some(day24::run);
    days[25] = Some(day25::run);
    // check argument for day number, otherwise run most recent one:
    let day: usize = match std::env::args().nth(1) {
        Some(day) => day.parse().expect("integer day expected"),
        None => {
            // find last element in array that is not None (i.e.: first Some)
            days.iter().enumerate().filter(|(_, &d)| d != None).last().unwrap().0
        }
    };
    days[day].unwrap()();
}
