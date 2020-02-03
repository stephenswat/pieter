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

pub struct Program {
    initial_state: u64,
    transitions: HashMap<u64, (u64, Operation)>
}

impl Program {
    pub fn new(i: u64, t: HashMap<u64, (u64, Operation)>) -> Program {
        Program {
            initial_state: i,
            transitions: t
        }
    }
}

pub struct Machine {
    stack: Vec<i64>,
    state: u64,
    program: Program
}

impl Machine {
    pub fn new(p: Program) -> Machine {
        Machine { 
            stack: Vec::new(), 
            state: p.initial_state,
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
