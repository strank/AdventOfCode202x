"""
https://adventofcode.com/2020/day/11

>>> main()
Occupied seats when stable, adjacent seats, 4:  2338
Occupied seats when stable, visible seats, 5:  2134

>>> main(EXAMPLE_INPUT)
Occupied seats when stable, adjacent seats, 4:  37
Occupied seats when stable, visible seats, 5:  26
"""

from pathlib import Path
import sys
import itertools
import functools

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"""


def yield_lines(puzzle_input):
    """Yield all non-empty lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield line


class SeatMapping:
    """Represent a seat mapping as a list of lines. Memoize neighbor lookup functions."""
    coord_displacements = list(itertools.product((0, 1, -1), repeat=2))[1:]

    def __init__(self, seating):
        self.seating = seating
        self.row_max = len(seating) - 1
        self.col_max = len(seating[0]) - 1

    @functools.lru_cache(maxsize=None)
    def list_neighbors(self, row, col):
        """Return a list of neighbors for seat `row` `col`."""
        neighs = [(row + r, col + c) for r, c in SeatMapping.coord_displacements]
        neighs = [(r, c) for r, c in neighs if 0 <= r <= self.row_max and 0 <= c <= self.col_max]
        return neighs

    @functools.lru_cache(maxsize=None)
    def list_visible_neighbors(self, row, col):
        """Return a list of neighbors visible from seat `row` `col`."""
        neighs = [(row + r, col + c) for r, c in SeatMapping.coord_displacements]
        neighs_dir_zipped = zip(neighs, SeatMapping.coord_displacements)
        for n_index, ((rrr, ccc), (d_r, d_c)) in enumerate(neighs_dir_zipped):
            while (0 <= rrr <= self.row_max
                    and 0 <= ccc <= self.col_max
                    and self.seating[rrr][ccc] == "."):
                rrr, ccc = rrr + d_r, ccc + d_c
            neighs[n_index] = (rrr, ccc)
        neighs = [(r, c) for r, c in neighs if 0 <= r <= self.row_max and 0 <= c <= self.col_max]
        return neighs


def check_rules(seating, row, col, list_neighbors, max_occ=4):
    """Return new value for cell (row, col) according to the rules."""
    if seating[row][col] == ".":
        return "."
    sum_occupied = sum(1 if seating[r][c] == "#" else 0
                       for r, c in list_neighbors(row, col))
    if sum_occupied == 0 and seating[row][col] == "L":
        return "#"
    if sum_occupied >= max_occ and seating[row][col] == "#":
        return "L"
    return seating[row][col]


def process_seating(seating, neighbor_func, max_occ=4):
    """Repeatedly apply seating rules until no change."""
    prev_seating = seating
    while True:
        new_seating = []
        for row_index, row in enumerate(prev_seating):
            new_row = []
            for col_index, _col in enumerate(row):
                new_row.append(check_rules(prev_seating, row_index, col_index,
                                           neighbor_func, max_occ))
            new_seating.append(''.join(new_row))
        if prev_seating == new_seating:
            return new_seating
        prev_seating = new_seating


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = list(yield_lines(puzzle_input))
    seat_mapping = SeatMapping(input_list)
    final_seating = process_seating(input_list, seat_mapping.list_neighbors, max_occ=4)
    occupied_count = ''.join(final_seating).count("#")
    print("Occupied seats when stable, adjacent seats, 4: ", occupied_count)
    final_seating = process_seating(input_list, seat_mapping.list_visible_neighbors, max_occ=5)
    occupied_count = ''.join(final_seating).count("#")
    print("Occupied seats when stable, visible seats, 5: ", occupied_count)


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
