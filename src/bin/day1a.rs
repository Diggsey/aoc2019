const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let mut fuel = 0;
    for line in INPUT.lines() {
        let mass: i64 = line.parse().unwrap();
        fuel += (mass/3)-2;
    }
    println!("{}", fuel);
}
