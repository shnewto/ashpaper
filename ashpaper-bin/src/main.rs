extern crate ashpaper;

#[macro_use]
extern crate clap;
use clap::App;

use std::fs;

pub fn main() {
    let matches = App::new("ashpaper")
        .version("0.1.0")
        .author("Shea Newton <shnewto@gmail.com>")
        .about("An AshPaper interpreter that executes poetry!")
        .args_from_usage("<INPUT>    '.eso file to compile'")
        .get_matches();

    let fname = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");
    ashpaper::program::execute(&contents);
}
