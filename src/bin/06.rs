use std::collections::HashSet;
use itertools::Itertools;
use pariter::{scope, IteratorExt};

advent_of_code::solution!(6);

struct Map {
    dimensions: (isize, isize),
    obstacles: HashSet<(isize, isize)>,
    visited_pos: HashSet<(isize, isize)>,
    starting_pos: (isize, isize),
}

impl Map {

    fn parse(input: &str) -> Map {
        let (mut columns, mut rows) = (0, 0);
        let mut starting_pos = (0, 0);
        let mut obstacles = HashSet::new();
        let visited_pos = HashSet::new();
        input.lines()
            .enumerate()
            .map(|(y, line)| {
                rows = y;
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        columns = x;
                        match char {
                            '^' => starting_pos = (x as isize, y as isize),
                            '#' => {
                                obstacles.insert((x as isize, y as isize));
                            }
                            _ => {}
                        }
                        char
                    })
                    .collect_vec()
            })
            .collect_vec();
        Map {
            dimensions: (columns as isize + 1, rows as isize + 1),
            obstacles,
            visited_pos,
            starting_pos,
        }
    }

    fn is_obstacle(&self, pos: &(isize, isize)) -> bool {
        self.obstacles.contains(pos)
    }

    fn farthest_point(&mut self, starting_pos: &(isize, isize), direction: &(isize, isize)) -> Option<(isize, isize)> {
        let mut pos = *starting_pos;
        while self.is_in_map(&pos) {
            self.visited_pos.insert(pos);
            let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
            if self.is_obstacle(&next_pos) {
                return Some(pos);
            }
            pos = next_pos;
        }
        None
    }

    fn is_in_map(&self, pos: &(isize, isize)) -> bool {
        (0..self.dimensions.0).contains(&pos.0) && (0..self.dimensions.1).contains(&pos.1)
    }

    fn predict_path(&mut self) -> bool {
        let mut current_pos = self.starting_pos;
        let mut direction = 0;
        while let Some(next_pos) = self.farthest_point(&current_pos, &DIRECTIONS[direction]) {
            direction = (direction + 1) % 4;
            current_pos = next_pos;
        }
        false
    }

    fn has_loop(&mut self) -> bool {
        let mut current_pos = self.starting_pos;
        let mut direction = 0;
        let mut turns = HashSet::new();
        while let Some(next_pos) = self.farthest_point(&current_pos, &DIRECTIONS[direction]) {
            if turns.contains(&(current_pos, direction)) {
                return true;
            }
            turns.insert((current_pos, direction));
            direction = (direction + 1) % 4;
            current_pos = next_pos;
        }
        false
    }

}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::parse(input);
    map.predict_path();
    Some(map.visited_pos.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::parse(input);
    map.predict_path();
    let mut original_path = map.visited_pos;
    original_path.remove(&map.starting_pos);
    Some(scope(|scope| original_path.iter()
        .parallel_filter_scoped(scope, move |pos| {
            let mut new_obstacles = map.obstacles.clone();
            new_obstacles.insert(**pos);
            let mut map = Map {
                obstacles: new_obstacles,
                visited_pos: HashSet::new(),
                ..map
            };
            map.has_loop()
        })
        .count() as u32).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
