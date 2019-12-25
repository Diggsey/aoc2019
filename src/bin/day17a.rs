use std::collections::HashMap;
use std::time::Duration;

const INPUT: &str = include_str!("../../inputs/day17.txt");

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

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn apply(self, other: (i64, i64)) -> (i64, i64) {
        match self {
            Direction::North => (other.0, other.1 - 1),
            Direction::South => (other.0, other.1 + 1),
            Direction::West => (other.0 - 1, other.1),
            Direction::East => (other.0 + 1, other.1),
        }
    }
}

impl From<Value> for Direction {
    fn from(other: Value) -> Self {
        match other {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Unknown direction: {}", other),
        }
    }
}

#[derive(Debug)]
struct IoState {
    map: HashMap<(i64, i64), bool>,
    pos: (i64, i64),
    dir: Direction,
    output_pos: (i64, i64),
}

impl IoState {
    fn new() -> Self {
        print!("\n\x1B[s");
        IoState {
            map: HashMap::new(),
            pos: (0, 0),
            output_pos: (0, 0),
            dir: Direction::North,
        }
    }
}

impl IoState {
    fn display(&self) {
        let min_x = self.map.keys().map(|&(x, _)| x).min().unwrap_or(0);
        let max_x = self.map.keys().map(|&(x, _)| x).max().unwrap_or(0);
        let min_y = self.map.keys().map(|&(_, y)| y).min().unwrap_or(0);
        let max_y = self.map.keys().map(|&(_, y)| y).max().unwrap_or(0);

        let mut s = String::new();
        s += "\x1B[u";
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if (x, y) == self.pos {
                    s.push(match self.dir {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::West => '<',
                    });
                } else {
                    if *self.map.get(&(x, y)).unwrap_or(&false) {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
            }
            s.push('\n');
        }
        println!("{}", s);
        std::thread::sleep(Duration::from_millis(1));
    }
    fn calculate_result(&self) -> i64 {
        let min_x = self.map.keys().map(|&(x, _)| x).min().unwrap_or(0)+1;
        let max_x = self.map.keys().map(|&(x, _)| x).max().unwrap_or(0)-1;
        let min_y = self.map.keys().map(|&(_, y)| y).min().unwrap_or(0)+1;
        let max_y = self.map.keys().map(|&(_, y)| y).max().unwrap_or(0)-1;

        let mut result = 0;
        for y in min_y..=max_y {
            'next: for x in min_x..=max_x {
                if self.map[&(x, y)] {
                    for dir in 1..=4 {
                        let dir: Direction = dir.into();
                        let pos = dir.apply((x, y));
                        if !self.map[&pos] {
                            continue 'next;
                        }
                    }
    
                    println!("{}, {}", x, y);
                    result += x*y;
                }
            }
        }
        result
    }
}

impl Io for IoState {
    fn input(&mut self) -> Option<Value> {
        unimplemented!()
    }
    fn output(&mut self, value: Value) {
        match value {
            35 | 94 | 60 | 62 | 118 => {
                self.map.insert(self.output_pos, true);
                if value != 35 {
                    self.pos = self.output_pos;
                    match value {
                        94 => self.dir = Direction::North,
                        60 => self.dir = Direction::West,
                        62 => self.dir = Direction::East,
                        118 => self.dir = Direction::South,
                        _ => unreachable!(),
                    }
                }
                self.output_pos.0 += 1;
            },
            46 => {
                self.map.insert(self.output_pos, false);
                self.output_pos.0 += 1;
            },
            10 => {
                self.output_pos.0 = 0;
                self.output_pos.1 += 1;
            },
            _ => panic!("Unexpected output: {}", value),
        }
    }
}

fn main() {
    let memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut program = Program::new(memory, IoState::new());
    program.run();

    program.io.display();

    println!("{}", program.io.calculate_result());
}
