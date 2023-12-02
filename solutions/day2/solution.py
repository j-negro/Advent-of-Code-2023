import os
from typing import Any, Generator

main_dir = os.path.dirname(os.path.abspath(__file__))


INPUT_FILENAME = "input.txt"


def main():
    lines = parse_file(os.path.join(main_dir, INPUT_FILENAME))
    sum_possible_sets = sum([game_id_if_possible(line) for line in lines])
    print(sum_possible_sets)  # Answer: 3099


def parse_file(filename: str) -> Generator[str, Any, None]:
    with open(filename, "r") as file:
        for line in file:
            yield line


def game_id_if_possible(line: str) -> int:
    game_id, sets = line[5:].split(":")
    game_id = int(game_id)
    sets = sets.strip().split(";")
    for game_set in sets:
        if not is_set_possible(game_set):
            return 0
    return game_id


max_colored_cube_amounts = {"red": 12, "green": 13, "blue": 14}


def is_set_possible(game_set: str) -> bool:
    colored_cubes: list[str] = [cube.strip() for cube in game_set.split(",")]
    for cube in colored_cubes:
        amount, color = cube.split(" ")
        if max_colored_cube_amounts[color] < int(amount):
            return False
    return True


if __name__ == "__main__":
    main()
