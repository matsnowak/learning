# https://leetcode.com/problems/flood-fill/
from collections import deque
from typing import List


class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        num_rows, num_cols = len(image), len(image[0])

        def get_neighbors(coord):
            row, col = coord
            delta_row = [-1, 0, 1, 0]
            delta_col = [0, 1, 0, -1]
            res = []
            for i in range(len(delta_row)):
                neighbor_row = row + delta_row[i]
                neighbor_col = col + delta_col[i]
                if 0 <= neighbor_row < num_rows and 0 <= neighbor_col < num_cols:
                    res.append((neighbor_row, neighbor_col))
            return res

        old_color = image[sr][sc]

        def bfs(root):
            queue = deque([root])
            visited = [[False for sc in range(num_cols)] for sr in range(num_rows)]
            while len(queue) > 0:
                (current_r, current_c) = queue.popleft()
                for (n_r, n_c) in get_neighbors((current_r, current_c)):
                    if image[n_r][n_c] == old_color and not visited[n_r][n_c]:
                        queue.append((n_r, n_c))
                        visited[n_r][n_c] = True
                image[current_r][current_c] = color

        bfs((sr, sc))
        return image
