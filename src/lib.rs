extern crate wordsworth;
use std::default::Default;
use std::fmt;
use std::slice;

impl Memory {
    pub fn new() -> Memory {
        Default::default()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            register_1: Register::Register1(0),
            register_2: Register::Register2(0),
            stack: vec![],
        }
    }
}

#[derive(Default)]
pub struct Input {
    tokens: String,
}

impl Input {
    /// Construct a new `Input`
    fn new() -> Input {
        Input {
            ..Default::default()
        }
    }

    // Get `Input` by parsing a string
    fn from_str(s: &str) -> Input {
        Input {
            tokens: String::from(s),
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.tokens)
    }
}

pub struct Iter<'a> {
    iterator: slice::Iter<'a, Input>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Input;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

pub struct IterMut<'a> {
    iterator: slice::IterMut<'a, Input>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Input;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Program {
    /// Construct a new `Program`
    pub fn new() -> Program {
        Program {
            ..Default::default()
        }
    }

    /// Construct an `Program` from `Input`s
    fn from_parts(v: Vec<Input>) -> Program {
        Program {
            input: v,
            ..Default::default()
        }
    }

    // Get `Program` by parsing a string
    pub fn from_str(s: &str) -> Program {
        Program {
            input: s.lines().map(|l| Input::from_str(l)).collect::<Vec<_>>(),
            ..Default::default()
        }
    }

    /// Get iterator of the `Program`
    pub fn iter(&self) -> Iter {
        Iter {
            iterator: self.input.iter(),
        }
    }

    /// Get mutable iterator of the `Production`
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iterator: self.input.iter_mut(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.input
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    GOTOIF,
    STORE,
    NEGATE,
    MULTIPLY,
    ADD,
    PRINT,
    PRINTR,
    POP,
    PUSH,
    GOTO,
}

enum Register {
    Register1(i64),
    Register2(i64),
}

struct Memory {
    register_1: Register,
    register_2: Register,
    stack: Vec<i64>,
}

#[derive(Default)]
pub struct Program {
    input: Vec<Input>,
    memory: Memory,
    execution_index: u64,
    output: String,
}

// input file is called instructions!
pub fn interpret(input: &str) -> Program {
    let mut program = Program::from_str(input);
    for input in program.iter_mut() {}

    program
}

fn interpret_line(input: Input) -> Instruction {
    let syllables = wordsworth::syllable_counter(&input.tokens);
    let register = 0;

    Instruction::STORE
}

// fn execute_line(line_number: u64) -> Program {
//     Program::new()
// }
