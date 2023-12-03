def readlines(filename: str) -> [str]:
    with open(filename, "r") as input_file:
        lines = input_file.readlines()
    return [l.strip() for l in lines]


def solution1(lines: [str]) -> str:
    result = ""
    return result


def solution2(lines: [str]) -> str:
    result = ""
    return result


test_lines = readlines("part1_test_input.txt")
print(f"test solution1 result = {solution1(test_lines)}")

lines = readlines("part1_input.txt")
print(f"solution1 result = {solution1(lines)}")


test_lines = readlines("part2_test_input.txt")
print(f"test solution2 result = {solution2(test_lines)}")

lines = readlines("part2_input.txt")
print(f"solution2 result = {solution2(lines)}")
