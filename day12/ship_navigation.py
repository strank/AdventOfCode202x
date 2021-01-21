"""
"""
from pathlib import Path
import itertools
import functools
import re
import collections
import math


def yield_nav_commands():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield line[0], int(line[1:])


HEADINGS = ["E", "S", "W", "N"]

def process_nav_commands(nav_commands, heading="E"):
    """
    Apply all nav commands and return final position
    """
    current = [0, 0]
    for command, value in nav_commands:
        if command == "F":
            command = heading
        if command == "N":
            current[1] += value
        elif command == "S":
            current[1] -= value
        elif command == "E":
            current[0] += value
        elif command == "W":
            current[0] -= value
        elif command == "R":
            heading = HEADINGS[(HEADINGS.index(heading) + value // 90) % len(HEADINGS)]
        elif command == "L":
            heading = HEADINGS[(HEADINGS.index(heading) - value // 90) % len(HEADINGS)]
    return current


def rotate_vector(vector, degrees):
    angle = math.radians(degrees)
    return [
        round(vector[0] * math.cos(angle) - vector[1] * math.sin(angle)),
        round(vector[0] * math.sin(angle) + vector[1] * math.cos(angle)),
    ]


def process_waypoint_nav_commands(nav_commands, waypoint):
    """
    Apply all nav commands and return final position. Use waypoint movement.
    """
    current = [0, 0]
    for command, value in nav_commands:
        if command == "F":
            current = [c + w * value for c, w in zip(current, waypoint)]
        elif command == "N":
            waypoint[1] += value
        elif command == "S":
            waypoint[1] -= value
        elif command == "E":
            waypoint[0] += value
        elif command == "W":
            waypoint[0] -= value
        else:
            waypoint = rotate_vector(waypoint, (1 if command == "L" else -1) * value)
    return current


def main():
    input_list = list(yield_nav_commands())
    final_pos = process_nav_commands(input_list, heading="E")
    manhattan = abs(final_pos[0]) + abs(final_pos[1])
    print("Manhattan distance of final pos: ", manhattan)
    final_pos = process_waypoint_nav_commands(input_list, waypoint=[10, 1])
    manhattan = abs(final_pos[0]) + abs(final_pos[1])
    print("Manhattan distance of final pos with waypoint: ", manhattan)


if __name__ == "__main__":
    main()
