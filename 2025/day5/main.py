def read_file(fname, transformer):
    with open(fname) as f:
        data = [line for line in f.readlines()]
    return transformer(data)


def parser(data):
    fresh_ranges = []
    available_ids = []

    is_range = True
    for d in data:
        if d == "\n":
            is_range = False
            continue

        if is_range:
            parts = d.split("-")
            start = int(parts[0])
            end = int(parts[-1])
            fresh_ranges.append((start, end))
        else:
            available_ids.append(int(d))

    return fresh_ranges, available_ids


def count_fresh_ids(fresh_ranges, available_ids):
    result = 0

    for i in available_ids:
        for start, end in fresh_ranges:
            if i >= start and i <= end:
                result += 1
                break

    return result


def merge_fresh_ranges(fresh_ranges):
    merged = []

    fresh_ranges.sort(key=lambda x: x[0])

    curr_start, curr_end = fresh_ranges[0]

    for start, end in fresh_ranges[1:]:
        if start <= curr_end:
            curr_end = max(curr_end, end)
        else:
            merged.append((curr_start, curr_end))
            curr_start, curr_end = start, end

    merged.append((curr_start, curr_end))

    return merged


def count_merged_ranges(merged_ranges):
    result = 0

    for start, end in merged_ranges:
        result += end - start + 1

    return result


def part1():
    # fresh_ranges, available_ids = read_file("sample.txt", parser)
    fresh_ranges, available_ids = read_file("input.txt", parser)
    merged_ranges = merge_fresh_ranges(fresh_ranges)
    result = count_fresh_ids(merged_ranges, available_ids)
    print(f"Part 1: {result}")


def part2():
    # fresh_ranges, _ = read_file("sample.txt", parser)
    fresh_ranges, _ = read_file("input.txt", parser)
    merged_ranges = merge_fresh_ranges(fresh_ranges)
    result = count_merged_ranges(merged_ranges)
    print(f"Part 2: {result}")


if __name__ == "__main__":
    part1()
    part2()
