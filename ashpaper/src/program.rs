extern crate pest;

use pest::Parser;
use std::io::{self, BufRead};
use wordsworth;

type Instructions<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AshPaper;

#[derive(Debug, Clone)]
struct Memory {
    register0: usize,
    register1: usize,
    stack: Vec<usize>,
    active: Register,
}

#[derive(Debug, Clone)]
enum Register {
    Register0,
    Register1,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            register0: 0,
            register1: 0,
            stack: vec![],
            active: Register::Register0,
        }
    }

    fn store_syllables(&mut self, syllables: usize) {
        match self.active {
            Register::Register0 => self.register0 = syllables,
            Register::Register1 => self.register1 = syllables,
        }
    }

    fn push(&mut self) {
        match self.active {
            Register::Register0 => self.stack.push(self.register0),
            Register::Register1 => self.stack.push(self.register1),
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            match self.active {
                Register::Register0 => self.register0 = val,
                Register::Register1 => self.register1 = val,
            }
        }
    }

    fn multiply(&mut self) {
        match self.active {
            Register::Register0 => {
                self.register0 = self.register0.wrapping_mul(self.register1);
            }
            Register::Register1 => {
                self.register1 = self.register1.wrapping_mul(self.register0);
            }
        }
    }

    fn add(&mut self) {
        match self.active {
            Register::Register0 => {
                self.register0 = self.register0.wrapping_add(self.register1);
            }
            Register::Register1 => {
                self.register1 = self.register1.wrapping_add(self.register0);
            }
        }
    }

    fn get_active(&self) -> usize {
        match self.active {
            Register::Register0 => self.register0,
            Register::Register1 => self.register1,
        }
    }

    fn get_inactive(&self) -> usize {
        match self.active {
            Register::Register0 => self.register1,
            Register::Register1 => self.register0,
        }
    }

    fn negate(&mut self) {
        match self.active {
            // We can't negate something we can feasibly convert to
            // a character or use to access a position in a vector.
            // For now we're emulating a 'negate' by wrapping around.
            Register::Register0 => {
                self.register0 = self.register0.wrapping_neg();
            }
            Register::Register1 => {
                self.register1 = self.register1.wrapping_neg();
            }
        }
    }
}

// TODO: define actual error types instead of `()`
fn parse(line: &str) -> Result<Instructions, ()> {
    AshPaper::parse(Rule::program, line)
        .map_err(|_| ())? // ignore pest's custom error type
        .next()
        .ok_or(())
}

// TODO: define actual error types instead of `()`
// TODO (maybe?): instead of printing output of execution,
// accumulate into a String which is returned in the Result.
// This would make the output more useable via the API,
// but I haven't read the paper yet so maybe that's a bad idea.
pub fn execute(program: &str) -> Result<(), ()> {
    let cursor = io::Cursor::new(program);
    let lines = cursor.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut mem = Memory::new();

    let instructions = lines
        .iter()
        .map(|line| parse(line))
        .collect::<Result<Vec<Instructions>, _>>()?;
    let mut instruction_pointer: usize = 0;

    'outer: while let Some(instruction) = instructions.get(instruction_pointer) {
        let syllables = wordsworth::syllable_counter(instruction.as_str()) as usize;

        for instruction in instruction.clone().into_inner() {
            match instruction.as_rule() {
                Rule::goto => {
                    if mem.get_active() > syllables {
                        instruction_pointer = mem.get_inactive() % instructions.len();
                        continue 'outer;
                    }
                }
                Rule::negate => {
                    mem.negate();
                }
                Rule::multiply => {
                    mem.multiply();
                }
                Rule::add => {
                    mem.add();
                }
                Rule::print_char => {
                    // :shrug: print a non-existant ascii value?
                    let active = mem.get_active();
                    let truncated = active % std::u8::MAX as usize;
                    let printable = truncated as u8;
                    println!("{}", printable as char);
                }
                Rule::print_value => {
                    println!("{}", mem.get_active());
                }
                Rule::pop => {
                    mem.pop();
                }
                Rule::push => {
                    mem.push();
                }
                Rule::store_syllables => {
                    mem.store_syllables(syllables);
                }
                Rule::noop => {}
                Rule::register0 => {
                    mem.active = Register::Register0;
                }
                Rule::register1 => {
                    mem.active = Register::Register1;
                }
                _ => {}
            }
        }
        instruction_pointer += 1;
    }

    Ok(())
}
