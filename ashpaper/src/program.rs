extern crate pest;
extern crate log;

use pest::Parser;
use std::io::{self, BufRead};
use wordsworth;

type Instructions<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AshPaper;

#[derive(Debug, Clone)]
struct Memory {
    register0: i64,
    register1: i64,
    stack: Vec<i64>,
    active: Register,
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

    fn store_syllables(&mut self, syllables: i64) {
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
            Register::Register0 => self.register0 *= self.register1,
            Register::Register1 => self.register1 *= self.register0,
        }
    }

    fn add(&mut self) {
        match self.active {
            Register::Register0 => self.register0 += self.register1,
            Register::Register1 => self.register1 += self.register0,
        }
    }

    fn get_active(&self) -> i64 {
        match self.active {
            Register::Register0 => self.register0,
            Register::Register1 => self.register1,
        }
    }

    fn get_inactive(&self) -> i64 {
        match self.active {
            Register::Register0 => self.register1,
            Register::Register1 => self.register0,
        }
    }

    fn negate(&mut self) {
        match self.active {
            Register::Register0 => self.register0 = -self.register0,
            Register::Register1 => self.register1 = -self.register1,
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
pub fn execute(program: &str) -> Result<String, ()> {
    let cursor = io::Cursor::new(program);
    let lines = cursor.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut mem = Memory::new();

    let mut output: String = String::new();

    let instructions = lines
        .iter()
        .map(|line| parse(line))
        .collect::<Result<Vec<Instructions>, _>>()?;
    let mut instruction_pointer: usize = 0;

    log::info!("{: <51} | {: ^4} | {: ^4} | {: ^7}", "instruction", "r0", "r1", "stack");
    log::info!("{:-<51} | {:-^4} | {:-^4} | {:-^7}", "", "", "", "");

    'outer: while let Some(instruction) = instructions.get(instruction_pointer) {
        let syllables = i64::from(wordsworth::syllable_counter(instruction.as_str()));

        for instruction in instruction.clone().into_inner() {
            match instruction.as_rule() {
                Rule::goto => {
                    if mem.get_active() > syllables {
                        instruction_pointer = ((mem.get_inactive().abs() as u64)
                            % (instructions.len() as u64))
                            as usize;
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
                    let printable = (mem.get_active().abs() as u64 % u64::from(std::u8::MAX)) as u8;
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
        log::info!("{: <51} | {: ^4} | {: ^4} | {:^?}", instruction.as_str(), mem.register0, mem.register1, mem.stack);

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
        let instruction = "";
        check_instruction_qualifier(Rule::noop, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn register_0() {
        let instruction = "no leading whitespace";
        check_instruction_qualifier(Rule::register0, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn register_1() {
        let instruction = " leading whitespace";
        check_instruction_qualifier(Rule::register1, instruction);
        assert!(execute(instruction).is_ok());
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
        let mut instruction = "/";
        check_instruction(Rule::goto, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " /";
        check_instruction(Rule::goto, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn negate() {
        let mut instruction = "aB";
        check_instruction(Rule::negate, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " aB";
        check_instruction(Rule::negate, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn multiply() {
        let mut instruction = "B";
        check_instruction_qualifier(Rule::register0, instruction);
        check_instruction(Rule::multiply, instruction);
        instruction = " B";
        check_instruction_qualifier(Rule::register1, instruction);
        check_instruction(Rule::multiply, instruction);

        let mut mem = Memory::new();
        mem.active = Register::Register0;
        mem.multiply();
        mem.active = Register::Register1;
        mem.multiply();
    }

    #[test]
    fn like_add() {
        let mut instruction = "like";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " like";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "like at the start";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "at the end like";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "word like in the mix";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "word \"like\" in quotes";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());

        instruction = "blike does not count";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "likes does not count";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "and not this either abdlikedef";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn as_add() {
        let mut instruction = "as";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " as";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "as at the start";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "at the end as";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "word as in the mix";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "word \"as\" in quotes";
        check_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());

        instruction = "has does not count";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "asi does not count";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
        instruction = "and not this either abdasdef";
        check_not_instruction(Rule::add, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn print_char() {
        let mut instruction = "?";
        check_instruction(Rule::print_char, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " ?";
        check_instruction(Rule::print_char, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn print_value() {
        let mut instruction = ".";
        check_instruction(Rule::print_value, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " .";
        check_instruction(Rule::print_value, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn pop() {
        let mut instruction = ",";
        check_instruction(Rule::pop, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " ,";
        check_instruction(Rule::pop, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn push() {
        let mut instruction = "-";
        check_instruction(Rule::push, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " -";
        check_instruction(Rule::push, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn store_syllables() {
        let mut instruction = "12345";
        check_instruction(Rule::store_syllables, instruction);
        assert!(execute(instruction).is_ok());
        instruction = " 12345";
        check_instruction(Rule::store_syllables, instruction);
        assert!(execute(instruction).is_ok());
    }

    #[test]
    fn mem_get_inactive() {
        let mut mem = Memory::new();
        let r0 = 10;
        let r1 = 11;
        mem.active = Register::Register0;
        mem.store_syllables(r0);
        mem.active = Register::Register1;
        mem.store_syllables(r1);

        mem.active = Register::Register0;
        assert_eq!(mem.get_inactive(), r1);
        mem.active = Register::Register1;
        assert_eq!(mem.get_inactive(), r0);
    }

    #[test]
    fn mem_push() {
        let mut mem = Memory::new();
        mem.active = Register::Register0;
        mem.store_syllables(1);
        mem.push();
        assert_eq!(mem.stack, vec![1]);
        mem.active = Register::Register1;
        mem.store_syllables(2);
        mem.push();
        assert_eq!(mem.stack, vec![1, 2]);
    }

    #[test]
    fn factorial() {
        let factorial_program = "

  it is a calculator, like a
      poem, is a poem, and finds
        factori-
          als
  The input is the syllAbles
in the title, count them, as one counts
  (q) what other poem, programs can be writ
  (a) anything a Turing
    machine-machine-machine
    would do
re/cur
    sion works too, in poems, programs, and this
       a lovely.
poem or a calculator or nothing
how lovely can it be?
";
        let four_factorial = format!("lovely poem\n{}", factorial_program);
        let four_factorial_res = "24\n".to_string();
        assert_eq!(execute(&four_factorial), Ok(four_factorial_res));

        let five_factorial = format!("lovely poem and\n{}", factorial_program);
        let five_factorial_res = "120\n".to_string();
        assert_eq!(execute(&five_factorial), Ok(five_factorial_res));
    }
}
