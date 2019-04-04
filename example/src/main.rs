extern crate ash_paper;

use std::env;
use std::fs;

fn main() {
    let fname = "poems/factorial.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ash_paper::interpret(&contents);
}
