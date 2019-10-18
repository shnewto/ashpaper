extern crate ashpaper;

use std::fs;

pub fn main() {
    let fname = "ashpaper-bin/poems/lovely-poem.eso";
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ashpaper::program::execute(&contents);
}
