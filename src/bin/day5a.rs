use std::io::Write;

use scan_fmt::{scan_fmt, scanln_fmt};

const INPUT: &str = include_str!("../../inputs/day5.txt");

type Value = i64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
}

impl OpCode {
    fn decode(input: &mut Value) -> Self {
        use OpCode::*;
        let res = match *input % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            other => panic!("Unknown instruction: {}", other),
        };
        *input /= 100;
        res
    }
    fn len(&self) -> usize {
        use OpCode::*;
        match self {
            Add | Mul => 3,
            Input | Output => 1,
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
}

impl Program {
    fn new(memory: Vec<Value>) -> Self {
        Program {
            memory,
            pc: 0,
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
                print!("> ");
                std::io::stdout().flush().ok();
                let a = scanln_fmt!("{}", Value).unwrap();
                self.write_param(op.args[0], a);
            },
            OpCode::Output => {
                let a = self.read_param(op.args[0]);
                println!("{}", a);
            },
        }
    }
    fn run(&mut self) {
        while self.memory[self.pc] != 99 {
            let op = self.decode_op();
            //println!("{:?}", op);
            self.execute(op);
        }
    }
}

fn main() {
    let memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let mut program = Program::new(memory);
    program.run();
}
