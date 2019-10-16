extern crate ashpaper;

use std::fs;

pub fn main() {
    let fname = "poems/factorial.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ashpaper::program::execute(&contents);
}
