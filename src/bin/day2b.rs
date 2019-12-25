const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    for a in 0..100 {
        for b in 0..100 {
            let mut memory: Vec<usize> = INPUT.trim().split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            
            memory[1] = a;
            memory[2] = b;
            
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

            if memory[0] == 19690720 {
                println!("{}", a*100+b);
                return;
            }
        }
    }
}
