use aoc_utils::parse_lines;

#[derive(Debug)]
struct PowerBank {
    batteries: Vec<u32>,
}

impl PowerBank {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let joltages = line.chars().filter_map(|c| c.to_digit(10)).collect();

        Some(Self {
            batteries: joltages,
        })
    }

    fn len(&self) -> usize {
        self.batteries.len()
    }
}

// O(n^2), O(1)
fn max_joltage_brute_force(power_bank: &PowerBank) -> u32 {
    let mut max_joltage = 0;

    for i in 0..power_bank.len() {
        for j in (i + 1)..power_bank.len() {
            let val = power_bank.batteries[i] * 10 + power_bank.batteries[j];
            max_joltage = max_joltage.max(val);
        }
    }

    max_joltage
}

// O(n), O(n)
fn max_joltage_greedy(power_bank: &PowerBank) -> u32 {
    let mut max_joltage = 0;

    let mut suffix_max = vec![0; power_bank.len()];
    for i in (0..power_bank.len() - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(power_bank.batteries[i + 1]);
    }

    for i in 0..power_bank.len() - 1 {
        let curr = power_bank.batteries[i] * 10 + suffix_max[i];
        max_joltage = max_joltage.max(curr);
    }

    max_joltage
}

fn find_max_combination(
    start: usize,
    remaining: usize,
    batteries: &Vec<u32>,
    current: u64,
    max_result: &mut u64,
) {
    // base case
    if remaining == 0 {
        *max_result = (*max_result).max(current);
        return;
    }

    // not enough batteries left
    if start + remaining > batteries.len() {
        return;
    }

    // try each battery from start position
    for i in start..batteries.len() {
        let new_val = current * 10 + batteries[i] as u64;
        find_max_combination(i + 1, remaining - 1, &batteries, new_val, max_result);
    }
}

fn max_joltage_k_digits(power_bank: &PowerBank, k: usize) -> u64 {
    let n = power_bank.batteries.len();

    if k == 0 || k > n {
        return 0;
    }

    let mut stack: Vec<u32> = Vec::new();

    for (i, &digit) in power_bank.batteries.iter().enumerate() {
        // see if we can remove smaller digits
        while !stack.is_empty() && stack.last().unwrap() < &digit && stack.len() + (n - i) > k {
            stack.pop();
        }

        if stack.len() < k {
            stack.push(digit);
        }
    }

    stack
        .iter()
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64)

    // max_joltage
}

fn part1(power_banks: &Vec<PowerBank>) -> u32 {
    power_banks.iter().map(|pb| max_joltage_greedy(&pb)).sum()
}

fn part2(power_banks: &Vec<PowerBank>) -> u64 {
    power_banks
        .iter()
        .map(|pb| max_joltage_k_digits(&pb, 12) as u64)
        .sum()
}

fn main() -> std::io::Result<()> {
    let power_banks = parse_lines("2025/day3/input.txt", PowerBank::parse)?;

    println!("Loaded {} power banks", power_banks.len());

    println!("Part 1: {}", part1(&power_banks));
    println!("Part 2: {}", part2(&power_banks));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_FULL: &str = "\
        987654321111111
        811111111111119
        234234234234278
        818181911112111";

    fn get_test_power_banks_part_1() -> Vec<(PowerBank, u32)> {
        vec![
            (
                PowerBank {
                    batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                },
                98,
            ),
            (
                PowerBank {
                    batteries: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                },
                89,
            ),
            (
                PowerBank {
                    batteries: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                },
                78,
            ),
            (
                PowerBank {
                    batteries: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                },
                92,
            ),
        ]
    }

    fn get_test_power_banks_part_2() -> Vec<(PowerBank, u64)> {
        vec![
            (
                PowerBank {
                    batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                },
                987654321111,
            ),
            (
                PowerBank {
                    batteries: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                },
                811111111119,
            ),
            (
                PowerBank {
                    batteries: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                },
                434234234278,
            ),
            (
                PowerBank {
                    batteries: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                },
                888911112111,
            ),
        ]
    }

    #[test]
    fn test_battery_parse() {
        let power_banks: Vec<PowerBank> = TEST_INPUT_FULL
            .split("\n")
            .filter_map(|item| PowerBank::parse(item))
            .collect();
        for b in power_banks {
            assert_eq!(b.len(), 15);
        }
    }

    #[test]
    fn test_max_joltage_brute_force() {
        let test_power_banks = get_test_power_banks_part_1();
        for (input, expected) in test_power_banks {
            let result = max_joltage_brute_force(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_max_joltage_greedy() {
        let test_power_banks = get_test_power_banks_part_1();
        for (input, expected) in test_power_banks {
            let result = max_joltage_greedy(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_max_joltage_n() {
        let test_power_banks = get_test_power_banks_part_1();
        for (input, expected) in test_power_banks {
            let result = max_joltage_k_digits(&input, 2);
            assert_eq!(result as u32, expected);
        }

        let test_power_banks = get_test_power_banks_part_2();
        for (input, expected) in test_power_banks {
            let result = max_joltage_k_digits(&input, 12);
            assert_eq!(result as u64, expected);
        }
    }

    #[test]
    fn test_day_3_part1() {
        let power_banks: Vec<PowerBank> = TEST_INPUT_FULL
            .split("\n")
            .filter_map(|item| PowerBank::parse(item))
            .collect();
        assert_eq!(part1(&power_banks), 357);
    }

    #[test]
    fn test_day_3_part2() {
        let power_banks: Vec<PowerBank> = TEST_INPUT_FULL
            .split("\n")
            .filter_map(|item| PowerBank::parse(item))
            .collect();
        assert_eq!(part2(&power_banks), 3121910778619);
    }
}
