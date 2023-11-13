from typing import List
from collections import deque


def flood_fill(r: int, c: int, replacement: int, image: List[List[int]]) -> List[List[int]]:
    num_rows, num_cols = len(image), len(image[0])

    def get_neighbors(coord):
        row, col = coord
        # [N, NE, E, SE, S, SW, W, NW] but could be only [N, E, S, W]
        delta_row = [-1, -1, 0, 1, 1, 1, 0, -1]
        delta_col = [0, 1, 1, 1, 0, -1, -1, -1]
        res = []
        for i in range(len(delta_row)):
            neighbor_row = row + delta_row[i]
            neighbor_col = col + delta_col[i]
            if 0 <= neighbor_row < num_rows and 0 <= neighbor_col < num_cols:
                res.append((neighbor_row, neighbor_col))
        return res

    old_color = image[r][c]

    def bfs(root):
        row, col = root
        queue = deque([root])
        visited = [[False for c in range(num_cols)] for r in range(num_rows)]
        while len(queue) > 0:
            (current_r, current_c) = queue.popleft()
            for (n_r, n_c) in get_neighbors((current_r, current_c)):
                if image[n_r][n_c] == old_color and not visited[n_r][n_c]:
                    queue.append((n_r, n_c))
                    visited[n_r][n_c] = True
            image[current_r][current_c] = replacement

    bfs((r, c))
    return image


if __name__ == '__main__':
    r = int(input())
    c = int(input())
    replacement = int(input())
    image = [[int(x) for x in input().split()] for _ in range(int(input()))]
    res = flood_fill(r, c, replacement, image)
    for row in res:
        print(' '.join(map(str, row)))

# Input
#
# 2
# 2
# 9
# 5
# 0 1 3 4 1
# 3 8 8 3 3
# 6 7 8 8 3
# 12 2 8 9 1
# 12 3 1 3 2
#
# Output
#
# 0 1 3 4 1
# 3 9 9 3 3
# 6 7 9 9 3
# 12 2 9 9 1
# 12 3 1 3 2
