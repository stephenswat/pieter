use std::collections::HashMap;

pub enum Operation {
    NoOp,
    Push(i64),
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InNum,
    InChar,
    OutNum,
    OutChar,
    Error(&'static str)
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn rotate(&self, r: Chooser) -> Direction {
        match (self, r) {
            (Direction::Up,    Chooser::Left)  => Direction::Left,
            (Direction::Up,    Chooser::Right) => Direction::Right,
            (Direction::Left,  Chooser::Left)  => Direction::Down,
            (Direction::Left,  Chooser::Right) => Direction::Up,
            (Direction::Down,  Chooser::Left)  => Direction::Right,
            (Direction::Down,  Chooser::Right) => Direction::Left,
            (Direction::Right, Chooser::Left)  => Direction::Up,
            (Direction::Right, Chooser::Right) => Direction::Down
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Chooser {
    Left,
    Right
}

pub type MachineNode = (u32, Direction, Chooser);

pub type Program = HashMap<MachineNode, (MachineNode, Operation)>;

pub struct Machine {
    block: u32,
    stack: Vec<i64>,
    direction: Direction,
    chooser: Chooser,
    program: Program
}

impl Machine {
    pub fn new(p: Program) -> Machine {
        Machine { 
            block: 1, 
            stack: Vec::new(), 
            direction: Direction::Right,
            chooser: Chooser::Left,
            program: p
        }
    }

    fn execute_instruction(&mut self, op: Operation) {
        match op {
            Operation::NoOp => {
            },

            Operation::Push(n) => {
                self.stack.push(n);
            },

            Operation::Pop => {
                self.stack.pop();
            },

            Operation::Add => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(n1), Some(n2)) => { self.stack.push(n1 + n2) },
                    _ => ()
                }
            },

            Operation::Subtract => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(n1), Some(n2)) => { self.stack.push(n2 - n1) },
                    _ => ()
                }
            },

            _ => panic!("Not yet implemented!")
        }
    }
}
