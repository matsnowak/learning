from typing import List
from collections import deque
from string import ascii_lowercase


def word_ladder(begin: str, end: str, word_list: List[str]) -> int:
    def adjacent_words(word):
        result = []
        for i in range(len(word)):
            for c in ascii_lowercase:
                result.append(word[:i] + c + word[i + 1 :])
        return result

    def bfs(root, target) -> int:
        queue = deque([root])
        visited = set([root])
        words = set(word_list)
        steps = 0
        while len(queue) > 0:
            n = len(queue)
            for _ in range(n):
                curr = queue.popleft()
                if curr == target:
                    return steps
                for adj in adjacent_words(curr):
                    if adj in visited or adj not in words:
                        continue
                    visited.add(adj)
                    queue.append(adj)
            steps += 1
        return steps  # should not happened when target is accessible

    return bfs(begin, end)


if __name__ == "__main__":
    begin = input()
    end = input()
    word_list = input().split()
    res = word_ladder(begin, end, word_list)
    print(res)
