with open("input_1.txt", "r") as input_file:
    lines = input_file.readlines()


sum = 0
for l in lines:
    only_digits = "".join(char for char in l if char.isdigit())
    if len(only_digits) < 1:
        continue
    number = int(only_digits[0] + only_digits[-1])
    sum += number

print("SUM: " + str(sum))
