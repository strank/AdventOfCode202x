"""
The form asks a series of 26 yes-or-no questions marked a through z.
All you need to do is identify the questions for which anyone in your group answers "yes".

"""
from pathlib import Path
import itertools
import functools
import re


def all_answer_counts(anyone):
    input_path = Path(__file__).parent / "input"
    all_counts = []
    with input_path.open() as input_file:
        current_set = None
        for line in input_file:
            line = line.strip()
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


def main():
    print("sum of answer counts: ", sum(all_answer_counts(anyone=False)))


if __name__ == "__main__":
    main()
