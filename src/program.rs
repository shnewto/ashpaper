use pest::Parser;
use wordsworth;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AshPaper;

use std::default::Default;
use std::fmt;
use std::slice;

// impl Memory {
//     pub fn new() -> Memory {
//         Default::default()
//     }
// }

// impl Default for Memory {
//     fn default() -> Self {
//         Memory {
//             r1: Register::Register1(0),
//             r2: Register::Register2(0),
//             stack: vec![],
//         }
//     }
// }

// type Insctructions = String;

// pub struct Iter<'a> {
//     iterator: slice::Iter<'a, Insctructions>,
// }

// impl<'a> Iterator for Iter<'a> {
//     type Item = &'a Insctructions;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iterator.next()
//     }
// }

// pub struct IterMut<'a> {
//     iterator: slice::IterMut<'a, Insctructions>,
// }

// impl<'a> Iterator for IterMut<'a> {
//     type Item = &'a mut Insctructions;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iterator.next()
//     }
// }

// impl Program {
//     /// Construct a new `Program`
//     pub fn new() -> Program {
//         Program {
//             ..Default::default()
//         }
//     }

//     /// Construct an `Program` from `Insctructions`s
//     pub fn from_parts(v: Vec<Insctructions>) -> Program {
//         Program {
//             instructions: v,
//             ..Default::default()
//         }
//     }

//     /// Get iterator of the `Program`
//     pub fn iter(&self) -> Iter {
//         Iter {
//             iterator: self.instructions.iter(),
//         }
//     }

//     /// Get mutable iterator of the `Production`
//     pub fn iter_mut(&mut self) -> IterMut {
//         IterMut {
//             iterator: self.instructions.iter_mut(),
//         }
//     }
// }

// impl fmt::Display for Program {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.instructions
//                 .iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<_>>()
//                 .join("")
//         )
//     }
// }

// #[derive(Clone, Debug)]
// pub enum Command {
//     GOTOIF,
//     STORE,
//     NEGATE,
//     MULTIPLY,
//     ADD,
//     PRINT,
//     PRINTR,
//     POP,
//     PUSH,
//     GOTO,
//     NOOP,
// }

// pub struct Memory {
//     pub r1: Register,
//     pub r2: Register,
//     pub stack: Vec<i64>,
// }

// pub enum Register {
//     Register1(i64),
//     Register2(i64),
// }

// #[derive(Default)]
// pub struct Program {
//     instructions: Vec<Insctructions>,
//     _memory: Memory,
//     _execution_index: u64,
//     _output: String,
// }

pub fn execute(instructions: &str) {
    let program = AshPaper::parse(Rule::program, instructions)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    for (i, line) in program.into_inner().enumerate() {
        interpret_line(line);
    }

    ()
}

fn interpret_instruction(rules: pest::iterators::Pair<'_, Rule>, register: usize) {
    let syllables = wordsworth::syllable_counter(rules.as_str());
    let lineclone = rules.clone();
    for instruction in rules.into_inner() {
        match instruction.as_rule() {
            Rule::goto => {
                println!(
                    "{} -- if register 0 > {} goto line number indicated by register {}\n\n",
                    lineclone.as_str(),
                    syllables,
                    register
                );
                return;
            }
            Rule::negate => {
                println!("{} -- negate register {}\n\n", lineclone.as_str(), register);
                return;
            }
            Rule::multiply => {
                println!(
                    "{} -- multiply registers and store in register {}\n\n",
                    lineclone.as_str(),
                    register
                );
                return;
            }
            Rule::add => {
                println!(
                    "{} -- add register 0 and 1 in register {}\n\n",
                    lineclone.as_str(),
                    register
                );
                return;
            }
            Rule::print => {
                println!("{} -- print as string\n\n", lineclone.as_str());
                return;
            }
            Rule::print_register => {
                println!(
                    "{} -- print contents of register {}\n\n",
                    lineclone.as_str(),
                    register
                );
                return;
            }
            Rule::pop => {
                println!(
                    "{} -- pop the stack onto register {}\n\n",
                    lineclone.as_str(),
                    register
                );
                return;
            }
            Rule::push => {
                println!(
                    "{} -- push contents of register {} onto the stack\n\n",
                    lineclone.as_str(),
                    register
                );
                return;
            }
            Rule::store_syllables => {
                println!(
                    "{} -- store_syllables {} in register {}\n\n",
                    lineclone.as_str(),
                    syllables,
                    register
                );
                return;
            }
            _ => {
                interpret_instruction(instruction.clone(), register);
                // println!("noop: {}\n{}\n", lineclone.as_str(), instruction);
            }
        }
    }
}

fn interpret_line(line: pest::iterators::Pair<'_, Rule>) {
    // println!("{:#?}\n\n", line);

    for instruction in line.into_inner() {
        match instruction.as_rule() {
            Rule::register_one => {
                interpret_instruction(instruction.clone(), 1);
            }
            Rule::register_zero => {
                interpret_instruction(instruction.clone(), 0);
            }
            _ => {
                // println!("noop: {}\n{}\n", lineclone.as_str(), instruction);
            }
        }
    }
}

// print = { "?" }
// negate = {
//     whitespace* ~ LETTER  ~ UPPERCASE_LETTER ~ LETTER*
// }
// multiply = { whitespace* ~ UPPERCASE_LETTER ~ LETTER* }
// storesyllables = {
//     whitespace
//     | lowerword
//     | PUNCTUATION
//     | SYMBOL
//     | NUMBER
//     | MARK
// }

// fn _execute_line(_line: usize, _instruction: Insctructions) -> Program {
//     Program::new()
// }
