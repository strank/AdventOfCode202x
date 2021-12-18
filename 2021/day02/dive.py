"""
https://adventofcode.com/2021/day/2

>>> main()
simple position 2003 depth 872 product 1746616
aiming position 2003 depth 869681 product 1741971043

>>> main(EXAMPLE_INPUT)
simple position 15 depth 10 product 150
aiming position 15 depth 60 product 900
"""

from pathlib import Path
import sys

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""


def yield_dirs(puzzle_input):
    """Yield a tuple of direction as single character and amount as int per line in input."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        dive_command = line.split()
        yield (dive_command[0][0], int(dive_command[1]))


def process_commands(input_commands):
    """Return the position and depth after procesing the `input_commands`."""
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
    """Return the position and depth after procesing the `input_commands`."""
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


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_commands = list(yield_dirs(puzzle_input))
    position, depth = process_commands(input_commands)
    print(f"simple position {position} depth {depth} product {position * depth}")
    position, depth = process_commands_updated(input_commands)
    print(f"aiming position {position} depth {depth} product {position * depth}")


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
