const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let map: Vec<(usize, usize)> = INPUT.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().flat_map(move |(x, c)| {
            if c == '#' {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect();

    let best = map.iter().map(|&(rel_x, rel_y)| {
        let mut visited: Vec<(i64, i64)> = Vec::new();
        'next: for &(x, y) in map.iter() {
            let off_x = x as i64 - rel_x as i64;
            let off_y = y as i64 - rel_y as i64;

            for &(ox, oy) in visited.iter() {
                if ox*off_y == off_x*oy && off_x.signum() == ox.signum() && off_y.signum() == oy.signum() {
                    continue 'next;
                }
            }
            visited.push((off_x, off_y));
        }

        println!("{},{} - {}", rel_x, rel_y, visited.len() - 1);

        visited.len() - 1
    }).max();
    
    println!("{:?}", best);
}
