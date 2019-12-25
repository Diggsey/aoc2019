use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut pos = (0, 0);
    let mut result = HashSet::new();
    for part in input.split(",") {
        let (dir, dist_str) = part.split_at(1);
        let dist: usize = dist_str.parse().unwrap();
        for _ in 0..dist {
            match dir {
                "U" => pos.1 -= 1,
                "D" => pos.1 += 1,
                "L" => pos.0 -= 1,
                "R" => pos.0 += 1,
                _ => panic!("{}", dir),
            }
            result.insert(pos);
        }
    }
    result
}

fn main() {
    let lines: Vec<_> = INPUT.lines().collect();
    let a = parse(&lines[0]);
    let b = parse(&lines[1]);

    let res = a.intersection(&b)
        .map(|p| p.0.abs() + p.1.abs())
        .min();
    println!("{:?}", res);
}
