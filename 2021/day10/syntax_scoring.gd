## https://adventofcode.com/2021/day/10
## Matching brackets.
##
## TODO: translate python doctests to godot testing framework?
## >>> main()
##
## >>> main(EXAMPLE_INPUT)

extends MainLoop

var INPUT = read_input(self.get_script().get_path().get_base_dir() + "/input")

const EXAMPLE_INPUT = """
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"""


## Return the contents of file at `input_path`
func read_input(input_path: String) -> String:
    var file = File.new()
    file.open(input_path, File.READ)
    var input = file.get_as_text().strip_edges()
    file.close()
    return input


const CLOSING_MAP := {
    '(' : ')',
    '[' : ']',
    '{' : '}',
    '<' : '>',
}


const ERROR_SCORE_MAP := { # error scores are negative so scoring can be done in one pass
    ')' : -3,
    ']' : -57,
    '}' : -1197,
    '>' : -25137,
}


const COMPLETION_SCORE_MAP := {
    ')' : 1,
    ']' : 2,
    '}' : 3,
    '>' : 4,
}


## Return a negative error score or positive completion score
## for a line of either mismatched or incomplete brackets
func score_line(line: String) -> int:
    var brack_stack = []
    for char in line:
        if char in ['(', '[', '{', '<']:
            brack_stack.push_back(CLOSING_MAP[char])
        else:
            var expected = brack_stack.pop_back()
            if char != expected:
                return ERROR_SCORE_MAP[char]
    # completion case:
    var result = 0
    while brack_stack.size() > 0:
        result *= 5
        result += COMPLETION_SCORE_MAP[brack_stack.pop_back()]
    return result


## Return the sum of an array of int.
func sum(arr: Array) -> int:
    var result = 0
    for ele in arr:
        result += ele
    return result


## Find solutions to both parts of the puzzle based on puzzle_input.
func main(puzzle_input: String):
    var lines = puzzle_input.split('\n')
    var scores = Array(lines).map(score_line)
    print("sum of error scores %s" % -sum(scores.filter(func (num): return num < 0)))
    var completion_scores = scores.filter(func (num): return num > 0)
    completion_scores.sort()
    print("middle completion score %s" % completion_scores[completion_scores.size() / 2])


## constructor callback, here used as main entry-point.
func _initialize():
    main(INPUT if 'x' not in OS.get_cmdline_args() else EXAMPLE_INPUT)


## Request quitting main loop immediately by returning true.
func _process(_delta: float) -> bool:
    return true
