"""
https://adventofcode.com/2020/day/13

>>> main()
Bus times wait:  222
Earliest magic timestamp:  408270049879073

>>> main(EXAMPLE_INPUT)
Bus times wait:  295
Earliest magic timestamp:  1068781
"""

from pathlib import Path
import sys

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
939
7,13,x,x,59,x,31,19
"""


def yield_lines(puzzle_input):
    """Yield all non-empty lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line:
            continue
        yield line


def process_bus_times(target_time, buses):
    """Return bus number and wait time to get the bus."""
    min_bus = min(buses, key=lambda bus: bus - target_time % bus)
    wait = min_bus - target_time % min_bus
    # print("min_bus {} wait {} target_time // min_bus {} next_arrival {}".format(
    #        min_bus, wait, target_time // min_bus, min_bus * ((target_time // min_bus) + 1)
    # ))
    return min_bus, wait


def find_bus_constellation(buses):
    """Return the earliest magic timestamp."""
    # print(len(buses))
    time, step = 0, 1
    # Possible improvement: find iteratively better step increases for candidates
    # by finding the periodicity of possible solutions for one bus at a time
    offset_buses = [(offset, int(ele)) for offset, ele in enumerate(buses) if ele != "x"]
    # print(offset_buses)
    for offset, bus in offset_buses:
        #orig_time = time
        #print("time {} step {} offset {} bus {}".format(time, step, offset, bus))
        time += step
        while (time + offset) % bus != 0:
            time += step
        first_time = time
        #print("first_time ", first_time)
        time += step
        while (time + offset) % bus != 0:
            time += step
        # print("orig_time {} --> {} --> first_time {} --> {} --> time {}".format(
        #        orig_time, first_time - orig_time, first_time, time - first_time, time
        # ))
        time, step = first_time, time - first_time
    return time


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = list(yield_lines(puzzle_input))
    target_time = int(input_list[0])
    buses = input_list[1].split(",")
    bus, wait = process_bus_times(target_time, [int(b) for b in buses if b != "x"])
    print("Bus times wait: ", bus * wait)
    timestamp = find_bus_constellation(buses)
    print("Earliest magic timestamp: ", timestamp)


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
