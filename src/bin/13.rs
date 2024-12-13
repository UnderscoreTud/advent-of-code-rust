use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

const PREFIX: i64 = 10000000000000;

fn parse(input: &str) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    let pattern = Regex::new(r"\n\s*\n").unwrap();
    pattern.split(input).map(parse_equation).collect()
}

fn parse_with_prefix(input: &str) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    let pattern = Regex::new(r"\n\s*\n").unwrap();
    pattern.split(input).map(parse_equation_with_prefix).collect()
}

fn parse_equation(input: &str) -> ((i64, i64, i64), (i64, i64, i64)) {
    let equation: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            let first = &line[(line.find('+').or(line.find('=')).unwrap() + 1)..line.find(',').unwrap()];
            let second = &line[(line.rfind('+').or(line.rfind('=')).unwrap() + 1)..].trim_end();
            let first: i64 = first.parse().unwrap();
            let second: i64 = second.parse().unwrap();
            (first, second)
        })
        .collect_vec();
    ((equation[0].0, equation[1].0, equation[2].0), (equation[0].1, equation[1].1, equation[2].1))
}

fn parse_equation_with_prefix(input: &str) -> ((i64, i64, i64), (i64, i64, i64)) {
    let equation: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            let first = &line[(line.find('+').or(line.find('=')).unwrap() + 1)..line.find(',').unwrap()];
            let second = &line[(line.rfind('+').or(line.rfind('=')).unwrap() + 1)..].trim_end();
            let first: i64 = first.parse().unwrap();
            let second: i64 = second.parse().unwrap();
            (first, second)
        })
        .collect_vec();
    ((equation[0].0, equation[1].0, PREFIX + equation[2].0), (equation[0].1, equation[1].1, PREFIX + equation[2].1))
}

fn solve(eq1: (i64, i64, i64), eq2: (i64, i64, i64)) -> Option<i64> {
    let (a1, b1, c1) = eq1;
    let (a2, b2, c2) = eq2;

    #[allow(non_snake_case)]
    let det_A = a1 * b2 - b1 * a2;
    if det_A == 0 {
        return None;
    }

    let x = (c1 * b2 - b1 * c2) / det_A;
    let y = (a1 * c2 - c1 * a2) / det_A;

    if a1 * x + b1 * y == c1 && a2 * x + b2 * y == c2 {
        Some(x * 3 + y)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(parse(input).iter()
        .filter_map(|(eq1, eq2)| solve(*eq1, *eq2))
        .sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(parse_with_prefix(input).iter()
        .filter_map(|(eq1, eq2)| solve(*eq1, *eq2))
        .sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
