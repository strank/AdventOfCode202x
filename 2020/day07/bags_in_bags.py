"""
How many colors of bags can eventually contain at least one shiny gold bag
"""
from pathlib import Path
import itertools
import functools
import re
import collections


TARGET = "shiny gold bag"
NO_BAG = "other bag"


def get_bag_mappings():
    """
    Return two bag mappings: bag_mapping, inside_out_mapping
    """
    input_path = Path(__file__).parent / "input"
    bag_mapping = {}
    inside_out_mapping = collections.defaultdict(set)
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip().strip(".")
            outside, inside = line.split("s contain ")
            inside = [ele.strip("s") for ele in inside.split(", ")]
            inside = {bag_color: num_bags for num_bags, bag_color in
                    (bag_spec.split(" ", 1) for bag_spec in inside)}
            bag_mapping[outside] = inside
            for bag in inside:
                inside_out_mapping[bag].add(outside)
    #print(bag_mapping)
    return bag_mapping, inside_out_mapping


def outer_colors(inside_out_mapping):
    """
    Return list of all possible outer colors for a shiny gold bag
    """
    all_outer = prev_outer = set(inside_out_mapping[TARGET])
    while True:
        print("prev {} all {}".format(prev_outer, all_outer))
        new_outer = set()
        for bag_type in prev_outer:
            for outer_type in inside_out_mapping[bag_type]:
                new_outer.add(outer_type)
        if all_outer.issuperset(new_outer):
            return all_outer
        all_outer |= new_outer # update
        prev_outer = new_outer


bag_map = None
#@functools.lru_cache(maxsize=None)
def num_inside_bags(target):
    """
    Return number of all contained bags inside target bag
    """
    sum_bags = 0
    bag_to_num_map = bag_map[target]
    for bag_type in bag_to_num_map:
        if bag_type == NO_BAG:
            return 0
        num_bags = int(bag_to_num_map[bag_type])
        #print("{} times {} inside {}".format(bag_type, num_bags, target))
        sum_bags += num_bags + num_inside_bags(bag_type) * num_bags
    return sum_bags


def main():
    bag_mapping, inside_out_mapping = get_bag_mappings()
    print("possible outer colors: ", len(outer_colors(inside_out_mapping)))
    global bag_map
    bag_map = bag_mapping
    print("num bags inside {}: {}".format(TARGET, num_inside_bags(TARGET)))


if __name__ == "__main__":
    main()
