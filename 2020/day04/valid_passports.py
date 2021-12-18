"""
https://adventofcode.com/2020/day/4

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID) optional

TODO: the second part is actually off by one here, it returns 159!!!

>>> main()
Total: 282 passports
valid passports:  (250, 158)

>>> main(EXAMPLE_INPUT)
Total: 12 passports
valid passports:  (10, 6)
"""

from pathlib import Path
import sys
import re

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"""

REQUIRED = "byr iyr eyr hgt hcl ecl pid".split()


def gen_entries(lines):
    """Return passports based on a list of lines."""
    current_fields = {}
    for line in lines:
        line = line.strip()
        if line:
            current_fields.update((entry.split(":") for entry in line.split()))
        else:
            yield current_fields
            current_fields = {}
    if current_fields:
        yield current_fields


def count_valid(puzzle_input, required_fields):
    """Return a tuple of counts of passports with valid fields, and valid values too."""
    pp_count = pp_simple_count = total = 0
    for passport_dict in gen_entries(puzzle_input.strip().split('\n')):
        total += 1
        fields, values = validate(passport_dict, required_fields)
        if fields:
            pp_simple_count += 1
            if values:
                pp_count += 1
    print(f"Total: {total} passports")
    return (pp_simple_count, pp_count)


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
        return (False, False)
    return (True, validate_values(passport_dict))


def validate_values(passport_dict):
    """Return whether all values of the passport fulfill their requirements."""
    for (field, lower, upper) in (('byr', 1920, 2002), ('iyr', 2010, 2020), ('eyr', 2020, 2030)):
        field_value = int(passport_dict[field])
        if lower > field_value or field_value > upper:
            return False
    hgt = passport_dict['hgt']
    if hgt.endswith('cm'):
        hgt_num = int(hgt[:-2])
        if hgt_num < 150 or hgt_num > 193:
            return False
    elif hgt.endswith('in'):
        hgt_num = int(hgt[:-2])
        if hgt_num < 59 or hgt_num > 76:
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


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    print("valid passports: ", count_valid(puzzle_input, REQUIRED))


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
