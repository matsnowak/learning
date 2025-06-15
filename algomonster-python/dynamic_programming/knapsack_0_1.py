# bruteforce
def knapsack_dfs(weights: list[int], values: list[int], item: int, weight_left: int) -> int:
    if item == 0 or weight_left == 0:
        return 0

    if weights[item - 1] > weight_left:
        return knapsack_dfs(weights, values, item - 1, weight_left)

    # include item
    include_value = values[item - 1] + knapsack_dfs(weights, values, item - 1, weight_left - weights[item - 1])

    # exclude item
    exclude_value = knapsack_dfs(weights, values, item - 1, weight_left)

    return max(include_value, exclude_value)


def knapsack(weights: list[int], values: list[int], max_weight: int) -> int:
    n = len(weights)
    # WRITE YOUR BRILLIANT CODE HERE
    return knapsack_dfs(weights, values, n, max_weight)


#
# if __name__ == "__main__":
#     weights = [int(x) for x in input().split()]
#     values = [int(x) for x in input().split()]
#     max_weight = int(input())
#     res = knapsack(weights, values, max_weight)
#     print(res)


def knapsack_dfs_memo(weights: list[int], values: list[int], item: int, weight_left: int, memo: list[list[int]]) -> int:
    if item == 0 or weight_left == 0:
        return 0

    if memo[item][weight_left] != -1:
        return memo[item][weight_left]

    res = 0
    if weights[item - 1] > weight_left:
        res = knapsack_dfs_memo(weights, values, item - 1, weight_left)
    else:
        # include item
        res = max(
            values[item - 1] + knapsack_dfs_memo(weights, values, item - 1, weight_left - weights[item - 1], memo),
            knapsack_dfs_memo(weights, values, item - 1, weight_left, memo)
        )
    memo[item][weight_left] = res
    return res


def knapsack_memo(weights: list[int], values: list[int], max_weight: int) -> int:
    n = len(weights)
    memo = [[-1 for i in range(max_weight + 1)] for j in range(n + 1)]
    # WRITE YOUR BRILLIANT CODE HERE
    return knapsack_dfs_memo(weights, values, n, max_weight, memo)


if __name__ == "__main__":
    weights = [int(x) for x in input().split()]
    values = [int(x) for x in input().split()]
    max_weight = int(input())
    res = knapsack_memo(weights, values, max_weight )
    print(res)
