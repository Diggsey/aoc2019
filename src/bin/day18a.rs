use std::collections::{HashMap, BinaryHeap};

const INPUT: &str = include_str!("../../inputs/day18.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Key(usize),
    Door(usize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn apply(self, other: (usize, usize, usize)) -> (usize, usize, usize) {
        match self {
            Direction::North => (other.0, other.1 - 1, other.2),
            Direction::South => (other.0, other.1 + 1, other.2),
            Direction::West => (other.0 - 1, other.1, other.2),
            Direction::East => (other.0 + 1, other.1, other.2),
        }
    }
}

impl From<i32> for Direction {
    fn from(other: i32) -> Self {
        match other {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!("Unknown direction: {}", other),
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    let mut initial_pos = (0, 0);
    let mut all_keys = 0usize;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), match c {
                '.' | '@' => Cell::Empty,
                '#' => Cell::Wall,
                'a'..='z' => {
                    let key = (c as u8 - b'a') as usize;
                    all_keys |= 1 << key;
                    Cell::Key(key)
                },
                'A'..='Z' => Cell::Door((c as u8 - b'A') as usize),
                _ => panic!("Unexpected character: {}", c),
            });

            if c == '@' {
                initial_pos = (x, y);
            }
        }
    }

    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, (initial_pos.0, initial_pos.1, 0usize)));

    let mut best_keys = 0;
    let total_keys = all_keys.count_ones();

    while let Some((neg_dist, pos)) = queue.pop() {
        if let Some(prev_neg_dist) = visited.insert(pos, -neg_dist) {
            if prev_neg_dist >= neg_dist {
                continue;
            }
        }

        let num_keys = pos.2.count_ones();
        if num_keys > best_keys {
            best_keys = num_keys;
            println!("Keys: {}/{}", best_keys, total_keys);
        }

        if pos.2 == all_keys {
            println!("{}", -neg_dist);
            break;
        }

        for dir in 0..4 {
            let dir: Direction = dir.into();
            let mut new_pos = dir.apply(pos);
            new_pos.2 = match map[&(new_pos.0, new_pos.1)] {
                Cell::Empty => new_pos.2,
                Cell::Wall => continue,
                Cell::Key(k) => new_pos.2 | (1 << k),
                Cell::Door(k) => if (new_pos.2 & (1 << k)) != 0 {
                    new_pos.2
                } else {
                    continue
                }
            };
            if !visited.contains_key(&new_pos) {
                queue.push((neg_dist-1, new_pos));
            }
        }
    }
}
