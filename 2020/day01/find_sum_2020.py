"""
https://adventofcode.com/2020/day/1

>>> main()
2sum (634, 1386) product: 878724
3sum (765, 266, 989) product: 201251610

>>> main(EXAMPLE_INPUT)
2sum (299, 1721) product: 514579
3sum (675, 979, 366) product: 241861950
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


def find_2sum(in_numbers):
    numbers = []
    for new_number in in_numbers:
        for number in numbers:
            if new_number + number == 2020:
                return (new_number, number)
        numbers.append(new_number)


def find_3sum(in_numbers):
    numbers = []
    for new_number in in_numbers:
        for two_numbers in combinations(numbers, 2):
            number_one, number_two = list(two_numbers)
            if new_number + sum(two_numbers) == 2020:
                return (new_number, number_one, number_two)
        numbers.append(new_number)


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    numbers = list(yield_ints(puzzle_input))
    two_sum = find_2sum(numbers)
    print(f"2sum {two_sum} product: {two_sum[0] * two_sum[1]}")
    three_sum = find_3sum(numbers)
    print(f"3sum {three_sum} product: {three_sum[0] * three_sum[1] * three_sum[2]}")


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
