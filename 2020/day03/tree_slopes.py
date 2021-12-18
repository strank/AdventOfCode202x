"""
https://adventofcode.com/2020/day/3

3 right down 1 slope, count trees

>>> main()
trees on (3,1) slope:  265
product:  3154761400

>>> main(EXAMPLE_INPUT)
trees on (3,1) slope:  7
product:  336
"""

from pathlib import Path
import sys
import functools

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"""


def count_trees(lines, step):
    """Return count of trees encountered in `lines` when going down by `step`."""
    step_right = step[0]
    step_down = step[1]
    tree_count = 0
    x_pos = 0
    y_skip = 1
    for line in lines:
        y_skip -= 1
        if y_skip > 0:
            continue
        else:
            y_skip = step_down
        line = line.strip()
        x_pos = x_pos % len(line)
        #replace_with = "O"
        if line[x_pos] == "#":
            #replace_with = "X"
            tree_count += 1
        #print(line[:x_pos] + replace_with + line[x_pos + 1:])
        x_pos += step_right
    #print("trees: ", tree_count)
    return tree_count


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    lines = puzzle_input.strip().split('\n')
    print("trees on (3,1) slope: ", count_trees(lines, (3, 1)))
    answers = (count_trees(lines, step) for step in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)))
    product = functools.reduce(lambda a, b: a * b, answers)
    print("product: ", product)


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
