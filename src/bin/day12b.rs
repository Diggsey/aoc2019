use scan_fmt::{scan_fmt};

const INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MoonDim {
    pos: i64,
    vel: i64,
}

fn update_vel(moons: &mut [MoonDim]) {
    for i in 0..(moons.len()-1) {
        for j in (i+1)..moons.len() {
            let pos_a = moons[i].pos;
            let pos_b = moons[j].pos;
            let d = (pos_b - pos_a).signum();
            moons[i].vel += d;
            moons[j].vel -= d;
        }
    }
}

fn update_pos(moons: &mut [MoonDim]) {
    for moon in moons {
        moon.pos += moon.vel;
    }
}

fn find_repetition(dim: &mut Vec<MoonDim>) -> u64 {
    let initial = dim.clone();
    for i in 1.. {
        update_vel(&mut dim[..]);
        update_pos(&mut dim[..]);
        if *dim == initial {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let (mut dim_x, dim_yz): (Vec<_>, Vec<_>) = INPUT.lines().map(|line| {
        let (x, y, z) = scan_fmt!(line, "<x={}, y={}, z={}>", i64, i64, i64).unwrap();
        (MoonDim { pos: x, vel: 0, }, (MoonDim { pos: y, vel: 0, }, MoonDim { pos: z, vel: 0, }))
    }).unzip();
    let (mut dim_y, mut dim_z) = dim_yz.into_iter().unzip();

    let (x, y, z) = (
        find_repetition(&mut dim_x),
        find_repetition(&mut dim_y),
        find_repetition(&mut dim_z),
    );
    println!("{} {} {}", x, y, z);
}
