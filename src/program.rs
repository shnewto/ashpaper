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
        // println!("{} : {:?}\n\n", i + 1, line);
        // println!("{}", line.as_str());
        // println!("{}", wordsworth::syllable_counter(line.as_str()))
        interpret_line(line);
    }

    ()
}

fn get_register(rule: pest::iterators::Pair<'_, Rule>) -> u8 {
    if let Some(r) = rule.into_inner().next() {
        match r.as_rule() {
            Rule::whitespace => 1,
            _ => 0,
        }
    } else {
        0
    }
}

fn interpret_line(line: pest::iterators::Pair<'_, Rule>) {
    // println!("{:#?}", line);
    let syllables = wordsworth::syllable_counter(line.as_str());

    for instruction in line.into_inner() {
        match instruction.as_rule() {
            Rule::goto => {
                println!(
                    "{} -- if register 0 > {} goto line number indicated by register {}",
                    instruction.as_str(),
                    syllables,
                    get_register(instruction)
                );
                return;
            }
            Rule::negate => {
                println!(
                    "{} -- negate register {}",
                    instruction.as_str(),
                    get_register(instruction)
                );
                return;
            }
            Rule::multiply => {
                println!(
                    "{} -- multiply registers and store in register {}",
                    instruction.as_str(),
                    get_register(instruction)
                );
                return;
            }
            Rule::add => {
                println!(
                    "{} -- add register 0 and 1 in register {}",
                    instruction.as_str(),
                    get_register(instruction)
                );
                return;
            }
            Rule::print => {
                println!("{} -- print as string", instruction.as_str());
                return;
            }
            Rule::printregister => {
                println!("{} -- print contents of registers", instruction.as_str());
                return;
            }
            Rule::pop => {
                println!(
                    "{} -- pop the stack onto register {}",
                    instruction.as_str(),
                    get_register(instruction)
                );
                return;
            }
            Rule::push => {
                println!(
                    "{} -- push contents of register {} onto the stack",
                    instruction.as_str(),
                    get_register(instruction)
                );
                return;
            }
            Rule::storesyllables => {
                println!(
                    "{} -- store {} in register {}",
                    instruction.as_str(),
                    syllables,
                    get_register(instruction)
                );
                return;
            }
            Rule::whitespace => {}
            _ => {
                println!("noop: {}", instruction.as_str());
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
