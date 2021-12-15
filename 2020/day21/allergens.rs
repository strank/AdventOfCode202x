//! https://adventofcode.com/2020/day/21
//! Identify allergens in unknown language
//!
//! ```
//! use advent_of_code_202x::generated::year2020day21::run;
//! assert!(run().contains(
//!     "Safe ingredient count: 2556\nCanonical dangerous: vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh"
//! ));
//! ```

const INPUT: &str = include_str!("input");

/// example answer
/// part1: kfcds, nhms, sbzzf, or trh cannot contain an allergen
/// they appear 5 times in total
/// ```
/// use advent_of_code_202x::generated::year2020day21::run_example;
/// assert!(run_example().contains(
///     "Safe ingredient count: 5\nCanonical dangerous: mxmxvkd,sqjhc,fvjkl"
/// ));
/// ```
const EXAMPLE_INPUT: &str = "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

use std::collections::{HashMap, HashSet};

fn food_splitter(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut ingrs_allergens = line.split(" (contains ");
    let ingrs = ingrs_allergens.next().unwrap().split(' ').collect();
    let allergens = ingrs_allergens
        .next()
        .unwrap()
        .trim_end_matches(')')
        .split(", ")
        .collect();
    (ingrs, allergens)
}

pub fn process_input(input: &str) -> String {
    let input = input.trim().split('\n');
    let foods: Vec<_> = input.map(food_splitter).collect();
    //println!("Input: {:?}", &foods);
    let mut ingr_counts = HashMap::new();
    let mut allergen_options = HashMap::new();
    for (ingrs, allergens) in foods {
        for &ingr in ingrs.iter() {
            *ingr_counts.entry(ingr).or_insert(0) += 1;
        }
        let ingr_set: HashSet<_> = ingrs.into_iter().collect();
        for &allergen in allergens.iter() {
            if !allergen_options.contains_key(allergen) {
                allergen_options.insert(allergen, ingr_set.clone());
            } else {
                allergen_options
                    .entry(allergen)
                    .and_modify(|e| *e = e.intersection(&ingr_set).cloned().collect());
            }
        }
    }
    //println!(
    //    "Ingr counts:\n{:?}\nAllergen options:\n{:?}",
    //    &ingr_counts, &allergen_options
    //);
    let mut possible_allergens: HashSet<&&str> = HashSet::new();
    allergen_options
        .values()
        .for_each(|hs| possible_allergens.extend(hs));
    //println!("Possible allergens:\n{:?}", possible_allergens);
    let safe_ingr_count: i32 = ingr_counts
        .iter()
        .filter_map(|(ing, &count)| {
            if possible_allergens.contains(ing) {
                None
            } else {
                Some(count)
            }
        })
        .sum();
    let mut allergen_translations: HashMap<&str, &str> = HashMap::new();
    loop {
        let allergens_found: Vec<_> = allergen_options
            .iter()
            .filter_map(|(&k, v)| if v.len() == 1 { Some(k) } else { None })
            .collect();
        if allergens_found.is_empty() {
            break;
        }
        for aller in allergens_found {
            let trans = *allergen_options
                .remove(aller)
                .unwrap()
                .iter()
                .next()
                .unwrap();
            allergen_options.iter_mut().for_each(|(_k, v)| {
                v.remove(trans);
            });
            allergen_translations.insert(aller, trans);
        }
    }
    //println!("Allergen translations:\n{:?}", &allergen_translations);
    // sort alphabetically
    let mut allergens: Vec<_> = allergen_translations.keys().collect();
    allergens.sort();
    let translations: Vec<_> = allergens
        .iter()
        .map(|&&a| allergen_translations[a])
        .collect();
    format!(
        "Safe ingredient count: {}\nCanonical dangerous: {}",
        safe_ingr_count,
        translations.join(","),
    )
}

pub fn run_example() -> String {
    process_input(EXAMPLE_INPUT)
}

pub fn run() -> String {
    process_input(INPUT)
}
