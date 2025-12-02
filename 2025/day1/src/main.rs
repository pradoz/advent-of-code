use aoc_utils::parse_lines;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }

    fn step(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    distance: i32,
}

impl Action {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let ch = line.chars().next()?;
        let direction = Direction::from_char(ch)?;
        let distance = line[1..].parse().ok()?;

        Some(Self {
            direction,
            distance,
        })
    }
}

struct Dial {
    position: i32,
    modulo: i32,
}

impl Dial {
    fn new(starting_position: i32, modulo: i32) -> Self {
        Self {
            position: starting_position,
            modulo,
        }
    }

    fn step(&mut self, direction: Direction) -> bool {
        self.position = (self.position + direction.step()).rem_euclid(self.modulo);
        self.position == 0
    }

    fn execute(&mut self, action: &Action) -> (i32, i32) {
        let mut crosses_during = 0;

        for _ in 0..action.distance {
            if self.step(action.direction) {
                crosses_during += 1;
            }
        }

        if self.position == 0 {
            // if we end at 0, it was counted in crosses_during
            crosses_during -= 1;
        }

        (if self.position == 0 { 1 } else { 0 }, crosses_during)
    }
}

fn part1(actions: &[Action]) -> i32 {
    let mut dial = Dial::new(50, 100);
    let mut count = 0;

    for action in actions {
        let (ends_at_zero, _) = dial.execute(action);
        count += ends_at_zero;
    }

    count
}

fn part2(actions: &[Action]) -> i32 {
    let mut dial = Dial::new(50, 100);
    let mut count = 0;

    for action in actions {
        let (ends_at_zero, crosses_during) = dial.execute(action);
        count += ends_at_zero + crosses_during;
    }

    count
}

fn main() -> std::io::Result<()> {
    let actions = parse_lines("2025/day1/input.txt", Action::parse)?;

    println!("Loaded {} actions", actions.len());
    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

    #[test]
    fn test_direction_from_char() {
        assert_eq!(Direction::from_char('L'), Some(Direction::Left));
        assert_eq!(Direction::from_char('R'), Some(Direction::Right));
        assert_eq!(Direction::from_char('X'), None);
    }

    #[test]
    fn test_action_parse() {
        let action = Action::parse("L68").unwrap();
        assert_eq!(action.direction, Direction::Left);
        assert_eq!(action.distance, 68);

        let action = Action::parse("R48").unwrap();
        assert_eq!(action.direction, Direction::Right);
        assert_eq!(action.distance, 48);

        assert!(Action::parse("").is_none());
        assert!(Action::parse("X10").is_none());
    }

    #[test]
    fn test_dial_step() {
        let mut dial = Dial::new(50, 100);

        // step left from 50 to 49
        assert!(!dial.step(Direction::Left));
        assert_eq!(dial.position, 49);

        // step right from 49 to 50
        assert!(!dial.step(Direction::Right));
        assert_eq!(dial.position, 50);
    }

    #[test]
    fn test_dial_wrapping() {
        let mut dial = Dial::new(0, 100);

        // step left from 0 to 99
        assert!(!dial.step(Direction::Left));
        assert_eq!(dial.position, 99);

        // step right from 99 to 0
        assert!(dial.step(Direction::Right));
        assert_eq!(dial.position, 0);
    }

    #[test]
    fn test_day_1_part1() {
        let actions: Vec<Action> = TEST_INPUT
            .lines()
            .filter_map(|line| Action::parse(line))
            .collect();
        assert_eq!(part1(&actions), 3);
    }

    #[test]
    fn test_day_1_part2() {
        let actions: Vec<Action> = TEST_INPUT
            .lines()
            .filter_map(|line| Action::parse(line))
            .collect();
        assert_eq!(part2(&actions), 6);
    }
}
