"""
https://adventofcode.com/2020/day/8

Detect infinite loop and track value of acc

>>> main()
ACC value:  1654
FIXED ACC value:  833

>>> main(EXAMPLE_INPUT)
ACC value:  5
FIXED ACC value:  8
"""

from pathlib import Path
import sys
import collections

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""


def parse_program(puzzle_input):
    """Return program as list of (instr, value) tuples."""
    program = []
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        #print(line, line.split())
        opcode, value = line.split()
        program.append((opcode, int(value)))
    return program


def get_acc_value(program):
    """Return value of acc."""
    acc = 0
    visited = set()
    i_p = 0
    while True:
        if i_p > len(program):
            assert False
        if i_p in visited or i_p == len(program):
            return acc
        visited.add(i_p)
        opcode, value = program[i_p]
        if opcode == "jmp":
            i_p += value
        else:
            i_p += 1
            if opcode == "acc":
                acc += value


def origins_for(index, backwards_map):
    """Return set of reachable origins."""
    directly_reachable_from = backwards_map[index]
    indirectly_reachable_from = set()
    for new_index in directly_reachable_from:
        indirectly_reachable_from.update(origins_for(new_index, backwards_map))
    return directly_reachable_from | indirectly_reachable_from


def fix_program(program):
    """Return `program` fixed."""
    backwards_map = collections.defaultdict(set)
    fixed_backwards_map = collections.defaultdict(set)
    for index, (opcode, value) in enumerate(program):
        if opcode == "acc":
            backwards_map[index + 1].add(index)
            fixed_backwards_map[index + 1].add(index)
        elif opcode == "nop":
            backwards_map[index + 1].add(index)
            fixed_backwards_map[index + value].add(index)
        elif opcode == "jmp":
            backwards_map[index + value].add(index)
            fixed_backwards_map[index + 1].add(index)
    # recursively fill set of indices that can reach the end:
    from_back = origins_for(len(program), backwards_map)
    #print("ORIGINS for program end: ", from_back)
    # go forward and change if any instruction could reach the from_back set
    i_p = 0
    visited = set()
    while True:
        if i_p > len(program):
            assert False
        if i_p in visited or i_p == len(program):
            break
        visited.add(i_p)
        opcode, value = program[i_p]
        if opcode == "jmp":
            # check if fix can work here:
            if i_p + 1 in from_back:
                program[i_p] = ("nop", value)
                break
            i_p += value
        else:
            if opcode == "nop":
                # check if fix can work here:
                if i_p + value in from_back:
                    program[i_p] = ("jmp", value)
                    break
            i_p += 1
    return program


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    program = parse_program(puzzle_input)
    acc_value = get_acc_value(program)
    print("ACC value: ", acc_value)
    program = fix_program(program)
    print("FIXED ACC value: ", get_acc_value(program))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
