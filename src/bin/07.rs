use rayon::iter::ParallelIterator;
use itertools::Itertools;
use rayon::prelude::ParallelString;

advent_of_code::solution!(7);

fn parse_and_solve<F>(input: &str, equation_checker: F) -> u64
    where F: Fn(&u64, &[u64], &u64) -> bool + Sync
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
            if equation_checker(&terms[0], &terms[1..], &expected) { Some(expected) } else { None }
        })
        .sum()
}

fn is_valid_equation(current_term: &u64, terms: &[u64], expected: &u64) -> bool {
    if terms.is_empty() {
        return current_term == expected;
    }

    let next_term = terms[0];
    let new_terms = &terms[1..];
    [current_term * next_term, current_term + next_term].iter()
        .filter(|term| *term <= expected)
        .any(|term| is_valid_equation(term, new_terms, expected))
}

fn is_valid_equation_with_concat(current_term: &u64, terms: &[u64], expected: &u64) -> bool {
    if terms.is_empty() {
        return current_term == expected;
    }

    let next_term = terms[0];
    let new_terms = &terms[1..];
    [
        current_term * next_term,
        current_term + next_term,
        (current_term.to_string() + &next_term.to_string()).parse().unwrap(),
    ].iter()
        .filter(|term| *term <= expected)
        .any(|term| is_valid_equation_with_concat(term, new_terms, expected))
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
