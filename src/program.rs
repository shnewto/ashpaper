use crate::parsers;
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
            register_1: Register::Register1,
            register_2: Register::Register2,
            stack: vec![],
        }
    }
}

type Insctructions = String;

pub struct Iter<'a> {
    iterator: slice::Iter<'a, Insctructions>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Insctructions;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

pub struct IterMut<'a> {
    iterator: slice::IterMut<'a, Insctructions>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Insctructions;

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

    /// Construct an `Program` from `Insctructions`s
    fn from_parts(v: Vec<Insctructions>) -> Program {
        Program {
            instructions: v,
            ..Default::default()
        }
    }

    // Get `Program` by parsing a string
    pub fn from_str(s: &str) -> Program {
        Program {
            instructions: s.lines().map(|i| String::from(i)).collect::<Vec<String>>(),
            ..Default::default()
        }
    }

    /// Get iterator of the `Program`
    pub fn iter(&self) -> Iter {
        Iter {
            iterator: self.instructions.iter(),
        }
    }

    /// Get mutable iterator of the `Production`
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iterator: self.instructions.iter_mut(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.instructions
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone, Debug)]
pub enum Command {
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

pub enum Register {
    Register1,
    Register2,
}

struct Memory {
    register_1: Register,
    register_2: Register,
    stack: Vec<i64>,
}

#[derive(Default)]
pub struct Program {
    instructions: Vec<Insctructions>,
    memory: Memory,
    execution_index: u64,
    output: String,
}

pub fn execute(instructions: &str) -> Program {
    let mut program = Program::from_str(instructions);
    for (i, instruction) in program.iter_mut().enumerate() {
        println!("|{}|", instruction);
        match parsers::register(instruction.as_bytes()) {
            Ok((_, Register::Register1)) => {
                println!("r1:\n{}", instruction);
            }

            Ok((_, Register::Register2)) => {
                println!("r2:\n{}", instruction);
            }
            Err(e) => println!("??:\n{} -- {}", instruction, e),
        }
    }

    program
}

fn execute_line(line: usize, instruction: Insctructions) -> Program {
    let syllables = wordsworth::syllable_counter(&instruction);
    let register = 0;

    Program::new()
}

// fn choose_register(instruction: Insctructions) -> Register {
//     match parsers::starts_with_ws(instruction.tokens.as_bytes()) {
//         Some(_) => Register::Register1,
//         None => Register::Register2,
//     }
// }
// fn execute_line(line_number: u64) -> Program {
//     Program::new()
// }
