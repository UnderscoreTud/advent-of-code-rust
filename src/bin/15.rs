use itertools::Itertools;
use std::cmp::PartialEq;

advent_of_code::solution!(15);

const UP: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (1, 0);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum FatTile {
    Empty,
    Wall,
    LeftBox,
    RightBox,
}

#[derive(Debug)]
struct Map<T> {
    width: usize,
    height: usize,
    tiles: Vec<T>,
    robot: usize,
}

trait GenericMap {
    fn parse(input: &str) -> (Self, Vec<(isize, isize)>) where Self: Sized;
    fn play(&mut self, instructions: &[(isize, isize)]) {
        for instruction in instructions {
            self.move_robot(instruction);
        }
    }

    fn move_robot(&mut self, direction: &(isize, isize));
    fn move_box(&mut self, pos: usize, direction: &(isize, isize)) -> bool;

    fn xy_to_index(&self, pos: (usize, usize)) -> Option<usize> {
        if (0..self.width()).contains(&pos.0) && (0..self.height()).contains(&pos.1) {
            Some(pos.1 * self.width() + pos.0)
        } else {
            None
        }
    }

    fn index_to_xy(&self, index: &usize) -> Option<(usize, usize)> {
        if index < &(self.width() * self.height()) {
            Some((index % self.width(), index / self.width()))
        } else {
            None
        }
    }

