advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<u32>().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap()
        )
    }).unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_numbers, mut right_numbers) = parse_input(input);
    left_numbers.sort();
    right_numbers.sort();
    Some(left_numbers.iter().zip(right_numbers)
        .map(|(left, right)| left.abs_diff(right))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_numbers, right_numbers) = parse_input(input);
    Some(left_numbers.iter()
        .map(|left| left * right_numbers.iter().filter(|&&right| left == &right).count() as u32)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
