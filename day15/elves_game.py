"""
"""
from pathlib import Path
import itertools
import functools
import re
import collections
import math

INPUT = [int(x) for x in "6,13,1,15,2,0".split(",")]
MAX1 = 2020
MAX2 = 30000000


def main():
    history = collections.defaultdict(int)
    last = INPUT[0]
    for turn, num in enumerate(INPUT):
        history[last] = turn
        last = num
    for turn in range(len(INPUT), MAX1):
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
    main()
