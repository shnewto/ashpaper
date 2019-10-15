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

pub fn execute(program: &String) {

    let lines: Vec<&str> = program.rsplit(|c| c == '\n' || c == '\r').collect();

    for line in lines {
        let instruction = AshPaper::parse(Rule::program, line)
            .unwrap_or_else(|e| panic!("{}", e))
            .next()
            .unwrap();

            println!("{:?}\n\n", instruction);
    }
}
