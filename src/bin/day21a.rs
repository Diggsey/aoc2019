const INPUT: &str = include_str!("../../inputs/day21.txt");

type Value = i64;

trait Io {
    fn input(&mut self) -> Option<Value>;
    fn output(&mut self, value: Value);
}

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
    AdjustRbo,
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
            9 => AdjustRbo,
            other => panic!("Unknown instruction: {}", other),
        };
        *input /= 100;
        res
    }
    fn len(&self) -> usize {
        use OpCode::*;
        match self {
            Add | Mul | LessThan | Equals => 3,
            Input | Output | AdjustRbo => 1,
            JumpIfTrue | JumpIfFalse => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParameterMode {
    Positional,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn decode(input: &mut Value) -> Self {
        use ParameterMode::*;
        let res = match *input % 10 {
            0 => Positional,
            1 => Immediate,
            2 => Relative,
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
struct Program<T: Io> {
    memory: Vec<Value>,
    pc: usize,
    rbo: Value,
    io: T,
}

impl<T: Io> Program<T> {
    fn new(memory: Vec<Value>, io: T) -> Self {
        Program {
            memory,
            pc: 0,
            rbo: 0,
            io,
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
    fn read_memory(&self, addr: Value) -> Value {
        if addr < 0 { panic!("Read from negative address: {}", addr) }
        let addr = addr as usize;
        if addr >= self.memory.len() {
            0
        } else {
            self.memory[addr]
        }
    }
    fn write_memory(&mut self, addr: Value, value: Value) {
        if addr < 0 { panic!("Write to negative address: {}", addr) }
        let addr = addr as usize;
        if addr >= self.memory.len() {
            self.memory.resize(addr+1, 0);
        }
        self.memory[addr] = value;
    }
    fn read_param(&self, param: Parameter) -> Value {
        match param.mode {
            ParameterMode::Positional => self.read_memory(param.value),
            ParameterMode::Immediate => param.value,
            ParameterMode::Relative => self.read_memory(param.value + self.rbo),
        }
    }
    fn write_param(&mut self, param: Parameter, v: Value) {
        match param.mode {
            ParameterMode::Positional => self.write_memory(param.value, v),
            ParameterMode::Immediate => panic!("Cannot write to immedaite parameter"),
            ParameterMode::Relative => self.write_memory(param.value + self.rbo, v),
        }
    }
    fn execute(&mut self, op: Op) -> bool {
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
                if let Some(a) = self.io.input() {
                    self.write_param(op.args[0], a);
                } else {
                    return true;
                }
            },
            OpCode::Output => {
                let a = self.read_param(op.args[0]);
                self.io.output(a);
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
            OpCode::AdjustRbo => {
                let a = self.read_param(op.args[0]);
                self.rbo += a;
            },
        }
        false
    }
    fn run(&mut self) {
        while self.memory[self.pc] != 99 {
            let op = self.decode_op();
            if self.execute(op) {
                break;
            }
        }
    }
}

#[derive(Debug)]
struct IoState {
    input_pos: usize,
    last_output: Value,
}

impl IoState {
    fn new() -> Self {
        IoState {
            input_pos: 0,
            last_output: 0,
        }
    }
}

const PROGRAM_INPUT: &[u8] = b"NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";

impl Io for IoState {
    fn input(&mut self) -> Option<Value> {
        let result = PROGRAM_INPUT[self.input_pos];
        self.input_pos += 1;

        Some(result as Value)
    }
    fn output(&mut self, value: Value) {
        self.last_output = value;
    }
}

fn main() {
    let memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut program = Program::new(memory.clone(), IoState::new());
    program.run();

    println!("{}", program.io.last_output);
}
