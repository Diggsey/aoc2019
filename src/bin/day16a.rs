use std::iter;

const INPUT: &str = include_str!("../../inputs/day16.txt");
const PATTERN: &[i32] = &[0, 1, 0, -1];

fn next_seq(input: Vec<i32>) -> Vec<i32> {
    (1..=input.len()).map(|r| {
        PATTERN.iter()
            .flat_map(|el| iter::repeat(el).take(r))
            .cycle()
            .skip(1)
            .zip(input.iter())
            .map(|(f, v)| f*v)
            .sum::<i32>()
            .abs() % 10
    }).collect()
}

fn main() {
    let mut seq: Vec<_> = INPUT.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();

    for _ in 0..100 {
        seq = next_seq(seq);
        let res: String = seq.iter().take(8).map(|v| v.to_string()).collect();

        println!("{}", res);
    }
}
