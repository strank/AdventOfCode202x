"""
"""
from pathlib import Path
import sys
import itertools
import numpy as np


def yield_lines():
    input_path = Path(__file__).parent / "input"
    with input_path.open() as input_file:
        for line in input_file:
            line = line.strip()
            #if not line:
            #    continue
            yield line


def parse_boards(lines):
    boards = []
    current_lines = []
    for line in lines:
        if line:
            current_lines.append([int(x) for x in line.split()])
        else:
            boards.append(np.array(current_lines))
            current_lines = []
    return boards


def call_bingo(boards, calls_iter):
    board_states = []
    for board in boards:
        # board state tracks called numbers, set to 0 when called
        board_states.append(np.ones_like(board))
    winning = losing = None
    bingos = len(boards)
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
    '''Check if any row or column is all 0s'''
    for row in board:
        if np.all(row == 0):
            return True
    for col in board.T:
        if np.all(col == 0):
            return True
        

def main():
    line_iter = yield_lines()
    calls_iter = (int(x) for x in next(line_iter).split(','))
    next(line_iter)
    boards = parse_boards(line_iter)
    (wboard, wstate, wcall), (lboard, lstate, lcall) = call_bingo(boards, calls_iter)
    print(wboard)
    print(wstate)
    print(wcall)
    print(f"Winning Board Score: {np.sum(wboard * wstate) * wcall}")
    print(lboard)
    print(lstate)
    print(lcall)
    print(f"Losing Board Score: {np.sum(lboard * lstate) * lcall}")


if __name__ == "__main__":
    main()
