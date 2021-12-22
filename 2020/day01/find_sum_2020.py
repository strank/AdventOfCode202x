"""
https://adventofcode.com/2020/day/1

>>> main()
2sum [634, 1386] product: 878724
3sum [266, 765, 989] product: 201251610

>>> main(EXAMPLE_INPUT)
2sum [299, 1721] product: 514579
3sum [366, 675, 979] product: 241861950
"""

from pathlib import Path
import sys
from itertools import combinations

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
1721
979
366
299
675
1456
"""


def yield_ints(puzzle_input):
    """Return one integer per line in the input."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield int(line)


def find_sum(numbers, num, target):
    """Return tuple of `num` numbers that sum to `target`."""
    for sum_numbers in combinations(numbers, num):
        if sum(sum_numbers) == target:
            return sum_numbers
    return None


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    numbers = list(yield_ints(puzzle_input))
    two_sum = find_sum(numbers, 2, 2020)
    print(f"2sum {sorted(two_sum)} product: {two_sum[0] * two_sum[1]}")
    three_sum = find_sum(numbers, 3, 2020)
    print(f"3sum {sorted(three_sum)} product: {three_sum[0] * three_sum[1] * three_sum[2]}")


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
