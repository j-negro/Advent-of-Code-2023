import os
from typing import Any, Generator

main_dir = os.path.dirname(os.path.abspath(__file__))


INPUT_FILENAME = "input.txt"

class Card:

    def __init__(self, id: int, winning_numbers: list[int], actual_numbers: list[int]) -> None:
        self.id = id
        self.winning_numbers = winning_numbers
        self.actual_numbers = actual_numbers
        self.copies = 1

    def matching_numbers_amount(self) -> int:
        return len(list(set(self.winning_numbers) & set(self.actual_numbers)))
    
    def calculate_points(self) -> int:
        count = self.matching_numbers_amount()
        if count != 0:
            return 2 ** (count - 1) 
        else:
            return 0
    
    def add_copies(self, num: int) -> None:
        self.copies += num



def main():
    lines = parse_file(os.path.join(main_dir, INPUT_FILENAME))

    cards = [build_card(line) for line in lines]

    sum_of_points = 0
    for idx in range(len(cards)):
        sum_of_points += cards[idx].calculate_points()

        amount_won = cards[idx].matching_numbers_amount()

        for i in range(amount_won):
            if i < len(cards) - idx:
                cards[idx + i + 1].add_copies(cards[idx].copies)

    total_copies = sum([card.copies for card in cards])
        

    # First puzzle solution: 22488
    print("The sum of all cards is: ", sum_of_points)

    # Second puzzle solution: 7013204
    print("The amount of cards with their copies is: ", total_copies)


def parse_file(filename: str) -> Generator[str, Any, None]:
    with open(filename, "r") as file:
        for line in file:
            yield line

def build_card(line: str) -> Card:

    card_id, numbers = line[5:].split(":")

    winning_numbers, actual_numbers = numbers.split("|")

    winning: list[int] = []
    actual: list[int] = []


    for num in winning_numbers.strip().split(" "):
        if num.isdigit():
            winning.append(int(num))
        
    for num in actual_numbers.strip().split(" "):
        if num.isdigit():
            actual.append(int(num))


    return Card(
        id=int(card_id.strip()),
        winning_numbers= winning,
        actual_numbers= actual
    )


if __name__ == "__main__":
    main()