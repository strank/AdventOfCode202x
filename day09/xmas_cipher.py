"""
XMAS starts by transmitting a preamble of 25 numbers.
After that, each number you receive should be the sum of any two of the 25 immediately previous numbers.
The two numbers will have different values, and there might be more than one such pair.
"""
from pathlib import Path
import itertools
import functools
import re
import collections


def generate_number_list():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield int(line)


def first_failed_xmas(input_list):
    """
    """
    last_25 = []
    for next_num in input_list:
        if len(last_25) < 25:
            last_25.append(next_num)
        else:
            if check_xmas_num(set(last_25), next_num):
                last_25 = last_25[1:] + [next_num]
            else:
                return next_num


def check_xmas_num(allowed_summands, next_num):
    for summand in allowed_summands:
        if next_num - summand in allowed_summands:
            return True
    return False


def find_sum_list(all_nums, target):
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


def main():
    input_list = list(generate_number_list())
    first_fail = first_failed_xmas(input_list)
    print("First failed: ", first_fail)
    sum_list = find_sum_list(input_list, first_fail)
    print("smallest + largest in sum_list: ", min(sum_list) + max(sum_list))


if __name__ == "__main__":
    main()
