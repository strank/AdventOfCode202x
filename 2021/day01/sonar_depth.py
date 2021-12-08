"""
"""
from pathlib import Path
import sys
import itertools


def yield_ints():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
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


def main():
    input_ints = list(yield_ints())
    increases = process_measurements(input_ints)
    print("no of increases: ", increases)
    window_sums = [sum(window) for window in sliding_window(input_ints, 3)]
    increases = process_measurements(window_sums)
    print("no of sliding-window increases: ", increases)




if __name__ == "__main__":
    main()
