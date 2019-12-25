use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day6.txt");

#[derive(Debug)]
struct Node {
    parent: String,
}

fn main() {
    let mut orbits: HashMap<String, Node> = HashMap::new();

    for line in INPUT.lines() {
        let x = &line[0..3];
        let y = &line[4..7];
        orbits.insert(y.into(), Node { parent: x.into() });
    }

    let mut count = 0;
    for mut k in orbits.keys() {
        while let Some(node) = orbits.get(k) {
            k = &node.parent;
            count += 1;
        }
    }
    println!("{}", count);
}
