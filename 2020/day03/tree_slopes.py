"""
3 right down 1 slope, count trees
"""
from pathlib import Path
import itertools
import functools


def count_trees(step):
    step_right = step[0]
    step_down = step[1]
    input_path = Path(__file__).parent / "input"
    tree_count = 0
    x_pos = 0
    y_skip = 1
    with input_path.open() as input_file:
        for line in input_file:
            y_skip -= 1
            if y_skip > 0:
                continue
            else:
                y_skip = step_down
            line = line.strip()
            #print(line)
            x_pos = x_pos % len(line)
            replace_with = "O"
            if line[x_pos] == "#":
                replace_with = "X"
                tree_count += 1
            #print(line[:x_pos] + replace_with + line[x_pos + 1:])
            x_pos += step_right
    print("trees: ", tree_count)
    return tree_count


def main():
    answers = (count_trees(step) for step in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)))
    product = functools.reduce(lambda a, b: a * b, answers)
    print("product: ", product)


if __name__ == "__main__":
    main()
