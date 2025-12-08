use aoc_utils::{FromGrid, parse_grid_from};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: char,
}

impl Point {
    fn new(x: usize, y: usize, value: char) -> Self {
        Self { x, y, value }
    }

    fn coords(&self) -> Coord {
        (self.x, self.y)
    }

    fn is_start(&self) -> bool {
        self.value == 'S'
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    start: Coord,
    data: Vec<Vec<Point>>,
}

impl Grid {
    fn new(data: Vec<Vec<Point>>) -> Self {
        let mut start = None;

        for row in &data {
            for point in row {
                if point.is_start() {
                    start = Some(point.coords());
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }

        let start = start.expect("No starting point found");
        Self { start, data }
    }

    fn at(&self, row: usize, col: usize) -> char {
        self.data[row][col].value
    }

    fn start(&self) -> Coord {
        self.start
    }

    fn num_rows(&self) -> usize {
        self.data.len()
    }

    fn num_cols(&self) -> usize {
        self.data[0].len()
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.num_rows() && col < self.num_cols()
    }
}

impl FromGrid for Grid {
    type Element = Point;

    fn parse_element(row: usize, col: usize, c: char) -> Self::Element {
        Point::new(row, col, c)
    }

    fn from_grid(grid: Vec<Vec<Self::Element>>) -> Self {
        Grid::new(grid)
    }
}

fn bfs(grid: &Grid) -> u64 {
    let mut splits = 0;
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    queue.push_back(grid.start());

    while let Some((row, col)) = queue.pop_front() {
        if !grid.in_bounds(row, col) {
            continue;
        }

        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));

        if row == grid.num_rows() - 1 {
            continue;
        }

        let cell = grid.at(row, col);

        if cell != '^' {
            queue.push_back((row + 1, col));
        } else {
            splits += 1;
            if col > 0 {
                queue.push_back((row, col - 1));
            }
            queue.push_back((row, col + 1));
        }
    }

    splits
}

fn dfs(grid: &Grid) -> usize {
    let mut memo: HashMap<Coord, usize> = HashMap::new();

    fn _recurse(
        grid: &Grid,
        row: usize,
        col: usize,
        visited: Rc<HashSet<Coord>>,
        memo: &mut HashMap<Coord, usize>,
    ) -> usize {
        if !grid.in_bounds(row, col) {
            return 0;
        }
        if visited.contains(&(row, col)) {
            return 0;
        }

        if row == grid.num_rows() - 1 {
            return 1;
        }

        if let Some(&cached) = memo.get(&(row, col)) {
            return cached;
        }

        let mut path_visited = (*visited).clone();
        path_visited.insert((row, col));
        let path_visited = Rc::new(path_visited);

        let cell = grid.at(row, col);

        let result = if cell == '^' {
            let left = if col > 0 {
                _recurse(grid, row + 1, col - 1, Rc::clone(&path_visited), memo)
            } else {
                0
            };
            let right = _recurse(grid, row + 1, col + 1, Rc::clone(&path_visited), memo);
            left + right
        } else {
            _recurse(grid, row + 1, col, Rc::clone(&path_visited), memo)
        };

        if !visited.contains(&(row, col)) {
            memo.insert((row, col), result);
        }

        result
    }

    let (x, y) = grid.start();
    _recurse(grid, x, y, Rc::new(HashSet::new()), &mut memo)
}

fn part1(grid: &Grid) -> u64 {
    bfs(grid)
}

fn part2(grid: &Grid) -> usize {
    dfs(grid)
}

fn main() -> std::io::Result<()> {
    let grid: Grid = parse_grid_from("2025/day7/input.txt")?;

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FULL: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    fn parse_test_input() -> Grid {
        Grid::from_grid(
            TEST_INPUT_FULL
                .lines()
                .enumerate()
                .map(|(row_index, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(|(col_index, c)| Grid::parse_element(row_index, col_index, c))
                        .collect()
                })
                .collect(),
        )
    }

    #[test]
    fn test_parse() {
        let grid = parse_test_input();
        assert_eq!(grid.num_rows(), 16);
        assert_eq!(grid.num_cols(), 15);
        assert_eq!(grid.start, (0, 7));
    }

    #[test]
    fn test_part1() {
        let grid = parse_test_input();
        assert_eq!(part1(&grid), 21);
    }

    #[test]
    fn test_part2() {
        let grid = parse_test_input();
        assert_eq!(part2(&grid), 40);
    }
}
