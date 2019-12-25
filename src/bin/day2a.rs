const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let mut memory: Vec<usize> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    memory[1] = 12;
    memory[2] = 2;
    
    let mut pc = 0;

    while memory[pc] != 99 {
        let opcode = memory[pc];
        let arg0 = memory[pc+1];
        let arg1 = memory[pc+2];
        let res = memory[pc+3];
        match opcode {
            1 => memory[res] = memory[arg0] + memory[arg1],
            2 => memory[res] = memory[arg0] * memory[arg1],
            _ => panic!("Unrecognised op: {}", opcode),
        }
        pc += 4;
    }

    println!("{:?}", memory);
}
