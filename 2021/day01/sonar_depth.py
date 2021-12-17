"""
https://adventofcode.com/2021/day/1

>>> main()
no of increases:  1766
no of sliding-window increases:  1797

>>> main(EXAMPLE_INPUT)
no of increases:  7
no of sliding-window increases:  5
"""

from pathlib import Path
import sys
import itertools

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
199
200
208
210
200
207
240
269
260
263
"""


def yield_ints(puzzle_input):
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield int(line)


def process_measurements(input_ints):
    """
    """
    count = 0
    prev_measurement = sys.maxsize
    for measurement in input_ints:
        if measurement > prev_measurement:
            count += 1
        prev_measurement = measurement
    return count


def sliding_window(iterable, n=2):
    iterables = itertools.tee(iterable, n)
    for iterable, num_skipped in zip(iterables, itertools.count()):
        for _ in range(num_skipped):
            next(iterable, None)
    return zip(*iterables)


def main(puzzle_input=INPUT):
    input_ints = list(yield_ints(puzzle_input))
    increases = process_measurements(input_ints)
    print("no of increases: ", increases)
    window_sums = [sum(window) for window in sliding_window(input_ints, 3)]
    increases = process_measurements(window_sums)
    print("no of sliding-window increases: ", increases)


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
