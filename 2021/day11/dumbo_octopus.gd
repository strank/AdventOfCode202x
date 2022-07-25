## https://adventofcode.com/2021/day/11
## Flashing octopus simulation.
##
## TODO: translate python doctests to godot testing framework?
## >>> main()
##
## >>> main(EXAMPLE_INPUT)

extends MainLoop

var INPUT: String:
    get:
        return read_input(self.get_script().get_path().get_base_dir() + "/input")

const EXAMPLE_INPUT = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""


## Return the contents of file at `input_path`
func read_input(input_path: String) -> String:
    var file = File.new()
    file.open(input_path, File.READ)
    var input = file.get_as_text().strip_edges()
    file.close()
    return input


## 2D array but backed by a single array, index access is automatically translated
class IntArray2D:
    var numbers: Array[int] = []
    var col_count: int = 0
    var row_count: int = 0


    ## initialize array by something that can be iterated, nested
    func _init(iter_of_iter):
        for inner_iter in iter_of_iter:
            var current_col_count: int = 0
            for ele in inner_iter:
                self.numbers.append(ele.to_int())
                current_col_count += 1
            if self.col_count == 0:
                self.col_count = current_col_count
            else:
                assert(col_count == current_col_count)
        row_count = self.numbers.size() / col_count


    ## return int at position (row, col), 0-based indexing
    func get_at(row: int, col: int) -> int:
        return numbers[row * col_count + col]


    ## set int at position (row, col), 0-based indexing
    func set_at(row: int, col: int, value: int):
        numbers[row * col_count + col] = value


    ## return indices of all orthogonal and diagonal neighbours of (row, col)
    func get_neighbour_indices(row: int, col: int) -> Array:
        # TODO: also return diagonal!!!
        var result: Array = []
        if col > 0:
            result.append([row, col - 1])
            if row > 0:
                result.append([row - 1, col - 1])
            if row < row_count - 1:
                result.append([row + 1, col - 1])
        if col < col_count - 1:
            result.append([row, col + 1])
            if row > 0:
                result.append([row - 1, col + 1])
            if row < row_count - 1:
                result.append([row + 1, col + 1])
        if row > 0:
            result.append([row - 1, col])
        if row < row_count - 1:
            result.append([row + 1, col])
        return result


    ## return all entries that have a specific value
    func find_value_indices(value: int) -> Array:
        var result: Array = []
        for col in range(col_count):
            for row in range(row_count):
                if value == self.get_at(row, col):
                    result.append([row, col])
        return result


    ## increase all elements by one
    func increase_all():
        for index in numbers.size():
            numbers[index] += 1


## Return 2d array of digits in `puzzle_input`
func get_int_array_2d(puzzle_input: String) -> IntArray2D:
    var number_lines := []
    for line in puzzle_input.strip_edges().split('\n'):
        number_lines.append(line)
    return IntArray2D.new(number_lines)


## Simulate `num_steps` steps and return the number of flashes observed
## or the first time step at which all nodes flash, if `all_flash_step` is true
func simulate(arr: IntArray2D, num_steps: int, all_flash_step: bool) -> int:
    var flashes = 0
    for step in num_steps:
        arr.increase_all()
        var flash_stack = arr.find_value_indices(10)
        var index = 0
        while index < flash_stack.size():
            var coord = flash_stack[index]
            var neighs = arr.get_neighbour_indices(coord[0], coord[1])
            for neigh in neighs:
                var val = arr.get_at(neigh[0], neigh[1])
                arr.set_at(neigh[0], neigh[1], val + 1)
                if val == 9:
                    flash_stack.push_back(neigh)
            index += 1
        for coord in flash_stack:
            arr.set_at(coord[0], coord[1], 0)
        if all_flash_step and flash_stack.size() == arr.numbers.size():
            return step + 1
        flashes += flash_stack.size()
    return flashes


## Find solutions to both parts of the puzzle based on puzzle_input.
func main(puzzle_input: String):
    var numbers = get_int_array_2d(puzzle_input)
    var flashes = simulate(numbers, 100, false)
    print("flashes after %s steps: %s" % [100, flashes])
    var all_flash_step = simulate(numbers, 1000, true)
    print("first all flash timestep: %s" % (100 + all_flash_step))


## constructor callback, here used as main entry-point.
func _initialize():
    main(INPUT if 'x' not in OS.get_cmdline_args() else EXAMPLE_INPUT)


## Request quitting main loop immediately by returning true.
func _process(_delta: float) -> bool:
    return true
