# https://leetcode.com/problems/word-ladder/
from typing import List
from collections import deque
from string import ascii_lowercase


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        def adjacent_words(word):
            result = []
            for i in range(len(word)):
                for c in ascii_lowercase:
                    result.append(word[:i] + c + word[i + 1 :])
            return result

        def bfs(root, target) -> int:
            queue = deque([root])
            visited = set([root])
            words = set(wordList)
            steps = 1
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
            return 0  # should not happened when target is accessible

        return bfs(beginWord, endWord)
