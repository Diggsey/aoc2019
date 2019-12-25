use std::mem;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day24.txt");

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

fn advance(board: &mut Vec<Vec<bool>>, scratch: &mut Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut neighbours = 0;
            if x > 0 && board[y][x-1] { neighbours += 1; }
            if x < WIDTH-1 && board[y][x+1] { neighbours += 1; }
            if y > 0 && board[y-1][x] { neighbours += 1; }
            if y < HEIGHT-1 && board[y+1][x] { neighbours += 1; }
            scratch[y][x] = if board[y][x] {
                neighbours == 1
            } else {
                neighbours == 1 || neighbours == 2
            };
        }
    }
    mem::swap(board, scratch);
}

fn rating(board: &Vec<Vec<bool>>) -> u32 {
    let mut result = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] {
                let bit = y*WIDTH+x;
                result |= 1 << bit;
            }
        }
    }
    result
}

fn main() {
    let mut board = INPUT.lines().map(|line| {
        line.chars().map(|c| c == '#').collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut scratch = board.clone();

    let mut seen = HashSet::new();

    while seen.insert(rating(&board)) {
        advance(&mut board, &mut scratch);
    }

    println!("{}", rating(&board));
}
