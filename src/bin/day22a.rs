use scan_fmt::{scan_fmt};

const INPUT: &str = include_str!("../../inputs/day22.txt");
const SIZE: usize = 10007;

fn main() {
    let mut cards: Vec<_> = (0..SIZE).collect();
    for line in INPUT.lines() {
        if line == "deal into new stack" {
            cards.reverse();
        } else if let Ok(mut n) = scan_fmt!(line, "cut {}", isize) {
            if n < 0 { n += SIZE as isize; }
            cards.rotate_left(n as usize);
        } else if let Ok(n) = scan_fmt!(line, "deal with increment {}", usize) {
            let mut res = vec![0; SIZE];
            for i in 0..SIZE {
                res[i*n % SIZE] = cards[i];
            }
            cards = res;
        } else {
            panic!("Unknown instruction: `{}`", line);
        }
    }

    println!("{}", cards.into_iter().position(|card| card == 2019).unwrap());
}
