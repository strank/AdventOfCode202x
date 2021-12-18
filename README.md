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

Run any puzzle by directly running the python file:

    python 2021/day04/bingo_squid.py

Add an `x` argument to use the example input of the puzzle instead of the full input.

Run all tests (doctests only, it will skip benchmarks) checking that the answers are still correct:

    pytest

Run a specific one by using parts of its name with the `-k` option, e.g. `-k "2021 and day04"` (pytest is not happy with path separators).

Get code improvement hints / linting with `pylint` (should happen automatically in vscode) and format code with Alt-Shift-F in vscode (currently using autopep8).

Run all benchmarks, using pytest-benchmark (this will skip normal tests, but it might take some time):

    pytest --benchmark-only

Run only a specific with `-k` as above, e.g. `-k 21day04`.

## GDScript-based solutions

TODO
