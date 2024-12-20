use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node(usize, usize, NodeType);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum NodeType {
    Empty,
    Wall,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    node: Node,
    cost: u32,
    direction: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> (Node, Node, Vec<Vec<Node>>) {
    let (mut source_node, mut target_node) = (Node(0, 0, NodeType::Empty), Node(0, 0, NodeType::Empty));
    let nodes = input.lines()
        .enumerate()
        .map(|(y, line)| {
            let mut local_source_node = None;
            let mut local_target_node = None;
            let nodes = line.char_indices().map(|(x, char)| {
                match char {
                    '#' => Node(x, y, NodeType::Wall),
                    _ => {
                        let node = Node(x, y, NodeType::Empty);
                        if char == 'S' {
                            local_source_node = Some(node);
                        } else if char == 'E' {
                            local_target_node = Some(node);
                        }
                        node
                    },
                }
            }).collect_vec();
            if let Some(node) = local_source_node {
                source_node = node;
            }
            if let Some(node) = local_target_node {
                target_node = node;
            }
            nodes
        })
        .collect_vec();
    (source_node, target_node, nodes)
}

fn find_shortest_path(source: Node, target: Node, nodes: Vec<Vec<Node>>) -> Option<u32> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(source, 0);
    heap.push(State { node: source, cost: 0, direction: (1, 0) });

    while let Some(State { node, cost, direction }) = heap.pop() {
        if node == target {
            return Some(cost);
        }

        if cost > *dist.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        for new_direction in &DIRECTIONS {
            let Some(neighbor) = offset(&(node.0, node.1), new_direction)
                .map(|(x, y)| nodes[y][x])
                .filter(|node| node.2 != NodeType::Wall) else { continue };

            let new_cost = cost + 1 + (1000 * u32::from(*new_direction != direction));
            let next = State { node: neighbor, cost: new_cost, direction: *new_direction };

            if next.cost < *dist.get(&next.node).unwrap_or(&u32::MAX) {
                heap.push(next);
                dist.insert(next.node, next.cost);
            }
        }
    }

    None
}

fn find_all_shortest_paths(source: Node, target: Node, nodes: Vec<Vec<Node>>) -> Option<u32> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut paths: HashMap<_, Vec<_>> = HashMap::new();

    dist.insert((source, (1, 0)), 0);
    dist.insert((source, (0, -1)), 1000);
    heap.push(State { node: source, cost: 0, direction: (1, 0) });
    heap.push(State { node: source, cost: 1000, direction: (0, -1) });

    while let Some(State { node, cost, direction }) = heap.pop() {
        if node == target {
            break;
        }

        if cost > *dist.get(&(node, direction)).unwrap_or(&u32::MAX) {
            continue;
        }

        let Some(neighbor) = offset(&(node.0, node.1), &direction)
            .map(|(x, y)| nodes[y][x])
            .filter(|node| node.2 != NodeType::Wall) else { continue };

        for new_direction in &DIRECTIONS {
            if *new_direction == (-direction.0, -direction.1) {
                continue;
            }
            let new_cost = cost + 1 + (1000 * u32::from(*new_direction != direction));
            let next = State { node: neighbor, cost: new_cost, direction: *new_direction };

            match next.cost.cmp(dist.get(&(next.node, *new_direction)).unwrap_or(&u32::MAX)) {
                Ordering::Less => {
                    heap.push(next);
                    dist.insert((next.node, next.direction), next.cost);
                    paths.insert((next.node, next.direction), vec![(node, direction)]);
                }
                Ordering::Equal => paths.entry((next.node, next.direction)).or_default().push((node, direction)),
                _ => {}
            }
        }
    }

    let mut best_path_nodes = HashSet::new();
    if !dist.contains_key(&(target, (1, 0))) && !dist.contains_key(&(target, (0, -1))) {
        return None;
    }

    let mut stack = vec![(target, (1, 0))];
    if dist.contains_key(&(target, (0, -1))) {
        stack.push((target, (0, -1)));
    }

    while let Some((node, direction)) = stack.pop() {
        best_path_nodes.insert(node);
        let Some(prev_nodes) = paths.get(&(node, direction)) else { continue };
        for &(prev_node, prev_direction) in prev_nodes {
            stack.push((prev_node, prev_direction));
        }
    }

    Some(best_path_nodes.len() as u32)
}

fn offset(pos: &(usize, usize), direction: &(isize, isize)) -> Option<(usize, usize)> {
    let x = pos.0.checked_add_signed(direction.0)?;
    let y = pos.1.checked_add_signed(direction.1)?;
    Some((x, y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (source_node, target_node, nodes) = parse(input);
    find_shortest_path(source_node, target_node, nodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (source_node, target_node, nodes) = parse(input);
    find_all_shortest_paths(source_node, target_node, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
