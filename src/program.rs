use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AshPaper;

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
            r1: Register::Register1(0),
            r2: Register::Register2(0),
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
    pub fn from_parts(v: Vec<Insctructions>) -> Program {
        Program {
            instructions: v,
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

pub struct Memory {
    pub r1: Register,
    pub r2: Register,
    pub stack: Vec<i64>,
}

pub enum Register {
    Register1(i64),
    Register2(i64),
}

#[derive(Default)]
pub struct Program {
    instructions: Vec<Insctructions>,
    _memory: Memory,
    _execution_index: u64,
    _output: String,
}

pub fn execute(instructions: &str) -> Program {
    let program = AshPaper::parse(Rule::program, instructions).unwrap_or_else(|e| panic!("{}", e));;

    println!("{:?}", program.as_str());
    // for (i, instruction) in program.iter_mut().enumerate() {
    //     println!("|{}|", instruction);
    //     match parsers::register(instruction.as_bytes()) {
    //         Ok((_, Register::Register1(val))) => {
    //             println!("r1:\n{}", instruction);
    //         }

    //         Ok((_, Register::Register2(val))) => {
    //             println!("r2:\n{}", instruction);
    //         }
    //         Err(e) => println!("??:\n{} -- {}", instruction, e),
    //     }
    // }

    Program::new()
    // program
}

fn _execute_line(_line: usize, _instruction: Insctructions) -> Program {
    // let syllables = wordsworth::syllable_counter(&instruction);
    // let register = 0;

    Program::new()
}
