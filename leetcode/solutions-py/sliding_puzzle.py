# https://leetcode.com/problems/sliding-puzzle/
from collections import deque
from typing import List


class Solution:
    def slidingPuzzle(self, board: List[List[int]]) -> int:
        directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        target = ((1, 2, 3), (4, 5, 0))

        init_pos = tuple(tuple(line) for line in board)
        if init_pos == target:
            return 0
        moves_map = {init_pos: 0}
        moves_queue = deque([init_pos])
        while moves_queue:
            top = moves_queue.popleft()
            row, col = 0, 0
            for i, line in enumerate(top):
                for j, entry in enumerate(line):
                    if entry == 0:
                        row, col = i, j
            for delta_row, delta_col in directions:
                new_row, new_col = row + delta_row, col + delta_col
                if new_row >= 0 and new_row < 2 and new_col >= 0 and new_col < 3:
                    new_state = list(list(line) for line in top)
                    new_state[new_row][new_col], new_state[row][col] = (
                        new_state[row][col],
                        new_state[new_row][new_col],
                    )
                    new_tuples = tuple(tuple(line) for line in new_state)
                    if new_tuples not in moves_map:
                        moves_map[new_tuples] = moves_map[top] + 1
                        moves_queue.append(new_tuples)
                        if new_tuples == target:
                            return moves_map[new_tuples]
        return -1
