string_to_number = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def replace_numbers(s: str) -> str:
    found_first = False
    found_last = False
    for i in range(len(s)):
        for word, number in string_to_number.items():
            if s[i:].startswith(word) and not found_first:
                s = s.replace(word, number, 1)
                found_first = True
            if s[-i:].startswith(word) and not found_last:
                s = s[:-i] + number + s[-i + 1 :]
                found_last = True

    return s


with open("input_2.txt", "r") as input_file:
    lines = input_file.readlines()
lines = [replace_numbers(x) for x in lines]
sum = 0
for l in lines:
    only_digits = "".join(char for char in l if char.isdigit())
    if len(only_digits) < 1:
        continue
    number = int(only_digits[0] + only_digits[-1])
    # print(l.strip() + " => " + str(number))
    sum += number

print("SUM: " + str(sum))
