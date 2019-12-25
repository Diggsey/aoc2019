use std::collections::{HashSet, HashMap};

const INPUT: &str = include_str!("../../inputs/day14.txt");

#[derive(Debug, Clone)]
struct Component {
    material: String,
    amount: i64,
}

impl Component {
    fn new(s: &str) -> Self {
        let mut s = s.splitn(2, " ");
        Component {
            amount: s.next().unwrap().parse().unwrap(),
            material: s.next().unwrap().into(),
        }
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Component>,
    output: Component,
}

impl Reaction {
    fn new(line: &str) -> Self {
        let mut line = line.splitn(2, " => ");
        let lhs = line.next().unwrap();
        let rhs = line.next().unwrap();

        let inputs = lhs.split(", ").map(Component::new).collect();
        Reaction {
            inputs,
            output: Component::new(rhs),
        }
    }
}

struct Process {
    reactions: HashMap<String, Reaction>,
    amounts: HashMap<String, i64>,
    queued: HashSet<String>,
}

impl Process {
    fn step(&mut self, mat: &str) {
        self.queued.remove(mat);
        let reaction = self.reactions[mat].clone();
        if let Some(amount) = self.amounts.get_mut(mat) {
            if *amount < 0 {
                let multiple = (reaction.output.amount - *amount - 1) / reaction.output.amount;
                *amount += multiple * reaction.output.amount;
                for input in reaction.inputs {
                    self.take(&input.material, input.amount * multiple);
                }
            }
        }
    }
    fn take(&mut self, material: &str, amount: i64) {
        let amount_mut = self.amounts.entry(material.into()).or_insert(0);
        *amount_mut -= amount;
        if *amount_mut < 0 && material != "ORE" {
            self.queued.insert(material.into());
        }
    }
    fn run(&mut self) {
        while let Some(mat) = self.queued.iter().next().map(|s| s.to_owned()) {
            self.step(&mat);
        }
    }
}

fn main() {
    let mut reactions = HashMap::new();
    for line in INPUT.lines() {
        let reaction = Reaction::new(line);
        let output_material = reaction.output.material.clone();
        reactions.insert(output_material, reaction);
    }

    let mut process = Process {
        reactions,
        amounts: HashMap::new(),
        queued: HashSet::new(),
    };
    process.take("FUEL", 1376631);

    process.run();
    println!("{:?}", -process.amounts["ORE"]);
    println!("{:?}", -process.amounts["ORE"] > 1000000000000i64);
}
