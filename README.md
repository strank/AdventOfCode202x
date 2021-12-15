# AdventOfCode202x

Advent of Code solutions, from 2020 and later, in **Python** or **Rust** or **GDScript** for my own enjoyment and learning.

See https://adventofcode.com/events

All the code snippets below should be run from the repo root directory.

## Rust-based solutions

Run the latest puzzle:

    cargo run

Run a specific day from the latest year:

    cargo run 7

Run a specific day from another year:

    cargo run 23 2020

Add an `x` argument anywhere to use the example input of the puzzle instead of the full input.

Run all tests (doctests) checking that the answers are still correct:

    cargo test --doc

Run a specific one by adding any part of its name, e.g. `2021day05`.

Get code improvement hints / linting with `cargo clippy` (should happen automatically in vscode) and format all code with `cargo fmt` (or Alt-Shift-F per file in vscode).

Run all benchmarks, using criterion.rs (this might take some time):

    cargo bench

Run only a specific one by adding any part of its name, e.g. `21day05`.

## Python-based solutions

TODO

## GDScript-based solutions

TODO
