use rayon::iter::ParallelIterator;
use itertools::Itertools;
use rayon::prelude::ParallelString;

advent_of_code::solution!(7);

fn parse_and_solve<F>(input: &str, equation_checker: F) -> u64
    where F: Fn(&[u64], &u64) -> bool + Sync
{
    input.par_lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let Some(expected) = split.next() else { panic!() };
            let expected = expected[0..expected.len() - 1].parse::<u64>().unwrap();
            let terms = split.map(|num| num.parse::<u64>().unwrap()).collect_vec();
            (terms, expected)
        })
        .filter_map(|(terms, expected)| {
            if equation_checker(&terms, &expected) { Some(expected) } else { None }
        })
        .sum()
}

fn is_valid_equation(terms: &[u64], expected: &u64) -> bool {
    if terms.len() == 1 {
        return &terms[0] == expected;
    }

    let x = terms.last().unwrap();
    let slice = &terms[..terms.len() - 1];
    if expected % x == 0 && is_valid_equation(slice, &(expected / x)) {
        true
    } else {
        expected >= x && is_valid_equation(slice, &(expected - x))
    }
}

fn is_valid_equation_with_concat(terms: &[u64], expected: &u64) -> bool {
    if terms.len() == 1 {
        return &terms[0] == expected;
    }

    let right = terms.last().unwrap();
    let slice = &terms[..terms.len() - 1];
    if expected % right == 0 && is_valid_equation_with_concat(slice, &(expected / right)) {
        true
    } else if expected >= right && is_valid_equation_with_concat(slice, &(expected - right)) {
        true
    } else {
        let n_digits = right.ilog10() + 1;
        let y = 10u64.pow(n_digits);
        let suffix = expected % y;
        &suffix == right && is_valid_equation_with_concat(slice, &(expected / y))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_and_solve(input, is_valid_equation))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_and_solve(input, is_valid_equation_with_concat))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
