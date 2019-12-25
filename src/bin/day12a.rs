use scan_fmt::{scan_fmt};

const INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Clone, Copy)]
struct Moon {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

fn update_vel(moons: &mut [Moon]) {
    for i in 0..(moons.len()-1) {
        for j in (i+1)..moons.len() {
            let pos_a = moons[i].pos;
            let pos_b = moons[j].pos;
            let dx = (pos_b.0 - pos_a.0).signum();
            let dy = (pos_b.1 - pos_a.1).signum();
            let dz = (pos_b.2 - pos_a.2).signum();
            moons[i].vel.0 += dx;
            moons[i].vel.1 += dy;
            moons[i].vel.2 += dz;
            moons[j].vel.0 -= dx;
            moons[j].vel.1 -= dy;
            moons[j].vel.2 -= dz;
        }
    }
}

fn update_pos(moons: &mut [Moon]) {
    for moon in moons {
        moon.pos.0 += moon.vel.0;
        moon.pos.1 += moon.vel.1;
        moon.pos.2 += moon.vel.2;
    }
}

fn calc_energy(moons: &[Moon]) -> i64 {
    moons.iter().map(|moon| {
        (moon.pos.0.abs() + moon.pos.1.abs() + moon.pos.2.abs()) *
        (moon.vel.0.abs() + moon.vel.1.abs() + moon.vel.2.abs())
    }).sum()
}

fn main() {
    let mut moons: Vec<_> = INPUT.lines().map(|line| {
        let (x, y, z) = scan_fmt!(line, "<x={}, y={}, z={}>", i64, i64, i64).unwrap();
        Moon {
            pos: (x, y, z),
            vel: (0, 0, 0),
        }
    }).collect();

    for _ in 0..1000 {
        update_vel(&mut moons[..]);
        update_pos(&mut moons[..]);
        // println!("{:?}\n", moons);
    }

    let energy = calc_energy(&moons[..]);
    println!("{}", energy);
}
