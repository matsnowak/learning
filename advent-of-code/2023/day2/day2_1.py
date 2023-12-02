with open("input_1.txt", "r") as input_file:
    lines = input_file.readlines()
req = {"red": 12, "green": 13, "blue": 14}

games_id_sum = 0
for line in lines:
    game_line, sets_line = line.strip().split(":")
    _, game_id = game_line.strip().split(" ")
    print(f"processing game:{game_id}")

    game_ok = True
    for cube_tokens in sets_line.split(";"):
        for cube_token in cube_tokens.split(","):
            print(f"cube_token: {cube_token}")
            number_of_cubes, color = cube_token.strip().split(" ")
            max_possible = req[color]
            if int(number_of_cubes) > max_possible:
                print(f"game {game_id} is not ok")
                game_ok = False
                break
    if game_ok:
        print(f"game {game_id} is ok")
        games_id_sum += int(game_id)
print(f"sum: {games_id_sum}")
