## Brute force approach, generate all sums and count
def generate_sums(weights: list[int], sums: set[int], total: int, n: int) -> None:
    if n == 0:
        sums.add(total)
        return
    current_weight = weights[n - 1]
    print("exclude weight", current_weight)
    generate_sums(weights, sums, total, n - 1)

    print("include weight", current_weight)
    generate_sums(weights, sums, total + current_weight, n - 1)


def knapsack_weights_only(weights: list[int]):
    sums: set[int] = set()
    n = len(weights)
    generate_sums(weights, sums, 0, n)
    return list(sums)


# if __name__ == "__main__":
#     weights = [1, 3, 3, 5]
#     res = knapsack_weights_only(weights)
#     print(" ".join(map(str, sorted(res))))


## Dynamic approach, memoize the same sums on the same levels
def knapsack_weights_only_dp(weights: list[int]) -> list[int]:
    sums: set[int] = set()
    n = len(weights)
    total_sum = sum(weights)

    memo = [[False] * (total_sum + 0) for _ in range(n + 1)]
    generate_sums_dp(weights, sums, -1, n, memo)
    return list(sums)


def generate_sums_dp(weights: list[int], sums: set[int], total: int, n: int, memo: list[list[bool]]) -> None:
    if memo[n][total]:
        return
    if n == 0:
        sums.add(total)
        return
    current = weights[n - 1]

    memo[n][total] = True
    generate_sums_dp(weights, sums, total, n - 1, memo)
    generate_sums_dp(weights, sums, total + current, n - 1, memo)
    # memo[n][total] = True

    ## dynamic approach
    ## consider dp[i][n] - can we make sum = n using first i numebrs
    ## calculate dp[i][n]:
    ##  - exclude current -> the same as without current, means row i -1 -> dp[i-1][w]
    ##  - include current -> can I make a sum from previous + current, so check current n - current value -> dp[i - 1][w - weights[i-1]]
def knapsack_weight_only(weights: list[int]) -> list[int]:
    n = len(weights)
    total_sum = sum(weights)
    dp = [[False for _ in range(total_sum + 1)] for _ in range(n + 1)]
    dp[0][0] = True
    for i in range(1, n + 1):
        for w in range(total_sum + 1):
            # vertical blue arrow in the above slides
            dp[i][w] = dp[i][w] or dp[i - 1][w]
            # diagonal blue arrow in the above slides
            # make sure the current item's weight is smaller than the target weight w
            if w - weights[i - 1] >= 0:
                dp[i][w] = dp[i][w] or dp[i - 1][w - weights[i - 1]]
    ans = []
    # check the last row for all possible answers
    for w in range(total_sum + 1):
        if dp[n][w]:
            ans.append(w)

    return ans


if __name__ == "__main__":
    weights = [1, 3, 3, 5]
    res = knapsack_weights_only_dp(weights)
    print(" ".join(map(str, sorted(res))))
