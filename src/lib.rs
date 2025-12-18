#![warn(clippy::all)]

/// all references to individual puzzles are auto-generated:
pub mod generated;

/// Two arguments possible: day year
/// both optional, select the latest one (year or day) by default
/// (on command line, the year can only be specified if day is present too)
pub fn run_puzzles(year_arg: Option<&String>, day_arg: Option<&String>, use_example: bool) {
    let year: usize = match year_arg {
        Some(year) => year.parse().expect("integer year expected"),
        None => *generated::get_years().last().unwrap(), // latest year
    };
    let days = generated::get_days(year);
    // check argument for day number, otherwise run most recent one:
    let day: usize = match day_arg {
        Some(day) => day.parse().expect("integer day expected"),
        None => {
            // find last element in array that is not None (i.e.: first Some)
            days.iter()
                .enumerate()
                .filter(|(_, d)| d.is_some())
                .last()
                .unwrap()
                .0
        }
    };
    let aoc = days[day].as_ref().unwrap();
    println!("{}", (if use_example { aoc.example } else { aoc.run })());
}
