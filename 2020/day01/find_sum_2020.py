from itertools import combinations


def main_2sum():
    numbers = []
    with open("input") as inputfile:
        for line in inputfile:
            new_number = int(line)
            for number in numbers:
                if new_number + number == 2020:
                    print(new_number, " * ", number, " = ", new_number * number)
                    return
            numbers.append(new_number)
                

def main_3sum():
    numbers = []
    with open("input") as inputfile:
        for line in inputfile:
            new_number = int(line)
            for two_numbers in combinations(numbers, 2):
                number_one, number_two = list(two_numbers)
                if new_number + sum(two_numbers) == 2020:
                    print(new_number, " * ", number_one, " * ", number_two, " = ", new_number * number_one * number_two)
                    return
            numbers.append(new_number)
                

if __name__ == "__main__":
    main_3sum()
