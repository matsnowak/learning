from collections import deque


def get_knight_shortest_path(x: int, y: int) -> int:
    def get_neigbhours(node):
        nx, ny = node
        result = [
            (nx + 1, ny + 2),
            (nx + 2, ny + 1),
            (nx + 2, ny - 1),
            (nx + 1, ny - 2),
            (nx - 1, ny - 2),
            (nx - 2, ny - 1),
            (nx - 2, ny + 1),
            (nx - 1, ny + 2)
        ]
        return result

    def bfs(root):
        queue = deque([root])
        visited = set([root])
        ans = 0
        while len(queue) > 0:
            n = len(queue)
            for i in range(n):
                current = queue.popleft()
                cx, cy = current

                if cx == x and cy == y:
                    return ans

                for neigbhour in get_neigbhours(current):
                    if neigbhour not in visited:
                        queue.append(neigbhour)
                        visited.add(neigbhour)
            ans += 1

    ans = bfs((0, 0))
    return ans


if __name__ == '__main__':
    x = int(input())
    y = int(input())
    res = get_knight_shortest_path(x, y)
    print(res)
