const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let mut total_fuel = 0;
    for line in INPUT.lines() {
        let mass: i64 = line.parse().unwrap();
        let mut fuel = (mass/3)-2;
        while fuel > 0 {
            total_fuel += fuel;
            fuel = (fuel/3)-2;
        }
    }
    println!("{}", total_fuel);
}
