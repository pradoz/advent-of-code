use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let reader = read_file(path)?;
    reader.lines().collect()
}

pub fn parse_lines<P, T, F>(path: P, parser: F) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Option<T>,
{
    let lines = read_lines(path)?;
    Ok(lines.iter().filter_map(|line| parser(line)).collect())
}

pub fn parse_file<P, T, F>(path: P, parser: F) -> io::Result<T>
where
    P: AsRef<Path>,
    F: Fn(Vec<String>) -> io::Result<T>,
{
    let lines = read_lines(path)?;
    parser(lines)
}

pub fn parse_lines_split<P, T, F>(path: P, separator: &str, parser: F) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Option<T>,
{
    let lines = read_lines(path)?;
    Ok(lines
        .iter()
        .flat_map(|line| line.split(separator))
        .filter_map(|item| parser(item))
        .collect())
}

pub fn parse_grid<P, T, F>(path: P, parser: F) -> io::Result<Vec<Vec<T>>>
where
    P: AsRef<Path>,
    F: Fn(usize, usize, char) -> T,
{
    let lines = read_lines(path)?;
    Ok(lines
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| parser(row_index, col_index, c))
                .collect()
        })
        .collect())
}

pub trait FromGrid: Sized {
    type Element;

    fn parse_element(row: usize, col: usize, c: char) -> Self::Element;
    fn from_grid(grid: Vec<Vec<Self::Element>>) -> Self;
}

pub fn parse_grid_from<P, T>(path: P) -> io::Result<T>
where
    P: AsRef<Path>,
    T: FromGrid,
{
    let grid_data = parse_grid(path, T::parse_element)?;
    Ok(T::from_grid(grid_data))
}

pub fn parse_sections_split<P, T, F>(path: P, parser: F) -> io::Result<T>
where
    P: AsRef<Path>,
    F: Fn(Vec<Vec<String>>) -> io::Result<T>,
{
    let content = std::fs::read_to_string(path)?;

    let sections = content
        .split("\n\n")
        .map(|s| s.lines().map(String::from).collect())
        .collect();

    parser(sections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_lines() {
        let temp_file = "test_temp.txt";
        let mut file = File::create(temp_file).unwrap();
        writeln!(file, "line1").unwrap();
        writeln!(file, "line2").unwrap();

        let lines = read_lines(temp_file).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");

        fs::remove_file(temp_file).unwrap();
    }
}
