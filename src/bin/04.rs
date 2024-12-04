use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {

    fn offset(&self, offset: &(isize, isize)) -> Pos {
        Pos { x: self.x + offset.0, y: self.y + offset.1 }
    }

    fn is_within(&self, dimensions: &(isize, isize)) -> bool {
        (0..dimensions.0).contains(&self.x) && (0..dimensions.1).contains(&self.y)
    }

}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos { x: value.0 as isize, y: value.1 as isize }
    }
}

const SEQUENCE: &[char; 4] = &['X', 'M', 'A', 'S'];
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_char(matrix: &[Vec<char>], pos: &Pos) -> char {
    matrix[pos.y as usize][pos.x as usize]
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(search_xmas(input))
}

fn search_xmas(input: &str) -> u32 {
    let matrix = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let dimensions = (matrix[0].len() as isize, matrix.len() as isize);
    (0..dimensions.0)
        .cartesian_product(0..dimensions.1)
        .map(|(x, y)| Pos { x, y })
        .map(|pos| count_xmas_seq(&matrix, &dimensions, &pos, &DIRECTIONS, 0))
        .sum()
}

fn count_xmas_seq(matrix: &[Vec<char>], dimensions: &(isize, isize), pos: &Pos, directions: &[(isize, isize)], seq_index: usize) -> u32 {
    let seq_char = SEQUENCE[seq_index];
    if get_char(matrix, pos) != seq_char {
        return 0;
    }
    if seq_index + 1 >= SEQUENCE.len() {
        return 1;
    }
    let mut matches = 0;
    for direction in directions {
        let offset_pos = pos.offset(direction);
        if !offset_pos.is_within(dimensions) {
            continue;
        }
        matches += count_xmas_seq(matrix, dimensions, &offset_pos, &[*direction], seq_index + 1)
    }
    matches
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(search_cross_mas(input))
}

fn search_cross_mas(input: &str) -> u32 {
    let matrix = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let dimensions = (matrix[0].len() as isize, matrix.len() as isize);
    (0..dimensions.0)
        .cartesian_product(0..dimensions.1)
        .map(|(x, y)| Pos { x, y })
        .filter(|pos| is_cross_mas(&matrix, &dimensions, pos))
        .count() as u32
}

fn is_cross_mas(matrix: &[Vec<char>], dimensions: &(isize, isize), pos: &Pos) -> bool {
    if !(1..(dimensions.0 - 1)).contains(&pos.x) || !(1..(dimensions.1 - 1)).contains(&pos.y) {
        return false
    }
    let center = get_char(matrix, pos);
    if center != 'A' {
        return false;
    }

    let top_left = get_char(matrix, &pos.offset(&(-1, -1)));
    let top_right = get_char(matrix, &pos.offset(&(-1, 1)));
    let bottom_left = get_char(matrix, &pos.offset(&(1, -1)));
    let bottom_right = get_char(matrix, &pos.offset(&(1, 1)));

    let diagonal1 = [top_left, center, bottom_right];
    let diagonal2 = [bottom_left, center, top_right];

    (diagonal1 == ['M', 'A', 'S'] || diagonal1 == ['S', 'A', 'M'])
        && (diagonal2 == ['M', 'A', 'S'] || diagonal2 == ['S', 'A', 'M'])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
