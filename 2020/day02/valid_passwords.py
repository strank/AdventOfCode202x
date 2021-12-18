"""
https://adventofcode.com/2020/day/2

2-6 w: wkwwwfwwpvw

>>> main()
valid part 1:  536
valid part 2:  558

>>> main(EXAMPLE_INPUT)
valid part 1:  2
valid part 2:  1
"""

from pathlib import Path
import sys

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"""


def yield_triples(puzzle_input):
    """Yield triples of (spec, letter, passw) for each line."""
    for line in puzzle_input.strip().split('\n'):
        spec, letter, passw = line.strip().split()
        letter = letter[0]
        spec_a, spec_b = (int(spec) for spec in spec.split("-"))
        yield ((spec_a, spec_b), letter, passw)


def valid_count(triples):
    """Return count of valid passwords."""
    valid = 0
    for ((spec_from, spec_to), letter, passw) in triples:
        count_letter = passw.count(letter)
        if spec_from <= count_letter <= spec_to:
            # print("found: {} {} times in {}, allows {}-{}".format(
            #        letter, count_letter, pw, spec_from, spec_to))
            valid += 1
    return valid


def valid_count_updated(triples):
    """Return count of valid passwords."""
    valid = 0
    for ((spec_first, spec_second), letter, passw) in triples:
        if (passw[spec_first - 1] == letter) ^ (passw[spec_second - 1] == letter):
            # print("found: {} {} times in {}, allows {}-{}".format(
            #        letter, count_letter, pw, spec_from, spec_to))
            valid += 1
    return valid


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    print("valid part 1: ", valid_count(yield_triples(puzzle_input)))
    print("valid part 2: ", valid_count_updated(yield_triples(puzzle_input)))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
