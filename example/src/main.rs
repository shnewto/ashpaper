extern crate ash_paper;

use std::fs;

pub fn main() {
    let fname = "poems/factorial.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ash_paper::program::execute(&contents);
}
