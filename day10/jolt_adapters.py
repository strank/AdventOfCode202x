"""
"""
from pathlib import Path
import itertools
import functools
import re
import collections


def pairwise(iterable):
    "s -> (s0,s1), (s1,s2), (s2, s3), ..."
    a, b = itertools.tee(iterable)
    next(b, None)
    return zip(a, b)


def generate_number_list():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield int(line)


def find_diffs_and_reachable(input_list):
    """
    """
    jolts = collections.defaultdict(int)
    reachable = collections.defaultdict(int)
    reachable[0] = 1
    for a, b in pairwise(input_list):
        jolts[b - a] += 1
        reachable[b] = reachable[b - 3] + reachable[b - 2] + reachable[b - 1]
    return jolts, reachable


def main():
    input_list = [0] + sorted(generate_number_list())
    input_list.append(input_list[-1] + 3)
    jolt_diffs, reachable = find_diffs_and_reachable(input_list)
    print("One jolts times three jolts: ", jolt_diffs[1] * jolt_diffs[3])
    print("Final adapter reachabe in x ways: ", reachable[input_list[-1]])


if __name__ == "__main__":
    main()
