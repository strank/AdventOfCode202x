"""
"""
from pathlib import Path
import sys
import itertools


def yield_lines():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            if not line:
                continue
            yield line


def calc_one_counts(bin_strings):
    one_counts = [0] * len(bin_strings[0])
    for bin_string in bin_strings:
        for index, bin_digit in enumerate(bin_string):
            if bin_digit == '1':
                one_counts[index] += 1
    return one_counts


def process_bin_strings(bin_strings, one_counts):
    """
    """
    half_num_bins = len(bin_strings) // 2
    gamma = "".join('1' if count > half_num_bins else '0' for count in one_counts)
    epsilon = "".join('1' if digit == '0' else '0' for digit in gamma)
    return (int(gamma, base=2), int(epsilon, base=2))


def process_bin_strings_again(bin_strings):
    """
    """
    oxy_candidates = bin_strings[:]
    co2_candidates = bin_strings[:]
    len_num = len(bin_strings[0])
    for index in range(len_num):
        if len(oxy_candidates) > 1:
            one_count = len(list(filter(lambda x: x[index] == '1', oxy_candidates)))
            half_num_bins = len(oxy_candidates) // 2
            target = '1' if one_count >= half_num_bins else '0'
            oxy_candidates = [o for o in oxy_candidates if o[index] == target]
        if len(co2_candidates) > 1:
            one_count = len(list(filter(lambda x: x[index] == '1', co2_candidates)))
            half_num_bins = len(co2_candidates) // 2
            target = '1' if one_count < half_num_bins else '0'
            co2_candidates = [c for c in co2_candidates if c[index] == target]
        #print("oxy " + " ".join(oxy_candidates[:10]))
        #print("co2 " + " ".join(co2_candidates[:10]))
    assert len(oxy_candidates) == 1 and len(co2_candidates) == 1
    return (int(oxy_candidates[0], base=2), int(co2_candidates[0], base=2))


def main():
    bin_strings = list(yield_lines())
    one_counts = calc_one_counts(bin_strings)
    gamma, epsilon = process_bin_strings(bin_strings, one_counts)
    print(f"gamma {gamma} epsilon {epsilon} product {gamma * epsilon}")
    oxy, co2 = process_bin_strings_again(bin_strings)
    print(f"oxy {oxy} co2 {co2} product {oxy * co2}")


if __name__ == "__main__":
    main()
