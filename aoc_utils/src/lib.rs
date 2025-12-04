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
