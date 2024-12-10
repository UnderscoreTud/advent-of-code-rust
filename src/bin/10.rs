use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(10);

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    heightmap: Vec<u8>,
    trailheads: Vec<usize>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let width = input.lines().next().unwrap_or_default().len();
        let mut trailheads = Vec::new();
        let heightmap = input.lines()
            .flat_map(|line| line.chars())
            .map(|char| char.to_digit(10).unwrap() as u8)
            .enumerate()
            .map(|(index, height)| {
                if height == 0 { trailheads.push(index) }
                height
            })
            .collect_vec();
        let height = heightmap.len() / width;
        Map {
            height,
            width,
            heightmap,
            trailheads,
        }
    }

    fn xy_to_index(&self, pos: (usize, usize)) -> Option<usize> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(pos.1 * self.width + pos.0)
        } else {
            None
        }
    }

    fn index_to_xy(&self, index: &usize) -> Option<(usize, usize)> {
        if index < &(self.width * self.height) {
            Some((index % self.width, index / self.width))
        } else {
            None
        }
    }

    fn score(&self, pos: (usize, usize), current: u8, visited: &mut HashSet<(usize, usize)>) {
        if self.xy_to_index(pos).map(|index| self.heightmap[index]).is_none_or(|height| current != height) {
            return;
        }
        if current == 9 {
            visited.insert(pos);
            return;
        }
        for direction in DIRECTIONS {
            let Some(pos) = offset(&pos, &direction) else { continue };
            self.score(pos, current + 1, visited);
        }
    }

    fn rating(&self, pos: (usize, usize), current: u8) -> u32 {
        if self.xy_to_index(pos).map(|index| self.heightmap[index]).is_none_or(|height| current != height) {
            return 0;
        }
        if current == 9 {
            return 1;
        }
        let mut sum = 0;
        for direction in DIRECTIONS {
            let Some(pos) = offset(&pos, &direction) else { continue };
            sum += self.rating(pos, current + 1);
        }
        sum
    }
}

fn offset(pos: &(usize, usize), direction: &(isize, isize)) -> Option<(usize, usize)> {
    let x = pos.0.checked_add_signed(direction.0)?;
    let y = pos.1.checked_add_signed(direction.1)?;
    Some((x, y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::parse(input);
    Some(map.trailheads.iter()
        .map(|trailhead| {
            let mut visited = HashSet::new();
            map.score(map.index_to_xy(trailhead).unwrap(), 0, &mut visited);
            visited.len() as u32
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::parse(input);
    Some(map.trailheads.iter()
        .map(|trailhead| map.rating(map.index_to_xy(trailhead).unwrap(), 0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
