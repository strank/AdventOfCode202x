"""
https://adventofcode.com/2021/day/4

>>> main()
Winning Board Score: 2745
Losing Board Score: 6594

>>> main(EXAMPLE_INPUT)
Winning Board Score: 4512
Losing Board Score: 1924
"""

from pathlib import Path
import sys
import numpy as np

INPUT = (Path(__file__).parent / "input").read_text()

EXAMPLE_INPUT = """
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""


def yield_lines(puzzle_input):
    """Yield all lines one by one."""
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        # if not line:
        #    continue
        yield line


def parse_boards(lines):
    """Return a list of numpy arrays representing the boards.""" 
    boards = []
    current_lines = []
    for line in lines:
        if line:
            current_lines.append([int(x) for x in line.split()])
        else:
            boards.append(np.array(current_lines))
            current_lines = []
    if current_lines: # final board might not be followed by an empty line
        boards.append(np.array(current_lines))
    return boards


def call_bingo(boards, calls_iter):
    """Return the winning and losing boards as triples (board, board_state, last_call)."""
    board_states = []
    for board in boards:
        # board state tracks called numbers, set to 0 when called
        board_states.append(np.ones_like(board))
    winning = losing = None
    for call in calls_iter:
        win_indices = []
        for index, (board, board_state) in enumerate(zip(boards, board_states)):
            if call in board:
                board_state[board == call] = 0
                # check if bingo:
                if is_bingo(board_state):
                    if winning is None:
                        winning = board, board_state, call
                    losing = board, board_state, call
                    win_indices.append(index)
        for index in win_indices[::-1]:
            del boards[index]
            del board_states[index]
    return winning, losing


def is_bingo(board):
    """Return whether any row or column is all 0s."""
    for row in board:
        if np.all(row == 0):
            return True
    for col in board.T:
        if np.all(col == 0):
            return True
    return False


def main(puzzle_input=INPUT):
    """Find solutions to both parts of the puzzle based on puzzle_input."""
    line_iter = yield_lines(puzzle_input)
    calls_iter = (int(x) for x in next(line_iter).split(','))
    next(line_iter)
    boards = parse_boards(line_iter)
    (wboard, wstate, wcall), (lboard, lstate, lcall) = call_bingo(boards, calls_iter)
    #print(wboard)
    #print(wstate)
    #print(wcall)
    print(f"Winning Board Score: {np.sum(wboard * wstate) * wcall}")
    #print(lboard)
    #print(lstate)
    #print(lcall)
    print(f"Losing Board Score: {np.sum(lboard * lstate) * lcall}")


if __name__ == "__main__":
    main(INPUT if 'x' not in sys.argv else EXAMPLE_INPUT)
