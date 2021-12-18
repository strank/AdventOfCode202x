"""
https://adventofcode.com/2020/day/14

>>> main()
sum of memory:  7440382076205
sum of memory:  4200656704538

>>> main(EXAMPLE_INPUT)
sum of memory:  165
sum of memory:  208
"""

from pathlib import Path
import sys

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"""

EXAMPLE_INPUT_PART_2 = """
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"""


def yield_lines(puzzle_input):
    """Yield all non-empty lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield line


def process_memory(input_lines):
    """Return memory after processing `input_lines`."""
    mem = {}
    or_mask = and_mask = 1
    for line in input_lines:
        lhs, rhs = line.split(" = ")
        if lhs == "mask":
            or_mask = int(rhs.replace("X", "0"), base=2)
            and_mask = int(rhs.replace("X", "1"), base=2)
        else:
            value = int(rhs)
            value &= and_mask
            value |= or_mask
            mem[int(lhs[4:-1])] = value
    return mem


def gen_floatings(mask):
    """Yield all possible floating bit versions of `mask`."""
    x_index = mask.find("X")
    if x_index == -1:
        yield mask
    else:
        yield from gen_floatings(mask[:x_index] + "0" + mask[x_index + 1:])
        yield from gen_floatings(mask[:x_index] + "1" + mask[x_index + 1:])


def process_memory_floating(input_lines):
    """Return memory after processing `input_lines`."""
    mem = {}
    or_mask = 0
    and_masks = or_masks = []
    for line in input_lines:
        lhs, rhs = line.split(" = ")
        if lhs == "mask":
            or_mask = int(rhs.replace("X", "0"), base=2)
            floatings = list(gen_floatings(rhs.replace("0", "_").replace("1", "_")))
            and_masks = [int(v.replace("_", "1"), base=2) for v in floatings]
            or_masks = [int(v.replace("_", "0"), base=2) for v in floatings]
        else:
            address = int(lhs[4:-1]) | or_mask
            for (and_m, or_m) in zip(and_masks, or_masks):
                fl_address = address
                fl_address &= and_m
                fl_address |= or_m
                mem[fl_address] = int(rhs)
    return mem


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = list(yield_lines(puzzle_input))
    memory = process_memory(input_list)
    print("sum of memory: ", sum(memory.values()))
    if puzzle_input == EXAMPLE_INPUT:  # special case with different example for part 2
        input_list = list(yield_lines(EXAMPLE_INPUT_PART_2))
    memory = process_memory_floating(input_list)
    print("sum of memory: ", sum(memory.values()))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
