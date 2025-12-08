from dataclasses import dataclass
from collections import deque


@dataclass
class Grid:
    start: tuple[int]
    grid: list[list]
    rows: int
    cols: int


def read_file(fname):
    with open(fname) as f:
        lines = [line.rstrip("\n") for line in f.readlines() if line.strip()]
    return lines


def parser(data):
    rows = len(data)
    cols = len(data[0])
    start = (-1, -1)

    grid = [['.' for _ in range(cols)] for _ in range(rows)]

    for i in range(rows):
        for j in range(cols):
            curr = data[i][j]
            grid[i][j] = curr
            if curr == "S":
                start = (i, j)

    return Grid(start=start, grid=grid, rows=rows, cols=cols)


def bfs(grid: Grid):
    queue = deque([grid.start])
    splits = 0
    visited = set()

    while queue:
        row, col = queue.popleft()

        if row < 0 or row > grid.rows - 1 or col < 0 or col > grid.cols - 1:
            continue

        if (row, col) in visited:
            continue

        visited.add((row, col))

        cell = grid.grid[row][col]

        # reached the bottom
        if row == grid.rows - 1:
            continue

        if cell != "^":
            # go down
            queue.append((row + 1, col))
        else:
            # split beam, go left and right
            splits += 1
            queue.append((row, col - 1))
            queue.append((row, col + 1))

    return splits


def dfs(grid: Grid):
    memo = dict()

    def _helper(row, col, visited):
        if row < 0 or row > grid.rows - 1 or col < 0 or col > grid.cols - 1:
            return 0

        if (row, col) in visited:
            return 0

        if row == grid.rows - 1:
            return 1

        if (row, col) not in visited and (row, col) in memo:
            return memo[(row, col)]

        cell = grid.grid[row][col]
        path_visited = visited.union({(row, col)})

        if cell == "^":
            left = _helper(row + 1, col - 1, path_visited)
            right = _helper(row + 1, col + 1, path_visited)
            result = left + right
        else:
            result = _helper(row + 1, col, path_visited)

        if (row, col) not in visited:
            memo[(row, col)] = result

        return result

    return _helper(grid.start[0], grid.start[1], set())


def solve(fname, part1=True):
    lines = read_file(fname)
    grid = parser(lines)
    # print(grid)
    if part1:
        tachyon_splits = bfs(grid)
        total = tachyon_splits
    else:
        timelines = dfs(grid)
        total = timelines
    return total


def part1():
    # result = solve("sample.txt")
    result = solve("input.txt")
    print(f"Part 1: {result}")


def part2():
    # result = solve("sample.txt", False)
    result = solve("input.txt", False)
    print(f"Part 2: {result}")


if __name__ == "__main__":
    part1()
    part2()
