from typing import List
from collections import deque


def num_steps(target_combo: str, trapped_combos: List[str]) -> int:

    def generate_neighbors(lock):
        """
        This function generates the neighbors of a given lock combination.
        For each wheel, it can move one step forward or backward.
        """
        neighbors = []
        for i in range(4):
            digit = int(lock[i])
            # Move one step forward
            forward = (digit + 1) % 10
            neighbors.append(lock[:i] + str(forward) + lock[i + 1:])
            # Move one step backward
            backward = (digit - 1) % 10
            neighbors.append(lock[:i] + str(backward) + lock[i + 1:])

        return neighbors

    def bfs(start: str):
        queue = deque([start])
        level = 0
        visited = set([start])
        while len(queue) > 0:
            n = len(queue)
            for _ in range(n):
                curr = queue.popleft()
                if curr == target_combo:
                    return level
                for nei in generate_neighbors(curr):
                    if nei in trapped_combos or nei in visited:
                        continue
                    queue.append(nei)
                    visited.add(nei)
            level += 1
        return -1

    return bfs("0000")


if __name__ == '__main__':
    target_combo = input()
    trapped_combos = input().split()
    res = num_steps(target_combo, trapped_combos)
    print(res)
