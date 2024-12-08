use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut antennas: HashMap<_, Vec<_>> = HashMap::new();
        let height = input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char.is_ascii_alphanumeric() {
                    antennas.entry(char).or_default().push((x, y));
                } 
            });
            y
        }).max().unwrap_or_default() + 1;
        let width = input.lines().next().unwrap_or_default().len();
        Grid {
            width,
            height,
            antennas,
        }
    }

    fn find_antinodes(&self) -> Vec<bool> {
        let mut antinodes = vec![false; self.width * self.height];
        self.antennas.iter().for_each(|(_, positions)| positions.iter().enumerate().for_each(|(index, pos1)| positions.iter().skip(index + 1).for_each(|pos2| {
            let dx= (pos2.0 as isize - pos1.0 as isize) * 2;
            let dy= (pos2.1 as isize - pos1.1 as isize) * 2;

            if let (Some(x), Some(y)) = (pos1.0.checked_add_signed(dx), pos1.1.checked_add_signed(dy)) {
                if let Some(index) = self.xy_to_index((x, y)) { antinodes[index] = true; }
            };

            if let (Some(x), Some(y)) = (pos2.0.checked_add_signed(-dx), pos2.1.checked_add_signed(-dy)) {
                if let Some(index) = self.xy_to_index((x, y)) { antinodes[index] = true; }
            }
        })));
        antinodes
    }

    fn find_antinodes_with_resonance(&self) -> Vec<bool> {
        let mut antinodes = vec![false; self.width * self.height];
        self.antennas.iter().for_each(|(_, positions)| positions.iter().enumerate().for_each(|(index, pos1)| positions.iter().skip(index + 1).for_each(|pos2| {
            let dx = pos2.0 as isize - pos1.0 as isize;
            let dy = pos2.1 as isize - pos1.1 as isize;

            let mut count = 0;
            while let (Some(x), Some(y)) = (pos1.0.checked_add_signed(-dx * count), pos1.1.checked_add_signed(-dy * count)) {
                if let Some(index) = self.xy_to_index((x, y)) { antinodes[index] = true; }
                else { break };
                count += 1;
            };

            count = 0;
            while let (Some(x), Some(y)) = (pos2.0.checked_add_signed(dx * count), pos2.1.checked_add_signed(dy * count)) {
                if let Some(index) = self.xy_to_index((x, y)) { antinodes[index] = true; }
                else { break };
                count += 1;
            };
        })));
        antinodes
    }

    fn xy_to_index(&self, pos: (usize, usize)) -> Option<usize> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(pos.1 * self.width + pos.0)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    Some(grid.find_antinodes().iter().filter(|x| **x).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    Some(grid.find_antinodes_with_resonance().iter().filter(|x| **x).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
