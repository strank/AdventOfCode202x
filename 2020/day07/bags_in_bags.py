"""
https://adventofcode.com/2020/day/7

How many colors of bags can eventually contain at least one shiny gold bag

>>> main()
possible outer colors:  235
num bags inside shiny gold bag: 158493

>>> main(EXAMPLE_INPUT)
possible outer colors:  4
num bags inside shiny gold bag: 32
"""

from pathlib import Path
import sys
import collections

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""

TARGET = "shiny gold bag"
NO_BAG = "other bag"


def get_bag_mappings(puzzle_input):
    """Return two bag mappings: bag_mapping, inside_out_mapping."""
    bag_mapping = {}
    inside_out_mapping = collections.defaultdict(set)
    for line in puzzle_input.strip().split('\n'):
        line = line.strip().strip(".")
        outside, inside = line.split("s contain ")
        inside = [ele.strip("s") for ele in inside.split(", ")]
        inside = {bag_color: num_bags for num_bags, bag_color in
                  (bag_spec.split(" ", 1) for bag_spec in inside)}
        bag_mapping[outside] = inside
        for bag in inside:
            inside_out_mapping[bag].add(outside)
    # print(bag_mapping)
    return bag_mapping, inside_out_mapping


def outer_colors(inside_out_mapping):
    """Return list of all possible outer colors for a shiny gold bag."""
    all_outer = prev_outer = set(inside_out_mapping[TARGET])
    while True:
        #print("prev {} all {}".format(prev_outer, all_outer))
        new_outer = set()
        for bag_type in prev_outer:
            for outer_type in inside_out_mapping[bag_type]:
                new_outer.add(outer_type)
        if all_outer.issuperset(new_outer):
            return all_outer
        all_outer |= new_outer  # update
        prev_outer = new_outer


BAG_MAP = None
# @functools.lru_cache(maxsize=None)
def num_inside_bags(target):
    """
    Return number of all contained bags inside target bag
    """
    sum_bags = 0
    bag_to_num_map = BAG_MAP[target]
    for bag_type in bag_to_num_map:
        if bag_type == NO_BAG:
            return 0
        num_bags = int(bag_to_num_map[bag_type])
        #print("{} times {} inside {}".format(bag_type, num_bags, target))
        sum_bags += num_bags + num_inside_bags(bag_type) * num_bags
    return sum_bags


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    bag_mapping, inside_out_mapping = get_bag_mappings(puzzle_input)
    print("possible outer colors: ", len(outer_colors(inside_out_mapping)))
    global BAG_MAP
    BAG_MAP = bag_mapping
    print("num bags inside {}: {}".format(TARGET, num_inside_bags(TARGET)))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
