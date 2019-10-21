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
pub fn execute(program: &str) -> Result<(String), ()> {
    let cursor = io::Cursor::new(program);
    let lines = cursor.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut mem = Memory::new();

    let mut output: String = String::new();

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
                    output = format!("{}{}", output, printable as char);
                }
                Rule::print_value => {
                    output = format!("{}{}", output, mem.get_active());
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

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_instruction_qualifier(rule: Rule, program: &str) {
        let res = parse(program)
            .unwrap()
            .into_inner()
            .nth(0)
            .unwrap()
            .as_rule();
        assert_eq!(res, rule)
    }

    #[test]
    fn noop() {
        check_instruction_qualifier(Rule::noop, "")
    }

    #[test]
    fn comment() {
        check_instruction_qualifier(Rule::comment, ";;")
    }

    #[test]
    fn register_0() {
        check_instruction_qualifier(Rule::register0, "no leading whitespace")
    }

    #[test]
    fn register_1() {
        check_instruction_qualifier(Rule::register1, " leading whitespace")
    }

    fn check_instruction(rule: Rule, program: &str) {
        let res = parse(program)
            .unwrap()
            .into_inner()
            .nth(1)
            .unwrap()
            .as_rule();
        assert_eq!(rule, res)
    }

    fn check_not_instruction(rule: Rule, program: &str) {
        let res = parse(program)
            .unwrap()
            .into_inner()
            .nth(1)
            .unwrap()
            .as_rule();
        assert_ne!(rule, res)
    }

    #[test]
    fn goto() {
        check_instruction(Rule::goto, "/")
    }

    #[test]
    fn negate() {
        check_instruction(Rule::negate, "aB")
    }

    #[test]
    fn multiply() {
        check_instruction(Rule::multiply, "B")
    }

    #[test]
    fn like_add() {
        check_instruction(Rule::add, "like");
        check_instruction(Rule::add, "like at the start");
        check_instruction(Rule::add, "at the end like");
        check_instruction(Rule::add, "word like in the mix");
        check_instruction(Rule::add, "word \"like\" in quotes");

        check_not_instruction(Rule::add, "likes does not count");
        check_not_instruction(Rule::add, "and not this either abdlikedef");
    }

    #[test]
    fn as_add() {
        check_instruction(Rule::add, "as");
        check_instruction(Rule::add, "as at the start");
        check_instruction(Rule::add, "at the end as");
        check_instruction(Rule::add, "word as in the mix");
        check_instruction(Rule::add, "word \"as\" in quotes");

        check_not_instruction(Rule::add, "ass does not count");
        check_not_instruction(Rule::add, "and not this either abdasdef");
    }

    #[test]
    fn print_char() {
        check_instruction(Rule::print_char, "?")
    }

    #[test]
    fn print_value() {
        check_instruction(Rule::print_value, ".")
    }

    #[test]
    fn pop() {
        check_instruction(Rule::pop, ",")
    }

    #[test]
    fn push() {
        check_instruction(Rule::push, "-")
    }

    #[test]
    fn store_syllables() {
        check_instruction(Rule::store_syllables, "12345")
    }
}