    fn offset(&self, index: &usize, direction: &(isize, isize)) -> Option<usize> {
        let pos = self.index_to_xy(index)?;
        let x = pos.0.checked_add_signed(direction.0)?;
        let y = pos.1.checked_add_signed(direction.1)?;
        self.xy_to_index((x, y))
    }

    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl GenericMap for Map<Tile> {
    fn parse(input: &str) -> (Map<Tile>, Vec<(isize, isize)>) {
        let width = input.lines().next().unwrap_or_default().len();
        let mut robot = 0;
        let mut instructions = Vec::new();
        let tiles = input.lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .filter_map(|(index, char)| {
                match char {
                    '#' => Some(Tile::Wall),
                    'O' => Some(Tile::Box),
                    '@' => {
                        robot = index;
                        Some(Tile::Empty)
                    }
                    '.' => Some(Tile::Empty),
                    '^' => {
                        instructions.push(UP);
                        None
                    }
                    '>' => {
                        instructions.push(RIGHT);
                        None
                    }
                    'v' => {
                        instructions.push(DOWN);
                        None
                    }
                    '<' => {
                        instructions.push(LEFT);
                        None
                    }
                    _ => None,
                }
            })
            .collect_vec();
        let height = tiles.len() / width;
        (Map {
            height,
            width,
            tiles,
            robot,
        }, instructions)
    }

    fn move_robot(&mut self, direction: &(isize, isize)) {
        let Some(new_pos) = self.offset(&self.robot, direction) else { return };
        match self.tiles[new_pos] {
            Tile::Empty => self.robot = new_pos,
            Tile::Box => if self.move_box(new_pos, direction) {
                self.robot = new_pos;
            }
            _ => {}
        }
    }

    fn move_box(&mut self, pos: usize, direction: &(isize, isize)) -> bool {
        let mut new_pos = Some(pos);
        loop {
            new_pos = self.offset(&new_pos.unwrap(), direction);
            let Some(new_pos) = new_pos else { break false };
            match self.tiles[new_pos] {
                Tile::Empty => {
                    self.tiles[pos] = Tile::Empty;
                    self.tiles[new_pos] = Tile::Box;
                    break true
                }
                Tile::Wall => break false,
                _ => continue
            }
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl GenericMap for Map<FatTile> {
    fn parse(input: &str) -> (Map<FatTile>, Vec<(isize, isize)>) {
        let width = input.lines().next().unwrap_or_default().len() * 2;
        let mut robot = 0;
        let mut instructions = Vec::new();
        let tiles = input.lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .flat_map(|(index, char)| {
                match char {
                    '#' => vec![FatTile::Wall, FatTile::Wall],
                    'O' => vec![FatTile::LeftBox, FatTile::RightBox],
                    '@' => {
                        robot = index * 2;
                        vec![FatTile::Empty, FatTile::Empty]
                    }
                    '.' => vec![FatTile::Empty, FatTile::Empty],
                    '^' => {
                        instructions.push(UP);
                        vec![]
                    }
                    '>' => {
                        instructions.push(RIGHT);
                        vec![]
                    }
                    'v' => {
                        instructions.push(DOWN);
                        vec![]
                    }
                    '<' => {
                        instructions.push(LEFT);
                        vec![]
                    }
                    _ => vec![],
                }
            })
            .collect_vec();
        let height = tiles.len() / width;
        (Map {
            height,
            width,
            tiles,
            robot,
        }, instructions)
    }

    fn move_robot(&mut self, direction: &(isize, isize)) {
        let Some(new_pos) = self.offset(&self.robot, direction) else { return };
        match self.tiles[new_pos] {
            FatTile::Empty => self.robot = new_pos,
            FatTile::LeftBox | FatTile::RightBox => if self.move_box(new_pos, direction) {
                self.robot = new_pos;
            }
            _ => {}
        }
    }

    fn move_box(&mut self, pos: usize, direction: &(isize, isize)) -> bool {
        if direction.1 != 0 {
            let mut backup = self.tiles.clone();
            if self.try_move_box_vertically(&mut backup, pos, direction) {
                self.tiles = backup;
                return true;
            }
            return false;
        }

        let current_tile = self.tiles[pos];
        let Some(new_pos) = self.offset(&pos, direction) else { return false };
        match self.tiles[new_pos] {
            FatTile::Empty => {
                self.tiles[pos] = FatTile::Empty;
                self.tiles[new_pos] = current_tile;
                true
            }
            FatTile::LeftBox | FatTile::RightBox => {
                if !self.move_box(new_pos, direction) {
                    return false;
                }
                self.tiles[pos] = FatTile::Empty;
                self.tiles[new_pos] = current_tile;
                true
            }
            FatTile::Wall => false,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Map<FatTile> {
    fn try_move_box_vertically(&self, tiles: &mut [FatTile], pos: usize, direction: &(isize, isize)) -> bool {
        let current_tile = tiles[pos];
        let other_pos = self.offset(&pos, &if current_tile == FatTile::LeftBox { RIGHT } else { LEFT }).unwrap();
        self.move_box_half_vertically(tiles, pos, direction) && self.move_box_half_vertically(tiles, other_pos, direction)
    }

    fn move_box_half_vertically(&self, tiles: &mut [FatTile], pos: usize, direction: &(isize, isize)) -> bool {
        let current_tile = tiles[pos];
        let Some((new_pos, new_tile)) = self.offset(&pos, direction)
            .map(|new_pos| (new_pos, tiles[new_pos])) else { return false };
        match new_tile {
            FatTile::Empty => {
                tiles[pos] = FatTile::Empty;
                tiles[new_pos] = current_tile;
                true
            }
            FatTile::Wall => false,
            FatTile::LeftBox | FatTile::RightBox => {
                if self.try_move_box_vertically(tiles, new_pos, direction) {
                    tiles[pos] = FatTile::Empty;
                    tiles[new_pos] = current_tile;
                    return true;
                }
                false
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut map, instructions) = Map::<Tile>::parse(input);
    map.play(&instructions);
    Some(map.tiles.iter()
        .enumerate()
        .filter(|(_, tile)| **tile == Tile::Box)
        .map(|(index, _)| map.index_to_xy(&index).unwrap())
        .map(|pos| pos.1 * 100 + pos.0)
        .sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut map, instructions) = Map::<FatTile>::parse(input);
    map.play(&instructions);
    Some(map.tiles.iter()
        .enumerate()
        .filter(|(_, tile)| **tile == FatTile::LeftBox)
        .map(|(index, _)| map.index_to_xy(&index).unwrap())
        .map(|pos| pos.1 * 100 + pos.0)
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2028));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9021));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(618));
    }
}
