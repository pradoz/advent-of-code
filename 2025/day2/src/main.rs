use aoc_utils::parse_lines_split;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
struct RangePair {
    start: i64,
    end: i64,
}

impl RangePair {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let (start_str, end_str) = line.split_once('-')?;
        let start = start_str.parse().ok()?;
        let end = end_str.parse().ok()?;
        // println!("Parsed range pair: [{}, {}]", start, end);

        Some(Self { start, end })
    }

    fn to_range(&self) -> RangeInclusive<i64> {
        self.start..=self.end
    }
}

fn is_invalid_id1(id: &i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let left = &s[..half];
    let right = &s[half..];

    left == right
}

fn is_invalid_id2(id: &i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            if s.chars()
                .collect::<Vec<_>>()
                .chunks(pattern_len)
                .all(|chunk| chunk.iter().collect::<String>() == pattern)
            {
                return true;
            }
        }
    }

    false
}

// fn check_range(pair: &RangePair) -> i64 {
//     pair.to_range()
//         .filter(|id| is_invalid_id(id))
//         .sum()
//     // let mut invalid_count = 0;
//
//     // for i in pair.to_range() {
//     //     invalid_count += if is_invalid_id(&i) { i } else { 0 };
//     // }
//
//     // invalid_count
// }

fn part1(range_pairs: &Vec<RangePair>) -> i64 {
    range_pairs
        .iter()
        .flat_map(|pair| pair.to_range())
        .filter(|id| is_invalid_id1(id))
        .sum()
    // let mut result: i64 = 0;

    // for p in range_pairs {
    //     result += check_range(&p);
    // }

    // result
}

fn part2(range_pairs: &Vec<RangePair>) -> i64 {
    range_pairs
        .iter()
        .flat_map(|pair| pair.to_range())
        .filter(|id| is_invalid_id2(id))
        .sum()
}

fn main() -> std::io::Result<()> {
    let ranges = parse_lines_split("2025/day2/input.txt", ",", RangePair::parse)?;

    println!("Loaded {} ranges", ranges.len());

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const FULL_TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_range_pair_parse() {
        let range = RangePair::parse("11-22").unwrap();
        assert_eq!(range.start, 11);
        assert_eq!(range.end, 22);

        let range = RangePair::parse("1188511880-1188511890").unwrap();
        assert_eq!(range.start, 1188511880);
        assert_eq!(range.end, 1188511890);

        assert!(RangePair::parse("").is_none());
        assert!(RangePair::parse("blah").is_none());
    }

    #[test]
    fn test_range_pair_parse_multiple() {
        let input = "11-22,95-115,998-1012";

        let range_pairs: Vec<RangePair> = input
            .split(",")
            .filter_map(|item| RangePair::parse(item))
            .collect();

        assert_eq!(range_pairs.len(), 3);

        assert_eq!(range_pairs[0].start, 11);
        assert_eq!(range_pairs[0].end, 22);

        assert_eq!(range_pairs[1].start, 95);
        assert_eq!(range_pairs[1].end, 115);

        assert_eq!(range_pairs[2].start, 998);
        assert_eq!(range_pairs[2].end, 1012);
    }

    #[test]
    fn test_is_invalid_id1() {
        assert!(is_invalid_id1(&11));
        assert!(is_invalid_id1(&22));
        assert!(is_invalid_id1(&6464));
        assert!(is_invalid_id1(&123123));
        assert!(is_invalid_id1(&1010));
        assert!(is_invalid_id1(&1188511885));
        assert!(is_invalid_id1(&222222));
        assert!(is_invalid_id1(&446446));
        assert!(is_invalid_id1(&38593859));

        assert!(!is_invalid_id1(&10));
        assert!(!is_invalid_id1(&101));
        assert!(!is_invalid_id1(&35353));
    }

    #[test]
    fn test_is_invalid_id2() {
        assert!(is_invalid_id2(&11));
        assert!(is_invalid_id2(&22));
        assert!(is_invalid_id2(&99));
        assert!(is_invalid_id2(&111));
        assert!(is_invalid_id2(&999));
        assert!(is_invalid_id2(&6464));
        assert!(is_invalid_id2(&123123));
        assert!(is_invalid_id2(&1010));
        assert!(is_invalid_id2(&1188511885));
        assert!(is_invalid_id2(&222222));
        assert!(is_invalid_id2(&2121212121));
        assert!(is_invalid_id2(&446446));
        assert!(is_invalid_id2(&353535));
        assert!(is_invalid_id2(&38593859));

        assert!(!is_invalid_id2(&12));
        assert!(!is_invalid_id2(&123));
        assert!(!is_invalid_id2(&1234));
        assert!(!is_invalid_id2(&100));
        assert!(!is_invalid_id2(&1698522));
    }

    #[test]
    fn test_day_1_part1() {
        let range_pairs: Vec<RangePair> = FULL_TEST_INPUT
            .split(",")
            .filter_map(|item| RangePair::parse(item))
            .collect();
        assert_eq!(part1(&range_pairs), 1227775554);
    }

    #[test]
    fn test_day_1_part2() {
        let range_pairs: Vec<RangePair> = FULL_TEST_INPUT
            .split(",")
            .filter_map(|item| RangePair::parse(item))
            .collect();
        assert_eq!(part2(&range_pairs), 4174379265);
    }
}
