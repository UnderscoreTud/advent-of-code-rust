advent_of_code::solution!(2);

fn is_safe(data: &Vec<u32>) -> bool {
    let mut previous_direction = None;
    for i in 0..(data.len() - 1) {
        let diff = data[i].abs_diff(data[i + 1]);
        if diff == 0 || diff > 3 {
            return false;
        }
        let direction = data[i].cmp(&data[i + 1]);
        match previous_direction {
            Some(previous_direction) => if previous_direction != direction {
                return false;
            },
            None => previous_direction = Some(direction)
        }
    }
    true
}

fn is_safe_lenient(data: &Vec<u32>) -> bool {
    if is_safe(data) {
        return true;
    }
    for i in 0..data.len() {
        let mut modified_data = data.clone();
        modified_data.remove(i);
        if is_safe(&modified_data) {
            return true
        }
    }
    false
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
    Some(count_safe_reports(input, is_safe))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_safe_reports(input, is_safe_lenient))
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
