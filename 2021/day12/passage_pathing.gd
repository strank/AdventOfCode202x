## https://adventofcode.com/2021/day/12
## Enumerate all paths in a graph. Probably breadth-first / dikstra / uniform-cost search
##
## TODO: add doctests feature to godot testing framework Gut
## for now tests are in a TestClass at the end of the file

extends MainLoop

var INPUT: String:
    get:
        return read_input(self.get_script().get_path().get_base_dir() + "/input")

const EXAMPLE_INPUT = """
start-A
start-b
A-c
A-b
b-d
A-end
b-end
""" # 10 paths

const EXAMPLE_INPUT_2 = """
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
""" # 19 paths

const EXAMPLE_INPUT_3 = """
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
""" # 226 paths


## Return the contents of file at `input_path`
func read_input(input_path: String) -> String:
    var file = File.new()
    file.open(input_path, File.READ)
    var input = file.get_as_text().strip_edges()
    file.close()
    return input


## Add a connection to the connections dictionary
## filters out connections to 'start' and from 'end'
func add_connection(connections: Dictionary, from: String, to: String):
    if from == 'end' or to == 'start':
        return
    if from not in connections:
        connections[from] = []
    connections[from].append(to)


## Return a dict from node name to list of node names, both directions are in the dict for every edge
func parse_connections(puzzle_input: String) -> Dictionary:
    var result := {}
    for line in puzzle_input.strip_edges().split('\n'):
        var nodes = line.split('-')
        add_connection(result, nodes[0], nodes[1])
        add_connection(result, nodes[1], nodes[0])
    return result


## depth-first search with special cases based on the case of a node label. uppercase can be visited unlimited times
## lowercase can only be visited once, unless `small_visited_twice` is false, then one can be visited twice
## TODO: optimization after testing is in place: don't remember paths, just return a count!
## possible further optimization: graph could be based on ints or even bit-based adjacency
func DFS(connections: Dictionary, node: String, goal: String, current_path: Array, paths: Array, small_visited_twice: bool = true) -> void:
    current_path.push_back(node)
    #prints("considering", current_path)
    if node == goal:
        paths.append(current_path.duplicate())
    else:
        for neigh in connections[node]:
            if neigh[0] == neigh[0].to_lower() and neigh in current_path:
                # small cave that we've already visited, check if still possible:
                if not small_visited_twice:
                    DFS(connections, neigh, goal, current_path, paths, true)
            else:
                DFS(connections, neigh, goal, current_path, paths, small_visited_twice)
    current_path.pop_back()


## Count the numer of paths from start to end, only allowing repeat visits to uppercase nodes in the connection graph
## (assumes that there are no direct connections between uppercase nodes - that would result in infinite loops)
## modified depth-first (or uniform-cost ?) search that counts all possible paths 
func count_paths(connections: Dictionary, start: String, end: String, small_visited_twice: bool = true) -> int:
    var paths := []
    var path := []
    DFS(connections, start, end, path, paths, small_visited_twice)
    #print(paths)
    return paths.size()


## Find solutions to both parts of the puzzle based on puzzle_input.
func main(puzzle_input: String) -> String:
    var result := ""
    var connections := parse_connections(puzzle_input)
    #print(connections)
    var num_paths := count_paths(connections, "start", "end")
    result += "num paths from start to end: %s\n" % [num_paths, ]
    var num_paths_small_twice := count_paths(connections, "start", "end", false)
    result += "num paths from start to end: %s\n" % [num_paths_small_twice, ]
    return result


## constructor callback, here used as main entry-point.
func _initialize():
    print(main(INPUT if 'x' not in OS.get_cmdline_args() else EXAMPLE_INPUT))
    print(self.get_script().get_path())
    #TestClass.new().before_all()


# Request quitting main loop immediately by returning true.
func _process(_delta: float) -> bool:    
   return true


class TestClass:
    extends "res://addons/gut/test.gd"

    var outer = load("res://2021/day12/passage_pathing.gd")
    var to_test

    func before_all():
        to_test = outer.new()
        print(to_test.INPUT)

    func test_main():
        assert_string_contains(to_test.INPUT, "b")

    func test_main_example():
        assert_string_contains("a", "b")#main(EXAMPLE_INPUT), "NOT HERE")

