from dataclasses import dataclass
import heapq


@dataclass
class Point:
    x: int
    y: int
    z: int


class UnionFind:
    def __init__(self, n):
        # each parent starts at itself
        self.parent = [i for i in range(n)]

        # size of each set
        self.size = [1 for _ in range(n)]

    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, u, v):
        # find roots
        root_u = self.find(u)
        root_v = self.find(v)

        # in the same set, no merge needed
        if root_u == root_v:
            return False

        # union by size. attach smaller tree to larger tree
        if self.size[root_u] < self.size[root_v]:
            self.parent[root_u] = root_v
            self.size[root_v] += self.size[root_u]
        else:
            self.parent[root_v] = root_u
            self.size[root_u] += self.size[root_v]

        # we merged
        return True

    def get_component_sizes(self):
        # find all unique roots and their sizes
        comp_sizes = dict()

        for i in range(len(self.parent)):
            root = self.find(i)
            if root not in comp_sizes:
                comp_sizes[root] = 0
            comp_sizes[root] += 1

        return list(comp_sizes.values())


def read_file(fname):
    with open(fname) as f:
        lines = [line.rstrip("\n") for line in f.readlines() if line.strip()]
    return lines


def parser(data):
    boxes = []
    for d in data:
        x, y, z = d.split(",")
        boxes.append(Point(int(x), int(y), int(z)))
    return boxes


def euclidean_distance(p1: Point, p2: Point) -> int:
    return (abs(p1.x - p2.x) ** 2 + abs(p1.y - p2.y) ** 2 + abs(p1.z - p2.z) ** 2) ** 0.5


def generate_edges(points: list[Point]) -> list:
    edges = []
    for i in range(len(points)):
        for j in range(i + 1, len(points)):
            u = points[i]
            v = points[j]
            distance = euclidean_distance(u, v)
            heapq.heappush(edges, (distance, i, j))
    return edges


def solve_part1(fname, k, pick):
    data = read_file(fname)
    result = 1
    points = parser(data)
    edges = generate_edges(points)

    uf = UnionFind(len(points))

    for _ in range(k):
        if not edges:
            break
        dist, u, v = heapq.heappop(edges)
        uf.union(u, v)

    comp_sizes = uf.get_component_sizes()
    comp_sizes.sort(reverse=True)

    for i in range(pick):
        result *= comp_sizes[i]

    return result


def solve_part2(fname):
    data = read_file(fname)
    points = parser(data)
    edges = generate_edges(points)

    n = len(points)
    uf = UnionFind(n)

    connections = 0
    while edges and connections < n - 1:
        dist, u, v = heapq.heappop(edges)

        if uf.union(u, v):
            connections += 1
        if connections == n - 1:
            point_u = points[u]
            point_v = points[v]

            result = point_u.x * point_v.x
            return result
    return -1


def part1():
    # result = solve_part1("sample.txt", 10, 3)
    result = solve_part1("input.txt", 1000, 3)
    print(f"Part 1: {result}")


def part2():
    # result = solve_part2("sample.txt")
    result = solve_part2("input.txt")
    print(f"Part 2: {result}")


if __name__ == "__main__":
    part1()
    part2()
