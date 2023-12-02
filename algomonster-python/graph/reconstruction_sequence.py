from typing import List
from collections import deque


def sequence_reconstruction(original: List[int], seqs: List[List[int]]) -> bool:
    def find_indegree(graph):
        indegree = {node: 0 for node in graph}  # dict
        for node in graph:
            for neighbor in graph[node]:
                indegree[neighbor] += 1
        return indegree

    def topo_sort(graph):
        res = []
        q = deque()
        indegree = find_indegree(graph)
        for node in indegree:
            if indegree[node] == 0:
                q.append(node)
        while len(q) > 0:
            if len(q) > 1:
                return False
            node = q.popleft()
            res.append(node)
            for neighbor in graph[node]:
                indegree[neighbor] -= 1
                if indegree[neighbor] == 0:
                    q.append(neighbor)
        return res == original

    graph = {t: [] for t in original}
    for seq in seqs:
        for i in range(len(seq) - 1):
            a, b = seq[i], seq[i + 1]
            graph[a].append(b)

    return topo_sort(graph)


if __name__ == "__main__":
    original = [int(x) for x in input().split()]
    seqs = [[int(x) for x in input().split()] for _ in range(int(input()))]
    res = sequence_reconstruction(original, seqs)
    print("true" if res else "false")
