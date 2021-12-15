const INPUT: &str = include_str!("input");

const EXAMPLE_INPUT: &str = "TODO";

use std::collections::HashMap;
use std::ops::Range;

type Ticket = Vec<u32>;

fn split_int(in_string: &str) -> Ticket {
    in_string.split(',').map(|i| i.parse().unwrap()).collect()
}

type TwoRanges = (Range<u32>, Range<u32>);

fn split_range_spec(in_string: &str) -> (&str, TwoRanges) {
    let pairs: Vec<&str> = in_string.split(": ").collect();
    let ranges: Vec<Vec<&str>> = pairs[1]
        .split(" or ")
        .map(|a| a.split('-').collect())
        .collect();
    let mut ranges: Vec<Range<u32>> = ranges
        .iter()
        .map(|a: &Vec<&str>| -> Range<u32> {
            Range {
                start: a[0].parse().unwrap(),
                end: a[1].parse::<u32>().unwrap() + 1,
            }
        })
        .collect();
    (pairs[0], (ranges.remove(0), ranges.remove(0)))
}

pub fn process_input(input: &str) -> String {
    let input: Vec<&str> = input
        .split("\n\n")
        .map(|a| a.trim())
        .collect();
    let field_specs: HashMap<&str, TwoRanges> =
        input[0].split('\n').map(split_range_spec).collect();
    let my_ticket = split_int(input[1].split('\n').last().unwrap().trim());
    let nearby_tickets: Vec<Ticket> = input[2].trim().split('\n').skip(1).map(split_int).collect();
    println!("field_specs:\n{:?}\n", field_specs);
    println!("my_ticket:\n{:?}\n", my_ticket);
    //println!("nearby_tickets:\n{:?}\n", nearby_tickets);
    // Check all tickets for impossible fields and keep only valid tickets:
    let mut ticket_s_err_rate = 0;
    let mut valid_tickets: Vec<&Ticket> = Vec::new();
    for ticket in &nearby_tickets {
        let mut valid = true;
        for &field in ticket {
            let valid_field = field_specs
                .values()
                .find(|(r1, r2)| r1.contains(&field) || r2.contains(&field));
            if valid_field.is_none() {
                valid = false;
                ticket_s_err_rate += field;
            }
        }
        if valid {
            valid_tickets.push(ticket);
        }
    }
    println!("ticket scanning error rate: {}", ticket_s_err_rate);
    // create a list of options for each field, initally all field names
    let mut field_options: Vec<Vec<&str>> = (0..my_ticket.len())
        .map(|_| field_specs.keys().copied().collect())
        .collect();
    // iterate over all tickets to only retain the possible options
    for ticket in valid_tickets {
        for (field, options) in ticket.iter().zip(field_options.iter_mut()) {
            options.retain(|&o| {
                let field_spec = &field_specs[o];
                field_spec.0.contains(field) || field_spec.1.contains(field)
            });
        }
    }
    //println!("field options after first round: {:?}", field_options);
    println!(
        "field options lengths summed: {}",
        field_options.iter().map(|l| l.len()).sum::<usize>()
    );
    // then loop checking for those with only one candidate until all resolved
    // when a field starting with "departure" is found, multiply it for the part2 answer
    let mut departures: u64 = 1;
    loop {
        let resolved_i = field_options.iter().position(|fo| fo.len() == 1);
        if let Some(index) = resolved_i {
            let resolved_field = field_options[index][0];
            println!("My {}: {}", resolved_field, my_ticket[index]);
            if resolved_field.starts_with("departure") {
                departures *= u64::from(my_ticket[index]);
            }
            field_options
                .iter_mut()
                .for_each(|os| os.retain(|&o| o != resolved_field))
        } else {
            break;
        }
    }
    println!("Product of departures: {}", departures);
    format!("TODO")
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
