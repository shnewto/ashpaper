/// # ashpaper-bin
/// CLI for the [ashpaper crate](https://crates.io/crates/ashpaper), an inpterpreter for Esopo language AshPaper conceived by William Hicks. Now you can run poetry-programs from the command line!
///
/// ## Usage
///
/// Take the following "poegram" called 'lovely-poem.eso' (in this repositories poetry directory):
/// ```ignore
/// lovely poem
///
///   it is a calculator, like a
///       poem, is a poem, and finds
///         factori-
///           als
///   The input is the syllAbles
/// in the title, count them, as one counts
///   (q) what other poem, programs can be writ
///   (a) anything a Turing
///     machine-machine-machine
///     would do
/// re/cur
///     sion works too, in poems, programs, and this
///        a lovely.
/// poem or a calculator or nothing
/// how lovely can it be?
/// ```
///
/// You can run it with:
/// ```ignore
/// ashpaper-bin lovely-poem.eso
/// ```
///
/// And it will produce the output:
/// ```ignore
/// 24
/// ```
extern crate ashpaper;
extern crate clap;
use clap::App;

use std::fs;

pub fn main() {
    let matches = App::new("ashpaper")
        .version("0.1.3")
        .author("Shea Newton <shnewto@gmail.com>")
        .about("An AshPaper interpreter that executes poetry!")
        .args_from_usage("<INPUT>    '.eso file to compile'")
        .get_matches();

    let fname = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(fname).expect("Something went wrong reading input file!");

    if let Ok(res) = ashpaper::program::execute(&contents) {
        print!("{}", res);
    } else {
        // TODO: really need some helpful error reporting
        eprintln!("Error executing program!");
    }
}
