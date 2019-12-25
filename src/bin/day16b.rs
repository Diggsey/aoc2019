const INPUT: &str = include_str!("../../inputs/day16.txt");

fn next_seq(input: &mut [i32]) {
    let mut sum = 0;
    for x in input {
        let y = *x;
        *x = (y + sum) % 10;
        sum += y;
    }
}

fn main() {
    let offset: usize = INPUT[0..7].parse().unwrap();
    let mut seq: Vec<_> = INPUT.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    
    let actual_len = seq.len()*10000;
    seq = seq.into_iter().cycle().skip(offset).take(actual_len - offset).collect();
    seq.reverse();

    for _ in 0..100 {
        next_seq(&mut seq);
        let start: String = seq.iter().rev().take(8).map(|v| v.to_string()).collect();
        let end: String = seq.iter().take(8).rev().map(|v| v.to_string()).collect();

        println!("{}...{}", start, end);
    }
}
