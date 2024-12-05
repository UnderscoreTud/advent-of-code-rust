use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let split_index = input.rfind('|').unwrap() + 3;
    let ordering_rules = &input[..split_index];
    let split_index = input.find(',').unwrap() - 2;
    let pages = &input[split_index..];
    Some(count_sorted_updates(pages, &parse_ordering_rules(ordering_rules)))
}

fn parse_ordering_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut ordering_rules: HashMap<_, Vec<_>> = HashMap::new();
    input.lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|(x, y)| {
            ordering_rules.entry(y).or_default().push(x);
        });
    ordering_rules
}

fn count_sorted_updates(input: &str, ordering_rules: &HashMap<u32, Vec<u32>>) -> u32 {
    input.lines()
        .map(|line| line.split(',')
            .map(|page| page.parse::<u32>().unwrap())
            .collect_vec())
        .filter(|pages| is_update_sorted(pages, ordering_rules))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn is_update_sorted(pages: &[u32], ordering_rules: &HashMap<u32, Vec<u32>>) -> bool {
    pages.iter().enumerate()
        .map(|(index, page)| (index, ordering_rules.get(page)))
        .filter(|(_, page)| page.is_some())
        .map(|(index, page)| (index, page.unwrap()))
        .all(|(index, ordering_rules)| pages.iter()
            .skip(index + 1)
            .all(|next_page| !ordering_rules.contains(next_page)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_index = input.rfind('|').unwrap() + 3;
    let ordering_rules = &input[..split_index];
    let split_index = input.find(',').unwrap() - 2;
    let pages = &input[split_index..];
    Some(count_unsorted_updates(pages, &parse_ordering_rules(ordering_rules)))
}

fn count_unsorted_updates(input: &str, ordering_rules: &HashMap<u32, Vec<u32>>) -> u32 {
    input.lines()
        .map(|line| line.split(',')
            .map(|page| page.parse::<u32>().unwrap())
            .collect_vec())
        .filter_map(|pages| {
            let sorted_pages = sort_update(&pages, ordering_rules);
            if pages == sorted_pages {
                None
            } else {
                Some(sorted_pages)
            }
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn sort_update(pages: &[u32], ordering_rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut sorted_pages = Vec::new();
    let empty_vec = Vec::default();
    for page in pages {
        let preceding_pages = ordering_rules.get(page).unwrap_or(&empty_vec);
        let index = sorted_pages.iter()
            .enumerate()
            .rfind(|(_, current_page)| preceding_pages.contains(current_page))
            .map(|(index, _)| index + 1)
            .unwrap_or(0);
        sorted_pages.insert(index, *page);
    }
    sorted_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
