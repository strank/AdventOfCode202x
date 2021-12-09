"""
"""
from pathlib import Path
import sys
import itertools


def yield_dirs():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            dive_command = line.split()
            yield (dive_command[0][0], int(dive_command[1]))


def process_commands(input_commands):
    """
    """
    pos = depth = 0
    for cmd_dir, cmd_val in input_commands:
        if cmd_dir == 'f':
            pos += cmd_val
        elif cmd_dir == 'd':
            depth += cmd_val
        elif cmd_dir == 'u':
            depth -= cmd_val
    return (pos, depth)


def process_commands_updated(input_commands):
    """
    """
    pos = depth = aim = 0
    for cmd_dir, cmd_val in input_commands:
        if cmd_dir == 'f':
            pos += cmd_val
            depth += cmd_val * aim
        elif cmd_dir == 'd':
            aim += cmd_val
        elif cmd_dir == 'u':
            aim -= cmd_val
    return (pos, depth)


def main():
    input_commands = list(yield_dirs())
    position, depth = process_commands(input_commands)
    print(f"simple position {position} depth {depth} product {position * depth}")
    position, depth = process_commands_updated(input_commands)
    print(f"aiming position {position} depth {depth} product {position * depth}")


if __name__ == "__main__":
    main()
