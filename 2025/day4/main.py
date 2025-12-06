from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int
    char: str

    def coords(self):
        return (self.x, self.y)

    def is_roll(self):
        return self.char == '@'

    @staticmethod
    def make(x, y, data):
        return Point(x, y, data[x][y])


directions = [
    (1, 0),    # right
    (-1, 0),   # left
    (0, 1),    # up
    (0, -1),   # down
    (1, 1),    # top-right
    (1, -1),   # top-left
    (-1, 1),   # bot-right
    (-1, -1),  # bot-left
]


def read_file(fname, transformer):
    with open(fname) as f:
        data = [line.strip() for line in f.readlines()]
    return transformer(data)


def init_grid(data):
    rows = len(data)
    cols = len(data[0])

    grid = []
    for i in range(rows):
        row = []
        for j in range(cols):
            p = Point.make(i, j, data)
            row.append(p)
        grid.append(row)
    return grid


def count_neighbors(p, grid, rows, cols, removed={}):
    count = 0

    for dx, dy in directions:
        new_x = p.x + dx
        new_y = p.y + dy
        if (new_x < 0 or new_x > rows - 1 or new_y < 0 or new_y > cols - 1):
            continue
        neighbor = grid[new_x][new_y]
        if neighbor.char == '@' and neighbor.coords() not in removed:
            count += 1
    return count


def forkliftable(grid, should_remove=False):
    result = 0
    removed = set()

    while True:
        accessible = find_accessible(grid, removed)
        if not accessible:
            break

        for a in accessible:
            removed.add(a)

        result += len(accessible)

        # only run this loop once if we are doing part 1
        if not should_remove:
            break

        removed.update(accessible)

    return result


def find_accessible(grid, removed):
    accessible = []
    rows = len(grid)
    cols = len(grid[0])

    for i in range(rows):
        for j in range(cols):
            p = grid[i][j]
            if not p.is_roll() or p.coords() in removed:
                continue
            neighbors = count_neighbors(p, grid, rows, cols, removed)
            if neighbors < 4:
                accessible.append(p.coords())
    return accessible


def part1():
    # grid = read_file("sample.txt", init_grid)
    grid = read_file("input.txt", init_grid)
    result = forkliftable(grid)
    print(f"Part 1: {result}")


def part2():
    # grid = read_file("sample.txt", init_grid)
    grid = read_file("input.txt", init_grid)
    result = forkliftable(grid, should_remove=True)
    print(f"Part 2: {result}")


if __name__ == "__main__":
    part1()
    part2()
