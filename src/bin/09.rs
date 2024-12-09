use std::ops::Range;
use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug)]
struct DiskMap<T> {
    data: Vec<T>
}

impl DiskMap<Option<usize>> {
    fn parse(input: &str) -> DiskMap<Option<usize>> {
        let data = (&input.chars().chunks(2)).into_iter()
            .enumerate()
            .flat_map(|(id_number, mut chunk)| {
                let Some(Some(file_size)) = chunk.next().map(|char| char.to_digit(10)) else { return Vec::new() };
                let mut file = vec![Some(id_number); file_size as usize];
                if let Some(Some(free_space)) = chunk.next().map(|char| char.to_digit(10)) {
                    file.append(&mut vec![None; free_space as usize]);
                    file
                } else {
                    file
                }
            })
            .collect();
        DiskMap {
            data
        }
    }

    fn checksum(&self) -> u64 {
        let len = self.data.len();
        let mut sum = 0;
        let (mut head, mut tail) = (1, len);
        while head < tail {
            let mut value = None;
            if let Some(val) = self.data[head] {
                value = Some(val);
            } else {
                while head < tail {
                    tail -= 1;
                    if let Some(val) = self.data[tail] {
                        value = Some(val);
                        break;
                    };
                }
            }
            let Some(value) = value else { break };
            sum += head * value;
            head += 1;
        }
        sum as u64
    }
}

#[derive(Default, Clone, Debug)]
struct Interval {
    id: u64,
    interval: Range<u32>,
}

impl DiskMap<Interval> {
    fn parse_intervals(input: &str) -> DiskMap<Interval> {
        let mut last_index = 0;
        let data = (&input.chars().chunks(2)).into_iter()
            .enumerate()
            .map(|(id_number, mut chunk)| {
                let Some(Some(file_size)) = chunk.next().map(|char| char.to_digit(10)) else { return Interval::default() };
                let file = Interval {
                    id: id_number as u64,
                    interval: last_index..(last_index + file_size)
                };
                last_index = file.interval.end;
                if let Some(Some(free_space)) = chunk.next().map(|char| char.to_digit(10)) {
                    last_index += free_space;
                }
                file
            })
            .collect_vec();
        DiskMap {
            data
        }
    }

    fn checksum(&self) -> u64 {
        let mut data = self.data.clone();
        for index in (1..data.len()).rev() {
            let interval = &self.data[index];
            let actual_index = data.iter().rposition(|x| x.id == interval.id).unwrap();
            let interval_size = interval.interval.end - interval.interval.start;
            let Some((new_index, start_index, _)) = (0..actual_index)
                .map(|index| (index, &data[index].interval, &data[index + 1].interval))
                .map(|(index, x, y)| (index, x.end, y.start - x.end))
                .find(|(_, _, size)| size >= &(interval_size)) else { continue };
            let interval = Interval {
                interval: start_index..(start_index + interval_size),
                ..*interval
            };
            data.remove(actual_index);
            data.insert(new_index + 1, interval);
        }
        data.iter()
            .map(|Interval { id, interval }| interval.clone().map(|x| id * (x as u64)).sum::<u64>())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(DiskMap::parse(input).checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = DiskMap::parse_intervals(input);
    Some(disk_map.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
