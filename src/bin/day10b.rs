use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let mut map: Vec<(usize, usize)> = INPUT.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().flat_map(move |(x, c)| {
            if c == '#' {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect();

    let pos_x = 11;
    let pos_y = 19;

    let mut remaining = 200;

    map.retain(|&(x, y)| x != (pos_x as usize) || y != (pos_y as usize));

    loop {
        let mut visited: Vec<(i64, i64)> = Vec::new();
        'next: for &(x, y) in map.iter() {
            let off_x = x as i64 - pos_x;
            let off_y = y as i64 - pos_y;

            for i in 0..visited.len() {
                let (ox, oy) = visited[i];
                if ox*off_y == off_x*oy && off_x.signum() == ox.signum() && off_y.signum() == oy.signum() {
                    if off_y*off_y + off_x*off_x < oy*oy + ox*ox {
                        visited[i] = (off_x, off_y);
                    }
                    continue 'next;
                }
            }
            visited.push((off_x, off_y));
        }

        if visited.len() >= remaining {
            visited.sort_by_cached_key(|&(ox, oy)| (-(ox as f64).atan2(oy as f64)*1e6) as i64);
            let best = visited[remaining-1];
            println!("{}, {}", best.0 + pos_x, best.1 + pos_y);
            break;
        } else {
            remaining -= visited.len();
            let to_delete: HashSet<_> = visited.into_iter().collect();
            map.retain(|&(x, y)| !to_delete.contains(&(x as i64 - pos_x, y as i64 - pos_y)));
        }
    }
}
