use aoc_utils::parse_file;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    operation: char,
    numbers: Vec<u64>,
}

impl Problem {
    fn new(operation: char, numbers: Vec<u64>) -> Self {
        Self { operation, numbers }
    }

    fn calculate(&self) -> u64 {
        match self.operation {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Worksheet {
    lines: Vec<String>,
}

impl Worksheet {
    fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    fn solve(&self, right_to_left: bool) -> u64 {
        self.parse_problems(right_to_left)
            .iter()
            .map(|p| p.calculate())
            .sum()
    }

    fn parse(lines: Vec<String>) -> std::io::Result<Self> {
        if lines.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Input Not Found",
            ));
        }
        Ok(Self::new(lines))
    }

    fn parse_problems(&self, right_to_left: bool) -> Vec<Problem> {
        if self.lines.is_empty() {
            return Vec::new();
        }

        let number_lines = &self.lines[..self.lines.len() - 1];
        let operations: Vec<char> = self.lines[self.lines.len() - 1]
            .split_whitespace()
            .filter_map(|s| s.chars().next())
            .collect();

        if right_to_left {
            Self::parse_right_to_left(number_lines, &operations)
        } else {
            Self::parse_left_to_right(number_lines, &operations)
        }
    }

    fn parse_left_to_right(number_lines: &[String], operations: &[char]) -> Vec<Problem> {
        let split_lines: Vec<Vec<&str>> = number_lines
            .iter()
            .map(|line| line.split_whitespace().collect())
            .collect();

        if split_lines.is_empty() {
            return Vec::new();
        }

        let num_problems = split_lines[0].len();
        let mut problems: Vec<Problem> = Vec::new();

        for col in 0..num_problems {
            let numbers: Vec<u64> = split_lines
                .iter()
                .filter_map(|row| row.get(col).and_then(|s| s.parse().ok()))
                .collect();

            if col < operations.len() {
                problems.push(Problem::new(operations[col], numbers));
            }
        }

        problems
    }

    fn parse_right_to_left(number_lines: &[String], operations: &[char]) -> Vec<Problem> {
        let split_lines: Vec<Vec<String>> = number_lines
            .iter()
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
            .collect();

        if split_lines.is_empty() {
            return Vec::new();
        }

        let num_cols = split_lines[0].len();
        let mut widths: Vec<usize> = Vec::new();

        for col in 0..num_cols {
            let max_width = split_lines
                .iter()
                .filter_map(|row| row.get(col).map(|s| s.len()))
                .max()
                .unwrap_or(0);
            widths.push(max_width);
        }

        let mut problems: Vec<Problem> = Vec::new();
        let mut skip = 0;

        for (width, &op) in widths.iter().zip(operations.iter()) {
            let mut numbers: Vec<u64> = Vec::new();
            let problem_chars: Vec<String> = number_lines
                .iter()
                .map(|line| {
                    let end = (skip + width).min(line.len());
                    let slice = if skip < line.len() {
                        &line[skip..end]
                    } else {
                        ""
                    };
                    format!("{:width$}", slice, width = width)
                })
                .collect();

            // transpose to get columns
            for digit_pos in 0..*width {
                let mut digit_str = String::new();
                for row in &problem_chars {
                    if let Some(ch) = row.chars().nth(digit_pos) {
                        if ch != ' ' {
                            digit_str.push(ch);
                        }
                    }
                }

                if !digit_str.is_empty() {
                    if let Ok(num) = digit_str.parse::<u64>() {
                        numbers.push(num);
                    }
                }
            }

            problems.push(Problem::new(op, numbers));
            skip += width + 1; // +1 for space separator
        }

        problems
    }
}

fn part1(ws: &Worksheet) -> u64 {
    ws.solve(false)
}

fn part2(ws: &Worksheet) -> u64 {
    ws.solve(true)
}

fn main() -> std::io::Result<()> {
    let ws = parse_file("2025/day6/input.txt", Worksheet::parse)?;

    println!("Loaded {} worksheet lines", ws.lines.len());

    println!("Part 1: {}", part1(&ws));
    println!("Part 2: {}", part2(&ws));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_FULL: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    fn parse_test_input() -> Worksheet {
        let lines: Vec<String> = TEST_INPUT_FULL.lines().map(|s| s.to_string()).collect();
        Worksheet::parse(lines).unwrap()
    }

    #[test]
    fn test_parse() {
        let ws = parse_test_input();
        assert_eq!(ws.lines.len(), 4);
    }

    #[test]
    fn test_parse_left_to_right() {
        let ws = parse_test_input();
        let problems = ws.parse_problems(false);

        assert_eq!(problems[0].operation, '*');
        assert_eq!(problems[0].numbers, vec![123, 45, 6]);

        assert_eq!(problems[1].operation, '+');
        assert_eq!(problems[1].numbers, vec![328, 64, 98]);

        assert_eq!(problems[2].operation, '*');
        assert_eq!(problems[2].numbers, vec![51, 387, 215]);

        assert_eq!(problems[3].operation, '+');
        assert_eq!(problems[3].numbers, vec![64, 23, 314]);
    }

    #[test]
    fn test_parse_right_to_left() {
        let ws = parse_test_input();
        let problems = ws.parse_problems(true);

        assert_eq!(problems[0].operation, '*');
        assert_eq!(problems[0].numbers, vec![1, 24, 356]);

        assert_eq!(problems[1].operation, '+');
        assert_eq!(problems[1].numbers, vec![369, 248, 8]);

        assert_eq!(problems[2].operation, '*');
        assert_eq!(problems[2].numbers, vec![32, 581, 175]);

        assert_eq!(problems[3].operation, '+');
        assert_eq!(problems[3].numbers, vec![623, 431, 4]);
    }

    #[test]
    fn test_calculate_addition() {
        let problem = Problem::new('+', vec![1, 2, 3]);
        let answer = problem.calculate();
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_calculate_multiplication() {
        let problem = Problem::new('*', vec![4, 5, 6]);
        let answer = problem.calculate();
        assert_eq!(answer, 120);
    }

    #[test]
    fn test_part1() {
        let ws = parse_test_input();
        assert_eq!(part1(&ws), 4277556);
    }

    #[test]
    fn test_part2() {
        let ws = parse_test_input();
        assert_eq!(part2(&ws), 3263827);
    }
}
