use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

const INPUT: &str = include_str!("../../inputs/day23.txt");

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
struct SharedState {
    channels: Vec<Mutex<VecDeque<[Value; 2]>>>,
    output: Mutex<Vec<[Value; 2]>>,
    done: AtomicBool,
}

#[derive(Debug)]
enum OutputState {
    Empty,
    Dest(usize),
    DestX(usize, Value),
}

#[derive(Debug)]
struct IoState {
    index: usize,
    shared: Arc<SharedState>,
    input: Option<Value>,
    out_state: OutputState,
}

impl IoState {
    fn new(shared: Arc<SharedState>, index: usize) -> Self {
        IoState {
            index,
            shared,
            input: Some(index as Value),
            out_state: OutputState::Empty,
        }
    }
}

impl Io for IoState {
    fn input(&mut self) -> Option<Value> {
        Some(if let Some(input) = self.input.take() {
            input
        } else {
            thread::sleep(Duration::from_millis(1));
            if self.shared.done.load(Ordering::Relaxed) {
                return None;
            }
            let mut guard = self.shared.channels[self.index].lock().unwrap();
            if let Some([x, y]) = guard.pop_front() {
                self.input = Some(y);
                x
            } else {
                -1
            }
        })
    }
    fn output(&mut self, value: Value) {
        self.out_state = match self.out_state {
            OutputState::Empty => OutputState::Dest(value as usize),
            OutputState::Dest(dst) => OutputState::DestX(dst, value),
            OutputState::DestX(dst, x) => {
                if dst == 255 {
                    self.shared.done.store(true, Ordering::Relaxed);
                    self.shared.output.lock().unwrap().push([x, value]);
                } else {
                    self.shared.channels[dst].lock().unwrap().push_back([x, value]);
                }
                OutputState::Empty
            },
        };
    }
}

fn main() {
    let memory: Vec<Value> = INPUT.trim().split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let shared_state = Arc::new(SharedState {
        channels: (0..50).map(|_| Mutex::new(VecDeque::new())).collect(),
        output: Mutex::new(Vec::new()),
        done: AtomicBool::new(false),
    });

    let threads: Vec<_> = (0..50).map(|index| {
        let shared_state = shared_state.clone();
        let memory = memory.clone();

        thread::spawn(move || {
            let mut program = Program::new(memory, IoState::new(shared_state, index));
            program.run();
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("{:?}", shared_state.output.lock().unwrap());
}
