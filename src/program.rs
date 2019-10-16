extern crate pest;
#[macro_use]
use pest::Parser;

use wordsworth;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AshPaper;

use std::default::Default;
use std::fmt;
use std::slice;

pub fn execute(program: &str) {
    let lines: Vec<&str> = program.rsplit(|c| c == '\n' || c == '\r').collect();

    for line in lines.iter().rev() {
        let instruction = AshPaper::parse(Rule::program, line)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap();

        interpret_line(instruction)
    }
}

fn interpret_line(line: pest::iterators::Pair<'_, Rule>) {
    interpret_instruction(line.clone(), 0);
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
            Rule::print_value => {
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
            Rule::noop => {
                println!("{} -- noop\n\n", lineclone.as_str());
                return;
            }
            _ => {
                // println!("who knows??: {}\n{:#?}\n", lineclone.as_str(), instruction);
                interpret_instruction(instruction.clone(), register);
            }
        }
    }
}
