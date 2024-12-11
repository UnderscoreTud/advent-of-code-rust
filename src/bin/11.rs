use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink(stone: u64, iterations_left: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if iterations_left == 0 {
        return 1;
    }
    if let Some(cached_value) = memo.get(&(stone, iterations_left)) {
        return *cached_value;
    }

    let (first_half, second_half) = blink_stone(stone);
    let mut sum = blink(first_half, iterations_left - 1, memo);
    if let Some(second_half) = second_half { sum += blink(second_half, iterations_left - 1, memo); }

    memo.insert((stone, iterations_left), sum);

    sum
}

fn blink_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 != 0 {
        return (stone * 2024, None)
    }
    let divisor = 10u64.pow(digits / 2);
    let first_half = stone / divisor;
    let second_half = stone % divisor;
    (first_half, Some(second_half))
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input.split_ascii_whitespace().map(|number| number.parse::<u64>().unwrap()).collect_vec();
    let mut memo = HashMap::new();
    Some(stones.iter().map(|stone| blink(*stone, 25, &mut memo)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input.split_ascii_whitespace().map(|number| number.parse::<u64>().unwrap()).collect_vec();
    let mut memo = HashMap::new();
    Some(stones.iter().map(|stone| blink(*stone, 75, &mut memo)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
