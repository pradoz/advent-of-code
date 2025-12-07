import operator
import functools


def read_file(fname):
    with open(fname) as f:
        lines = [line.rstrip("\n") for line in f.readlines() if line.strip()]
    return lines


def parser(lines, right_to_left):
    number_lines = lines[:-1]
    operations = lines[-1].split()

    naive_split = [line.split() for line in number_lines]
    numbers_grouped = zip(*naive_split)

    if not right_to_left:  # part 1
        problems = []
        for op, group in zip(operations, numbers_grouped):
            numbers = [int(num) for num in group]
            problems.append((op, numbers))
    else:  # part 2
        widths = [max(len(num) for num in group) for group in numbers_grouped]
        numbers_grouped = list(zip(*naive_split))

        problems = []
        skip = 0

        for op, width in zip(operations, widths):
            problem_chars = [line[skip : skip + width].ljust(width) for line in number_lines]
            digit_columns = zip(*problem_chars)

            numbers = []
            for col in digit_columns:
                digit_str = "".join(col).replace(" ", "")
                if digit_str:
                    numbers.append(int(digit_str))

            problems.append((op, numbers))
            skip += width + 1

    return problems


def calculate(op, nums):
    if op == "+":
        return sum(nums)
    elif op == "*":
        return functools.reduce(operator.mul, nums, 1)
    return 0


def solve(fname, right_to_left=False):
    lines = read_file(fname)
    problems = parser(lines, right_to_left)

    total = 0
    for op, nums in problems:
        total += calculate(op, nums)

    return total


def part1():
    # result = solve("sample.txt", right_to_left=False)
    result = solve("input.txt", right_to_left=False)
    print(f"Part 1: {result}")


def part2():
    # result = solve("sample.txt", right_to_left=True)
    result = solve("input.txt", right_to_left=True)
    print(f"Part 2: {result}")


if __name__ == "__main__":
    part1()
    part2()
