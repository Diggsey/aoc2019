use std::collections::{HashMap, BinaryHeap};
use std::time::Duration;

const INPUT: &str = include_str!("../../inputs/day15.txt");

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    OxygenSystem,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn rev(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
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
    map: HashMap<(i64, i64), Cell>,
    pos: (i64, i64),
    oxygen_pos: (i64, i64),
    stack: Vec<Direction>,
    attempt: Direction,
}

impl IoState {
    fn new() -> Self {
        print!("\n\x1B[s");
        IoState {
            map: HashMap::new(),
            pos: (0, 0),
            oxygen_pos: (0, 0),
            stack: Vec::new(),
            attempt: Direction::North,
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
                    s.push('@');
                } else {
                    match self.map.get(&(x, y)).unwrap_or(&Cell::Unknown) {
                        Cell::Empty => s.push(' '),
                        Cell::Wall => s.push('\u{2588}'),
                        Cell::OxygenSystem => s.push('o'),
                        Cell::Unknown => s.push('?'),
                    }
                }
            }
            s.push('\n');
        }
        println!("{}", s);
        std::thread::sleep(Duration::from_millis(1));
    }
}

impl Io for IoState {
    fn input(&mut self) -> Option<Value> {
        self.display();
        for dir in 1..=4 {
            let dir: Direction = dir.into();
            let new_pos = dir.apply(self.pos);
            if !self.map.contains_key(&new_pos) {
                self.attempt = dir;
                return Some(dir as Value);
            }
        }
        if let Some(back_dir) = self.stack.pop() {
            self.attempt = back_dir;
            Some(back_dir as Value)
        } else {
            None
        }
    }
    fn output(&mut self, value: Value) {
        let new_pos = self.attempt.apply(self.pos);
        let (moved, cell) = match value {
            0 => (false, Cell::Wall),
            1 => (true, Cell::Empty),
            2 => {
                self.oxygen_pos = new_pos;
                (true, Cell::OxygenSystem)
            },
            _ => panic!("Unknown output: {}", value),
        };

        let visited = self.map.insert(new_pos, cell).is_some();
        if moved {
            if !visited {
                self.stack.push(self.attempt.rev());
            }
            self.pos = new_pos;
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

    let map = program.io.map;

    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, program.io.oxygen_pos));
    while let Some((neg_dist, pos)) = queue.pop() {
        visited.insert(pos, -neg_dist);

        for dir in 1..=4 {
            let dir: Direction = dir.into();
            let new_pos = dir.apply(pos);
            if map[&new_pos] != Cell::Wall {
                if !visited.contains_key(&new_pos) {
                    queue.push((neg_dist-1, new_pos));
                }
            }
        }
    }

    let max_dist = visited.values().max().unwrap();
    println!("{}", max_dist);
}
