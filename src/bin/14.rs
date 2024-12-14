use itertools::Itertools;

advent_of_code::solution!(14);

#[cfg(not(test))]
const WIDTH: u8 = 101;
#[cfg(not(test))]
const HEIGHT: u8 = 103;

#[cfg(test)]
const WIDTH: u8 = 11;
#[cfg(test)]
const HEIGHT: u8 = 7;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn parse(input: &str) -> Robot {
        let pos_x = &input[input.find('=').unwrap() + 1..input.find(',').unwrap()];
        let pos_y = &input[input.find(',').unwrap() + 1..input.find(' ').unwrap()];

        let vel_x = &input[input.rfind('=').unwrap() + 1..input.rfind(',').unwrap()];
        let vel_y = &input[input.rfind(',').unwrap() + 1..];

        Robot {
            position: (pos_x.parse().unwrap(), pos_y.parse().unwrap()),
            velocity: (vel_x.parse().unwrap(), vel_y.parse().unwrap()),
        }
    }

    fn simulate(&mut self, seconds: isize) {
        let (offset_x, offset_y) = (self.velocity.0 * seconds, self.velocity.1 * seconds);
        let new_position = (self.position.0 + offset_x, self.position.1 + offset_y);
        self.position = (new_position.0.rem_euclid(WIDTH as isize), new_position.1.rem_euclid(HEIGHT as isize));
    }

    fn get_quadrant(&self) -> Option<u8> {
        let (center_column, center_row) = (WIDTH as isize / 2, HEIGHT as isize / 2);
        if self.position.0 < center_column && self.position.1 < center_row {
            Some(0)
        } else if self.position.0 > center_column && self.position.1 < center_row {
            Some(1)
        } else if self.position.0 < center_column && self.position.1 > center_row {
            Some(2)
        } else if self.position.0 > center_column && self.position.1 > center_row {
            Some(3)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut quadrants = [0; 4];
    input.lines()
        .map(Robot::parse)
        .map(|mut robot| {
            robot.simulate(100);
            robot
        })
        .filter_map(|robot| robot.get_quadrant())
        .for_each(|quadrant| quadrants[quadrant as usize] += 1);
    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut lowest_safety_factor, mut seconds_passed) = (u32::MAX, 0);
    let mut robots = input.lines().map(Robot::parse).collect_vec();
    for second in 1..=WIDTH as u16 * HEIGHT as u16 {
        let mut quadrants = [0; 4];
        robots.iter_mut()
            .map(|robot| {
                robot.simulate(1);
                robot
            })
            .filter_map(|robot| robot.get_quadrant())
            .for_each(|quadrant| quadrants[quadrant as usize] += 1);
        let safety_factor = quadrants.iter().product();
        if safety_factor < lowest_safety_factor {
            lowest_safety_factor = safety_factor;
            seconds_passed = second;
        }
    }
    Some(seconds_passed as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        // Cannot test part two
    }
}
