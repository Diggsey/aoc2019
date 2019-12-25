use std::iter::Product;
use std::ops::Mul;
use scan_fmt::{scan_fmt};

const INPUT: &str = include_str!("../../inputs/day22.txt");
const SIZE: u128 = 119315717514047;
const REPS: u128 = 101741582076661;
const FINAL_POS: u128 = 2020;

#[derive(Debug, Copy, Clone)]
struct Shuffle {
    mul: u128,
    add: u128,
}

impl Default for Shuffle {
    fn default() -> Self {
        Shuffle { mul: 1, add: 0, }
    }
}

impl Mul for Shuffle {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            mul: (self.mul * rhs.mul) % SIZE,
            add: (self.add * rhs.mul + rhs.add) % SIZE,
        }
    }
}

impl Mul<Shuffle> for u128 {
    type Output = Self;
    fn mul(self, rhs: Shuffle) -> Self {
        (self * rhs.mul + rhs.add) % SIZE
    }
}

impl Product for Shuffle {
    fn product<I: Iterator<Item=Shuffle>>(iter: I) -> Shuffle {
        iter.fold(Shuffle::default(), Mul::mul)
    }
}

fn bin_exp(value: Shuffle, exp: u128) -> Shuffle {
    let mut res = Shuffle::default();
    for bit in (0..128).rev() {
        res = res*res;
        if (exp >> bit) & 1 == 1 {
            res = res*value;
        }
    }
    res
}

fn main() {
    let shuffle: Shuffle = bin_exp(INPUT.lines().rev().map(|line| {
        if line == "deal into new stack" {
            Shuffle { mul: SIZE-1, add: SIZE-1, }
        } else if let Ok(n) = scan_fmt!(line, "cut {}", i128) {
            Shuffle { mul: 1, add: ((n + SIZE as i128) as u128) % SIZE, }
        } else if let Ok(n) = scan_fmt!(line, "deal with increment {}", u128) {
            bin_exp(Shuffle { mul: n, add: 0, }, SIZE-2)
        } else {
            panic!("Unknown instruction: `{}`", line);
        }
    }).product(), REPS);

    println!("{}", FINAL_POS * shuffle);
}
