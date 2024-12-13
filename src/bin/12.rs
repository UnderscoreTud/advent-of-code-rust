use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(12);

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let width = input.lines().next().unwrap_or_default().len();
        let data = input.lines()
            .flat_map(|line| line.chars())
            .collect_vec();
        let height = data.len() / width;
        Grid {
            height,
            width,
            data,
        }
    }

    fn get_perimeter(&self, index: usize, target: &char, visited: &mut HashSet<usize>) -> u32 {
        if visited.contains(&index) {
            return 0;
        }
        if self.data[index] != *target {
            return 1;
        }
        visited.insert(index);
        DIRECTIONS.iter()
            .map(|direction| self.offset(&index, direction))
            .map(|pos| pos.map(|pos| self.get_perimeter(pos, target, visited)).unwrap_or(1))
            .sum()
    }

    fn get_vertices(&self, index: usize, target: &char, visited: &mut HashSet<usize>) -> u32 {
        if self.data[index] != *target || visited.contains(&index) {
            return 0;
        }
        visited.insert(index);
    
        let vertices = [
            ((0, -1), (1, 0)),
            ((1, 0), (0, 1)),
            ((0, 1), (-1, 0)),
            ((-1, 0), (0, -1))
        ].iter()
            .filter_map(|adjecent| self.get_vertex(index, target, *adjecent))
            .count() as u32;

        vertices + (DIRECTIONS.iter()
            .filter_map(|direction| self.offset(&index, direction))
            .map(|pos| self.get_vertices(pos, target, visited))
            .sum::<u32>())
    }

    fn get_vertex(&self, index: usize, target: &char, adjacent: ((isize, isize), (isize, isize))) -> Option<(isize, isize)> {
        let corner_offset = (adjacent.0.0 + adjacent.1.0, adjacent.0.1 + adjacent.1.1);
        let is_corner_present = self.offset(&index, &corner_offset)
            .map(|index| self.data[index])
            .is_some_and(|char| char == *target);
        let is_first_present = self.offset(&index, &adjacent.0)
            .map(|index| self.data[index])
            .is_some_and(|char| char == *target);
        let is_second_present = self.offset(&index, &adjacent.1)
            .map(|index| self.data[index])
            .is_some_and(|char| char == *target);
        if (is_first_present && is_second_present && !is_corner_present)
            || (!is_first_present && !is_second_present) {
            let pos = self.index_to_xy(&index)?;
            return Some((pos.0 as isize + corner_offset.0, pos.1 as isize + corner_offset.1));
        }
        None
    }

    fn next_unvisited(&self, mut current: usize, visited: &HashSet<usize>) -> usize {
        while visited.contains(&current) {
            current += 1;
        }
        current
    }

    fn offset(&self, index: &usize, direction: &(isize, isize)) -> Option<usize> {
        let pos = self.index_to_xy(index)?;
        let x = pos.0.checked_add_signed(direction.0)?;
        let y = pos.1.checked_add_signed(direction.1)?;
        self.xy_to_index((x, y))
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

}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let len = grid.width * grid.height;
    let mut index = 0;
    let mut visited = HashSet::new();
    let mut sum = 0;
    while index < len {
        let mut area = HashSet::new();
        sum += grid.get_perimeter(index, &grid.data[index], &mut area) * area.len() as u32;
        visited.extend(area);
        index = grid.next_unvisited(index + 1, &visited);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let len = grid.width * grid.height;
    let mut index = 0;
    let mut visited = HashSet::new();
    let mut sum = 0;
    while index < len {
        let mut area = HashSet::new();
        sum += grid.get_vertices(index, &grid.data[index], &mut area) * area.len() as u32;
        visited.extend(area);
        index = grid.next_unvisited(index + 1, &visited);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
