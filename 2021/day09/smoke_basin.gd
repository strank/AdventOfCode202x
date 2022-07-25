## https://adventofcode.com/2021/day/9
## Find minima in a matrix of digits.
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
2199943210
3987894921
9856789892
8767896789
9899965678
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
    func get(row: int, col: int) -> int:
        return numbers[row * col_count + col]


    ## set int at position (row, col), 0-based indexing
    func set(row: int, col: int, value: int):
        numbers[row * col_count + col] = value


    ## return indices of all orthogonal neighbours of (row, col)
    func get_neighbour_indices(row: int, col: int) -> Array:
        var result: Array = []
        if col > 0:
            result.append([row, col - 1])
        if col < col_count - 1:
            result.append([row, col + 1])
        if row > 0:
            result.append([row - 1, col])
        if row < row_count - 1:
            result.append([row + 1, col])
        return result


    ## return all entries that are lower than all their neighbors
    func find_minima_indices() -> Array:
        var result: Array = []
        for col in range(col_count):
            for row in range(row_count):
                var num = self.get(row, col)
                var self_get = self.get
                var small_neighbours = self.get_neighbour_indices(row, col).map(
                        func (inds): return self_get.call(inds[0], inds[1])
                ).filter(
                        func (other_num): return other_num <= num
                )
                if small_neighbours.size() == 0:
                    result.append([row, col])
        return result


    ## replace all occurrences of old with new (actually not needed)
    func replace(old: int, new: int):
        for index in numbers.size():
            if numbers[index] == old:
                numbers[index] = new


    ## flood fill starting from a point, filling with value, until reaching the boundary value
    func flood_fill(coord: Array, value: int, boundary: int):
        var num = self.get(coord[0], coord[1]) 
        if num == boundary or num == value:
            return
        self.set(coord[0], coord[1], value)
        for neigh_inds in self.get_neighbour_indices(coord[0], coord[1]):
            self.flood_fill(neigh_inds, value, boundary)


## Return 2d array of digits in `puzzle_input`
func get_int_array_2d(puzzle_input: String) -> IntArray2D:
    var number_lines := []
    for line in puzzle_input.strip_edges().split('\n'):
        number_lines.append(line)
    return IntArray2D.new(number_lines)


## Flood fill areas in the array based on the points, and the boundary, return a list of sizes of filled areas
## TODO: expose set data structure to gdscript and use a set-based stack instead of recursion
func get_basin_sizes(arr: IntArray2D, points: Array, boundary: int) -> Array[int]:
    for point_index in points.size():
        arr.flood_fill(points[point_index], -point_index, boundary)
    var result: Array[int] = []
    for point_index in points.size():
        var area_size = arr.numbers.count(-point_index)
        result.append(area_size)
    return result


## Return the sum of an array of int.
func sum(arr: Array) -> int:
    var result = 0
    for ele in arr:
        result += ele
    return result


## Find solutions to both parts of the puzzle based on puzzle_input.
func main(puzzle_input: String):
    var numbers = get_int_array_2d(puzzle_input)
    #prints(numbers.col_count, numbers.row_count, numbers.numbers.size())
    var minima_indices = numbers.find_minima_indices()
    var minima = minima_indices.map(func (inds): return numbers.get(inds[0], inds[1]))
    #print(minima)
    print("sum of risk levels %s" % sum(minima.map(func (num): return num + 1)))
    var basin_sizes = get_basin_sizes(numbers, minima_indices, 9)
    basin_sizes.sort()
    basin_sizes.reverse()
    print("products of 3 largest basins: %s" % [basin_sizes[0] *  basin_sizes[1] * basin_sizes[2]])


## constructor callback, here used as main entry-point.
func _initialize():
    main(INPUT if 'x' not in OS.get_cmdline_args() else EXAMPLE_INPUT)


## Request quitting main loop immediately by returning true.
func _process(_delta: float) -> bool:
    return true
