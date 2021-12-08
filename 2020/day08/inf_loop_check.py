"""
Detect infinite loop and track value of acc
"""
from pathlib import Path
import itertools
import functools
import re
import collections


def parse_program():
    """
    return program as list of (instr, value)
    """
    input_path = Path(__file__).parent / "input"
    program = []
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            print(line, line.split())
            opcode, value = line.split()
            program.append((opcode, int(value)))
    return program
    

def get_acc_value(program):
    acc = 0
    visited = set()
    ip = 0
    while True:
        if ip > len(program):
            assert False
        if ip in visited or ip == len(program):
            return acc
        visited.add(ip)
        opcode, value = program[ip]
        if opcode == "jmp":
            ip += value
        else:
            ip += 1
            if opcode == "acc":
                acc += value


def origins_for(index, backwards_map):
    directly_reachable_from = backwards_map[index]
    indirectly_reachable_from = set()
    for new_index in directly_reachable_from:
        indirectly_reachable_from.update(origins_for(new_index, backwards_map))
    return directly_reachable_from | indirectly_reachable_from


def fix_program(program):
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
    print("ORIGINS for program end: ", from_back)
    # go forward and change if any instruction could reach the from_back set
    ip = 0
    visited = set()
    while True:
        if ip > len(program):
            assert False
        if ip in visited or ip == len(program):
            break
        visited.add(ip)
        opcode, value = program[ip]
        if opcode == "jmp":
            # check if fix can work here:
            if (ip + 1) in from_back:
                program[ip] = ("nop", value) 
                break
            ip += value
        else:
            if opcode == "nop":
                # check if fix can work here:
                if (ip + value) in from_back:
                    program[ip] = ("jmp", value) 
                    break
            ip += 1
    return program


def main():
    program = parse_program()
    acc_value = get_acc_value(program)
    print("ACC value: ", acc_value)
    program = fix_program(program)
    print("FIXED ACC value: ", get_acc_value(program))


if __name__ == "__main__":
    main()
