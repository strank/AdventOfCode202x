## https://adventofcode.com/2020/day/1
## Find 2 (and then 3) numbers that sum to 2020.
##
## TODO: translate python doctests to godot testing framework?
## >>> main()
## 2sum (634, 1386) product: 878724
## 3sum (765, 266, 989) product: 201251610
##
## >>> main(EXAMPLE_INPUT)
## 2sum (299, 1721) product: 514579
## 3sum (675, 979, 366) product: 241861950

extends MainLoop

var INPUT = read_input(self.get_script().get_path().get_base_dir() + "/input")

const EXAMPLE_INPUT = """
1721
979
366
299
675
1456
"""


## Return the contents of file at `input_path`
func read_input(input_path: String) -> String:
    var file = File.new()
    file.open(input_path, File.READ)
    var input = file.get_as_text().strip_edges()
    file.close()
    return input


## Return one int per line in `puzzle_input`
func get_ints(puzzle_input: String) -> Array[int]:
    var numbers: Array[int] = []
    for line in puzzle_input.strip_edges().split('\n'):
        numbers.append(line.to_int())
    return numbers


## Return tuple of two numbers that sum to 2020.
func find_2sum(in_numbers: Array[int]) -> Array[int]:
    var numbers = []
    for new_number in in_numbers:
        for number in numbers:
            if new_number + number == 2020:
                return [new_number, number]
        numbers.append(new_number)
    return []


## An iterator for returning combinations.
## (This would be perfect for a generator function if gdscript supported them.)
## see python's itertools.combinations
class Combinations:
    var arr: Array[int]
    var len_arr: int
    var indices: Array[int]
    var length: int

    func _init(array: Array, comb_length: int):
        self.len_arr = len(array)
        self.arr = array
        self.indices = range(comb_length)
        self.length = comb_length

    func _iter_init(_arg) -> bool:
        self.indices = range(self.length)
        return self.length <= self.len_arr

    func _iter_next(_arg) -> bool:
        var found_i = -1
        var to_check = range(length)
        to_check.reverse()
        for i in to_check:
            if self.indices[i] != i + self.len_arr - self.length:
                found_i = i
                break
        if found_i == -1:
            return false
        self.indices[found_i] += 1
        for j in range(found_i + 1, self.length):
            indices[j] = indices[j - 1] + 1
        return true

    func _iter_get(_arg) -> Array[int]:
        var result: Array[int] = []
        for index in indices:
            result.append(self.arr[index])
        return result


## Return the sum of an array of int.
func sum(arr: Array) -> int:
    var result = 0
    for ele in arr:
        result += ele
    return result


## Return array of `num` numbers that sum to target.
func find_sum(numbers: Array[int], num: int, target: int) -> Array[int]:
    for sum_numbers in Combinations.new(numbers, num):
        if sum(sum_numbers) == target:
            return sum_numbers
    return []


## Find solutions to both parts of the puzzle based on puzzle_input.
func main(puzzle_input: String):
    var numbers = get_ints(puzzle_input)
    var two_sum = find_2sum(numbers)
    print("2sum %s product: %s" % [two_sum, two_sum[0] * two_sum[1]])
    var three_sum = find_sum(numbers, 3, 2020)
    print("3sum %s product: %s" % [three_sum, three_sum[0] * three_sum[1] * three_sum[2]])


## constructor callback, here used as main entry-point.
func _initialize():
    main(INPUT if 'x' not in OS.get_cmdline_args() else EXAMPLE_INPUT)


## Request quitting main loop immediately by returning true.
func _process(_delta: float) -> bool:
    return true
