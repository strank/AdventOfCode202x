#![warn(clippy::all)]

/// all references to individual puzzles are auto-generated:
mod generated;

/// Two arguments possible: day year
/// both optional, select the latest one (year or day) by default
/// (year can only be specified if day is present too)
fn main() {
    let year: usize = match std::env::args().nth(2) {
        Some(year) => year.parse().expect("integer year expected"),
        None => *generated::get_years().last().unwrap(), // latest year
    };
    let days = generated::get_days(year);
    // check argument for day number, otherwise run most recent one:
    let day: usize = match std::env::args().nth(1) {
        Some(day) => day.parse().expect("integer day expected"),
        None => {
            // find last element in array that is not None (i.e.: first Some)
            days.iter()
                .enumerate()
                .filter(|(_, &d)| d != None)
                .last()
                .unwrap()
                .0
        }
    };
    days[day].unwrap()();
}
