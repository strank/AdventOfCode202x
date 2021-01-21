"""
2-6 w: wkwwwfwwpvw
"""
from pathlib import Path
import itertools


def main1():
    input_path = Path(__file__).parent / "input"
    valid_count = 0
    with input_path.open() as input_file:
        for line in input_file:
            spec, letter, pw = line.split()
            letter = letter[0]
            spec_from, spec_to = (int(spec) for spec in spec.split("-"))
            count_letter = pw.count(letter)
            if spec_from <= count_letter <= spec_to:
                #print("found: {} {} times in {}, allows {}-{}".format(
                #        letter, count_letter, pw, spec_from, spec_to))
                valid_count += 1
    print("valid: ", valid_count)


def main2():
    input_path = Path(__file__).parent / "input"
    valid_count = 0
    with input_path.open() as input_file:
        for line in input_file:
            spec, letter, pw = line.split()
            letter = letter[0]
            spec_first, spec_second = (int(spec) for spec in spec.split("-"))
            if (pw[spec_first - 1] == letter) ^ (pw[spec_second - 1] == letter):
                #print("found: {} {} times in {}, allows {}-{}".format(
                #        letter, count_letter, pw, spec_from, spec_to))
                valid_count += 1
    print("valid: ", valid_count)


if __name__ == "__main__":
    main2()
