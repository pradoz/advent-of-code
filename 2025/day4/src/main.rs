use aoc_utils::parse_grid;
use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // right
    (-1, 0),  // left
    (0, 1),   // up
    (0, -1),  // down
    (1, 1),   // top-right
    (1, -1),  // top-left
    (-1, 1),  // bot-right
    (-1, -1), // bot-left
];

type Coord = (usize, usize);
type Grid = Vec<Vec<Point>>;

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

    fn is_roll(&self) -> bool {
        self.value == '@'
    }
}

fn count_neighbors(point: &Point, grid: &Grid, removed: &HashSet<Coord>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    DIRECTIONS
        .iter()
        .filter_map(|(dx, dy)| {
            let new_x = point.x as isize + dx;
            let new_y = point.y as isize + dy;

            if new_x < 0 || new_x > (rows - 1) as isize || new_y < 0 || new_y > (cols - 1) as isize
            {
                return None;
            }

            let neighbor = &grid[new_x as usize][new_y as usize];
            if neighbor.is_roll() && !removed.contains(&neighbor.coords()) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn find_accessible(grid: &Grid, removed: &HashSet<Coord>) -> Vec<Coord> {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|point| point.is_roll() && !removed.contains(&point.coords()))
        .filter(|point| count_neighbors(point, grid, removed) < 4)
        .map(|point| point.coords())
        .collect()
}

fn forkliftable(grid: &Grid, should_remove: bool) -> usize {
    let mut result = 0;
    let mut removed = HashSet::new();

    loop {
        let accessible = find_accessible(grid, &removed);

        if accessible.is_empty() {
            break;
        }

        result += accessible.len();

        if !should_remove {
            break;
        }

        removed.extend(accessible);
    }

    result
}

fn part1(grid: &Grid) -> usize {
    forkliftable(grid, false)
}

fn part2(grid: &Grid) -> usize {
    forkliftable(grid, true)
}

fn main() -> std::io::Result<()> {
    let grid: Grid = parse_grid("2025/day4/input.txt", Point::new)?;

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_FULL: &str = "\
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    fn parse_test_input() -> Grid {
        TEST_INPUT_FULL
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(col_index, c)| Point::new(row_index, col_index, c))
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_parse() {
        let grid = parse_test_input();
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);

        assert_eq!(grid[0][0].value, '.');
        assert_eq!(grid[0][0].coords(), (0, 0));

        assert_eq!(grid[0][2].value, '@');
        assert!(grid[0][2].is_roll());
    }

    #[test]
    fn test_part1() {
        let grid = parse_test_input();
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_part2() {
        let grid = parse_test_input();
        assert_eq!(part2(&grid), 43);
    }
}
