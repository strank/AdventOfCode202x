"""
"""
from pathlib import Path
import itertools
import functools
import re
import collections
import math


def yield_lines():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield line


def process_memory(input_lines):
    """
    """
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
    x_index = mask.find("X")
    if x_index == -1:
        yield mask
    else:
        yield from gen_floatings(mask[:x_index] + "0" + mask[x_index + 1:])
        yield from gen_floatings(mask[:x_index] + "1" + mask[x_index + 1:])

def process_memory_floating(input_lines):
    """
    """
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


def main():
    input_list = list(yield_lines())
    memory = process_memory(input_list)
    print("sum of memory: ", sum(memory.values()))
    memory = process_memory_floating(input_list)
    print("sum of memory: ", sum(memory.values()))


if __name__ == "__main__":
    main()
