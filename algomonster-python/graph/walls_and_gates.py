from collections import deque
from typing import List

#  multi-source BFS
# instead of doing for loop n x m times starting from every point,
# it is better to find gates and start from gates. Each neighbour will have
# distance 1, neighbors of neighbors will have distance 2, etc...
# If given node is non INF, it means that should already been marked and shortest path exists

directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

INF = 2147483647


def map_gate_distances(dungeon_map: List[List[int]]) -> List[List[int]]:
    queue = deque()
    n = len(dungeon_map)
    m = len(dungeon_map[0])
    for i, row in enumerate(dungeon_map):
        for j, entry in enumerate(row):
            if entry == 0:
                queue.append((i, j))
    while queue:
        row, col = queue.popleft()
        for delta_row, delta_col in directions:
            total_row, total_col = row + delta_row, col + delta_col
            if total_row >= 0 and total_row < n and total_col >= 0 and total_col < m:
                if dungeon_map[total_row][total_col] == INF:
                    dungeon_map[total_row][total_col] = dungeon_map[row][col] + 1
                    queue.append((total_row, total_col))
    return dungeon_map


if __name__ == '__main__':
    dungeon_map = [[int(x) for x in input().split()] for _ in range(int(input()))]
    res = map_gate_distances(dungeon_map)
    for row in res:
        print(' '.join(map(str, row)))
