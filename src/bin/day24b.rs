use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day24.txt");

const SIZE: isize = 5;
const MID: isize = SIZE/2;

fn bit_for_pos(x: isize, y: isize) -> u32 {
    1 << (y*SIZE + x)
}

fn pattern_for_predicate<F: Fn(isize, isize) -> bool>(f: F) -> u32 {
    let mut result = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if f(x, y) {
                result |= bit_for_pos(x, y);
            }
        }
    }
    result
}

fn compute_patterns() -> Patterns {
    let mut neighbours = Vec::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            let mut hi = 0;
            let lo = if x == MID && y == MID {
                0
            } else {
                pattern_for_predicate(|nx, ny| ((nx-x).abs() + (ny-y).abs() == 1))
            };
            if x == 0 {
                hi |= bit_for_pos(MID-1, MID);
            } else if x == SIZE-1 {
                hi |= bit_for_pos(MID+1, MID);
            } else if x == MID {
                if y == MID-1 {
                    hi |= pattern_for_predicate(|_, ny| ny == 0);
                } else if y == MID+1 {
                    hi |= pattern_for_predicate(|_, ny| ny == SIZE-1);
                }
            }
            if y == 0 {
                hi |= bit_for_pos(MID, MID-1);
            } else if y == SIZE-1 {
                hi |= bit_for_pos(MID, MID+1);
            } else if y == MID {
                if x == MID-1 {
                    hi |= pattern_for_predicate(|nx, _| nx == 0);
                } else if x == MID+1 {
                    hi |= pattern_for_predicate(|nx, _| nx == SIZE-1);
                }
            }
            neighbours.push(lo as u64 | ((hi as u64) << 32));
        }
    }
    Patterns {
        neighbours,
        inner: pattern_for_predicate(|nx, ny| nx == 0 || ny == 0 || nx == SIZE-1 || ny == SIZE-1),
        outer: bit_for_pos(MID-1, MID) | bit_for_pos(MID+1, MID) | bit_for_pos(MID, MID-1) | bit_for_pos(MID, MID+1),
    }
}

#[derive(Debug)]
struct Patterns {
    neighbours: Vec<u64>,
    inner: u32,
    outer: u32,
}

struct State {
    boards: VecDeque<u32>,
    patterns: Patterns,
}

impl State {
    fn advance(&mut self) {
        if *self.boards.front().unwrap() != 0 {
            self.boards.push_front(0);
        }
        if *self.boards.back().unwrap() != 0 {
            self.boards.push_back(0);
        }

        let mut prev_board = 0u32;
        for i in 0..self.boards.len() {
            let next_board = if i+1 < self.boards.len() { self.boards[i+1] } else { 0 };
            let cur_board = self.boards[i];
            let mut new_board = 0u32;

            let hi = (prev_board & self.patterns.inner) | (next_board & self.patterns.outer);
            let lookup_state = cur_board as u64 | ((hi as u64) << 32);

            for i in 0..((SIZE*SIZE) as usize) {
                let neighbours = (self.patterns.neighbours[i] & lookup_state).count_ones();
                let present = (cur_board >> i) & 1 == 1;
                if neighbours == 1 || (!present && neighbours == 2) {
                    new_board |= 1 << i;
                }
            }
            self.boards[i] = new_board;

            prev_board = cur_board;
        }
    }
    fn count(&self) -> u64 {
        self.boards.iter().map(|b| b.count_ones() as u64).sum()
    }
}

fn main() {
    let board: u32 = INPUT.lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .map(|(i, c)| if c == '#' { 1u32 << i } else { 0 })
        .sum();

    let mut state = State {
        boards: Some(board).into_iter().collect(),
        patterns: compute_patterns(),
    };

    for _ in 0..200 {
        state.advance();
    }

    println!("{}", state.count());
}
