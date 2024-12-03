use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(regex.captures_iter(input)
        .map(|capture| {
            let x = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            x * y
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    Some(regex.captures_iter(input)
        .map(|capture| {
            match capture.get(0).unwrap().as_str() {
                "do()" => {
                    enabled = true;
                    0
                },
                "don't()" => {
                    enabled = false;
                    0
                },
                _ if enabled  => {
                    let x = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let y = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    x * y
                },
                _ => 0,
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
