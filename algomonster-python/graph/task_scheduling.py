from typing import List
from collections import deque


def task_scheduling(tasks: List[str], requirements: List[List[str]]) -> List[str]:
    def in_degree(graph):
        indegree = {node: 0 for node in graph}
        for node in indegree:
            for neighbour in graph[node]:
                indegree[neighbour] += 1
        return indegree

    graph = {node: [] for node in tasks}
    for node, neighbour in requirements:
        graph[node].append(neighbour)

    res = []
    queue = deque()
    indegree = in_degree(graph)
    for node in indegree:
        if indegree[node] == 0:
            queue.append(node)

    while len(queue) > 0:
        node = queue.popleft()
        res.append(node)
        for neighbour in graph[node]:
            indegree[neighbour] -= 1
            if indegree[neighbour] == 0:
                queue.append(neighbour)

    return res if len(res) == len(tasks) else None


if __name__ == "__main__":
    tasks = input().split()
    requirements = [input().split() for _ in range(int(input()))]
    res = task_scheduling(tasks, requirements)
    if len(res) != len(tasks):
        print(f"output size {len(res)} does not match input size {len(tasks)}")
        exit()
    indices = {task: i for i, task in enumerate(res)}
    for req in requirements:
        for task in req:
            if task not in indices:
                print(f"'{task}' is not in output")
                exit()
        a, b = req
        if indices[a] >= indices[b]:
            print(f"'{a}' is not before '{b}'")
            exit()
    print("ok")
