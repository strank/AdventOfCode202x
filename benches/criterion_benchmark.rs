use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code_202x::generated;

fn criterion_benchmark(c: &mut Criterion) {
    // iterate through years and days and set up a bench for each:
    for year in generated::get_years() {
        if year == 2020 {
            continue; // skip 2020 for now!
        }
        for (day, code_option) in generated::get_days(year).iter().enumerate() {
            if let Some(day_code) = code_option {
                c.bench_function(
                    format!("year{}day{}", year, day).as_str(),
                    |b| b.iter(|| (day_code.run)()));
            }
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
