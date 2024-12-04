use std::cmp::Ordering::Equal;
use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe(data: &[u32]) -> bool {
    let direction = data[0].cmp(&data[1]);
    if direction == Equal {
        return false;
    }
    data.iter().tuple_windows::<(&u32, &u32)>()
        .map(|(current, next)| (1..=3).contains(&current.abs_diff(*next)) && current.cmp(next) == direction)
        .all(|valid| valid)
}

fn is_safe_lenient(data: &[u32]) -> bool {
    if is_safe(data) {
        return true;
    }
    (0..data.len()).any(|index| {
        let mut modified_data = data.to_vec();
        modified_data.remove(index);
        is_safe(&modified_data)
    })
}

fn count_safe_reports<P>(input: &str, predicate: P) -> u32
where
    P: FnMut(&Vec<u32>) -> bool
{
    input.lines()
        .map(|line| line.split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .filter(predicate)
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_safe_reports(input, |data| is_safe(data)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_safe_reports(input, |data| is_safe_lenient(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
