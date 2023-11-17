# https://leetcode.com/problems/open-the-lock/
class Solution:
    def openLock(self, deadends: List[str], target: str) -> int:
        def generate_neighbors(lock):
            neighbors = []
            for i in range(4):
                digit = int(lock[i])
                forward = (digit + 1) % 10
                neighbors.append(lock[:i] + str(forward) + lock[i + 1:])
                backward = (digit - 1) % 10
                neighbors.append(lock[:i] + str(backward) + lock[i + 1:])

            return neighbors

        def bfs(start: str):
            if start in deadends:
                return -1
            queue = deque([start])
            level = 0
            visited = set([start])
            traps = set(deadends)
            while len(queue) > 0:
                n = len(queue)
                for _ in range(n):
                    curr = queue.popleft()
                    if curr == target:
                        return level
                    for nei in generate_neighbors(curr):
                        if nei in traps or nei in visited:
                            continue
                        queue.append(nei)
                        visited.add(nei)
                level += 1
            return -1

        return bfs("0000")
