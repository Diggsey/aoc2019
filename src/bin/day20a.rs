use std::collections::{HashMap, BinaryHeap};

const INPUT: &str = include_str!("../../inputs/day20.txt");

#[derive(Debug, Copy, Clone)]
enum ParseCell {
    Empty,
    Wall,
    Letter(char),
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Wall,
    Portal(usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn apply(self, other: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (other.0, other.1 - 1),
            Direction::South => (other.0, other.1 + 1),
            Direction::West => (other.0 - 1, other.1),
            Direction::East => (other.0 + 1, other.1),
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
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), match c {
                '#' => ParseCell::Wall,
                '.' => ParseCell::Empty,
                ' ' => continue,
                _ => ParseCell::Letter(c),
            });
        }
    }
    
    let mut in_portals = HashMap::new();
    let mut out_portals = HashMap::new();
    for (&(x, y), &v) in map.iter() {
        if let ParseCell::Letter(c) = v {
            let (d, in_pos, out_pos) = if let Some(&ParseCell::Letter(d)) = map.get(&(x+1, y)) {
                if let Some(ParseCell::Empty) = map.get(&(x+2, y)) {
                    (d, (x+1, y), (x+2, y))
                } else {
                    (d, (x, y), (x-1, y))
                }
            } else if let Some(&ParseCell::Letter(d)) = map.get(&(x, y+1)) {
                if let Some(ParseCell::Empty) = map.get(&(x, y+2)) {
                    (d, (x, y+1), (x, y+2))
                } else {
                    (d, (x, y), (x, y-1))
                }
            } else {
                continue
            };
            let out_portal: &mut Vec<_> = out_portals.entry([c, d]).or_default();
            let is_first = out_portal.is_empty();
            out_portal.push(out_pos);
            in_portals.insert(in_pos, (is_first, [c, d]));
        }
    }

    let start_pos = out_portals.remove(&['A', 'A']).unwrap()[0];
    let end_pos = out_portals.remove(&['Z', 'Z']).unwrap()[0];

    let map: HashMap<_, _> = map.into_iter().filter_map(|(pos, cell)| {
        if let Some((is_first, name)) = in_portals.get(&pos) {
            Some((pos, if let Some(pos_list) = out_portals.get(name) {
                let out_pos = if *is_first {
                    pos_list[1]
                } else {
                    pos_list[0]
                };
                Cell::Portal(out_pos.0, out_pos.1)
            } else {
                Cell::Wall
            }))
        } else {
            match cell {
                ParseCell::Empty => Some((pos, Cell::Empty)),
                ParseCell::Wall => Some((pos, Cell::Wall)),
                ParseCell::Letter(_) => None
            }
        }
    }).collect();

    
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, start_pos));

    while let Some((neg_dist, pos)) = queue.pop() {
        if let Some(prev_neg_dist) = visited.insert(pos, -neg_dist) {
            if prev_neg_dist >= neg_dist {
                continue;
            }
        }

        if pos == end_pos {
            println!("{}", -neg_dist);
            break;
        }

        for dir in 0..4 {
            let dir: Direction = dir.into();
            let mut new_pos = dir.apply(pos);
            new_pos = match map[&new_pos] {
                Cell::Empty => new_pos,
                Cell::Wall => continue,
                Cell::Portal(x, y) => (x, y),
            };
            if !visited.contains_key(&new_pos) {
                queue.push((neg_dist-1, new_pos));
            }
        }
    }
}
