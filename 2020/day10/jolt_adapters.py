"""
https://adventofcode.com/2020/day/10

>>> main()
One jolts times three jolts:  2040
Final adapter reachabe in x ways:  28346956187648

>>> main(EXAMPLE_INPUT)
One jolts times three jolts:  35
Final adapter reachabe in x ways:  8

>>> main(EXAMPLE_INPUT_ALT)
One jolts times three jolts:  220
Final adapter reachabe in x ways:  19208
"""

from pathlib import Path
import sys
import itertools
import collections

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
16
10
15
5
1
11
7
19
6
12
4
"""

EXAMPLE_INPUT_ALT = """
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"""


def pairwise(iterable):
    "s -> (s0,s1), (s1,s2), (s2, s3), ..."
    aaa, bbb = itertools.tee(iterable)
    next(bbb, None)
    return zip(aaa, bbb)


def generate_number_list(puzzle_input):
    """Yield every non-empty line as an int."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield int(line)


def find_diffs_and_reachable(input_list):
    """Return a tuple of dicts."""
    jolts = collections.defaultdict(int)
    reachable = collections.defaultdict(int)
    reachable[0] = 1
    for aaa, bbb in pairwise(input_list):
        jolts[bbb - aaa] += 1
        reachable[bbb] = reachable[bbb - 3] + reachable[bbb - 2] + reachable[bbb - 1]
    return jolts, reachable


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = [0] + sorted(generate_number_list(puzzle_input))
    input_list.append(input_list[-1] + 3)
    jolt_diffs, reachable = find_diffs_and_reachable(input_list)
    print("One jolts times three jolts: ", jolt_diffs[1] * jolt_diffs[3])
    print("Final adapter reachabe in x ways: ", reachable[input_list[-1]])


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
