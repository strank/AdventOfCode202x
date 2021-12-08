"""
"""
from pathlib import Path
import itertools
import functools
import re
import collections


def yield_non_empty_lines():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield line


class SeatMapping:
    coord_displacements = list(itertools.product((0, 1, -1), repeat=2))[1:]

    def __init__(self, seating):
        self.seating = seating
        self.row_max = len(seating) - 1
        self.col_max = len(seating[0]) - 1

    @functools.lru_cache(maxsize=None)
    def list_neighbors(self, row, col):
        neighs = [(row + r, col + c) for r, c in SeatMapping.coord_displacements]
        neighs = [(r, c) for r, c in neighs if 0 <= r <= self.row_max and 0 <= c <= self.col_max]
        return neighs

    @functools.lru_cache(maxsize=None)
    def list_visible_neighbors(self, row, col):
        neighs = [(row + r, col + c) for r, c in SeatMapping.coord_displacements]
        for n_index, ((r, c), (dr, dc)) in enumerate(zip(neighs, SeatMapping.coord_displacements)):
            while 0 <= r <= self.row_max and 0 <= c <= self.col_max and self.seating[r][c] == ".":
                r, c = r + dr, c + dc
            neighs[n_index] = (r, c)
        neighs = [(r, c) for r, c in neighs if 0 <= r <= self.row_max and 0 <= c <= self.col_max]
        return neighs


def check_rules(seating, row, col, list_neighbors, max_occ=4):
    """Return new value for cell (row, col) according to the rules"""
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
    """
    Repeatedly apply seating rules until no change
    """
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


def main():
    input_list = list(yield_non_empty_lines())
    seat_mapping = SeatMapping(input_list)
    final_seating = process_seating(input_list, seat_mapping.list_neighbors, max_occ=4)
    occupied_count = ''.join(final_seating).count("#")
    print("Occupied seats when stable, adjacent seats, 4: ", occupied_count)
    final_seating = process_seating(input_list, seat_mapping.list_visible_neighbors, max_occ=5)
    occupied_count = ''.join(final_seating).count("#")
    print("Occupied seats when stable, visible seats, 5: ", occupied_count)


if __name__ == "__main__":
    main()
