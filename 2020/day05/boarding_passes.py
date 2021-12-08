"""
this airline uses binary space partitioning to seat people.
 A seat might be specified like FBFBBFFRLR, where F means "front", B means "back",
L means "left", and R means "right".

The first 7 characters will either be F or B;
 these specify exactly one of the 128 rows on the plane (numbered 0 through 127).

The last three characters will be either L or R;
these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7).
The same process as above proceeds again, this time with only three steps.
L means to keep the lower half, while R means to keep the upper half.
"""
from pathlib import Path
import itertools
import functools
import re


def pairwise(iterable):
    "s -> (s0,s1), (s1,s2), (s2, s3), ..."
    a, b = itertools.tee(iterable)
    next(b, None)
    return zip(a, b)


def all_seat_ids():
    input_path = Path(__file__).parent / "input"
    all_ids = []
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            lower_row, upper_row = 0, 127
            lower_col, upper_col = 0, 7
            row_spec = line[:7]
            col_spec = line[7:]
            #print(line)
            #print("row {}-{} col {}-{}".format(lower_row, upper_row, lower_col, upper_col))
            for letter in row_spec:
                if letter == "F":
                    upper_row -= (upper_row - lower_row) // 2 + 1
                elif letter == "B":
                    lower_row += (upper_row - lower_row) // 2 + 1
                else:
                    assert False
                #print("row {}-{} col {}-{}".format(lower_row, upper_row, lower_col, upper_col))
            for letter in col_spec:
                if letter == "L":
                    upper_col -= (upper_col - lower_col) // 2 + 1
                elif letter == "R":
                    lower_col += (upper_col - lower_col) // 2 + 1
                else:
                    assert False
                #print("row {}-{} col {}-{}".format(lower_row, upper_row, lower_col, upper_col))
            assert upper_row == lower_row
            assert upper_col == lower_col
            new_id = lower_row * 8 + lower_col
            all_ids.append(new_id)
    all_ids.sort()
    return all_ids


def main():
    seat_ids = all_seat_ids()
    print("highest seat ID: ", max(seat_ids))
    for a, b in pairwise(seat_ids):
        if b - a == 2:
            print("my seat ID: ", a + 1)


if __name__ == "__main__":
    main()
