use aoc_utils::parse_sections_split;

type IngredientId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreshRange {
    start: IngredientId,
    end: IngredientId,
}

impl FreshRange {
    fn new(start: IngredientId, end: IngredientId) -> Self {
        Self { start, end }
    }

    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim().split('-').collect();
        let start = parts[0].parse().ok()?;
        let end = parts[1].parse().ok()?;
        Some(Self::new(start, end))
    }

    fn contains(&self, id: IngredientId) -> bool {
        id >= self.start && id <= self.end
    }

    fn overlaps(&self, other: &FreshRange) -> bool {
        self.start <= other.end + 1 && other.start <= self.end + 1
    }

    fn merge(&self, other: &FreshRange) -> FreshRange {
        FreshRange::new(self.start.min(other.start), self.end.max(other.end))
    }

    fn count(&self) -> u64 {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Database {
    fresh_ranges: Vec<FreshRange>,
    available_ids: Vec<IngredientId>,
}

impl Database {
    fn new(fresh_ranges: Vec<FreshRange>, available_ids: Vec<IngredientId>) -> Self {
        Self {
            fresh_ranges,
            available_ids,
        }
    }

    fn parse(sections: Vec<Vec<String>>) -> std::io::Result<Self> {
        if sections.len() != 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected 2 sections, got {}", sections.len()),
            ));
        }

        let fresh_ranges: Vec<FreshRange> = sections[0]
            .iter()
            .filter_map(|line| FreshRange::parse(line))
            .collect();
        let available_ids: Vec<IngredientId> = sections[1]
            .iter()
            .filter_map(|line| line.trim().parse().ok())
            .collect();

        Ok(Self::new(fresh_ranges, available_ids))
    }

    fn is_fresh(&self, id: IngredientId) -> bool {
        self.fresh_ranges.iter().any(|range| range.contains(id))
    }

    fn merge_ranges(&self) -> Vec<FreshRange> {
        if self.fresh_ranges.is_empty() {
            return Vec::new();
        }

        let mut sorted_ranges = self.fresh_ranges.clone();
        sorted_ranges.sort_by_key(|r| r.start);

        let mut merged = Vec::new();
        let mut curr = sorted_ranges[0].clone();

        for range in sorted_ranges.iter().skip(1) {
            if curr.overlaps(range) {
                curr = curr.merge(range);
            } else {
                merged.push(curr);
                curr = range.clone();
            }
        }

        merged.push(curr);
        merged
    }
}

fn part1(db: &Database) -> usize {
    db.available_ids
        .iter()
        .filter(|&&id| db.is_fresh(id))
        .count()
}

fn part2(db: &Database) -> u64 {
    db.merge_ranges().iter().map(|range| range.count()).sum()
}

fn main() -> std::io::Result<()> {
    let db: Database = parse_sections_split("2025/day5/input.txt", Database::parse)?;

    println!("Part 1: {}", part1(&db));
    println!("Part 2: {}", part2(&db));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_FULL: &str = "\
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    fn parse_test_input() -> Database {
        let sections: Vec<Vec<String>> = TEST_INPUT_FULL
            .split("\n\n")
            .map(|s| s.lines().map(String::from).collect())
            .collect();
        Database::parse(sections).unwrap()
    }

    #[test]
    fn test_parse() {
        let db = parse_test_input();
        assert_eq!(
            db.fresh_ranges,
            vec![
                FreshRange { start: 3, end: 5 },
                FreshRange { start: 10, end: 14 },
                FreshRange { start: 16, end: 20 },
                FreshRange { start: 12, end: 18 },
            ]
        );
        assert_eq!(db.available_ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_merge_ranges() {
        let db = parse_test_input();
        let merged = db.merge_ranges();
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0], FreshRange::new(3, 5));
        assert_eq!(merged[1], FreshRange::new(10, 20));
    }

    #[test]
    fn test_part1() {
        let db = parse_test_input();
        assert_eq!(part1(&db), 3);
    }

    #[test]
    fn test_part2() {
        let db = parse_test_input();
        assert_eq!(part2(&db), 14);
    }
}
