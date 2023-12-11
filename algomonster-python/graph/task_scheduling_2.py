from collections import deque
from typing import Dict, List


def count_parents(graph):
    counts = {node: 0 for node in graph}
    for parent in graph:
        for node in graph[parent]:
            counts[node] += 1
    return counts


def topo_sort(graph, task_times):
    ans = 0
    q = deque()
    # init distances
    dis: Dict[str, int] = dict()
    for node in graph:
        dis[node] = 0
    counts = count_parents(graph)
    for node in counts:
        if counts[node] == 0:
            q.append(node)
            dis[node] = task_times[node]
            ans = max(ans, dis[node])
    while len(q) > 0:
        node = q.popleft()
        for child in graph[node]:
            counts[child] -= 1
            # update distances for children and answer
            dis[child] = max(dis[child], dis[node] + task_times[child])
            ans = max(ans, dis[child])
            if counts[child] == 0:
                q.append(child)
    return ans


def task_scheduling_2(
    tasks: List[str], times: List[int], requirements: List[List[str]]
) -> int:
    # init graph and task_times
    graph: Dict[str, List[str]] = dict()
    task_times: Dict[str, int] = dict()
    for i in range(len(tasks)):
        graph[tasks[i]] = list()
        task_times[tasks[i]] = times[i]

    for req in requirements:
        graph[req[0]].append(req[1])

    return topo_sort(graph, task_times)


if __name__ == "__main__":
    tasks = input().split()
    times = [int(x) for x in input().split()]
    requirements = [input().split() for _ in range(int(input()))]
    res = task_scheduling_2(tasks, times, requirements)
    print(res)
