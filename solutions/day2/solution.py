import os
from functools import reduce
from typing import Any, Generator

main_dir = os.path.dirname(os.path.abspath(__file__))


INPUT_FILENAME = "input.txt"


def main():
    lines = parse_file(os.path.join(main_dir, INPUT_FILENAME))

    # Solution to the first puzzle
    sum_possible_sets = 0
    sum_power_sets = 0
    for game in lines:
        game_id_if_possible, min_colors = get_game_information(game)

        sum_possible_sets += game_id_if_possible

        sum_power_sets += reduce(lambda x, y: x * y, min_colors.values())

    # First puzzle answer: 3099
    print("The sum of possible game IDs is: ", sum_possible_sets)

    # Second puzzle answer: 72970
    print("The sum of the power of the minimum set of cubes is: ", sum_power_sets)


def parse_file(filename: str) -> Generator[str, Any, None]:
    with open(filename, "r") as file:
        for line in file:
            yield line


def get_game_information(line: str) -> tuple[int, dict[str, int]]:
    game_id, sets = line[5:].split(":")
    game_id = int(game_id)
    sets = sets.strip().split(";")

    min_colors = {"red": 0, "green": 0, "blue": 0}
    game_id_if_possible = game_id
    for game_set in sets:
        set_colors = colors_per_set(game_set)

        if not is_set_possible(set_colors):
            game_id_if_possible = 0

        for color in min_colors.keys():
            min_colors[color] = max(min_colors[color], set_colors.get(color, 0))

    return game_id_if_possible, min_colors


max_colored_cube_amounts = {"red": 12, "green": 13, "blue": 14}


def is_set_possible(colors_per_set: dict[str, int]) -> bool:
    for color, amount in colors_per_set.items():
        if max_colored_cube_amounts[color] < int(amount):
            return False
    return True


def colors_per_set(game_set: str) -> dict[str, int]:
    colored_cubes: list[str] = [cube.strip() for cube in game_set.split(",")]
    total_cubes = {}
    for cube in colored_cubes:
        amount, color = cube.split(" ")
        total_cubes[color] = int(amount)
    return total_cubes


if __name__ == "__main__":
    main()
