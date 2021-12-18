"""
https://adventofcode.com/2020/day/6

The form asks a series of 26 yes-or-no questions marked a through z.
All you need to do is identify the questions for which anyone in your group answers "yes".

>>> main()
sum of answer counts, part 1:  6310
sum of answer counts, part 2:  3193

>>> main(EXAMPLE_INPUT)
sum of answer counts, part 1:  11
sum of answer counts, part 2:  6
"""

from pathlib import Path
import sys

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
abc

a
b
c

ab
ac

a
a
a
a

b
"""


def yield_lines(puzzle_input):
    """Yield all lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        yield line.strip()


def all_answer_counts(lines, anyone=True):
    """Return all answer counts, where anyone/everyone answered yes."""
    all_counts = []
    current_set = None
    for line in lines:
        if line:
            if current_set is None:
                current_set = set(line)
            else:
                if anyone:
                    current_set.update(line)
                else:
                    current_set.intersection_update(line)
        else:
            all_counts.append(len(current_set))
            current_set = None
    if current_set:
        all_counts.append(len(current_set))
    return all_counts


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    lines = list(yield_lines(puzzle_input))
    print("sum of answer counts, part 1: ", sum(all_answer_counts(lines, anyone=True)))
    print("sum of answer counts, part 2: ", sum(all_answer_counts(lines, anyone=False)))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
