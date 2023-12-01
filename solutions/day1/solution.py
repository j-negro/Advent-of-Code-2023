import os

main_dir = os.path.dirname(os.path.abspath(__file__))


INPUT_FILENAME = "input.txt"


def main():
    lines = parse_file(os.path.join(main_dir, INPUT_FILENAME))
    calibration_number = sum([get_line_calibration_number(line) for line in lines])
    # Answers are 53651 for the first puzzle, 53894 for the second.
    print(calibration_number)


def parse_file(filename):
    with open(filename, "r") as file:
        for line in file:
            yield line


possible_digits = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def get_line_calibration_number(line: str):
    fst_digit: int | None = None
    snd_digit: int | None = None
    for idx in range(len(line)):
        digit: int | None = None
        if line[idx].isdigit():
            digit = int(line[idx])
        else:  # Remove this else case for the first puzzle.
            for key, value in possible_digits.items():
                if line[idx : idx + len(key)] == key:
                    digit = value

        if digit:
            if fst_digit is None:
                fst_digit = digit
            else:
                snd_digit = digit

    if fst_digit is None:
        raise ValueError("All lines must have at least one digit present")

    if snd_digit is None:
        snd_digit = fst_digit
    return 10 * fst_digit + snd_digit


if __name__ == "__main__":
    main()
