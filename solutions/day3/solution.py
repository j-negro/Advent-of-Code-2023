import itertools
import os
from functools import reduce
from typing import Any, Generator

main_dir = os.path.dirname(os.path.abspath(__file__))


INPUT_FILENAME = "input.txt"


def main():
    lines = parse_file(os.path.join(main_dir, INPUT_FILENAME))

    sum_part_numbers = sum(get_part_numbers(schematic=lines))

    # First puzzle answer: 540131
    # print("The sum of all part numbers is: ", sum_part_numbers)

    sum_gear_ratios = sum(get_gear_ratios(schematic=lines))

    # Second puzzle answer:
    print("The sum of all gear ratios is: ", sum_gear_ratios)


def parse_file(filename: str) -> list[str]:
    with open(filename, "r") as file:
        return file.readlines()


SCHEMATIC_MAX_LENGHT = 140


# First Part of the Puzzle
def get_part_numbers(schematic: list[str]) -> list[int]:
    part_numbers: list[int] = []

    for row_idx in range(SCHEMATIC_MAX_LENGHT):
        column_idx = 0
        while column_idx < SCHEMATIC_MAX_LENGHT:
            if str.isdigit(schematic[row_idx][column_idx]):
                end_idx = get_number_end_idx(schematic[row_idx], column_idx)

                for position in get_range_indexes_to_validate(
                    row_idx, column_idx, end_idx
                ):
                    element = schematic[position[0]][position[1]]

                    if not str.isdigit(element) and element != ".":
                        # Element is special symbol, number represents a part in engine
                        part_numbers.append(int(schematic[row_idx][column_idx:end_idx]))
                        break

                column_idx += end_idx - column_idx + 1
            else:
                column_idx += 1
    return part_numbers


def get_number_end_idx(row: str, column_idx: int) -> int:
    end_idx = column_idx
    while str.isdigit(row[end_idx]):
        end_idx += 1
    return end_idx


def get_range_indexes_to_validate(row_idx: int, start_idx: int, end_idx: int):
    rows = [row_idx]
    if row_idx > 0:
        rows.append(row_idx - 1)
    if row_idx < SCHEMATIC_MAX_LENGHT - 1:
        rows.append(row_idx + 1)

    columns = [i for i in range(start_idx, end_idx)]
    if start_idx > 0:
        columns.append(start_idx - 1)
    if end_idx < SCHEMATIC_MAX_LENGHT:
        columns.append(end_idx)

    return itertools.product(rows, columns)


# Second Part of the Puzzle
def get_gear_ratios(schematic: list[str]):
    gear_ratios: list[int] = []

    for row_idx in range(SCHEMATIC_MAX_LENGHT):
        for column_idx in range(SCHEMATIC_MAX_LENGHT):
            if schematic[row_idx][column_idx] == "*":
                adjacent_numbers = get_adjacent_numbers(schematic, row_idx, column_idx)
                if len(adjacent_numbers) == 2:
                    gear_ratios.append(reduce(lambda x, y: x * y, adjacent_numbers))

    return gear_ratios


def get_adjacent_numbers(schematic: list[str], row_idx: int, column_idx: int):
    indexes = get_range_indexes_to_validate(row_idx, column_idx, column_idx + 1)

    numbers: list[int] = []

    positions = [position for position in indexes]

    for position in positions:
        if str.isdigit(schematic[position[0]][position[1]]):
            start_idx = position[1]
            end_idx = position[1]

            while str.isdigit(schematic[position[0]][start_idx]):
                start_idx -= 1
            while str.isdigit(schematic[position[0]][end_idx]):
                end_idx += 1

            number = int(schematic[position[0]][start_idx + 1 : end_idx])

            numbers.append(number)

            # Remove from positions the ones included in the number

            for column in range(start_idx + 1, end_idx):
                to_remove = (position[0], column)

                if column == position[1]:
                    continue

                if to_remove in positions:
                    positions.remove(to_remove)

    return numbers


def get_indexes_to_validate(row_idx: int, column_idx: int):
    rows = [row_idx]
    if row_idx > 0:
        rows.append(row_idx - 1)
    if row_idx < SCHEMATIC_MAX_LENGHT - 1:
        rows.append(row_idx + 1)

    columns = [column_idx]
    if column_idx > 0:
        columns.append(column_idx - 1)
    if column_idx < SCHEMATIC_MAX_LENGHT:
        columns.append(column_idx + 1)

    return itertools.product(rows, columns)


if __name__ == "__main__":
    main()
