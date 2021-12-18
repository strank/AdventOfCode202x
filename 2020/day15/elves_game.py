"""
https://adventofcode.com/2020/day/15

>>> main()
Last spoken at turn 2020: 1194
Last spoken at turn 30000000: 48710

>>> main(EXAMPLE_INPUT)
Last spoken at turn 2020: 436
Last spoken at turn 30000000: 175594
"""

import sys
import collections

INPUT = "6,13,1,15,2,0"

EXAMPLE_INPUT = "0,3,6"

MAX1 = 2020
MAX2 = 30000000


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    input_list = [int(x) for x in puzzle_input.split(",")]
    history = collections.defaultdict(int)
    last = input_list[0]
    for turn, num in enumerate(input_list):
        history[last] = turn
        last = num
    for turn in range(len(input_list), MAX1):
        stored_turn = history[last]
        history[last] = turn
        last = 0 if stored_turn == 0 else turn - stored_turn
    print("Last spoken at turn {}: {}".format(MAX1, last))
    for turn in range(MAX1, MAX2):
        stored_turn = history[last]
        history[last] = turn
        last = 0 if stored_turn == 0 else turn - stored_turn
    print("Last spoken at turn {}: {}".format(MAX2, last))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
