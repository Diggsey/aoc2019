use std::collections::HashMap;
use std::time::Duration;

const INPUT: &str = include_str!("../../inputs/day13.txt");

type Value = i64;

trait Io {
    fn input(&mut self) -> Value;
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
                let a = self.io.input();
                self.write_param(op.args[0], a);
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
    }
    fn run(&mut self) {
        while self.memory[self.pc] != 99 {
            let op = self.decode_op();
            self.execute(op);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum OutState {
    X,
    Y,
    TileId
}

#[derive(Debug)]
struct IoState {
    screen: HashMap<(i64, i64), Value>,
    x: i64,
    y: i64,
    out_state: OutState,
    ball_x: i64,
    paddle_x: i64,
    score: i64,
}

impl IoState {
    fn new() -> Self {
        print!("\n\x1B[s");
        IoState {
            screen: HashMap::new(),
            x: 0,
            y: 0,
            out_state: OutState::X,
            ball_x: 0,
            paddle_x: 0,
            score: 0,
        }
    }
}

impl IoState {
    fn display(&self) {
        let min_x = self.screen.keys().map(|&(x, _)| x).min().unwrap_or(0);
        let max_x = self.screen.keys().map(|&(x, _)| x).max().unwrap_or(0);
        let min_y = self.screen.keys().map(|&(_, y)| y).min().unwrap_or(0);
        let max_y = self.screen.keys().map(|&(_, y)| y).max().unwrap_or(0);

        let mut s = String::new();
        s += "\x1B[u";
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.screen.get(&(x, y)).unwrap_or(&0) {
                    0 => s.push(' '),
                    1 => s.push('\u{2588}'),
                    2 => s.push('#'),
                    3 => s.push('='),
                    4 => s.push('o'),
                    _ => s.push('?'),
                }
            }
            s.push('\n');
        }
        println!("{}\n\n{}", s, self.score);
        std::thread::sleep(Duration::from_millis(5));
    }
}

impl Io for IoState {
    fn input(&mut self) -> Value {
        self.display();
        (self.ball_x - self.paddle_x).signum()
    }
    fn output(&mut self, value: Value) {
        let new_state = match self.out_state {
            OutState::X => { self.x = value; OutState::Y },
            OutState::Y => { self.y = value; OutState::TileId },
            OutState::TileId => {
                if self.x == -1 && self.y == 0 {
                    self.score = value;
                } else {
                    self.screen.insert((self.x, self.y), value);

                    if value == 4 {
                        self.ball_x = self.x;
                    } else if value == 3 {
                        self.paddle_x = self.x;
                    }
                }

                OutState::X
            },
        };
        self.out_state = new_state;
    }
}

fn main() {
    let mut memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    memory[0] = 2;
    
    let mut program = Program::new(memory, IoState::new());
    program.run();

    program.io.display();
}
