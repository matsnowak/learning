with open("input_2.txt", "r") as input_file:
    lines = input_file.readlines()
req = {"red": 12, "green": 13, "blue": 14}

final_sum = 0
for line in lines:
    max_colors = {
        "red": 0,
        "green": 0,
        "blue": 0,
    }
    game_line, sets_line = line.strip().split(":")
    _, game_id = game_line.strip().split(" ")
    print(f"processing game:{game_id}")

    for cube_tokens in sets_line.split(";"):
        for cube_token in cube_tokens.split(","):
            print(f"cube_token: {cube_token}")
            number_of_cubes, color = cube_token.strip().split(" ")
            curr_max = max_colors[color]
            if int(number_of_cubes) > curr_max:
                max_colors[color] = int(number_of_cubes)
    game_sum = max_colors["red"] * max_colors["green"] * max_colors["blue"]
    print(f"game sum for {game_id} is {game_sum}")
    final_sum += game_sum
print(f"sum: {final_sum}")
