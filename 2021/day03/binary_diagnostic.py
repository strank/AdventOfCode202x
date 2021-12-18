"""
https://adventofcode.com/2021/day/3

>>> main()
gamma 3004 epsilon 1091 product 3277364
oxy 3583 co2 1601 product 5736383

>>> main(EXAMPLE_INPUT)
gamma 22 epsilon 9 product 198
oxy 23 co2 10 product 230
"""

from pathlib import Path
import sys
import operator

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""


def yield_lines(puzzle_input):
    """Yield the non-empty input lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield line


def calc_one_counts(bin_strings):
    """Return the count of ones for every index position in the binary strings."""
    one_counts = [0] * len(bin_strings[0])
    for bin_string in bin_strings:
        for index, bin_digit in enumerate(bin_string):
            if bin_digit == '1':
                one_counts[index] += 1
    return one_counts


def process_bin_strings(bin_strings, one_counts):
    """Return two numbers, based on most/least common binary digits in the `bin_strings`."""
    half_num_bins = len(bin_strings) // 2
    gamma = "".join('1' if count > half_num_bins else '0' for count in one_counts)
    epsilon = "".join('1' if digit == '0' else '0' for digit in gamma)
    return (int(gamma, base=2), int(epsilon, base=2))


def reduce_candidates(candidates, index, comparison_op):
    """Return candidates but only numbers with the most/least common binary digit at `index`"""
    ones_at_index = [cnd for cnd in candidates if cnd[index] == '1']
    if comparison_op(len(ones_at_index), len(candidates) / 2):
        return ones_at_index
    return [cnd for cnd in candidates if cnd[index] == '0']


def process_bin_strings_again(bin_strings):
    """Return two numbers, see puzzle spec for details."""
    oxy_candidates = bin_strings[:]
    co2_candidates = bin_strings[:]
    len_num = len(bin_strings[0])
    for index in range(len_num):
        if len(oxy_candidates) > 1:
            oxy_candidates = reduce_candidates(oxy_candidates, index, operator.ge)
            #print("oxy " + " ".join(oxy_candidates[:10]))
        if len(co2_candidates) > 1:
            co2_candidates = reduce_candidates(co2_candidates, index, operator.lt)
            #print("co2 " + " ".join(co2_candidates[:10]))
    assert len(oxy_candidates) == 1 and len(co2_candidates) == 1
    return (int(oxy_candidates[0], base=2), int(co2_candidates[0], base=2))


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    bin_strings = list(yield_lines(puzzle_input))
    one_counts = calc_one_counts(bin_strings)
    gamma, epsilon = process_bin_strings(bin_strings, one_counts)
    print(f"gamma {gamma} epsilon {epsilon} product {gamma * epsilon}")
    oxy, co2 = process_bin_strings_again(bin_strings)
    print(f"oxy {oxy} co2 {co2} product {oxy * co2}")


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
