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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct State {
    nodes: [usize; 4],
    keys: usize,
}

struct Edge {
    node_id: usize,
    dist: i64,
}

struct Node {
    cell: Cell,
    edges: Vec<Edge>,
}

struct Graph {
    map: HashMap<(usize, usize), Cell>,
    nodes: Vec<Node>,
    node_map: HashMap<(usize, usize), usize>,
}

impl Graph {
    fn add_node(&mut self, node_pos: (usize, usize)) -> usize {
        if let Some(&node_id) = self.node_map.get(&node_pos) {
            return node_id;
        }

        let cell = self.map[&node_pos];
        let node_id = self.nodes.len();
        self.nodes.push(Node {
            cell,
            edges: Vec::new(),
        });
        self.node_map.insert(node_pos, node_id);

        let mut visited = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push((0, node_pos));

        while let Some((neg_dist, pos)) = queue.pop() {
            if let Some(prev_dist) = visited.insert(pos, -neg_dist) {
                if prev_dist <= -neg_dist {
                    continue;
                }
            }

            if node_pos != pos {
                if let Cell::Door(_) | Cell::Key(_) = self.map[&pos] {
                    let other_node_id = self.add_node(pos);
                    self.nodes[node_id].edges.push(Edge {
                        node_id: other_node_id,
                        dist: -neg_dist
                    });
                    continue;
                }
            }

            for dir in 0..4 {
                let dir: Direction = dir.into();
                let new_pos = dir.apply(pos);

                let cell = self.map[&new_pos];

                if cell == Cell::Wall {
                    continue;
                }

                if !visited.contains_key(&new_pos) {
                    queue.push((neg_dist-1, new_pos));
                }
            }
        }
        node_id
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

    map.insert((initial_pos.0, initial_pos.1), Cell::Wall);
    map.insert((initial_pos.0-1, initial_pos.1), Cell::Wall);
    map.insert((initial_pos.0, initial_pos.1-1), Cell::Wall);
    map.insert((initial_pos.0+1, initial_pos.1), Cell::Wall);
    map.insert((initial_pos.0, initial_pos.1+1), Cell::Wall);

    let total_keys = all_keys.count_ones();

    let mut graph = Graph {
        map,
        nodes: Vec::new(),
        node_map: HashMap::new(),
    };
    let robot_a = graph.add_node((initial_pos.0-1, initial_pos.1-1));
    let robot_b = graph.add_node((initial_pos.0-1, initial_pos.1+1));
    let robot_c = graph.add_node((initial_pos.0+1, initial_pos.1-1));
    let robot_d = graph.add_node((initial_pos.0+1, initial_pos.1+1));

    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, State {
        nodes: [robot_a, robot_b, robot_c, robot_d],
        keys: 0,
    }));

    let mut best_keys = 0;

    while let Some((neg_dist, state)) = queue.pop() {
        if let Some(prev_dist) = visited.insert(state, -neg_dist) {
            if prev_dist <= -neg_dist {
                continue;
            }
        }

        let num_keys = state.keys.count_ones();
        if num_keys > best_keys {
            best_keys = num_keys;
            println!("Keys: {}/{}", best_keys, total_keys);
        }

        if state.keys == all_keys {
            println!("{}", -neg_dist);
            break;
        }

        for robot in 0..4 {
            let node = &graph.nodes[state.nodes[robot]];
            for edge in node.edges.iter() {
                let mut new_state = state;
                let new_node = &graph.nodes[edge.node_id];
                new_state.nodes[robot] = edge.node_id;
                new_state.keys = match new_node.cell {
                    Cell::Empty => state.keys,
                    Cell::Wall => unreachable!(),
                    Cell::Key(k) => state.keys | (1 << k),
                    Cell::Door(k) => if (state.keys & (1 << k)) != 0 {
                        state.keys
                    } else {
                        continue
                    }
                };
                if !visited.contains_key(&new_state) {
                    queue.push((neg_dist - edge.dist, new_state));
                }
            }
        }
    }
}
