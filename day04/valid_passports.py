"""

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID) optional
"""
from pathlib import Path
import itertools
import functools
import re

REQUIRED = "byr iyr eyr hgt hcl ecl pid".split()


def gen_entries(line_generator):
    current_fields = {}
    for line in line_generator:
        line = line.strip()
        if line:
            current_fields.update((entry.split(":") for entry in line.split()))
        else:
            yield current_fields
            current_fields = {}
    if current_fields:
        yield current_fields


def count_valid(required_fields):
    input_path = Path(__file__).parent / "input"
    pp_count = 0
    with input_path.open() as input_file:
        for passport_dict in gen_entries(input_file):
            if validate(passport_dict, required_fields):
                pp_count += 1
                print("Valid: ", passport_dict)
            #else:
            #    print("INVALID: ", passport_dict)
    return pp_count


def validate(passport_dict, required_fields):
    """
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    """
    if not all((field in passport_dict for field in required_fields)):
        return False
    for (field, lower, upper) in (('byr', 1920, 2002), ('iyr', 2010, 2020), ('eyr', 2020, 2030)):
        field_value = int(passport_dict[field])
        if lower > field_value or field_value > upper:
            return False
    hgt = passport_dict['hgt']
    if hgt.endswith('cm'):
        hgt_num = int(hgt[:-2])
        if 150 > hgt_num or hgt_num > 193:
            return False
    elif hgt.endswith('in'):
        hgt_num = int(hgt[:-2])
        if 59 > hgt_num or hgt_num > 76:
            return False
    else:
        return False
    hcl = passport_dict['hcl']
    if not re.match(r"\#[a-f0-9]{6}", hcl):
        return False
    ecl = passport_dict['ecl']
    if ecl not in "amb blu brn gry grn hzl oth".split():
        return False
    pid = passport_dict['pid']
    if not re.match(r"[0-9]{9}", pid):
        return False
    return True


def main():
    print("valid passports: ", count_valid(REQUIRED))


if __name__ == "__main__":
    main()
