use std::sync::mpsc;
use std::time::Duration;
use std::thread;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day7.txt");

type Value = i64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl OpCode {
    fn decode(input: &mut Value) -> Self {
        use OpCode::*;
        let res = match *input % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            other => panic!("Unknown instruction: {}", other),
        };
        *input /= 100;
        res
    }
    fn len(&self) -> usize {
        use OpCode::*;
        match self {
            Add | Mul | LessThan | Equals => 3,
            Input | Output => 1,
            JumpIfTrue | JumpIfFalse => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParameterMode {
    Positional,
    Immediate,
}

impl ParameterMode {
    fn decode(input: &mut Value) -> Self {
        use ParameterMode::*;
        let res = match *input % 10 {
            0 => Positional,
            1 => Immediate,
            other => panic!("Unknown parameter mode: {}", other),
        };
        *input /= 10;
        res
    }
}

#[derive(Debug, Copy, Clone)]
struct Parameter {
    value: Value,
    mode: ParameterMode,
}

#[derive(Debug, Copy, Clone)]
struct Op {
    code: OpCode,
    args: [Parameter; 4],
}

#[derive(Debug)]
struct Program {
    memory: Vec<Value>,
    pc: usize,
    inputs: mpsc::Receiver<Value>,
    outputs: mpsc::Sender<Value>,
    last_output: Value,
}

impl Program {
    fn new(memory: Vec<Value>, inputs: mpsc::Receiver<Value>, outputs: mpsc::Sender<Value>) -> Self {
        Program {
            memory,
            pc: 0,
            inputs,
            outputs,
            last_output: 0,
        }
    }
    fn read_and_advance(&mut self) -> Value {
        let res = self.memory[self.pc];
        self.pc += 1;
        res
    }
    fn decode_op(&mut self) -> Op {
        let mut value = self.read_and_advance();
        let op_code = OpCode::decode(&mut value);
        let mut op = Op {
            code: op_code,
            args: [Parameter {
                value: 0,
                mode: ParameterMode::Positional,
            }; 4],
        };
        for i in 0..op_code.len() {
            let param = self.read_and_advance();
            let mode = ParameterMode::decode(&mut value);
            op.args[i] = Parameter {
                value: param,
                mode,
            };
        }
        op
    }
    fn read_param(&self, param: Parameter) -> Value {
        match param.mode {
            ParameterMode::Positional => self.memory[param.value as usize],
            ParameterMode::Immediate => param.value,
        }
    }
    fn write_param(&mut self, param: Parameter, v: Value) {
        match param.mode {
            ParameterMode::Positional => self.memory[param.value as usize] = v,
            ParameterMode::Immediate => panic!("Cannot write to immedaite parameter"),
        }
    }
    fn execute(&mut self, op: Op) {
        match op.code {
            OpCode::Add => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                self.write_param(op.args[2], a+b);
            },
            OpCode::Mul => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                self.write_param(op.args[2], a*b);
            },
            OpCode::Input => {
                let a = self.inputs.recv_timeout(Duration::from_secs(1)).unwrap();
                self.write_param(op.args[0], a);
            },
            OpCode::Output => {
                let a = self.read_param(op.args[0]);
                self.last_output = a;
                self.outputs.send(a).ok();
            },
            OpCode::JumpIfTrue => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                if a != 0 {
                    self.pc = b as usize;
                }
            },
            OpCode::JumpIfFalse => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                if a == 0 {
                    self.pc = b as usize;
                }
            },
            OpCode::LessThan => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                self.write_param(op.args[2], if a < b { 1 } else { 0 });
            },
            OpCode::Equals => {
                let a = self.read_param(op.args[0]);
                let b = self.read_param(op.args[1]);
                self.write_param(op.args[2], if a == b { 1 } else { 0 });
            },
        }
    }
    fn run(&mut self) -> Value {
        while self.memory[self.pc] != 99 {
            let op = self.decode_op();
            //println!("{:?}", op);
            self.execute(op);
        }
        self.last_output
    }
}

fn main() {
    let memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    

    let max_signal = (5..10).permutations(5).map(|phases| {
        // Create communication channels
        let mut channels: Vec<_> = (0..5).map(|_| {
            let (tx, rx) = mpsc::channel();
            (Some(tx), Some(rx))
        }).collect();

        // Start amplifiers
        let threads: Vec<_> = phases.iter().enumerate().map(|(i, &phase)| {
            let tx = channels[(i+1) % 5].0.take().unwrap();
            let rx = channels[i].1.take().unwrap();

            // Set up initial inputs
            tx.send(phase).unwrap();
            if i == 4 {
                tx.send(0).unwrap();
            }

            let mut program = Program::new(memory.clone(), rx, tx);
            thread::spawn(move || program.run())
        }).collect();

        // Wait for result
        let res = threads.into_iter()
            .map(|thread| thread.join().unwrap())
            .last()
            .unwrap();
        
        println!("{:?}: {}", phases, res);
        
        res
    }).max();

    println!("{:?}", max_signal);
}
