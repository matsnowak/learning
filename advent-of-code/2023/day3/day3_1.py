import re


def readlines(filename: str) -> [str]:
    with open(filename, "r") as input_file:
        lines = input_file.readlines()
    return [l.strip() for l in lines]


def is_adjecent_to_symbol(
    token: str, line: str, line_number: int, lines: [str], start_pos
) -> bool:
    print(f"checking token: {token}")
    min_row = max(0, line_number - 1)
    max_row = min(len(lines) - 1, line_number + 1)
    # start_pos = line.find(token)
    end_pos = start_pos + len(token)
    min_col = max(0, start_pos - 1)
    max_col = min(len(line) - 1, end_pos)
    symbols_to_check = []
    if min_row < line_number:
        symbols_to_check.extend(lines[min_row][min_col : max_col + 1])
    if min_col < start_pos:
        symbols_to_check.extend(line[min_col])
    if max_col >= end_pos:
        # print(f"appending right char: {line[max_col]} max_col: {max_col}")
        symbols_to_check.extend(line[max_col])
    if max_row > line_number:
        symbols_to_check.extend(lines[max_row][min_col : max_col + 1])

    print(f"symbols to check: {symbols_to_check}")
    filtered_symbols = list(
        filter(lambda c: c != "." and not c.isdigit(), symbols_to_check)
    )
    print(f"filtered symbols: {filtered_symbols}")
    return len(filtered_symbols) > 0


def solution(lines: [str]) -> str:
    sum = 0
    all_numbers = []
    for li, l in enumerate(lines):
        # tokens = l.split(".")
        # non_empty_tokens = list(filter(lambda t: len(t) > 0, tokens))
        # only_number_tokens = [
        #     "".join(char for char in t if char.isdigit()) for t in non_empty_tokens
        # ]
        # only_number_tokens = list(filter(lambda t: len(t) > 0, only_number_tokens))
        # only_number_tokens = re.findall(r"\b\d+\b", l)
        matches = [
            (match.group(), match.start()) for match in re.finditer(r"\b\d+\b", l)
        ]
        # print(f"only_number_tokens = {only_number_tokens}")
        for o, start_pos in matches:
            if is_adjecent_to_symbol(o, l, li, lines, start_pos):
                print(f"adding {o}")
                all_numbers.append(int(o))
                sum += int(o)

    # for n in all_numbers:
    #     print(f"{n},")
    return f"{sum}"


lines = readlines("day3_1_input.txt")
for l in lines:
    print(l)

result = solution(lines)
print(result)
