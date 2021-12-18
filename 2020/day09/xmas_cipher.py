"""
https://adventofcode.com/2020/day/9

XMAS starts by transmitting a preamble of 25 numbers.
After that, each number you receive should be
the sum of any two of the 25 immediately previous numbers.
The two numbers will have different values,
and there might be more than one such pair.

>>> main()
First failed:  31161678
smallest + largest in sum_list:  5453868

>>> main(EXAMPLE_INPUT)
First failed:  127
smallest + largest in sum_list:  62
"""

from pathlib import Path
import sys
import itertools

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"""


def generate_number_list(puzzle_input):
    """Yield an int for every non-empty line."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield int(line)


def first_failed_xmas(input_list, hist_len=25):
    """Return first num that is not a sum of two recent (`hist_len`) numbers."""
    recents = []
    for next_num in input_list:
        if len(recents) < hist_len:
            recents.append(next_num)
        else:
            if check_xmas_num(set(recents), next_num):
                recents = recents[1:] + [next_num]
            else:
                return next_num
    return None


def check_xmas_num(allowed_summands, next_num):
    """Return whether next_num can be eexpressed as a sum of two nums in `allowed_summands`."""
    for summand in allowed_summands:
        if next_num - summand in allowed_summands:
            return True
    return False


def find_sum_list(all_nums, target):
    """Return list of contiguous numbers that sum to target."""
    for start_index in range(len(all_nums)):
        candidates = itertools.islice(all_nums, start_index, None)
        test_sum = 0
        for index, cand in enumerate(candidates):
            test_sum += cand
            if test_sum < target:
                continue
            elif test_sum > target:
                break
            else:
                return all_nums[start_index:start_index + index + 1]


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = list(generate_number_list(puzzle_input))
    first_fail = first_failed_xmas(input_list, 5 if puzzle_input == EXAMPLE_INPUT else 25)
    print("First failed: ", first_fail)
    sum_list = find_sum_list(input_list, first_fail)
    print("smallest + largest in sum_list: ", min(sum_list) + max(sum_list))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
